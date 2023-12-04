use std::collections::HashMap;
use std::env;
mod predicer;
mod utilities;
mod input_data;
use hertta::julia_interface;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;
use tokio::time::{self, Duration};
use std::net::SocketAddr;
use std::fs;
use warp::Filter;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::mpsc;
use std::{num::NonZeroUsize, path::PathBuf};
use jlrs::prelude::*;
use predicer::RunPredicer;
use tokio::task;
use jlrs::error::JlrsError;
use tokio::task::JoinHandle;
use std::fmt;


#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the inner String of MyError to the provided formatter
        write!(f, "{}", self.0)
    }
}

impl warp::reject::Reject for MyError {}


fn _print_tuple_vector(vec: &Vec<(String, f64)>) {
    for (s, num) in vec {
        println!("{}: {}", s, num);
    }
}

async fn _make_post_request(url: &str, data: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the request headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token)).unwrap(),
    );

    // Construct the payload as a JSON object
    let payload = json!({
        "title": "REST Call Received",
        "message": format!("data: {}", data),
    });
	
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload) // Use the correct json! macro
        .send()
        .await?;

    // Check the response status
    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

async fn make_post_request_light(url: &str, entity_id: &str, token: &str, brightness: f64) -> Result<(), Box<dyn std::error::Error>> {
    // Construct the request headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token)).unwrap(),
    );

    // Construct the payload as a JSON object
    let payload = json!({
        "entity_id": entity_id,
        "brightness": brightness,
    });
    
	
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&payload) // Use the correct json! macro
        .send()
        .await?;

    // Check the response status
    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}


// Data structure for messaging between Home Assistant UI.
#[derive(Deserialize, Serialize, Debug)]
struct DataHass {
	entity_cat: i32,
	entity_id: String,
	data_type: i32,
	data_unit: String,
	data_str: String,
	data_int: i32,
	data_float: f32,
	data_bool: bool,
	date_time: String,
}

// Configuration options saved into a json file in the addon data directory.
#[derive(Deserialize, Debug)]
struct Options {
	floor_area: i32,
	stories: i32,
	insulation_u_value: f32,
    listen_ip: String,
    port: String,
    hass_token: String,
}

fn init_julia_runtime() -> Result<(AsyncJulia<Tokio>, JoinHandle<Result<(), Box<JlrsError>>>), MyError> {
    unsafe {
        RuntimeBuilder::new()
            .async_runtime::<Tokio>()
            .channel_capacity(NonZeroUsize::new(4).ok_or(MyError("Invalid channel capacity".to_string()))?)
            .start_async::<1>()
            .map_err(|e| MyError(format!("Could not init Julia: {:?}", e)))
    }
}

async fn execute_task(julia: &AsyncJulia<Tokio>) -> Result<(), MyError> {
    let (sender, receiver) = tokio::sync::oneshot::channel();

    julia
        .register_task::<RunPredicer, _>(sender)
        .dispatch_any()
        .await;

    let task_result = receiver.await.map_err(|e| MyError(format!("Channel error: {:?}", e)))?;
    task_result.map_err(|e| MyError(format!("Task execution error: {:?}", e)))
}

async fn send_task_to_runtime(
    julia: &AsyncJulia<Tokio>,
    data: input_data::InputData, // Replace with the actual type of your data
    predicer_dir: String, // Replace with the actual type of your predicer_dir
    sender: tokio::sync::oneshot::Sender<Result<Vec<(String, f64)>, Box<JlrsError>>>,
) -> Result<(), MyError> {

    let dispatch_result = julia
        .task(
            RunPredicer {
                data,
                predicer_dir,
            },
            sender,
        )
        .try_dispatch_any();

    match dispatch_result {
        Ok(()) => Ok(()),
        Err(dispatcher) => {
            // Handle the error or retry
            // For example, you could log the error and return a custom MyError
            Err(MyError("Failed to dispatch task".to_string()))
        }
    }
}

async fn receive_task_result(
    receiver: tokio::sync::oneshot::Receiver<Result<Vec<(String, f64)>, Box<JlrsError>>>,
) -> Result<Vec<(String, f64)>, MyError> {
    match receiver.await {
        Ok(result) => match result {
            Ok(value) => Ok(value),
            Err(e) => Err(MyError(format!("Task execution error: {:?}", e))),
        },
        Err(e) => Err(MyError(format!("Failed to receive from channel: {:?}", e))),
    }
}

async fn send_light_commands(
    url: &str,
    entity_id: &str,
    hass_token: &str,
    brightness_values: &[f64], // Assuming brightness values are u8, adjust as needed
) -> Result<(), MyError> {
    for brightness in brightness_values.iter().take(2) {
        println!("Setting brightness to: {}", brightness);
        if let Err(err) = make_post_request_light(url, entity_id, hass_token, *brightness).await {
            eprintln!("Error in making POST request for brightness {}: {:?}", brightness, err);
            // Decide how to handle the error: return or continue to the next iteration
        } else {
            println!("POST request successful for brightness: {}", brightness);
        }

        // Wait for 2 seconds before sending the next request
        println!("Waiting for 2 seconds before next request...");
        time::sleep(Duration::from_secs(2)).await;
    }
    Ok(())
}

async fn shutdown_julia_runtime(julia: AsyncJulia<Tokio>, handle: JoinHandle<Result<(), Box<JlrsError>>>) -> Result<(), MyError> {
    // Dropping `julia` to shut down the runtime
    std::mem::drop(julia);

    // Await the handle and handle any errors
    match handle.await {
        Ok(Ok(())) => Ok(()), // Both thread execution and task were successful
        Ok(Err(e)) => Err(MyError(format!("Julia task exited with an error: {:?}", e))), // Task returned an error
        Err(e) => Err(MyError(format!("Join handle failed: {:?}", e))), // Thread panicked or similar issue
    }
}

use tokio::sync::Mutex;

async fn run_predicer(
    julia: Arc<Mutex<AsyncJulia<Tokio>>>,
    data: input_data::InputData,
    predicer_dir: String,
) -> Result<Vec<(String, f64)>, MyError> {

    /* 
    let (julia, handle) = match init_julia_runtime() {
        Ok((julia, handle)) => (julia, handle),
        Err(e) => {
            eprintln!("Failed to initialize Julia runtime: {:?}", e);
            return Err(e); 
        }
    };
    */

    let julia_guard = julia.lock().await;

    match execute_task(&*julia_guard).await {
        Ok(()) => println!("Task executed successfully"),
        Err(e) => eprintln!("Task execution failed: {}", e),
    }

    let (sender, receiver) = tokio::sync::oneshot::channel();

    if let Err(e) = send_task_to_runtime(&*julia_guard, data, predicer_dir, sender).await {
        eprintln!("Failed to send task to runtime: {}", e);
        return Err(e);
    }

    let result = match receive_task_result(receiver).await {
        Ok(value) => {
            println!("Results received.");
            value // value is of type Vec<(String, f64)>
        },
        Err(e) => {
            eprintln!("Error receiving task result: {}", e);
            return Err(MyError(format!("Error receiving task result: {:?}", e))); // Updated error handling
        }
    };

    utilities::_print_tuple_vector(&result);

    // Uncomment and update the shutdown logic if needed
    /*
    match shutdown_julia_runtime(julia, handle).await {
        Ok(()) => println!("Julia shutdown succeeded."),
        Err(e) => eprintln!("Error in Julia shutdown: {:?}", e),
    }
    */

    // Return the result
    Ok(result)


}

/* 
async fn change_brightness(julia: AsyncJulia<Tokio>, handle: JoinHandle<Result<(), data: input_data::InputData, predicer_dir: String, hass_token: String, url: &str, entity_id: &str) {
    let results = run_predicer(data, predicer_dir, hass_token.clone()).await;

    match results {
        Ok(data) => utilities::_print_tuple_vector(&data),
        Err(e) => {
            // Handle the error or assign a default value
            eprintln!("Error occurred: {}", e);
        },
    }

    // Rest of the commented code block
    /* 
    let brightness_values: Vec<f64> = match results {
        Ok(data) => data.iter().map(|(_, value)| *value * 20.0).collect(),
        Err(e) => {
            // Handle the error or assign a default value
            eprintln!("Error occurred: {}", e);
            Vec::new() // Return an empty Vec<f64>
        }
    };

    println!("Brightness Values: {:?}", brightness_values);
    
    let light_command = send_light_commands(url, entity_id, &hass_token.clone(), &brightness_values).await;
    
    match light_command {
        Ok(()) => {
            // Process or use the results here
            println!("Light command succesful.");
        },
        Err(e) => {
            eprintln!("Error in light commands: {}", e);
        },
    }
    */
}

*/


use std::sync::Arc;
use tokio::sync::oneshot;

// Import necessary modules and types (make sure they are correctly referenced)
// use jlrs::prelude::{AsyncJulia, JlrsError};
// use your_module::{input_data, run_predicer, MyError, init_julia_runtime};

#[tokio::main]
async fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("First argument should be path to Predicer")
        .to_string();

    // Server configuration
    let listen_ip = "0.0.0.0";
    let port = "8002";
    let ip_port = format!("{}:{}", listen_ip, port);
    let ip_address: std::net::SocketAddr = ip_port.parse().expect("Unable to parse socket address");

    // Initialize the Julia runtime
    let (julia, handle) = match init_julia_runtime() {
        Ok((julia, handle)) => (julia, handle),
        Err(e) => {
            eprintln!("Failed to initialize Julia runtime: {:?}", e);
            return; // Exit the program if runtime couldn't start
        }
    };
    let julia = Arc::new(Mutex::new(julia));

    // Set up an mpsc channel for graceful shutdown
    let (shutdown_sender, mut shutdown_receiver) = mpsc::channel::<()>(1);

    // Define the route for handling POST requests to run the Julia task
    let my_route = {
        let julia = julia.clone();
        let predicer_dir = predicer_dir.clone();
        warp::path!("from_hass" / "post")
            .and(warp::post())
            .and(warp::body::json()) // Assuming you're receiving JSON data
            .map(move |data: input_data::HassData| { // Update the type of 'data' if needed
                // Clone shared resources
                let julia_clone = julia.clone();
                let predicer_dir_clone = predicer_dir.clone();

                let data = input_data::create_data(data.init_temp);

                // Spawn an asynchronous task to run the Julia task
                tokio::spawn(async move {
                    // Call the function to run the Julia task with the provided data
                    match run_predicer(julia_clone, data, predicer_dir_clone).await {
                        Ok(result) => {
                            // Handle the successful result of the Julia task
                            println!("Julia task completed successfully: {:?}", result);
                        }
                        Err(e) => {
                            // Handle any errors that occurred during the Julia task
                            eprintln!("Error running Julia task: {:?}", e);
                        }
                    }
                });

                // Respond to the request
                warp::reply::json(&"Request received, logic is running")
            })
    };

    // Define the route for triggering a graceful shutdown
    let shutdown_route = {
        let shutdown_sender_clone = shutdown_sender.clone();
        warp::path!("shutdown")
            .and(warp::post())
            .map(move || {
                // Send a shutdown signal
                let _ = shutdown_sender_clone.try_send(());
                warp::reply::json(&"Server is shutting down")
            })
    };

    // Combine the routes
    let routes = my_route.or(shutdown_route);

    // Start the Warp server with graceful shutdown
    let server = {
        let (_, server) = warp::serve(routes)
            .bind_with_graceful_shutdown(ip_address, async move {
                shutdown_receiver.recv().await;
            });
        server
    };
    println!("Server started at {}", ip_address);

    // Run the server and listen for Ctrl+C
    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c() => {
            // Trigger shutdown if Ctrl+C is pressed
            let _ = shutdown_sender.send(());
        },
    }

    std::mem::drop(julia);
    handle.await.expect("Julia runtime thread panicked");

    println!("Server has been shut down");
}




#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_julia_runtime_success() {
        let result = init_julia_runtime();
        assert!(result.is_ok());
    }

    /* 

    async fn test_julia_instance_functions() {

        //Init julia
        unsafe {
            RuntimeBuilder::new()
                .async_runtime::<Tokio>()
                .channel_capacity(NonZeroUsize::new(4).ok_or(MyError("Invalid channel capacity".to_string()))?)
                .start_async::<1>()
                .map_err(|e| MyError(format!("Could not init Julia: {:?}", e)));
        }

    }

    */


}


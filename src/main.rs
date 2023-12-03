//use std::collections::HashMap;
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
use serde::Deserialize;
use serde_json;
//use std::error::Error;
//use std::fmt;
//use warp::reject::Reject;
use tokio::task;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use tokio::sync::mpsc;

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

async fn make_post_request_light(url: &str, entity_id: &str, token: &str, brightness: f64, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("{}", token))
            .map_err(|e| format!("Failed to create HeaderValue from token: {}", e))?,
    );

    let payload = json!({
        "entity_id": entity_id,
        "brightness": brightness,
    });

    let response = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    if let Err(err) = response.error_for_status() {
        eprintln!("Error making POST request: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}

async fn _run_logic(hass_token: String, predicer_dir: String, client: &Client, hass_data: input_data::HassData) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Starting logic execution...");

    let data = input_data::create_data(hass_data);
    let vector: Vec<(String, f64)> = predicer::predicer(data, predicer_dir);

    let brightness_values: Vec<f64> = vector.iter().map(|(_, value)| *value * 20.0).collect();

    println!("Results obtained.");

    utilities::print_f64_vector(&brightness_values);

    let url = "http://192.168.1.171:8123/api/services/light/turn_on";
    let entity_id = "light.katto1";
    
    // Assuming brightness_values is a vector of integers
    for brightness in brightness_values.iter().take(1) {
        println!("Setting brightness to: {}", brightness);
        
        // Calculate and print brightness/20
        let power = *brightness as f64 / 20.0;
        println!("Power: {}", power);

        

        if let Err(err) = make_post_request_light(url, entity_id, &hass_token, *brightness, client).await {
            eprintln!("Error in making POST request for brightness {}: {:?}", brightness, err);
        } else {
            println!("POST request successful for brightness: {}", brightness);
        }

        // Wait for 3 seconds before sending the next request
        println!("Waiting for 3 seconds before next request...");
        time::sleep(Duration::from_secs(3)).await;
        
        
    }

    

    println!("Completed.");

    // You can return some confirmation if needed
    Ok(warp::reply::json(&"Logic executed successfully"))
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

#[tokio::main]
async fn main() {

    let client = reqwest::Client::new();

    let args: Vec<String> = env::args().collect();
    let predicer_dir = args
        .get(1)
        .expect("first argument should be path to Predicer").clone();
	
    // Define the path to the options.json file
    //let options_path = "/data/options.json";
    let options_path = "./src/options.json";

    // Read the options.json file as a string
    let options_str = match fs::read_to_string(options_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading options.json: {}", err);
            return;
        }
    };

    // Parse the options JSON string into an Options struct
    let options: Options = match serde_json::from_str(&options_str) {
        Ok(parsed_options) => parsed_options,
        Err(err) => {
            eprintln!("Error parsing options.json: {}", err);
            return;
        }
    };
	
    // Extract option data from the options.json file.
	let _floor_area = &options.floor_area;
	let _stories = &options.stories;
	let _insulation_u_value = &options.insulation_u_value;
    let listen_ip = &options.listen_ip;
    let port = &options.port;
	let hass_token = &options.hass_token;
	
	// Partially mask the hass token for printing.
	let _masked_token = if options.hass_token.len() > 4 {
		let last_part = &options.hass_token[options.hass_token.len() - 4..];
		let masked_part = "*".repeat(options.hass_token.len() - 4);
		format!("{}{}", masked_part, last_part)
	} else {
		// If the token is too short, just print it as is
		options.hass_token.clone()
	}; 
	
    // Combine IP address and port into a single string
    let ip_port = format!("{}:{}", listen_ip, port);

    // Parse the combined string into a SocketAddr
    let ip_address: SocketAddr = ip_port.parse().unwrap();

    let (tx, mut rx) = mpsc::channel::<input_data::HassData>(32);

    let my_route = warp::path!("from_hass" / "post")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |data: input_data::HassData| {
            let tx_clone = tx.clone();
            let _ = tx_clone.try_send(data); // Send the data to the channel
            warp::reply::json(&"Request received, command sent")
        });

    let is_running = Arc::new(Mutex::new(false));
    let hass_token_clone = hass_token.clone();
    let predicer_dir_clone = predicer_dir.clone();
    let client_clone = client.clone();

    tokio::spawn(async move {
        while let Some(data) = rx.recv().await {
            // Scope for the mutex guard
            {
                let mut running = is_running.lock().unwrap();
                if *running {
                    // Logic is already running
                    eprintln!("Logic is already running");
                    continue;
                }
                *running = true;
            } // MutexGuard is dropped here
    
            // Now it's safe to await, as the lock has been released
            if let Err(e) = _run_logic(hass_token_clone.clone(), predicer_dir_clone.clone(), &client_clone, data).await {
                eprintln!("Error running logic: {:?}", e);
            }
    
            // Reset the flag when done
            *is_running.lock().unwrap() = false;
        }
    });  
	
    // Print a message indicating that the server is starting
    
    println!("Server started at {}", ip_address);
    

    // Combine filters and start the warp server
    warp::serve(my_route).run(ip_address).await;
    
    
    
}


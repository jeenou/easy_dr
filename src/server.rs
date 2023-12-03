use warp::{http::StatusCode, Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::task;
use tokio::time::Duration;

#[derive(Deserialize, Serialize)]
struct Data {
    // define your data structure
    pub inside_temp: f64,
}

async fn handle_request(data: Arc<Mutex<Data>>) -> Result<impl Reply, Rejection> {
    let shared_data = data.lock().unwrap();
    // process the request
    Ok(warp::reply::json(&*shared_data))
}

async fn simulation_thread(shared_data: Arc<Mutex<Data>>) {
    loop {
        task::spawn_blocking(move || {
            let mut data = shared_data.lock().unwrap();
            // modify data
            // ... simulation logic here ...
        }).await.unwrap();
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

#[tokio::main]
async fn main() {
    let shared_data = Arc::new(Mutex::new(Data { /* ... */ }));
    let shared_data_clone = Arc::clone(&shared_data);

    tokio::spawn(simulation_thread(shared_data_clone));

    let routes = warp::post()
        .and(warp::path("handle"))
        .and(warp::any().map(move || Arc::clone(&shared_data)))
        .and_then(handle_request);

    warp::serve(routes).run(([127, 0, 0, 1], 8002)).await;
}
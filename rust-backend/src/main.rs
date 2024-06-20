use tokio::join;
use std::process;
mod input;
mod web;


#[tokio::main]
async fn main() {

    // Run the input listener in a separate thread
    let input_handle = tokio::spawn(async {
        input::start_input_listener();
    });

    // Serve web concurrently
    let web_handle = tokio::spawn(async {
        web::serve_web().await
    });

    // Handle the results of both tasks
    let (input_result, web_result) = join!(input_handle, web_handle);

    // Check the result of the input listener task
    if let Err(e) = input_result {
        eprintln!("Failed to run the input listener: {:?}", e);
        process::exit(1);
    }

    // Check the result of the web server task
    if let Err(e) = web_result {
        eprintln!("Web server failed: {:?}", e);
        process::exit(1);
    }
}

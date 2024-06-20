// main.rs
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
mod input;
mod web;

#[tokio::main]
async fn main() {

  // Create a channel for sending input events
  let (input_tx, input_rx) = mpsc::channel::<String>(100);

  // Create a map of connected clients
  let clients = Arc::new(Mutex::new(HashMap::new()));

  // Start a new thread to listen to input events
  // Input event stream is the transmitter
  input::input_event_stream(input_tx); 

  // Start the web server and WebSocket connection
  // Clients is the receiver
  web::serve_web(clients, input_rx).await;
}

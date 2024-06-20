use warp::Filter;
use warp::ws::{WebSocket, Message, Ws};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

type Clients = Arc<Mutex<HashMap<usize, UnboundedSender<Result<Message, warp::Error>>>>>;

fn get_unique_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}


/// Starts the web server and WebSocket connection
pub async fn serve_web(clients: Clients, mut input_rx: Receiver<String>) {

  // Serve the client page
  let client_page = warp::fs::dir("../dist");

  // Clone the clients map for use in the WebSocket route
  let clients_clone = clients.clone();

  // WebSocket route
  let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(move |ws: Ws| {
      let clients = clients_clone.clone();
      ws.on_upgrade(move |socket| handle_connection(socket, clients))
    });


  // BROADCAST INPUT EVENTS TO ALL CLIENTS ======================
  tokio::spawn(async move {
    while let Some(input_event) = input_rx.recv().await {
      
      let clients_guard = clients.lock().unwrap();
      for (id, socket_tx) in clients_guard.iter() {
        if let Err(e) = socket_tx.send(Ok(Message::text(input_event.clone()))) {
            eprintln!("Error sending input event to client {}: {:?}", id, e.to_string());
        } 
        else {
            println!("Sent input event to client {}: {}", id, input_event);
        }
      }
    }
    // This code runs after the loop exits, which only happens if the channel is closed
    println!("Input event channel has been closed.");
  });

  let routes = client_page.or(ws_route);
  println!("Server running on http://127.0.0.1:8080");
  warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}



/// Handles incoming WebSocket connections
async fn handle_connection(websocket: WebSocket, clients: Clients) {

  let (mut sender, mut receiver) = websocket.split();
  let id = get_unique_id(); // Define how to generate unique IDs
  let (socket_tx, mut socket_rx) = tokio::sync::mpsc::unbounded_channel();

  { // Add the client to the map
    let mut clients_guard = clients.lock().unwrap();
    clients_guard.insert(id, socket_tx);
  }

  println!("+ client_{:?} has connected to the WebSocket.", id);

  // Send a message to the client
  let initial_message = Message::text(format!("Hello from the server! Your Id is: {}", id));
  if let Err(e) = sender.send(initial_message).await {
    eprintln!("websocket send error: {:?}", e);
  }




  // KEEP THE CLIENT - SERVER SOCKET CONNECTION ALIVE ======================
  tokio::spawn(async move {
    while let Some(result) = receiver.next().await {
      match result {
        Ok(msg) => {
          println!("Received a message from client_{:?}: {:?}", id, msg);
        }
        Err(e) => {
          eprintln!("Error receiving WS message for client_{}: {:?}", id, e);
          break;
        }
      }
    }
    {
      let mut clients_guard = clients.lock().unwrap();
      clients_guard.remove(&id);
      println!("- Client_{:?} has disconnected.", id);
    }
  });


  // KEEP THE SOCKET - INPUTSTREAM CONNECTION ALIVE ======================
  tokio::spawn(async move {
    while let Some(input) = socket_rx.recv().await {
      match input {
        Ok(msg) => {
          println!("Received from socket_rx-{:?} : {:?}", id, msg);
  
          if let Err(e) = sender.send(msg).await {
            eprintln!("Error receiving from socket_rx-{:?} : {:?}", id, e);
            break;
          }
        }
        Err(e) => {
          eprintln!("Error receiving message from client_{:?}: {:?}", id, e);
          break;
        }
      }
    }
  });
}
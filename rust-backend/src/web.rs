use warp::Filter;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use warp::ws::{WebSocket, Message, Ws};

/// Function to setup web server and websocket
pub async fn serve_web() {
  // Serve static files from the dist directory
  let client_page = warp::fs::dir("../dist");

  // Setup WebSocket route
  let ws_route = warp::path("ws")
    .and(warp::ws())
    .map(|ws: Ws| {
        ws.on_upgrade(handle_connection)
    });

  let routes = client_page.or(ws_route);
  println!("Server running on http://127.0.0.1:8080");
  warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

/// Handles incoming WebSocket connections
async fn handle_connection(websocket: WebSocket) {
  println!("A client has connected to the WebSocket.");
  
  let (mut sender, mut receiver) = websocket.split();

  // Send a message to the client
  let initial_message = Message::text("Hello from the server!");
  if let Err(e) = sender.send(initial_message).await {
    eprintln!("websocket send error: {:?}", e);
  }

  while let Some(result) = receiver.next().await {
    match result {
      Ok(msg) => {
        if msg.is_text() {
          sender.send(Message::text("Echo: ".to_string() + msg.to_str().unwrap())).await.unwrap();
        }
      },
      Err(e) => eprintln!("websocket error: {:?}", e),
    }
  }
}
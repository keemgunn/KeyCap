use warp::{Filter, ws::{WebSocket, Message, Ws}};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio::sync::mpsc::{self, Receiver, UnboundedSender};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// Type alias for a thread-safe client map.
type Clients = Arc<Mutex<HashMap<usize, UnboundedSender<Message>>>>;

// Helper function to generate unique IDs for new clients.
fn get_unique_id() -> usize {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Handles incoming WebSocket connections.
async fn handle_connection(websocket: WebSocket, clients: Clients) {

    // SOCKET : Server <-> Client
    let (mut client_tx, client_rx) = websocket.split();
    let id = get_unique_id();

    // 🧵 MPSC CHANNEL : InputStream -> SocketServer
    let (socket_tx, mut socket_rx) = mpsc::unbounded_channel::<Message>();
    {
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.insert(id, socket_tx);
    }
    println!("+ client_{:?} has connected to the WebSocket.", id);

    // Send a welcome message to the client.
    let welcome_message = Message::text(format!("Hello from the server! Your ID is: {}", id));
    if let Err(e) = client_tx.send(welcome_message).await {
        eprintln!("Error sending welcome message to client_{}: {:?}", id, e);
    }

    // 💖 KEEP-ALIVE : (Socket) Client -> Server
    tokio::spawn(async move {
        let mut client_rx = client_rx;
        while let Some(result) = client_rx.next().await {
            if let Err(e) = result {
                eprintln!("Error receiving WS message from client_{}: {:?}", id, e);
                break;
            }
        }
        // Cleanup client when the WebSocket connection closes.
        let mut clients_guard = clients.lock().unwrap();
        clients_guard.remove(&id);
        println!("- Client_{:?} has disconnected.", id);
    });

    // 💖 KEEP-ALIVE : InputStream -> SocketServer
    tokio::spawn(async move {
        while let Some(message) = socket_rx.recv().await {
            if let Err(e) = client_tx.send(message).await {
                eprintln!("Error sending message to client_{} from input stream: {:?}", id, e);
                break;
            }
        }
    });
}

/// Starts the web server and WebSocket connection.
pub async fn serve_web(clients: Clients, mut input_rx: Receiver<String>, port: u16) {
    // Serve the static client page.
    let client_page = warp::fs::dir("../dist");

    // Setup WebSocket route.
    let clients_clone = clients.clone();
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: Ws| {
            let clients = clients_clone.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, clients))
        });

    // 💎 Broadcast InputStream to all connected clients
    tokio::spawn(async move {
        while let Some(input_event) = input_rx.recv().await {
            let clients_guard = clients.lock().unwrap();
            for (id, client) in clients_guard.iter() {
                let message = Message::text(input_event.clone());
                if let Err(e) = client.send(message) {
                    eprintln!("Error sending input event to client {}: {:?}", id, e);
                }
            }
        }
        println!("Input event channel has been closed.");
    });

    // Start the server.
    let routes = client_page.or(ws_route);
    println!("Server running on http://127.0.0.1:{:?}", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}

use rdev::{listen, EventType};
use tokio::sync::mpsc::Sender;

/// Captures input events and sends them through a provided sender.
pub fn input_event_stream(input_tx: Sender<String>) {

  // Start a new thread to listen to input events
  // [NOTE] On MacOS, you require the listen loop needs to be the primary app (no fork before) and need to have accessibility settings enabled.
  std::thread::spawn(move || {
    if let Err(error) = listen(move |event| {
      // if let EventType::KeyPress(key) = event.event_type {
      //     let message = format!("Key pressed: {:?}", key);
      //     println!("{:?}", message);
      //     // Asynchronous send with Tokio mpsc sender
      //     let _ = input_tx.blocking_send(message);
      // }

      let message = match event.event_type {
        EventType::KeyPress(key) => {
          Some(format!("Key pressed: {:?}", key))
        },
        EventType::KeyRelease(key) => {
          // Some(format!("Key released: {:?}", key))
          None
        },
        EventType::ButtonPress(button) => {
          Some(format!("Button pressed: {:?}", button))
        },
        EventType::MouseMove { x, y } => {
          // Some(format!("Mouse moved to: ({}, {})", x, y))
          None
        },
        EventType::Wheel { delta_x, delta_y } => {
          // Some(format!("Mouse wheel moved by: ({}, {})", delta_x, delta_y))
          None
        },
        _ => None, // Ignore other events
      };

      if let Some(message) = message {
        // println!("{:?}", message);
        // Asynchronous send with Tokio mpsc sender
        let _ = input_tx.blocking_send(message);
      }

    }) {
      println!("Error listening to input events: {:?}", error);
    }
  });
}


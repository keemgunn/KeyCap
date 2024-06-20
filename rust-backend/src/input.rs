use rdev::{listen, EventType};

/// Function to start listening to input events and print them
pub fn start_input_listener() {
  if let Err(error) = listen(|event| {
  // [NOTE] On MacOS, you require the listen loop needs to be the primary app (no fork before) and need to have accessibility settings enabled.
  match event.event_type {
    EventType::KeyPress(key) => println!("Key pressed: {:?}", key),
    // EventType::KeyRelease(key) => println!("Key released: {:?}", key),
    EventType::MouseMove { .. } => (), // Ignore mouse move events
    EventType::Wheel { .. } => (), // Ignore mouse wheel events
    // EventType::ButtonPress(button) => println!("Button pressed: {:?}", button),
    _ => () // Ignore other events
}

  }) {
      println!("Error listening to input events: {:?}", error);
  }
}

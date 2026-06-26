use rdev::{Event, EventType, listen};

fn main() {
  // This will block.
  if let Err(error) = listen(callback) {
    println!("Error: {:?}", error)
  }
}

fn callback(event: Event) {
  if let EventType::KeyPress(..) = event.event_type {
    println!("My callback {:?}", event);
  }
}

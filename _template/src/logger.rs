extern crate pretty_env_logger;

pub fn init_application() {
  pretty_env_logger::init();

  let mut header = String::new();
  let title = String::from("Application is running. Valid log types below.");

  for _token in title.chars() {
    header.push_str("=");
  }

  println!("\n\n");
  println!("{}==", &header);
  println!("\n{}\n", &title);
  trace!("message");
  debug!("message");
  info!("message");
  warn!("message");
  error!("message");
  println!("\n{}==", &header);
   
}
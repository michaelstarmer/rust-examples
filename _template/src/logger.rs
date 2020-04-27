extern crate pretty_env_logger;

/// Initialize logger and display some examples.
pub fn init_application() {
  pretty_env_logger::init();

  let mut header = String::new();
  let title = String::from("Application is running.");

  for _token in title.chars() {
    header.push_str("=");
  }

  println!("\n\n");
  println!("{}==\n", &header);
  trace!("trace!(\"message\")");
  debug!("debug!(\"message\")");
  info!("info!(\"message\")");
  warn!("warn!(\"message\")");
  error!("error!(\"message\")");
  println!("\n{}", &title);
  println!("\n{}==", &header);
   
}
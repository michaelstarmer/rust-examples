extern crate pretty_env_logger;

use std::process::Command;

fn main()
{
  pretty_env_logger::init();
}

/// Initialize logger and display some examples.
pub fn init_application()
{
  pretty_env_logger::init();
}

/// Clears the terminal window.
#[allow(dead_code)]
pub fn clear_terminal()
{
  let output = Command::new("clear").output().unwrap_or_else(|e| {
    panic!("failed to execute process: {}", e)
  });

  println!("{}", String::from_utf8_lossy(&output.stdout));
}

/// Display all available log types and the function name.
#[allow(dead_code)]
pub fn print_log_types()
{
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
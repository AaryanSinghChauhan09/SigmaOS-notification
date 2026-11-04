use sigmaos_notification::{Config, NotificationComponent, Component};
fn main() {
    env_logger::init();
    let mut c = NotificationComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS notification running");
    let _ = c.shutdown();
}

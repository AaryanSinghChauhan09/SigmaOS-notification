use sigmaos_notification::{Config, NotificationCenter, NotificationComponent, Component};

#[test]
fn test_notification_send_aaa() {
    let mut nc = NotificationCenter::new();
    let id = nc.send("System", "Update", "Update completed successfully");
    assert_eq!(id, 1);
    assert_eq!(nc.count(), 1);
}

#[test]
fn test_notification_dismiss_aaa() {
    let mut nc = NotificationCenter::new();
    let id = nc.send("Chat", "Message", "Hello!");
    assert_eq!(nc.count(), 1);
    nc.dismiss(id);
    assert_eq!(nc.count(), 0);
}

#[test]
fn test_notification_dnd_mode_aaa() {
    let mut nc = NotificationCenter::new();
    nc.toggle_dnd();
    assert!(nc.is_dnd());
    nc.send("App", "Alert", "Should be suppressed");
    assert_eq!(nc.count(), 0);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = NotificationComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = NotificationComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-notification");
}

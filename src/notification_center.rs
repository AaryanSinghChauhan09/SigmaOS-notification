use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationUrgency { Low, Normal, Critical }

#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub dismissed: bool,
}
impl Notification {
    pub fn new(id: u32, app: &str, summary: &str, body: &str) -> Self {
        Self { id, app_name: app.to_string(), summary: summary.to_string(), body: body.to_string(), urgency: NotificationUrgency::Normal, dismissed: false }
    }
}

pub struct NotificationCenter {
    notifications: VecDeque<Notification>,
    do_not_disturb: bool,
    next_id: u32,
}
impl NotificationCenter {
    pub fn new() -> Self { Self { notifications: VecDeque::new(), do_not_disturb: false, next_id: 1 } }
    pub fn send(&mut self, app: &str, summary: &str, body: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        if !self.do_not_disturb {
            self.notifications.push_back(Notification::new(id, app, summary, body));
        }
        id
    }
    pub fn dismiss(&mut self, id: u32) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.dismissed = true;
        }
    }
    pub fn clear_all(&mut self) { self.notifications.clear(); }
    pub fn toggle_dnd(&mut self) { self.do_not_disturb = !self.do_not_disturb; }
    pub fn count(&self) -> usize { self.notifications.iter().filter(|n| !n.dismissed).count() }
    pub fn is_dnd(&self) -> bool { self.do_not_disturb }
}
impl Default for NotificationCenter { fn default() -> Self { Self::new() } }

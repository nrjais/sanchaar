use tokio::sync::oneshot::Sender;

pub fn fmt_duration(d: std::time::Duration) -> String {
    let millis = d.as_millis();

    let mut duration = String::new();
    if millis > 1000 {
        duration.push_str(&format!("{}s ", millis / 1000));
    }
    let millis = millis % 1000;
    if millis > 0 {
        duration.push_str(&format!("{}ms", millis));
    }

    duration
}

#[derive(Debug)]
pub struct SendOnDrop {
    pub sender: Option<Sender<()>>,
}

impl SendOnDrop {
    pub fn new() -> Self {
        Self { sender: None }
    }

    pub fn from(sender: Sender<()>) -> Self {
        Self {
            sender: Some(sender),
        }
    }

    pub fn with(&mut self, sender: Sender<()>) {
        self.cancel();
        self.sender = Some(sender);
    }

    pub fn cancel(&mut self) {
        let sender = self.sender.take();
        if let Some(sender) = sender {
            let _ = sender.send(());
        }
    }
}

impl Drop for SendOnDrop {
    fn drop(&mut self) {
        self.cancel();
    }
}

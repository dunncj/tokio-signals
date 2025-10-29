use tokio::sync::watch;

// Simple implementation of a Signal using Tokio's watch channel
pub struct Signal<T: Clone + Send + Sync + 'static> {
    tx: watch::Sender<T>,
    rx: watch::Receiver<T>,
}

impl<T> Signal<T>
where
    T: Clone + Send + Sync + 'static,
{
    //Create a new Signal with an initial value
    pub fn new(value: T) -> Self {
        let (tx, rx) = watch::channel(value);
        Self { tx, rx }
    }

    /// Set the value of the Signal
    pub fn set(&self, value: T) {
        self.tx.send(value).unwrap();
    }

    /// Get the current value of the Signal
    pub fn get(&self) -> T {
        self.rx.borrow().clone()
    }

    /// Subscribe to changes of the Signal
    pub fn subscribe<F>(&self, f: F)
    where
        F: Fn(T) + Send + 'static,
    {
        let mut rx = self.rx.clone();
        tokio::spawn(async move {
            while rx.changed().await.is_ok() {
                f(rx.borrow().clone());
            }
        });
    }
}
use tokio::sync::oneshot::{self, Receiver};

pub fn chain_listener_to_receiver<T>(receiver: Receiver<T>) -> (Receiver<T>, Receiver<()>)
where
    T: Send + 'static,
{
    let (tx_notify, rx_notify) = oneshot::channel::<()>();
    let (tx, rx) = oneshot::channel::<T>();

    tokio::spawn(async move {
        let response = receiver.await;

        if let Ok(message) = response {
            let _ = tx.send(message);
            let _ = tx_notify.send(());
        }
    });

    (rx, rx_notify)
}

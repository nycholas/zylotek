use tokio::sync::broadcast;

use crate::event_bus::{EventHandler, EventPublishError, EventPublisher};

pub struct TokioEventPublisher<E> {
    sender: broadcast::Sender<E>,
}

impl<E> TokioEventPublisher<E>
where
    E: Clone + Send + Sync + 'static,
{
    pub fn new(buffer: usize) -> Self {
        let (sender, _) = broadcast::channel(buffer);

        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<E> {
        self.sender.subscribe()
    }
}

#[async_trait::async_trait]
impl<E> EventPublisher<E> for TokioEventPublisher<E>
where
    E: Clone + Send + Sync + 'static,
{
    async fn publish(&self, event: E) -> Result<(), EventPublishError> {
        self.sender
            .send(event)
            .map(|_| ())
            .map_err(|_| EventPublishError::NoSubscribers)
    }
}

pub async fn run_event_handler<E, H>(mut receiver: broadcast::Receiver<E>, handler: H)
where
    E: Clone + Send + Sync + 'static,
    H: EventHandler<E>,
{
    loop {
        match receiver.recv().await {
            Ok(event) => {
                if let Err(error) = handler.handle(event).await {
                    tracing::error!(?error, "event handler failed");
                }
            }

            Err(broadcast::error::RecvError::Lagged(count)) => {
                tracing::error!(count, "event handler lagged");
            }

            Err(broadcast::error::RecvError::Closed) => {
                tracing::info!("event channel closed");
                break;
            }
        }
    }
}

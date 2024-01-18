use std::ops::FnOnce;

use tokio::sync::oneshot;

pub type CommandClosureType<ServiceInstance> = Box<
    dyn FnOnce(ServiceInstance) -> Result<ServiceInstance, ErrAtomic<ServiceInstance>>
        + Send
        + Sync,
>;

pub struct Command<ServiceInstance, Resp> {
    pub command: CommandClosureType<ServiceInstance>,
    pub response: Option<oneshot::Receiver<Resp>>,
}

impl<ServiceInstance, Resp> Command<ServiceInstance, Resp> {
    pub fn from<F>(command: F) -> Command<ServiceInstance, Resp>
    where
        F: FnOnce(ServiceInstance) -> Result<ServiceInstance, ErrAtomic<ServiceInstance>>
            + Send
            + Sync
            + 'static,
    {
        Command {
            command: Box::new(command),
            response: None,
        }
    }

    pub fn with_response(
        self,
        response: oneshot::Receiver<Resp>,
    ) -> Command<ServiceInstance, Resp> {
        Self {
            command: self.command,
            response: Some(response),
        }
    }
}

// Result and errors
// IDEIA IMPROVEMENT: Maybe you could call for something with term "atomic" or "transacional"
pub struct ErrAtomic<Snapshot>
where
    Snapshot: Sized,
{
    pub snapshot: Snapshot,
    pub error_message: String,
}


use tokio::sync::oneshot;

pub type Functor<ServiceInstance> =
    Box<dyn FnOnce(ServiceInstance) -> ServiceInstance + Send + Sync>;

pub struct Command<ServiceInstance, Response> {
    pub command_fn: Functor<ServiceInstance>,
    pub response: Option<oneshot::Receiver<Response>>,
}

impl<ServiceInstance, Response> Command<ServiceInstance, Response> {
    pub fn from<F>(closure: F) -> Command<ServiceInstance, Response>
    where
        F: FnOnce(ServiceInstance) -> ServiceInstance + Send + Sync + 'static,
    {
        Command {
            command_fn: Box::new(closure),
            response: None,
        }
    }

    pub fn with_response(
        self,
        response: oneshot::Receiver<Response>,
    ) -> Command<ServiceInstance, Response> {
        Self {
            command_fn: self.command_fn,
            response: Some(response),
        }
    }
}

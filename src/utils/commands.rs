use std::ops::FnOnce;

pub type CommandClosureType<ServiceInstance> = Box<
    dyn FnOnce(ServiceInstance) -> Result<ServiceInstance, ErrAtomic<ServiceInstance>>
        + Send
        + Sync,
>;

// Result and errors
// IDEIA IMPROVEMENT: Maybe you could call for something with term "atomic" or "transacional"
pub struct ErrAtomic<Snapshot>
where
    Snapshot: Sized,
{
    snapshot: Snapshot,
}

// pub type ResultCommandService<T, Service> = Result<T, ErrCommandService<Service>>;

// pub fn from<Facade, Param, Return>(cl: impl FnOnce(Param) -> Return) -> Command<Facade> {
//     todo!()
// }

pub struct CommandsUtils;
impl CommandsUtils {
    pub fn chain<ServiceInstance>(
        iters_commands: impl IntoIterator<Item = CommandClosureType<ServiceInstance>>
            + Send
            + Sync
            + 'static,
    ) -> CommandClosureType<ServiceInstance> {
        Box::new(|service| {
            iters_commands
                .into_iter()
                .try_fold(service, |acc, command| command(acc))
        })
    }
}

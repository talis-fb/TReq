use std::ops::FnOnce;

pub type CommandClosureType<Facade> =
    Box<dyn FnOnce(Facade) -> Result<Facade, ErrAtomic<Facade>> + Send + Sync>;

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
    pub fn from<Facade>(
        func: impl FnOnce(Facade) -> Result<Facade, ErrAtomic<Facade>> + Send + Sync + 'static,
    ) -> CommandClosureType<Facade> {
        Box::new(func)
    }

    pub fn chain<Facade>(
        iters_commands: impl IntoIterator<Item = CommandClosureType<Facade>> + Send + Sync + 'static,
    ) -> CommandClosureType<Facade> {
        CommandsUtils::from(|service| {
            iters_commands
                .into_iter()
                .try_fold(service, |acc, command| command(acc))
        })
    }
}

use std::ops::FnOnce;

pub type Command<Facade> =
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

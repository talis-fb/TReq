use std::ops::FnOnce;

pub type CommandClosure<Facade> =
    dyn FnOnce(Box<Facade>) -> Result<Box<Facade>, String> + Send + Sync;
    
pub type Command<Facade> = Box<CommandClosure<Facade>>;

pub fn from<Facade, Param, Return>(cl: impl FnOnce(Param) -> Return) -> Command<Facade> {
    todo!()
}

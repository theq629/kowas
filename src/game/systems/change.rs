use hecs::{ComponentError, NoSuchEntity};

#[derive(Debug)]
pub struct ChangeOk;

#[derive(Debug)]
pub struct ChangeErr;

pub type ChangeResult = Result<ChangeOk, ChangeErr>;

impl From<ComponentError> for ChangeErr {
    fn from(_: ComponentError) -> Self {
        ChangeErr
    }
}

impl From<NoSuchEntity> for ChangeErr {
    fn from(_: NoSuchEntity) -> Self {
        ChangeErr
    }
}

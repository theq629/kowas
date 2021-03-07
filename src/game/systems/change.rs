use hecs::ComponentError;

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

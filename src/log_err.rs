use std::fmt::Debug;
use log::{error, warn};

pub fn result_error<T, E: Debug>(result: Result<T, E>) {
    if let Err(e) = result {
        error!("{:?}", e);
    }
}

pub fn result_warn<T, E: Debug>(result: Result<T, E>) {
    if let Err(e) = result {
        warn!("{:?}", e);
    }
}

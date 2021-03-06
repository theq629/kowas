use std::process::exit;
use super::super::Quitter;

struct QuitterImpl;

impl Quitter for QuitterImpl {
    fn quit(&self) {
        exit(0);
    }
}

pub fn get_quitter() -> Option<Box<dyn Quitter>> {
    Some(Box::new(QuitterImpl))
}

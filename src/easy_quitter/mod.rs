mod implementation;
pub use implementation::get_quitter;

pub trait Quitter {
    fn quit(&self);
}

use enum_map::Enum;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Liquid {
    Blood,
    Gore
}

impl Liquid {
    pub fn name(self) -> &'static str {
        match self {
            Liquid::Blood => "blood",
            Liquid::Gore => "gore"
        }
    }
}

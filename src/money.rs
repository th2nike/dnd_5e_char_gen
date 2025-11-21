use core::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MoneyType {
    Copper,
    Silver,
    Gold,
}

impl fmt::Display for MoneyType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            MoneyType::Copper => write!(f, "CP"),
            MoneyType::Silver => write!(f, "SP"),
            MoneyType::Gold => write!(f, "GP"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    pub coin_type: MoneyType,
    pub amount: u32,
}

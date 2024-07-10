mod cross_rates;
mod flows;
mod position;

pub use cross_rates::*;
pub use flows::*;
pub use position::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossMarginPublicError {
    FailedToGenerateCross(String),
    CrossPairNotFoundInMatrix(String),
    FailedToFindAssetForCrossCalculate(String),
    FailedToCalculateCross(String),
}
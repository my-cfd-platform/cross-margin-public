mod position;
mod flows;
mod cross_rates;

pub use position::*;
pub use flows::*;
pub use cross_rates::*;

pub enum CrossMarginPublicError{
    FailedToGenerateCross(String),
    CrossPairNotFoundInMatrix(String),
    FailedToFindAssetForCrossCalculate(String),
    FailedToCalculateCross(String),
}
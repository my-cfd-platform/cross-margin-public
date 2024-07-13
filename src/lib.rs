mod cross_rates;
mod flows;
mod position;

use cross_calculations::core::CrossCalculationsError;
pub use cross_rates::*;
pub use flows::*;
pub use position::*;
use serde::{Deserialize, Serialize};


pub use cross_calculations::core::CrossCalculationsPriceSource;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossMarginPublicError {
    FailedToGenerateCross(String),
    CrossPairNotFoundInMatrix(String),
    FailedToFindAssetForCrossCalculate(String),
    FailedToCalculateCross(String),
    FailedToFindSourceBidAsk(String),
}

impl From<CrossCalculationsError> for CrossMarginPublicError {
    fn from(value: CrossCalculationsError) -> Self {
        match value {
            CrossCalculationsError::FailedToGenerateCross(src) => {
                CrossMarginPublicError::FailedToGenerateCross(src)
            }
            CrossCalculationsError::CrossPairNotFoundInMatrix(src) => {
                CrossMarginPublicError::CrossPairNotFoundInMatrix(src)
            }
            CrossCalculationsError::FailedToFindAssetForCrossCalculate(src) => {
                CrossMarginPublicError::FailedToFindAssetForCrossCalculate(src)
            }
            CrossCalculationsError::FailedToCalculateCross(src) => {
                CrossMarginPublicError::FailedToCalculateCross(src)
            }
            CrossCalculationsError::FailedToFindSourceBidAsk(src) => {
                CrossMarginPublicError::FailedToFindSourceBidAsk(src)
            }
        }
    }
}

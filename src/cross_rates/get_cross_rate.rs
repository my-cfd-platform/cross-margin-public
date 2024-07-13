use cross_calculations::core::CrossCalculationsPriceSource;

use crate::{CrossMarginCrossRatesMatrix, CrossMarginPublicError};

pub struct CrossRate {
    pub bid: f64,
    pub ask: f64,
}

pub fn get_cross_rate(
    cross_matrix: &CrossMarginCrossRatesMatrix,
    price_src: &impl CrossCalculationsPriceSource,
    base: &str,
    quote: &str,
) -> Result<CrossRate, CrossMarginPublicError> {
    let result = cross_calculations::core::get_cross_rate(
        base,
        quote,
        &cross_matrix.matrix,
        price_src,
        false,
    );

    match result {
        Ok(rate) => Ok(CrossRate {
            bid: rate.bid,
            ask: rate.ask,
        }),
        Err(err) => Err(CrossMarginPublicError::from(err)),
    }
}

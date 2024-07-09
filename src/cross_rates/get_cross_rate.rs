use crate::{
    CrossMarginCrossRatesMatrix, CrossMarginPriceSource, CrossMarginPriceSourceBidAsk,
    CrossMarginPublicError,
};

pub struct CrossRate {
    pub bid: f64,
    pub ask: f64,
}

pub fn get_cross_rate(
    cross_matrix: &CrossMarginCrossRatesMatrix,
    price_src: &impl CrossMarginPriceSource,
    base: &str,
    quote: &str,
) -> Result<CrossRate, CrossMarginPublicError> {
    let target_pair = cross_matrix.get_target_cross(base, quote).ok_or(
        CrossMarginPublicError::CrossPairNotFoundInMatrix(format!("{}{}", base, quote)),
    )?;

    match &target_pair.price {
        crate::CrossMarginCrossPairType::SameSide { left, right } => {
            let left = price_src.get_bid_ask(left);
            let right = price_src.get_bid_ask(right);

            return Ok(CrossRate {
                bid: left.get_bid() * right.get_ask(),
                ask: left.get_ask() * right.get_bid(),
            });
        }
        crate::CrossMarginCrossPairType::DiffSide { left, right } => {
            let (left_bid, left_ask) = match left {
                crate::CrossMarginCrossPairDiffSideType::Direct(src) => {
                    let src_bid_ask = price_src.get_bid_ask(src);

                    (src_bid_ask.get_bid(), src_bid_ask.get_ask())
                }
                crate::CrossMarginCrossPairDiffSideType::Reversed(src) => {
                    let source_bid_ask = price_src.get_bid_ask(src);

                    (
                        1.0 / source_bid_ask.get_ask(),
                        1.0 / source_bid_ask.get_bid(),
                    )
                }
            };

            let (right_bid, right_ask) = match right {
                crate::CrossMarginCrossPairDiffSideType::Direct(src) => {
                    let src_bid_ask = price_src.get_bid_ask(src);

                    (src_bid_ask.get_bid(), src_bid_ask.get_ask())
                }
                crate::CrossMarginCrossPairDiffSideType::Reversed(src) => {
                    let source_bid_ask = price_src.get_bid_ask(src);

                    (
                        1.0 / source_bid_ask.get_ask(),
                        1.0 / source_bid_ask.get_bid(),
                    )
                }
            };

            return Ok(CrossRate {
                bid: left_bid * right_bid,
                ask: left_ask * right_ask,
            });
        }
    }
}

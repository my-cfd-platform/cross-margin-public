use std::collections::HashMap;

use crate::{
    CrossMarginCrossPairDiffSideType, CrossMarginCrossPairType, CrossMarginCrossRatePair,
    CrossMarginPublicError, CrossMarginSourceInstrument,
};

pub struct CrossMarginCrossRatesMatrix {
    pub pairs: HashMap<String, HashMap<String, CrossMarginCrossRatePair>>,
}

impl CrossMarginCrossRatesMatrix {
    pub fn new(
        collaterals: &[&str],
        instruments: &[&CrossMarginSourceInstrument],
    ) -> Result<Self, CrossMarginPublicError> {
        let mut result = HashMap::new();

        for (base, quote) in generate_required_crosses(collaterals, instruments) {
            let pair = find_cross_pair(&base, &quote, instruments)?;

            let base_map = result.entry(pair.base.clone()).or_insert(HashMap::new());
            base_map.entry(pair.quote.clone()).or_insert(pair);
        }

        Ok(Self { pairs: result })
    }

    pub fn get_target_cross(&self, base: &str, quote: &str) -> Option<&CrossMarginCrossRatePair>{
        self.pairs.get(base)?.get(quote)
    }
}

fn generate_required_crosses(
    collaterals: &[&str],
    instruments: &[&CrossMarginSourceInstrument],
) -> Vec<(String, String)> {
    let mut result = vec![];

    for instrument in instruments {
        for collateral in collaterals {
            if instrument.quote != **collateral {
                result.push((instrument.quote.clone(), collateral.to_string()));
            }
        }
    }

    result
}

fn find_cross_pair(
    base: &str,
    quote: &str,
    src: &[&CrossMarginSourceInstrument],
) -> Result<CrossMarginCrossRatePair, CrossMarginPublicError> {
    let base_contains_instruments = src
        .iter()
        .filter(|x| x.base == base || x.quote == base)
        .collect::<Vec<_>>();

    let quote_contains_instruments = src
        .iter()
        .filter(|x| x.base == quote || x.quote == quote)
        .collect::<Vec<_>>();

    for base_pair in &base_contains_instruments {
        for quote_pair in quote_contains_instruments.iter() {
            let to_check = [base_pair.base.clone(), base_pair.quote.clone()];
            if to_check.contains(&quote_pair.base) || to_check.contains(&quote_pair.quote) {
                let (left, right) = match base_pair.base == base || base_pair.quote == base {
                    true => (base_pair, quote_pair),
                    false => (quote_pair, base_pair),
                };

                let _type: CrossMarginCrossPairType = match base_pair.base == quote_pair.base
                    || base_pair.quote == quote_pair.quote
                {
                    true => CrossMarginCrossPairType::SameSide {
                        left: left.id.clone(),
                        right: right.id.clone(),
                    },
                    false => CrossMarginCrossPairType::DiffSide {
                        left: CrossMarginCrossPairDiffSideType::Direct(left.id.clone()),
                        right: if left.quote == right.base {
                            CrossMarginCrossPairDiffSideType::Direct(right.id.clone())
                        } else {
                            CrossMarginCrossPairDiffSideType::Reversed(right.id.clone())
                        },
                    },
                };

                return Ok(CrossMarginCrossRatePair {
                    base: base.to_string(),
                    quote: quote.to_string(),
                    price: _type,
                });
            }
        }
    }

    Err(CrossMarginPublicError::FailedToGenerateCross(format!(
        "Failed to find cross for {} - {}",
        base, quote
    )))
}

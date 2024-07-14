use cross_calculations::core::CrossCalculationsCrossPairsMatrix;

use crate::{CrossMarginPublicError, CrossMarginSourceInstrument};

pub struct CrossMarginCrossRatesMatrix {
    pub matrix: CrossCalculationsCrossPairsMatrix,
}

impl CrossMarginCrossRatesMatrix {
    pub fn new(
        collaterals: &[&str],
        instruments: &[&CrossMarginSourceInstrument],
    ) -> Result<Self, Vec<CrossMarginPublicError>> {
        let crosses = generate_required_crosses(collaterals, instruments);

        let matrix = match CrossCalculationsCrossPairsMatrix::new(
            &crosses
                .iter()
                .map(|(b, q)| (b.as_str(), q.as_str()))
                .collect::<Vec<_>>(),
            instruments,
        ) {
            Ok(src) => Ok(src),
            Err(err) => Err(err
                .into_iter()
                .map(CrossMarginPublicError::from)
                .collect::<Vec<_>>()),
        }?;

        Ok(Self { matrix })
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

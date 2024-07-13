use cross_calculations::core::CrossCalculationsSourceInstrument;

#[derive(Debug, Clone)]
pub struct CrossMarginSourceInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
}

impl CrossCalculationsSourceInstrument for CrossMarginSourceInstrument {
    fn get_id(&self) -> &str {
        &self.id
    }

    fn get_base(&self) -> &str {
        &self.base
    }

    fn get_quote(&self) -> &str {
        &self.quote
    }
}

pub struct CrossMarginCrossRatePair {
    pub base: String,
    pub quote: String,
    pub price: CrossMarginCrossPairType,
}

#[derive(Debug, Clone)]
pub enum CrossMarginCrossPairType {
    SameSide {
        left: String,
        right: String,
    },
    DiffSide {
        left: CrossMarginCrossPairDiffSideType,
        right: CrossMarginCrossPairDiffSideType,
    },
}

#[derive(Debug, Clone)]
pub enum CrossMarginCrossPairDiffSideType {
    Direct(String),
    Reversed(String),
}

pub trait CrossMarginPriceSourceBidAsk {
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
}

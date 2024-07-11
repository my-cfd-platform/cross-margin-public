#[derive(Debug, Clone)]
pub struct CrossMarginSourceInstrument {
    pub id: String,
    pub base: String,
    pub quote: String,
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

pub trait CrossMarginPriceSourceBidAsk{
    fn get_bid(&self) -> f64;
    fn get_ask(&self) -> f64;
}

pub trait CrossMarginPriceSource {
    fn get_bid_ask(&self, id: &str) -> Option<impl CrossMarginPriceSourceBidAsk>;
}
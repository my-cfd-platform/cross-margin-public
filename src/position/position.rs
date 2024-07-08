#[derive(Debug, Clone)]
pub enum CrossMarginPositionSide {
    Buy,
    Sell,
}

pub trait CrossMarginActivePosition{
    fn get_side(&self) -> &CrossMarginPositionSide;
    fn get_lots_size(&self) -> f64;
    fn get_lots_amount(&self) -> f64;
    fn get_active_price(&self) -> f64;
    fn get_open_price(&self) -> f64;
    fn get_profit_price(&self) -> f64;
    fn get_gross_pl(&self) -> f64;
}
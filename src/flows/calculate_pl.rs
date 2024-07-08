use crate::{CrossMarginActivePosition, CrossMarginPositionSide};

pub fn calculate_active_position_pl(position: &impl CrossMarginActivePosition) -> f64 {
    let open_side = position.get_open_price()
        * position.get_lots_size()
        * position.get_lots_amount();

    let close_side = position.get_active_price()
        * position.get_lots_size()
        * position.get_lots_amount();

    let pl = match position.get_side() {
        &CrossMarginPositionSide::Buy => position.get_active_price() - open_side,
        &CrossMarginPositionSide::Sell => open_side - close_side,
    } * position.get_profit_price();

    return pl;
}
use crate::CrossMarginActivePosition;

pub fn calculate_account_equity(
    account_balance: f64,
    active_positions: &[&impl CrossMarginActivePosition],
) -> f64 {
    return account_balance
        + active_positions
            .iter()
            .map(|x| x.get_gross_pl())
            .sum::<f64>();
}

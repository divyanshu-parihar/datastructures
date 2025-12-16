/// Max Profit with At Most Two Transactions
/// 
/// Returns the maximum profit.
/// Time Complexity: O(N)
/// Space Complexity: O(N)
pub fn max_profit_twice(prices: &[i32]) -> i32 {
    let n = prices.len();
    if n <= 1 {
        return 0;
    }
    
    let mut profit1 = vec![0; n];
    let mut profit2 = vec![0; n];
    
    // profit1[i] = max profit doing one transaction in 0..=i
    let mut min_price = prices[0];
    for i in 1..n {
        min_price = std::cmp::min(min_price, prices[i]);
        profit1[i] = std::cmp::max(profit1[i-1], prices[i] - min_price);
    }
    
    // profit2[i] = max profit doing one transaction in i..n
    let mut max_price = prices[n-1];
    for i in (0..n-1).rev() {
        max_price = std::cmp::max(max_price, prices[i]);
        profit2[i] = std::cmp::max(profit2[i+1], max_price - prices[i]);
    }
    
    let mut max_prof = 0;
    for i in 0..n {
        max_prof = std::cmp::max(max_prof, profit1[i] + profit2[i]);
    }
    
    max_prof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_twice() {
        let prices = vec![3, 3, 5, 0, 0, 3, 1, 4];
        // Buy at 3, Sell at 5 (Profit 2)
        // Buy at 0, Sell at 3 (Profit 3) -> Wait, Buy at 0, sell at 3?
        // Or Buy at 0, Sell at 4 (Profit 4) -> Total 2+4=6.
        assert_eq!(max_profit_twice(&prices), 6);
    }
    
    #[test]
    fn test_max_profit_twice_basic() {
        let prices = vec![10, 22, 5, 75, 65, 80];
        // Buy 10, Sell 22 (12). Buy 5, Sell 80 (75). Total 87.
        assert_eq!(max_profit_twice(&prices), 87);
    }
}

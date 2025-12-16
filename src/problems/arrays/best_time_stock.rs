/// Best Time to Buy and Sell Stock
/// 
/// Returns the maximum profit.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn max_profit(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    
    let mut min_price = prices[0];
    let mut max_prof = 0;
    
    for &price in prices.iter().skip(1) {
        if price < min_price {
            min_price = price;
        } else {
            max_prof = std::cmp::max(max_prof, price - min_price);
        }
    }
    
    max_prof
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_basic() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        // Buy at 1, sell at 6 => 5
        assert_eq!(max_profit(&prices), 5);
    }
    
    #[test]
    fn test_max_profit_none() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(&prices), 0);
    }
}

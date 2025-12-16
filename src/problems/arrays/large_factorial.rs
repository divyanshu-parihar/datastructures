/// Factorial of Large Number
/// 
/// Returns the factorial as a vector of digits (most significant first).
/// Time Complexity: O(N log(N!)) roughly.
pub fn factorial(n: u32) -> Vec<u8> {
    let mut res = vec![1];
    
    for x in 2..=n {
        multiply(&mut res, x);
    }
    
    res.reverse();
    res
}

fn multiply(res: &mut Vec<u8>, x: u32) {
    let mut carry = 0;
    for i in 0..res.len() {
        let prod = res[i] as u32 * x + carry;
        res[i] = (prod % 10) as u8;
        carry = prod / 10;
    }
    
    while carry > 0 {
        res.push((carry % 10) as u8);
        carry /= 10;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_small() {
        let res = factorial(5); // 120
        assert_eq!(res, vec![1, 2, 0]);
    }

    #[test]
    fn test_factorial_10() {
        let res = factorial(10); // 3628800
        assert_eq!(res, vec![3, 6, 2, 8, 8, 0, 0]);
    }
}

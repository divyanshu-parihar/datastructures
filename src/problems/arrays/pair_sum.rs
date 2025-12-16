use std::collections::HashMap;

/// Find All Pairs with Given Sum
/// 
/// Returns a vector of pairs (a, b) such that a + b = target.
/// Time Complexity: O(N)
/// Space Complexity: O(N)
pub fn pair_sum(arr: &[i32], target: i32) -> Vec<(i32, i32)> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut res = Vec::new();
    
    for &num in arr {
        let diff = target - num;
        if let Some(&count) = map.get(&diff) {
            for _ in 0..count {
                res.push((diff, num));
            }
        }
        *map.entry(num).or_insert(0) += 1;
    }
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_sum() {
        let arr = vec![1, 5, 7, 1];
        let target = 6;
        // Pairs: (1, 5), (5, 1) -> Actually based on order:
        // 1: map={1:1}
        // 5: diff=1, map has 1. push (1, 5). map={1:1, 5:1}
        // 7: diff=-1. map no. map={..., 7:1}
        // 1: diff=5. map has 5. push (5, 1). map={..., 1:2}
        let res = pair_sum(&arr, target);
        assert_eq!(res.len(), 2);
        assert!(res.contains(&(1, 5)));
        assert!(res.contains(&(5, 1)));
    }
}

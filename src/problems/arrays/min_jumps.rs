/// Minimum Number of Jumps to Reach End
/// 
/// Returns the minimum jumps, or -1 if unreachable.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn min_jumps(arr: &[i32]) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }
    if arr[0] == 0 {
        return -1;
    }

    let mut max_reach = arr[0] as usize;
    let mut step = arr[0] as usize;
    let mut jump = 1;

    for i in 1..n {
        if i == n - 1 {
            return jump;
        }

        max_reach = std::cmp::max(max_reach, i + arr[i] as usize);
        step -= 1;

        if step == 0 {
            jump += 1;
            if i >= max_reach {
                return -1;
            }
            step = max_reach - i;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_jumps() {
        let arr = vec![1, 3, 5, 8, 9, 2, 6, 7, 6, 8, 9];
        // 1 -> 3 -> 8 -> 9 (end) = 3 jumps
        assert_eq!(min_jumps(&arr), 3);
    }
    
    #[test]
    fn test_min_jumps_unreachable() {
        let arr = vec![1, 1, 0, 1];
        // 0 -> 1 -> 2 (0) stuck.
        assert_eq!(min_jumps(&arr), -1);
    }
}

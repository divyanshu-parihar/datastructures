/// Trapping Rain Water
/// 
/// Returns total water trapped.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn trap_rain_water(height: &[i32]) -> i32 {
    let n = height.len();
    if n <= 2 {
        return 0;
    }
    
    let mut left = 0;
    let mut right = n - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut result = 0;
    
    while left < right {
        if height[left] <= height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                result += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                result += right_max - height[right];
            }
            right -= 1;
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_rain_water() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        // Standard example -> 6
        assert_eq!(trap_rain_water(&height), 6);
    }
}

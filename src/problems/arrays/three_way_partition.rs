/// Three Way Partitioning
/// 
/// Partitions array around a range [low_val, high_val].
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn three_way_partition(arr: &mut [i32], low_val: i32, high_val: i32) {
    let mut start = 0;
    let mut end = arr.len() as i32 - 1;
    let mut i = 0;
    
    while i <= end {
        if arr[i as usize] < low_val {
            arr.swap(i as usize, start);
            i += 1;
            start += 1;
        } else if arr[i as usize] > high_val {
            arr.swap(i as usize, end as usize);
            end -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_way_partition() {
        let mut arr = vec![1, 14, 5, 20, 4, 2, 54, 20, 87, 98, 3, 1, 32];
        let low = 10;
        let high = 20;
        three_way_partition(&mut arr, low, high);
        
        let n = arr.len();
        let mut i = 0;
        // Check smaller
        while i < n && arr[i] < low { i += 1; }
        // Check range
        while i < n && arr[i] >= low && arr[i] <= high { i += 1; }
        // Check larger
        while i < n && arr[i] > high { i += 1; }
        
        assert_eq!(i, n);
    }
}

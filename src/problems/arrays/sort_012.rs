/// Sort Array of 0s, 1s, and 2s
/// 
/// Uses Dutch National Flag Algorithm.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn sort_012(arr: &mut [i32]) {
    let mut low = 0;
    let mut mid = 0;
    let mut high = arr.len();

    // Prevent underflow if empty
    if high == 0 { return; }
    high -= 1;

    while mid <= high {
        match arr[mid] {
            0 => {
                arr.swap(low, mid);
                low += 1;
                mid += 1;
            },
            1 => {
                mid += 1;
            },
            2 => {
                arr.swap(mid, high);
                if high == 0 { break; } // prevent underflow
                high -= 1;
            },
            _ => panic!("Array contains values other than 0, 1, 2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_012_basic() {
        let mut arr = vec![0, 2, 1, 2, 0];
        sort_012(&mut arr);
        assert_eq!(arr, vec![0, 0, 1, 2, 2]);
    }

    #[test]
    fn test_sort_012_empty() {
        let mut arr: Vec<i32> = vec![];
        sort_012(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_sort_012_all_same() {
        let mut arr = vec![2, 2, 2];
        sort_012(&mut arr);
        assert_eq!(arr, vec![2, 2, 2]);
    }
}

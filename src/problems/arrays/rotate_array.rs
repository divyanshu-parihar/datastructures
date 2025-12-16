/// Cyclically Rotate Array by One
/// 
/// Moves the last element to the front.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn rotate_by_one<T: Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let last = arr[len - 1].clone();
    for i in (1..len).rev() {
        arr[i] = arr[i - 1].clone();
    }
    arr[0] = last;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_by_one() {
        let mut arr = vec![1, 2, 3, 4, 5];
        rotate_by_one(&mut arr);
        assert_eq!(arr, vec![5, 1, 2, 3, 4]);
    }

    #[test]
    fn test_rotate_single() {
        let mut arr = vec![1];
        rotate_by_one(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}

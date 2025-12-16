/// Merge 2 Sorted Arrays Without Extra Space
/// 
/// Modifies arr1 and arr2 in-place such that arr1 contains the smaller elements
/// and arr2 contains the larger elements, both sorted.
/// Time Complexity: O(N * M) (worst case with this insertion-like approach)
pub fn merge_inplace(arr1: &mut [i32], arr2: &mut [i32]) {
    let n = arr1.len();
    let m = arr2.len();
    
    // We want to swap elements if arr1[i] > arr2[j]
    // But iterating backwards on arr1 and forwards on arr2 is tricky with standard swap logic directly.
    // Standard algorithm:
    // Iterate from end of arr1. If arr1[i] > arr2[0], swap and re-sort arr2.
    // But let's use a simpler logic: compare arr1[i] with arr2[j].
    // Actually, iterate i from 0 to n:
    // If arr1[i] > arr2[0]:
    //   swap(arr1[i], arr2[0])
    //   fix arr2 sortedness (insertion sort step)
    
    for i in 0..n {
        if arr2.is_empty() { break; }
        
        if arr1[i] > arr2[0] {
            std::mem::swap(&mut arr1[i], &mut arr2[0]);
            
            // Fix arr2
            let first = arr2[0];
            let mut k = 1;
            while k < m && arr2[k] < first {
                arr2[k - 1] = arr2[k];
                k += 1;
            }
            arr2[k - 1] = first;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_inplace() {
        let mut arr1 = vec![1, 3, 5, 7];
        let mut arr2 = vec![0, 2, 6, 8, 9];
        merge_inplace(&mut arr1, &mut arr2);
        
        assert_eq!(arr1, vec![0, 1, 2, 3]);
        assert_eq!(arr2, vec![5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_inplace_empty() {
        let mut arr1 = vec![1, 2];
        let mut arr2 = vec![];
        merge_inplace(&mut arr1, &mut arr2);
        assert_eq!(arr1, vec![1, 2]);
        assert_eq!(arr2, vec![]);
    }
}

/// Find Common Elements in 3 Sorted Arrays
/// 
/// Returns the common elements.
/// Time Complexity: O(n1 + n2 + n3)
/// Space Complexity: O(1) (excluding result)
pub fn common_elements(arr1: &[i32], arr2: &[i32], arr3: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut res = Vec::new();
    
    while i < arr1.len() && j < arr2.len() && k < arr3.len() {
        if arr1[i] == arr2[j] && arr2[j] == arr3[k] {
            if res.last() != Some(&arr1[i]) {
                res.push(arr1[i]);
            }
            i += 1;
            j += 1;
            k += 1;
        } else if arr1[i] < arr2[j] {
            i += 1;
        } else if arr2[j] < arr3[k] {
            j += 1;
        } else {
            k += 1;
        }
    }
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_elements() {
        let arr1 = vec![1, 5, 10, 20, 40, 80];
        let arr2 = vec![6, 7, 20, 80, 100];
        let arr3 = vec![3, 4, 15, 20, 30, 70, 80, 120];
        // Common: 20, 80
        assert_eq!(common_elements(&arr1, &arr2, &arr3), vec![20, 80]);
    }
}

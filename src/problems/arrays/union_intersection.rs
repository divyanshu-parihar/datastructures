/// Find Union of Two Sorted Arrays
/// 
/// Returns a new vector containing the union of two sorted arrays.
/// Time Complexity: O(n + m)
/// Space Complexity: O(n + m)
pub fn find_union(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            // Avoid duplicates
            if res.last() != Some(&arr1[i]) {
                res.push(arr1[i]);
            }
            i += 1;
        } else if arr1[i] > arr2[j] {
            if res.last() != Some(&arr2[j]) {
                res.push(arr2[j]);
            }
            j += 1;
        } else {
            if res.last() != Some(&arr1[i]) {
                res.push(arr1[i]);
            }
            i += 1;
            j += 1;
        }
    }

    while i < arr1.len() {
        if res.last() != Some(&arr1[i]) {
            res.push(arr1[i]);
        }
        i += 1;
    }

    while j < arr2.len() {
        if res.last() != Some(&arr2[j]) {
            res.push(arr2[j]);
        }
        j += 1;
    }

    res
}

/// Find Intersection of Two Sorted Arrays
/// 
/// Returns a new vector containing the intersection.
pub fn find_intersection(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut res = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            i += 1;
        } else if arr1[i] > arr2[j] {
            j += 1;
        } else {
            // Check for duplicates in result if needed, but intersection usually implies 
            // unique common elements or all common elements.
            // Standard problem usually asks for unique.
            if res.last() != Some(&arr1[i]) {
                res.push(arr1[i]);
            }
            i += 1;
            j += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union() {
        let arr1 = vec![1, 2, 4, 5, 6];
        let arr2 = vec![2, 3, 5, 7];
        // Union: 1, 2, 3, 4, 5, 6, 7
        assert_eq!(find_union(&arr1, &arr2), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_intersection() {
        let arr1 = vec![1, 2, 4, 5, 6];
        let arr2 = vec![2, 3, 5, 7];
        // Intersection: 2, 5
        assert_eq!(find_intersection(&arr1, &arr2), vec![2, 5]);
    }
}

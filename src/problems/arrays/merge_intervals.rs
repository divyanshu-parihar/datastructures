/// Merge Intervals
/// 
/// Returns a new vector of merged intervals.
/// Time Complexity: O(N log N)
/// Space Complexity: O(N)
pub fn merge_intervals(intervals: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
        return vec![];
    }
    
    // Sort by start time
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    
    let mut res: Vec<Vec<i32>> = Vec::new();
    res.push(intervals[0].clone());
    
    for i in 1..intervals.len() {
        let last_idx = res.len() - 1;
        let current = &intervals[i];
        let last = &mut res[last_idx];
        
        if current[0] <= last[1] {
            // Overlap, merge
            last[1] = std::cmp::max(last[1], current[1]);
        } else {
            // No overlap, push
            res.push(current.clone());
        }
    }
    
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let mut intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let res = merge_intervals(&mut intervals);
        // [1, 6], [8, 10], [15, 18]
        assert_eq!(res, vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    }
    
    #[test]
    fn test_merge_intervals_nested() {
        let mut intervals = vec![vec![1, 4], vec![2, 3]];
        let res = merge_intervals(&mut intervals);
        assert_eq!(res, vec![vec![1, 4]]);
    }
}

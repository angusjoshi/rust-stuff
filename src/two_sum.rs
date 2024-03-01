use std::collections::HashMap;

fn two_sum(nums: &Vec<i32>, target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i == j {
                continue
            }

            if nums[i] + nums[j] == target {
                return Some((i, j));
            }
        }
    }

    None
}

fn two_sum_faster(nums: &Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut complements = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        if complements.contains_key(num) {
            return Some((i, complements[num]))
        }

        complements.insert(target - num, i);
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::two_sum::{two_sum, two_sum_faster};

    #[test]
    fn test_two_sum() {
        let nums = vec![1, 5, 2, 1, 6, 8, 6];
        let target = 9;

        match two_sum(&nums, target) {
            Some((i, j)) => assert_eq!(nums[i] + nums[j], target),
            None => panic!("couldn't find a result!")
        }
    }

    #[test]
    fn test_two_sum_faster() {
        let nums = vec![1, 5, 2, 1, 6, 8, 6];
        let target = 9;

        match two_sum_faster(&nums, target) {
            Some((i, j)) => assert_eq!(nums[i] + nums[j], target),
            None => panic!("couldn't find a result!")
        }
    }
}
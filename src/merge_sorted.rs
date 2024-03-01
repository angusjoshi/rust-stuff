fn merge_sorted(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    result.reserve(a.len() + b.len());

    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    while i < a.len()  {
        result.push(a[i]);
        i += 1;
    }

    while j < b.len()  {
        result.push(b[j]);
        j += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::merge_sorted::merge_sorted;

    #[test]
    fn test_merge_sorted_with_empty_lists() {
        let a = vec![];
        let b = vec![];

        let expected_result: Vec<i32> = vec![];
        let result = merge_sorted(&a, &b);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_merge_sorted() {
        let a = vec![1, 4, 7, 9, 12];
        let b = vec![-1, 4, 18];

        let expected_result = vec![-1, 1, 4, 4, 7, 9, 12, 18];
        let result = merge_sorted(&a, &b);

        assert_eq!(expected_result, result)
    }

}

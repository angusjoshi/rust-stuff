mod sorting {
    fn insertion_sort(v: &mut Vec<i32>) {
        let mut i = 1;

        while i < v.len() {
            let mut j = i;
            while j > 0 && v[j] < v[j - 1] {
                (v[j], v[j - 1]) = (v[j - 1], v[j]);
                j -= 1;
            }

            i += 1;
        }
    }
    #[cfg(test)]
    mod tests {
        use crate::sorting::sorting::insertion_sort;

        #[test]
        fn test_insertion_sort() {
            let mut v = vec![4, 1, -1, 5, 2, 10, 15, 3, 0];

            let expected_result = vec![-1, 0, 1, 2, 3, 4, 5, 10, 15];
            insertion_sort(&mut v);

            assert_eq!(expected_result, v)
        }


    }
}
fn transforms<T, U>(v: Vec<T>, f: fn(&T) -> U) -> Vec<U> {
    v.iter()
        .map(f)
        .collect()
}
#[cfg(test)]
mod tests {
    use crate::transforms::transforms;

    #[test]
    fn test_transforms() {
        let v = vec![3, 1, 2, 67];

        let result = transforms(v, |i: &i32| {
            100 - i
        });
        let expected_result = vec![97, 99, 98, 33];

        assert_eq!(expected_result, result)
    }

}
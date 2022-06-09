pub(crate) fn check_sum_from_str(value: &str) -> usize {
    let sum: usize = value
        .as_bytes()
        .iter()
        .map(|item| {
            let sum: usize = item.clone().into();
            sum
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::check_sum_from_str;

    #[test]
    fn check_sum_from_str_test() {
        let sum = check_sum_from_str("Hello World");
        assert_eq!(1052, sum);

        let sum = check_sum_from_str("Key");
        assert_eq!(297, sum);
    }
}
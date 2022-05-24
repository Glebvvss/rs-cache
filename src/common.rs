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
pub fn same(n: u64) -> u64 {
    n
}

#[cfg(test)]
mod test {
    use super::same;

    #[test]
    fn test_same() {
        let numbers: Vec<u64> = vec![0, 1, 3, 4];
        for (_, number) in numbers.iter().enumerate() {
            assert_eq!(*number, same(*number));
        }
    }
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut result = vec![0; arr.len()];
    let mut index = 0;
    for &i in arr {
        if i != 0 {
            result[index] = i;
            index += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert_eq!(
            actual, expected,
            "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn sample_tests() {
        dotest(
            &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
            &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
        );
        dotest(
            &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
            &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}

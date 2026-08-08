pub fn relu<T>(x: T) -> T
where
    T: Copy + PartialOrd + From<i8>,
{
    if x <= 0.into() { 0.into() } else { x }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        assert_eq!(relu(1), 1);
        assert_eq!(relu(-1), 0);

        assert_eq!(relu(128.3), 128.3);
        assert_eq!(relu(-128.3), 0.);
    }
}

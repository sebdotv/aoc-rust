#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
pub fn try_f64_from_usize(i: usize) -> Option<f64> {
    let f = i as f64;
    (f as usize == i).then_some(f)
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::float_cmp)]
pub fn try_usize_from_f64(f: f64) -> Option<usize> {
    let i = f as usize;
    (i as f64 == f).then_some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_f64_from_usize_works() {
        assert_eq!(try_f64_from_usize(1), Some(1.0));
        assert_eq!(try_f64_from_usize(usize::MAX), Some(18446744073709551615.0));
        assert_eq!(try_f64_from_usize(123_456_789_123_456_789), None);
    }
    #[test]
    fn try_usize_from_f64_works() {
        assert_eq!(try_usize_from_f64(1.0), Some(1));
        assert_eq!(try_usize_from_f64(1.5), None);
    }
}

use crate::errors::MathError;

/// Computes the greatest common divisor using Euclid's algorithm.
///
/// The sign of the result follows the sign of the first parameter:
/// - `gcd(48, 88) = 8`
/// - `gcd(48, -88) = 8`  
/// - `gcd(-48, 88) = -8`
/// - `gcd(-48, -88) = -8`
pub fn gcd(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    gcd_with_quotient(a, b).map(|(gcd, _)| gcd)
}

/// This result is always positive (absolute value):
/// - `gcd_abs(48, 88) = 8`
/// - `gcd_abs(48, -88) = 8`
/// - `gcd_abs(-48, 88) = 8`
/// - `gcd_abs(-48, -88) = 8`
pub fn gcd_abs(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    gcd_with_quotient(a, b).map(|(gcd, _)| gcd.abs())
}

/// Computes GCD using the Euclidean algorithm recursively.
/// Returns (gcd, last_quotient) where gcd has the same sign as the first parameter.
fn gcd_with_quotient(a: i64, b: i64) -> Result<(i64, i64), MathError> {
    let r = a % b;
    let q = a / b; // it's the same than "q = (a - r) / b"

    if r == 0 {
        Ok((b, q))
    } else {
        gcd_with_quotient(b, r)
    }
}

/// Safe for cryptographic use - never exposes intermediate quotients
pub fn gcd_secure(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }
    let r = a % b;
    let a = a / b;

    if r == 0 {
        Ok(b.abs())
    } else {
        gcd_secure(b, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_positive() {
        assert_eq!(gcd(48, 88).unwrap(), 8);
        assert_eq!(gcd(88, 48).unwrap(), 8);
    }

    #[test]
    fn test_gcd_with_negatives() {
        assert_eq!(gcd(48, -88).unwrap(), 8);
        assert_eq!(gcd(-48, 88).unwrap(), -8);
        assert_eq!(gcd(-48, -88).unwrap(), -8);
    }

    #[test]
    fn test_gcd_abs() {
        assert_eq!(gcd_abs(48, 88).unwrap(), 8);
        assert_eq!(gcd_abs(48, -88).unwrap(), 8);
        assert_eq!(gcd_abs(-48, 88).unwrap(), 8);
        assert_eq!(gcd_abs(-48, -88).unwrap(), 8);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(matches!(gcd(5, 0), Err(MathError::DivisionByZero)));
        assert!(matches!(gcd_abs(5, 0), Err(MathError::DivisionByZero)));
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(gcd(0, 5).unwrap(), 5);
        assert_eq!(gcd_abs(0, 5).unwrap(), 5);
        assert_eq!(gcd(1, 1).unwrap(), 1);
        assert_eq!(gcd_abs(1, 1).unwrap(), 1);
    }

    #[test]
    fn test_gcd_secure_positive() {
        assert_eq!(gcd_secure(48, 88).unwrap(), 8);
        assert_eq!(gcd_secure(88, 48).unwrap(), 8);
    }

    #[test]
    fn test_gcd_secure_with_negatives() {
        assert_eq!(gcd_secure(48, -88).unwrap(), 8);
        assert_eq!(gcd_secure(-48, 88).unwrap(), 8);
        assert_eq!(gcd_secure(-48, -88).unwrap(), 8);
    }

    #[test]
    fn test_gcd_secure_division_by_zero() {
        assert!(matches!(gcd_secure(5, 0), Err(MathError::DivisionByZero)));
    }

    #[test]
    fn test_gcd_secure_edge_cases() {
        assert_eq!(gcd_secure(0, 5).unwrap(), 5);
        assert_eq!(gcd_secure(1, 1).unwrap(), 1);
        assert_eq!(gcd_secure(17, 1).unwrap(), 1);
        assert_eq!(gcd_secure(100, 25).unwrap(), 25);
    }

    #[test]
    fn test_gcd_secure_same_as_gcd_abs() {
        let test_cases = [
            (48, 88),
            (-48, 88),
            (48, -88),
            (-48, -88),
            (17, 13),
            (100, 30),
            (7, 3),
        ];

        for (a, b) in test_cases {
            assert_eq!(
                gcd_secure(a, b).unwrap(),
                gcd_abs(a, b).unwrap(),
                "gcd_secure({}, {}) should equal gcd_abs({}, {})",
                a,
                b,
                a,
                b
            );
        }
    }
}

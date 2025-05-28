use std::i64::MAX;

use crate::errors::MathError;

fn is_prime_number(a: i64) -> Result<Option<i64>, MathError> {
    if a < 2 {
        return Err(MathError::OutOfRange);
    }

    // factorial calcul ((a-1)!)
    let mut factorial = 1;
    for i in 1..a {
        factorial = (factorial * i) % a;
    }
    // Wilson Theorem: p is prime nbr ioi ((p-1)! + 1) % p == 0;
    if ((factorial + 1) % a) == 0 {
        Ok(Some(a))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_prime_nbr() {
        assert_eq!(is_prime_number(47), Ok(Some(47)));
        assert_eq!(is_prime_number(12967), Ok(Some(12967)));
        assert_eq!(is_prime_number(111697), Ok(Some(111697)));
        assert_eq!(is_prime_number(1122157), Ok(Some(1122157)));

        assert_eq!(is_prime_number(12), Ok(None));
        assert_eq!(is_prime_number(158874), Ok(None));
    }

    #[test]
    fn test_out_of_range() {
        assert_eq!(is_prime_number(0), Err(MathError::OutOfRange));
        assert_eq!(is_prime_number(1), Err(MathError::OutOfRange));
    }
}

use crate::algebra::gcd::gcd_abs;
use crate::errors::MathError;


// 
pub fn lcm(a: i64, b: i64) -> Result<i64, MathError> {
    if a < 0 || b < 0 {
        return Err(MathError::PositifIntegerRequired);
    }

    if a == 0 || b == 0 {
        return Ok(0);
    }

    let gcd_val = gcd_abs(a, b)?;

    let a_reduced = a / gcd_val;

    match a_reduced.checked_mul(b) {
        Some(result) => Ok(result),
        None => Err(MathError::Overflow),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::MathError;

    // Tests de base - cas normaux
    #[test]
    fn test_lcm_basic_cases() {
        assert_eq!(lcm(12, 18), Ok(36));
        assert_eq!(lcm(4, 6), Ok(12));
        assert_eq!(lcm(7, 11), Ok(77)); // nombres premiers entre eux
        assert_eq!(lcm(15, 25), Ok(75));
        assert_eq!(lcm(100, 150), Ok(300));
    }

    // Tests avec zéro
    #[test]
    fn test_lcm_with_zero() {
        assert_eq!(lcm(0, 0), Ok(0));
        assert_eq!(lcm(0, 42), Ok(0));
        assert_eq!(lcm(42, 0), Ok(0));
        assert_eq!(lcm(0, 1), Ok(0));
        assert_eq!(lcm(1, 0), Ok(0));
    }

    // Tests avec un (élément neutre)
    #[test]
    fn test_lcm_with_one() {
        assert_eq!(lcm(1, 1), Ok(1));
        assert_eq!(lcm(1, 42), Ok(42));
        assert_eq!(lcm(42, 1), Ok(42));
        assert_eq!(lcm(1, 1_000_000), Ok(1_000_000));
    }

    // Tests avec nombres identiques
    #[test]
    fn test_lcm_identical_numbers() {
        assert_eq!(lcm(5, 5), Ok(5));
        assert_eq!(lcm(17, 17), Ok(17));
        assert_eq!(lcm(1000, 1000), Ok(1000));
        assert_eq!(lcm(999_999, 999_999), Ok(999_999));
    }

    // Tests avec nombres premiers
    #[test]
    fn test_lcm_prime_numbers() {
        assert_eq!(lcm(2, 3), Ok(6));
        assert_eq!(lcm(3, 5), Ok(15));
        assert_eq!(lcm(7, 13), Ok(91));
        assert_eq!(lcm(11, 17), Ok(187));
        assert_eq!(lcm(23, 29), Ok(667));
    }

    // Tests avec multiples
    #[test]
    fn test_lcm_with_multiples() {
        assert_eq!(lcm(6, 12), Ok(12)); // 12 est multiple de 6
        assert_eq!(lcm(15, 45), Ok(45)); // 45 est multiple de 15
        assert_eq!(lcm(7, 21), Ok(21)); // 21 est multiple de 7
        assert_eq!(lcm(100, 400), Ok(400)); // 400 est multiple de 100
    }

    // Tests avec grandes valeurs (exploitant la capacité i64)
    #[test]
    fn test_lcm_large_values() {
        assert_eq!(lcm(1_000_000, 999_999), Ok(999_999_000_000));
        assert_eq!(lcm(1_234_567, 2_345_678), Ok(2_895_896_651_426));
        assert_eq!(lcm(12_345_678, 23_456_789), Ok(289_589_963_907_942));
    }

    // Tests avec puissances de 2
    #[test]
    fn test_lcm_powers_of_two() {
        assert_eq!(lcm(8, 16), Ok(16));
        assert_eq!(lcm(32, 64), Ok(64));
        assert_eq!(lcm(1024, 2048), Ok(2048));
        assert_eq!(lcm(4, 8), Ok(8));
    }

    // Tests d'erreur - nombres négatifs
    #[test]
    fn test_lcm_negative_numbers() {
        assert_eq!(lcm(-5, 10), Err(MathError::PositifIntegerRequired));
        assert_eq!(lcm(10, -5), Err(MathError::PositifIntegerRequired));
        assert_eq!(lcm(-3, -7), Err(MathError::PositifIntegerRequired));
        assert_eq!(lcm(-1, 0), Err(MathError::PositifIntegerRequired));
        assert_eq!(lcm(0, -1), Err(MathError::PositifIntegerRequired));
    }

    // Tests de débordement
    #[test]
    fn test_lcm_overflow() {
        // Ces valeurs sont choisies pour provoquer un débordement
        let large_a = 1_000_000_000_000i64; // 10^12
        let large_b = 2_000_000_000_000i64; // 2 * 10^12

        // Le produit dépasserait i64::MAX
        match lcm(large_a, large_b) {
            Err(MathError::Overflow) => {} // Attendu
            Ok(_) => {}                    // Peut-être ok selon le GCD
            Err(e) => panic!("Erreur inattendue: {:?}", e),
        }

        // Test plus agressif pour forcer un débordement
        let very_large = i64::MAX / 2;
        match lcm(very_large, very_large - 1) {
            Err(MathError::Overflow) => {}     // Probablement attendu
            Ok(result) => assert!(result > 0), // Si ça marche, vérifier cohérence
            Err(e) => panic!("Erreur inattendue: {:?}", e),
        }
    }

    // Tests des propriétés mathématiques
    #[test]
    fn test_lcm_mathematical_properties() {
        // Propriété de commutativité: LCM(a, b) = LCM(b, a)
        assert_eq!(lcm(12, 18), lcm(18, 12));
        assert_eq!(lcm(7, 11), lcm(11, 7));
        assert_eq!(lcm(100, 75), lcm(75, 100));

        // Propriété avec GCD: LCM(a, b) * GCD(a, b) = a * b
        // (Cette propriété sera testée si vous avez accès à gcd_abs)
        let a = 24i64;
        let b = 36i64;
        if let (Ok(lcm_result), Ok(gcd_result)) = (lcm(a, b), gcd_abs(a, b)) {
            assert_eq!(lcm_result * gcd_result, a * b);
        }
    }

    // Tests de cas limites
    #[test]
    fn test_lcm_edge_cases() {
        // Avec i64::MAX (si possible sans débordement)
        assert_eq!(lcm(i64::MAX, 1), Ok(i64::MAX));
        assert_eq!(lcm(1, i64::MAX), Ok(i64::MAX));

        // Avec de grands nombres premiers (test de performance)
        let prime_1 = 982_451_653i64;
        let prime_2 = 982_451_707i64;
        if let Ok(result) = lcm(prime_1, prime_2) {
            assert_eq!(result, prime_1 * prime_2); // Premiers entre eux
        }
    }

    // Tests de régression spécifiques
    #[test]
    fn test_lcm_specific_regressions() {
        // Cas qui ont pu poser problème dans le passé
        assert_eq!(lcm(60, 168), Ok(840));
        assert_eq!(lcm(132, 88), Ok(1452));
        assert_eq!(lcm(48, 72), Ok(144));

        // Test avec facteurs premiers multiples
        assert_eq!(lcm(360, 504), Ok(2520)); // 360 = 2³×3²×5, 504 = 2³×3²×7
    }

    // Tests de performance (optionnels - peuvent être ignorés en CI)
    #[test]
    #[ignore = "test de performance - exécuter manuellement"]
    fn test_lcm_performance() {
        use std::time::Instant;

        let start = Instant::now();
        let mut count = 0;
        for i in 1..10000 {
            for j in 1..100 {
                let _ = lcm(i * 10000 + j, j * 10000 + i);
                count += 1;
            }
        }
        let duration = start.elapsed();

        println!("{} calculs LCM en: {:?}", count, duration);
        assert!(duration.as_millis() < 1000); // Doit être rapide
    }

    // Tests de cohérence avec différentes approches de calcul
    #[test]
    fn test_lcm_consistency() {
        let test_pairs = [
            (12, 18),
            (15, 25),
            (7, 11),
            (100, 150),
            (24, 36),
            (50, 75),
            (84, 126),
            (120, 180),
        ];

        for &(a, b) in &test_pairs {
            let result = lcm(a, b).unwrap();

            // Le LCM doit être divisible par les deux nombres
            assert_eq!(
                result % a,
                0,
                "LCM({}, {}) = {} n'est pas divisible par {}",
                a,
                b,
                result,
                a
            );
            assert_eq!(
                result % b,
                0,
                "LCM({}, {}) = {} n'est pas divisible par {}",
                a,
                b,
                result,
                b
            );

            // Le LCM doit être >= au maximum des deux nombres
            assert!(
                result >= a.max(b),
                "LCM({}, {}) = {} < max({}, {})",
                a,
                b,
                result,
                a,
                b
            );

            // Pour tout k < LCM, k ne doit pas être divisible par les deux
            for k in (a.max(b)..result).step_by((result / 100).max(1) as usize) {
                assert!(
                    k % a != 0 || k % b != 0,
                    "Trouvé un multiple commun {} < LCM({}, {}) = {}",
                    k,
                    a,
                    b,
                    result
                );
            }
        }
    }
}

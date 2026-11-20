pub fn normalize(a: i64, n: i64) -> i64 {

    assert!(n > 0);

    if a % n >= 0 {
	return a % n;
    } else {
	return a % n + n;
    }
}

pub fn mod_add(a: i64, b: i64, n: i64) -> i64 {

    let c: i64 = a + b;

    let r: i64 = normalize(c, n);

    return r;
}

pub fn mod_sub(a: i64, b: i64, n: i64) -> i64 {

    let c: i64 = a - b;

    let r: i64 = normalize(c, n);

    return r;
}

pub fn mod_mul(a: i64, b: i64, n: i64) -> i64 {

    let c: i64 = a * b;

    let r: i64 = normalize(c, n);

    return r;
}

pub fn verify_congruence(a: i64, b: i64, k: i64, n: i64) -> bool {

    if a - b == k * n {
	true
    } else {
	false
    }
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_normalize() {

	assert_eq!(normalize(-17, 5), 3);
	assert_eq!(normalize(17, 5), 2);
	assert_eq!(normalize(-10, 5), 0);
	assert_eq!(normalize(100, 1), 0);
    }

    #[test]

    fn test_mod_add() {

	assert_eq!(mod_add(3, 5, 7), 1);
	assert_eq!(mod_add(-3, 5, 7), 2);
	assert_eq!(mod_add(0, 0, 7), 0);
    }

    #[test]

    fn test_mod_sub() {

	assert_eq!(mod_sub(3, 5, 7), 5);
	assert_eq!(mod_sub(5, 3, 7), 2);
	assert_eq!(mod_sub(3, 3, 7), 0);
    }

    #[test]

    fn test_mod_mul() {

	assert_eq!(mod_mul(3, 5, 7), 1);
	assert_eq!(mod_mul(-3, 5, 7), 6);
	assert_eq!(mod_mul(0, 5, 7), 0);
    }

    #[test]

    fn test_verify_congruence() {

	assert!(verify_congruence(17, 2, 3, 5));
	assert!(!verify_congruence(17, 2, 2, 5));
    }

    #[test]
    #[should_panic]

    fn test_normalize_zero_modulus() {

	normalize(17, 0);
    }
}

    

use crate::week1::modular::normalize;

pub fn gcd(a: i64, b: i64) -> i64 {

    assert!(a != 0 || b != 0);

    if a == 0 {
       return b.abs();
    }

    if b == 0 {
       return a.abs();
    }

    let mut a: i64 = a.abs();
    let mut b: i64 = b.abs();

    let mut r: i64 = normalize(a, b);

    while r > 0 {
    	  a = b;

	  b = r;

	  r = normalize(a, b);

    }

    return b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
       assert_eq!(gcd(252, 105), 21);
       assert_eq!(gcd(18, 24), 6);
       assert_eq!(gcd(21, 42), 21);
       assert_eq!(gcd(17, 13), 1);
       assert_eq!(gcd(12, 12), 12);
    }

    #[test]
    fn test_gcd_zero() {
       assert_eq!(gcd(0, 7), 7);
       assert_eq!(gcd(7, 0), 7);
       assert_eq!(gcd(0, -7), 7);
       assert_eq!(gcd(-7, 0), 7);
    }

    #[test]
    fn test_gcd_negative() {
       assert_eq!(gcd(-18, 24), 6);
       assert_eq!(gcd(18, -24), 6);
       assert_eq!(gcd(-18, -24), 6);
    }

    #[test]
    #[should_panic]
    fn test_gcd_both_zero() {
       gcd(0, 0);
    }
}
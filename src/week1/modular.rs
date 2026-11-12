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

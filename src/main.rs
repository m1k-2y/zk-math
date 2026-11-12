mod week1;

fn main() {

    println!("ZK Mathematics - Week 1");

    let r: i64 = week1::modular::normalize(-17, 5);

    println!("Normalize : {}", r);

    let r: i64 = week1::modular::mod_add(3, 5, 7);

    println!("Addition : {}", r);

    let r: i64 = week1::modular::mod_sub(3, 5, 7);

    println!("Subtraction : {}", r);

    let r: i64 = week1::modular::mod_mul(3, 5, 7);

    println!("Multiplicatiol : {}", r);

    let r: bool = week1::modular::verify_congruence(17, 2, 3, 5);

    println!("Congruence : {}", r);
}

use bigdecimal::{BigDecimal, Zero, FromPrimitive};
use std::str::FromStr;

fn main() {
    //1.23456789^2.55  (round 50 before starting calculations, precision 50 in the root loop)
    println!("{}", bigdecimal_powf(&BigDecimal::from_str("1.23456789").unwrap(), &BigDecimal::from_str("2.55").unwrap(), Some(50), Some(50)));
    //1.23456789^2.55  (no rounding or precision limits)
    println!("{}", bigdecimal_powf(&BigDecimal::from_str("1.23456789").unwrap(), &BigDecimal::from_str("2.55").unwrap(), None, None));

    //123^2
    println!("{}", bigdecimal_powi(&BigDecimal::from_str("123").unwrap(), &BigDecimal::from_str("2").unwrap()));

    //322nd root of 2  (precision 50 in the root loop)
    println!("{}", bigdecimal_root(&BigDecimal::from_str("322").unwrap(), &BigDecimal::from_str("2").unwrap(), Some(50)));
    //322nd root of 2 (no precision limit)
    println!("{}", bigdecimal_root(&BigDecimal::from_str("322").unwrap(), &BigDecimal::from_str("2").unwrap(), None));
}

//calculates integer and decimal bigdecimal powers powers
//be warned that passing none to prec may cause your calculations to take forever
//x is the base, e is the exponent, prec tries to remove effectively infinite calculations, round rounds the inputs of powi to be faster
pub fn bigdecimal_powf(x: &BigDecimal, e: &BigDecimal, prec: Option<u64>, round: Option<i64>) -> BigDecimal {
    let exponent_string = format!("{}", e);
    let split: Vec<&str> = exponent_string.split(".").collect();
    let split_whole = split.get(0).unwrap();
    let split_decimal = split.get(1).unwrap_or(&"");

    let whole_value = BigDecimal::from_str(split_whole).unwrap();

    if split_decimal.len() >= 1 {
        let numerator = split_whole.parse::<u32>().unwrap() * u32::pow(10, split_decimal.len() as u32) + split_decimal.parse::<u32>().unwrap();
        let denominator = u32::pow(10, split_decimal.len() as u32);
        
        //attempt to chop down the fraction to improve performance if a greatest common denominator can be found
        let gcd = euclid_gcd(numerator, denominator);
        let simplified_numerator = numerator / gcd;
        let simplified_denominator = denominator / gcd;

        let whole_result = if round.is_some() {
            bigdecimal_powi(&x.round(round.unwrap()), &BigDecimal::from_u32(simplified_numerator).unwrap()).round(round.unwrap())
        } else {
            bigdecimal_powi(&x, &BigDecimal::from_u32(simplified_numerator).unwrap())
        };
        return bigdecimal_root(&BigDecimal::from_u32(simplified_denominator).unwrap(), &whole_result, prec);
    } else {
        return bigdecimal_powi(&x, &whole_value);
    }
}

//simple greatest common denominator finder
//m is the numerator, n is the denominator
fn euclid_gcd(mut m: u32, mut n: u32) -> u32 {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    return n
 }

//calculates integer equivalent bigdecimal powers only
//x is the base, e is the exponent
pub fn bigdecimal_powi(x: &BigDecimal, e: &BigDecimal) -> BigDecimal {
    let mut r = BigDecimal::from_str("1").unwrap();
    let mut i = BigDecimal::zero();
    while i < *e {
        r *= x;
        i += 1;
    }
    return r;
}

//calculates integer equivalent bigdecimal roots only
//be warned that passing none to prec may cause your calculations to take forever
//n is the root, x is the base, prec tries to remove effectively infinite calculations
pub fn bigdecimal_root(n: &BigDecimal, x: &BigDecimal, prec: Option<u64>) -> BigDecimal {
    let mut d: BigDecimal;
    let mut r = BigDecimal::from_str("1").unwrap();
    if x == &BigDecimal::zero() {
        return BigDecimal::zero();
    }
    if n < &BigDecimal::from_str("1").unwrap() { //this if was `if (n < 1 || (x < 0 && !(n&1)))` in C, depending on the use case you may need to find a way to add the rest
        return BigDecimal::zero(); //substitute for NaN, you may want to convert this function to return an option if you need to handle this case
    }
    loop {
        r = if prec.is_some() {r.with_prec(prec.unwrap())} else {r};  //looping with round is too expensive, with_prec is used instead
        d = (x / bigdecimal_powi(&r, &(n - 1)) - &r) / n;
        r += &d;
        if !(&d >= &(BigDecimal::from_f64(f64::EPSILON).unwrap() * 10) || &d <= &(BigDecimal::from_f64(-f64::EPSILON).unwrap() * 10)) {
            break;
        }
    }
    return r;
}
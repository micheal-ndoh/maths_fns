use std::process::Output;

fn main() {
    let a = 4.0;
    let b = -2.7;
    let precision = 3;
    println!("The multiplication of {a} * {b} is  {:.1$}\n", multiplication(a, b), precision);
    println!("The division of {a} / {b} is {} with a remainder of {:.2$}\n", division(a, b), modulus(a, b), precision);
    
    println!("The power of {a} ^ {b} is {:?}", power(a, b) );
    // println!("The modulus of {a} % {b} is {:?}\n", modulus(a, b));
}

pub fn multiplication(a: f64, b: f64) -> f64 {
    let mut result = 0.0;
    let bb = b.abs();
    for _i in 0..bb as i64 {
        result += a.abs();
    }
    if (a < 0.0 && b > 0.0) || (a > 0.0 && b < 0.0) {
        -result
    } else {
        result
    }
}

pub fn division(mut a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("cannot divide by zero");
        return 0.0;
    }
    let mut result = 0.0;
    let bb = b.abs();
    while a.abs() >= bb {
        a -= bb;
        result += 1.0;
    }
    if (a < 0.0 && b > 0.0) || (a > 0.0 && b < 0.0) {
        -result
    } else {
        result
    }
}

pub fn power(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 1.0;
    }
    let mut result = 1.0;
    let bb = b.abs();
    for _i in 0..bb as i64 {
        result *= a;
    }

    if b < 0.0 {
        1.0 / result
    } else {
        result
    }
}

pub fn modulus(mut a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("cannot divide by zero");
        return a;
    }
    let bb = b.abs();
    while a.abs() >= bb {
        a -= bb;
    }
    if a < 0.0 {
        a + bb
    } else {
        a
    }
}
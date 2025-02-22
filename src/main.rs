use std::process::Output;


fn main() {
    let a = 4.8;
    let b = -2.6;
    let precision = 3;
    println!("The multiplication of {a} * {b} is  {:.1$}\n",  multiplication(a, b), precision);
    println!("The division of {a} / {b} is {} with a remainder of {:.2$}\n", division(a, b), modulus(a, b), precision);
    
    // println!("The power of {a} ^ {b} is {:?}", power(a, b) );
    // println!("The modulus of {a} % {b} is {:?}\n", modulus(a, b));
}
 
pub fn multiplication(a: f64, b: f64) -> f64 {
    let mut result = 0.0;
    let bb =  b.abs();
     for _i in 0..bb as i64 {
        result += a.abs();
      
      
    }
    let output = result;
        if a < 0.0 || bb < 0.0 {
          let  output = -result;
               
    }
    else if (a > 0.0 && bb > 0.0) || (a < 0.0 && bb < 0.0) { 
        let output = result;
        
    }
   output
    }
    


 pub fn division(mut a: f64, b:f64) -> f64 {
    let mut result = 0.0;
    while a > (b as i64) as f64 {
        a -= b;
        result += 1.0
        
    }
    if a < 0.0{
        result = 0.0;
    if b.abs() < 0.0 {
       println!("cannot divide");
    }

    }
     result 

}


// pub fn power(a: f64, b: f64) -> f64 {
//     if b == 0.0 {
//         return 1.0;
//     }
//     else {
        
//     }
// }


pub fn modulus(a: f64, b: f64) -> f64 {
    let mut remainder = a;
    while remainder >= b {
        remainder -= b;
    }
    remainder
 }







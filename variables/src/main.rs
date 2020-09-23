fn main() {

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z ) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    
    println!("The value at pos 1 is: {}", tup.0);
    println!("The value at pos 2 is: {}", tup.1);
    println!("The value at pos 3 is: {}", tup.2);

    let a: [i32;  5] = [1, 2, 3, 4, 5];
    println!("The value at pos 0 is: {}", a[0])
}

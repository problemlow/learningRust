fn main() {
    println!("Hello, world!");
    let mut x = 4;
    println!("x is: {}", x);
    {
        let x = 2;
        println!("x is: {}", x);
    }
    x += 1;
    println!("x is: {}", x);
}

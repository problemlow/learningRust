fn main() {
    println!("Hello, world!");
    let mut x = 4;
    println!("x is: {}", x);
    {
        x += 2;
        println!("{}",x);
    }
    x += 1;
    println!("x is: {}", x);
}

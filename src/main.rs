fn main() {
    println!("Hello, world!");
    let mut x = "meme";
    println!("x is: {}", x);
    {
        x = x+"emem";
        println!("{}",x);
    }
    x = "one";
    println!("x is: {}", x);
}

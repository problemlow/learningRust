fn main() {
    println!("Hello, world!");
    let mut x = "meme";
    println!("x is: {}", x);
    {
        x += "emem";
        println!("{}",x);
    }
    x = 1;
    println!("x is: {}", x);
}

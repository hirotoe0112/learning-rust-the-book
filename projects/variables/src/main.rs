fn main() {
    let mut x = 5;
    println!("The value is {x}");
    x = 6;
    println!("The value is {x}");

    let y = 1;
    println!("The y value is {y}");
    let y = 2;
    println!("The y value is {y}");
    {
        let y = 3;
        println!("The y value is {y}");
    }
    println!("The y value is {y}");
    let y = "abc";
    println!("The y value is {y}");
}

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

    // Wrap and Check an overflow with built-in functions
    let num: u8 = 255;
    let num = num.wrapping_add(10);
    println!("{num}");
    let num2: u8 = 255;
    println!("{:?}", num2);
    let num2 = num2.checked_add(10);
    println!("{:?}", num2);

    // Use a single quotation mark instead of a double quotation mark
    let c: char = 'a';
    println!("{c}");
    // Unicode
    let k = '喝';
    println!("{k}");

    let tuple: (char, bool, u32) = ('覇', true, 123);
    println!("{:?}", tuple);
    let (t1, t2, t3) = tuple;
    println!("{t1},{t2},{t3}");

    let arr: [char; 5] = ['心', '技', '体', '破', '滅'];
    println!("{:?}", arr);
}

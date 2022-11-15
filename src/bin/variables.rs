fn main() {
    // by default, the rust variable is immutable
    // we can use `mut` to let it mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!();


    // Constant:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing:
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
    // reuse the variable name and can change the type
    let spaces = "   ";
    let spaces = spaces.len();
    // the following are bad
    // let mut spaces = "   ";
    // spaces = spaces.len();


}
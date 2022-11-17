fn main() {
    another_function(5);

    expression_block();

    let y = {
        let x = 15;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("the function return is {}", ten());

    println!("the expression in function return is {}", plus_one(y));


}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn expression_block() {
    let y = {
        let x = 3;
        // this is an expression, if add semicolon, turn to an expression
        x + 1
    };

    println!("The value of y is: {y}");
}

fn ten() -> i32 {
    10
}

fn plus_one(x: i32) -> i32 {
    // need to be an expression
    x + 1
}
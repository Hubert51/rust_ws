use std::io;

fn main() {

    // Tuple and array:
    println!("\n=========Tuple and Array=========");
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("Another kind of access: element 0: {}, element 1: {}", tup.0, tup.1 );

    // [type; length]  or [value, length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];
    println!("The array of b is: {:?}", b);

    let mut c:[&str; 7] = ["Adil"; 7];
    println!("Array {:?}", c);

    for row in c.iter_mut() {
        *row = "Audi";
        break;
    }
    println!("Array {:?}", c);


    let a = [1, 2, 3, 4, 5];
    println!("\nPlease enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
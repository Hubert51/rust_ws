fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    // the first pointer will be invalid
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world", s2);

    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    // reference
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}",r3);

    let r4 = String::from("hello world");
    let word = first_word(&r4);
    // r4.clear();
    println!("r4 is {}",word);

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    // other slice:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);


}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    &s[..]
}

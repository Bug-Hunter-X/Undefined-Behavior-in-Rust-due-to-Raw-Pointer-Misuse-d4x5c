fn main() {
    let mut v = vec![1, 2, 3];
    let first_element = &mut v[0];
    *first_element = 10;
    println!("The first element is: {}", v[0]);
} 
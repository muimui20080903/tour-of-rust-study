fn main() {
    // let mut i32_vec = Vec::<i32>::new();
    // let mut i32_vec = Vec::new();
    // i32_vec.push(1);
    // i32_vec.push(2);
    // i32_vec.push(3);
    // let mut i32_vec = vec![1, 2, 3];
    // i32_vec.push(4);
    let i32_vec = vec![3; 5];
    println!("{:?}", i32_vec);

    let mut float_vec = Vec::new();
    // float_vec.push(1);
    float_vec.push(1.0);
    float_vec.push(2.0);
    float_vec.push(3.0);

    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}

fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    // let not_number = "abc";
    // let b = not_number.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}

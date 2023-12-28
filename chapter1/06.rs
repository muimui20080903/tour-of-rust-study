// 基本型の変換
fn main() {
    let a = 13u8;
    let b = 7u32;
    // let c = a + b;
//     error[E0308]: mismatched types
//  --> .\06.rs:5:17
//   |
// 5 |     let c = a + b;
//   |                 ^ expected `u8`, found `u32`

// error[E0277]: cannot add `u32` to `u8`
//  --> .\06.rs:5:15
//   |
// 5 |     let c = a + b;
//   |               ^ no implementation for `u8 + u32`
//   |
//   = help: the trait `Add<u32>` is not implemented for `u8`
//   = help: the following other types implement trait `Add<Rhs>`:      
//             <u8 as Add>
//             <u8 as Add<&u8>>
//             <&'a u8 as Add<u8>>
//             <&u8 as Add<&u8>>

// error: aborting due to 2 previous errors
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

// 文字列リテラル
fn main() {
    // let a = "こんにちは 🦀";
    let a: &'static str = "こんにちは 🦀";
    println!("{} {}", a, a.len());
}

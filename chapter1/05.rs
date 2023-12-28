// 基本的な型
// 型推論により、型を明示的に指定しなくてもコンパイラが型を推論してくれる
// u8: 符号なし8ビット整数
// f32: 32ビット浮動小数点数
// bool: 真偽値
// タプル: 複数の値をまとめたもの
// 配列：同じ型の値を複数保持するデータ構造

fn main() {
    let x = 12; // デフォルトでは i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
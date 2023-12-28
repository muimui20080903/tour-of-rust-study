fn main() {
    // 変数を宣言するには、letを使う
    // デフォルトでは変数は不変で書き込みは不可
    // let x = 42;
    // 可変の変数を宣言するには、letの前にmutをつける
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

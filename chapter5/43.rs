// 所有権
struct Foo {
    x: i32,
}

fn main() {
    // 構造体をインスタンス化し、変数に束縛してメモリリソースを作成
    let foo = Foo { x: 42 };
    // foo は所有者
}

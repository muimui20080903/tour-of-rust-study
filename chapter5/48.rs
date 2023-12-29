struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;
    // let f = foo; // これはfooがドロップしてエラー
    println!("{}", foo.x);
    println!("{}", f.x);
    // f はここでドロップ
    // foo はここでドロップ
}

// #[derive(Debug)]
// enum Item {
//     // Inventory(String),
//     None,
// }

// #[derive(Debug)]
// struct BagOfHolding {
//     item: Item,
// }
// fn main() {
//     let bag = BagOfHolding {
//         // item: Item::Inventory(String::from("torch")),
//         item: Item::None,
//     };
//     println!("{:?}", bag.item);
// }

enum Item {
    Inventory(String),
    // None は項目がないことを表す
    None,
}

struct BagOfHolding {
    item: Item,
}

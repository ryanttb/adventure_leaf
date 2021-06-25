#[derive(Debug)]
enum LeafPosition {
    Flying,
    Floating,
    Hovering,
}

#[derive(Debug)]
struct Leaf {
    position: LeafPosition
}

fn main() {
    println!("Hello, world!");

    let mut leaf = Leaf { position: LeafPosition::Floating };
    println!("leaf is {:?}", leaf);


    leaf.position = LeafPosition::Flying;
    println!("leaf is {:?}", leaf);

    leaf.position = LeafPosition::Hovering;
    println!("leaf is {:?}", leaf);
}

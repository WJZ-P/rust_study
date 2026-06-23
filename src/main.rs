#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut collection : [i32; 5] = [1, 2, 3, 4, 5];
    //let mut collection : [String; 5] = std::array::from_fn(|_| String::from("Hello, World!"));
    for i in 0..collection.len() {
        println!("{}", collection[i]);
        let j = collection[i];
        collection[i] = j;
    }
    let i = collection[1];
    collection[1] = i;
    println!("{:?}", collection);
}
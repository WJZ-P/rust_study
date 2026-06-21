use std::ops::Deref;

fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}

fn say_hello(s: &str) {
    println!("{}",s);
}
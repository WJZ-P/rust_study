use std::fs::File;

//  让我来实现一个简单的宏！

#[macro_export]
macro_rules! vec {
    ($($x:expr), *) => {
        let mut temp_vec = Vec::new();
        $(
        temp_vec.push($x);
        )*
        temp_vec
    };
}

fn main() {
    let f = File::open("hello.txt")?;
}
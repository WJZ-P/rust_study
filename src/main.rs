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

fn get_two_sites() {
    // 创建两个新线程执行任务
    let thread_one = thread::spawn(|| download("https://course.rs"));
    let thread_two = thread::spawn(|| download("https://fancy.rs"));

    // 等待两个线程的完成
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
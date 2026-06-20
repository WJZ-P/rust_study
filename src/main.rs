use std::ops::Deref;

fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&(*(s.deref())));//   这一行是上面那行的最终编译器执行的代码
    //  因为s类型是String，这里引用，用&s传入的话，类型就是&String，和函数的入参不符合。
    //  此时rust就会做隐式转换，尝试套*，变成&(*s)，因为*s的类型是str，就和入参的匹配了！
    //  最后rust再做展开，变成&(*(s.deref()))。
    say_hello(&s[..]);
    say_hello(s.as_str());
}

fn say_hello(s: &str) {
    println!("{}",s);
}
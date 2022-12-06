fn main() {
    // println!() 是 rust 中一个 预定义的宏，用于将传递给它的参数输出到标准输出
    println!("hello rust")
    // rustc hello.rs >>> hello 生成二进制的可执行文件
    // ./hello  >>> 打印出 hello rust
    // 在 rust 中 宏 都是以 ！结尾的，以 ！ 结尾的函数都是宏调用
    // 我们可以通过 宏 来进行 元编程，rust 中的 宏类似于 JS 中的函数，宏 可以理解为 函数的增强版
}

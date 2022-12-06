fn main() {
    // char 就是字符串的基本组成部分，也就是单个字符或字
    // rust 使用 utf-8 作为底层的编码，而不是 ascii，所以 char 类型的大小是 4 个字节
    // 而 JS 使用的是 utf-16，所以 JS 中的 char 类型的大小是 2 个字节

    // rust 中的 字符数据类型 包含了 数字、字母、Unicode 和 其他特殊字符
    let c = "x";
    let c1 = "⚡️";
    println!("c = {}, c1 = {}", c, c1);
    // >>> c = x, c1 = ⚡️
}

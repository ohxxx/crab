use std::io::Write;

fn main() {
    // I/O 就是输入输出，rust IO 输入输出的三大块内容：读取数据、写入数据、命令行参数

    // 读取和写入
    // rust 标准库 IO 输入输出两个 Trait：Read、Write
    // Read Trait 有一个 read 方法，用于读取数据，字节流
    // Write Trait 有一个 write 方法，用于写入数据，包含字节数据和 UTF-8 数据两种格式
    let mut in_word = String::new();
    let res = std::io::stdin().read_line(&mut in_word).unwrap();
    println!("输入的是: {}", in_word);
    println!("读取的字节数为：{}", res);

    // std::io::stdin() 返回标准输入流 stdin 的句柄。
    // read_line 方法是标准入流 stdin 的句柄上的方法，从标准输入流读取一行数据。返回值是一个 Result 枚举，
    // unwrap 则是一个帮助方法，用于简化可恢复错误的处理。它会返回 Result 中存储的实际值。read_line 方法会自动删除行尾的换行符。

    let res2 = std::io::stdout().write("hello world".as_bytes()).unwrap();
    println!("写入的字节数：{}", res2);
    let res3 = std::io::stdout().write("你好世界".as_bytes()).unwrap();
    println!("写入的字节数：{}", res3);
    // std::io::stdout() 会返回标准输出流 stdout 的句柄
    // write() 是标准输出流 stdout 的句柄上的一个方法，用于向标准输出流写入字节流内容
    // write() 方法的返回值是一个 Result 枚举，而 unwrap() 则是一个帮助方法，用于简化可恢复错误的处理。它会返回 Result 中存储的实际值。
    // write() 方法并不会输出结束时自动追加换行符，所以需要换行符的话需要手动添加。

    // 命令行参数
    // 使用 std::env::args() 方法获取命令行参数，返回的结果包含了程序名
    // 如果要传递多个参数，多个参数之间必须使用空格分隔，如果参数中有空格，则参数必须使用双引号包裹
    let input_args = std::env::args();
    for arg in input_args {
        println!("参数：{}", arg);
    }
}

fn main() {
    // 元组是一个复合类型
    // 可以存储多个不同类型的数据，rust 支持 tuple 类型，元组使用括号()来构造。

    // 定义元组
    // let 变量名:(数据类型1,数据类型2...) = (数据1,数据2...)
    let t: (&str, &str) = ("halo", "xxx");
    println!("{:?}", t);
    // 访问元素：元组变量.索引数字
    println!("元组第一个 {}", t.0);
    println!("元组第二个 {}", t.1);
    // 元组解构
    let (name1, name2) = t;
    println!("name1 {}", name1);
    println!("name2 {}", name2);
    // 元组作为参数
    print_name(t)
}

fn print_name(tuple: (&str, &str)) {
    println!("{:?}", tuple)
}

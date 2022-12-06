fn main() {
    // rust 是一个静态类型语言，所以在声明变量的时候，必须要指定变量的类型
    // 在赋值的时候不强制指定类型，rust 编译器会自动推断变量的类型

    // 定义：let 变量名: 变量类型 = 变量值
    let s = "hello rust";
    let f = 3.14;
    let i = 10;
    let b = true;

    println!("s = {}, f = {}, i = {}, b = {}", s, f, i, b);
    // >>> s = hello rust, f = 3.14, i = 10, b = true

    // rust 中有四种标量类型：
    //    整型
    //    浮点型
    //    布尔型
    //    字符型
}

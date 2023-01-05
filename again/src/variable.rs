struct Struct {
    e1: i32,
}

pub fn variable() {
    // rust 为什么要手动设置变量的可变性
    // 1、增加灵活性
    // 2、增加安全性
    // 3、提升运行时性能

    // 变量绑定
    // let x = 6;

    // rust 默认变量是不可变的，使用 mut 关键字声明变量可变
    let mut x1 = 6;
    println!("x1 = {}", x1);
    x1 = 7;
    println!("x1 = {}", x1);

    let _x2 = 6;

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {}, b = {}", a, b);
    b = true;
    assert_eq!(a, b);
    // rust 1.59 后可以在赋值语句的左式中使用元组、切片和结构体模式
    let (a1, b1, c1, d1, e1);
    (a1, b1) = (1, 2);
    [c1, .., d1, _] = [1, 2, 3, 4, 5];
    // 这里的 .. 表示忽略剩余的元素，和 JS 中解构的 ... 类似
    Struct { e1, .. } = Struct { e1: 5 };
    assert_eq!([1, 2, 1, 4, 5], [a1, b1, c1, d1, e1]);

    // += 语句不支持解构

    // 常量是始终不可变的，使用 const 关键字声明常量
    // 必须要指定类型，rust 中常量命名默是全部大写使用下划线分隔
    // 常量可以在任何作用域中声明，包括全局作用域
    const MAX_XXX: i32 = 5;
    println!("MAX_XXX = {}", MAX_XXX);
    // 项目中最好将用到的硬编码值都声明为常量
    // 硬编码值：用于表示固定的值，例如常数、常用单位或预定义的值

    // 变量遮蔽 shadowing
    // rust 允许声明相同变量名，后面声明的变量名会遮蔽前面的变量名
    // 和 TS 中的 let 一样，后面声明的变量会覆盖前面的变量
    let s = 6;
    let s = s + 1; // 这里生成了一个新的变量 s
    {
        let s = s * 2;
        println!("代码块中 s = {}", s)
    }
    println!("代码块外 s = {}", s);
    // mut 声明的变量可以修改同一个内存地址上的值，不会发生内存对象再分配的情况
    // 而 shadowing 声明的变量会生成一个新的内存地址，会发生内存对象再分配的情况

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces = {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // rust 中不允许将 usize 类型的值赋值给 &str 类型的变量
    // usize 是一种 CPU 相关的整数类型
}

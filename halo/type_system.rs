fn main() {
    // 类型转换
    // rust 不提供原生类型之间的隐式转换，需要使用 as 关键字进行显式转换
    let a = 1;
    let b: f64 = a as f64;
    println!("b = {}", b);

    // 字面量
    // 对于数值字面量，只要把类型作为后缀加上去，就完成了类型说明。比如指定字面量 42 的类型是 i32，只需要写 42i32
    // 无后缀的数值字面量，其类型取决于怎样使用它们，如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64

    // 带后缀的字面量，其类型在初始化时就已经确定了，无法改变
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们，如果没有限制，编译器会对整数使用 i32，对浮点数使用 f64
    let i = 1;
    let f = 1.0;

    // 类型推断
    // rust 的类型推断引擎很聪明，它不只是在初始化时看看右值（r-value）的类型而已，它还是考察变量之后怎么使用，借用此推断类型
    // 因为有类型说明，编译器知道类型是 u8
    let s = String::from("halo xxx");

    // 创建一个空向量 vector 既不定长，也不定类型
    let mut v = Vec::new();
    // 现在编译器还不知道 vec 的具体类型，只知道它是某个东西构成的向量 Vec<?>
    v.push(5);
    // 现在编译器知道 vec 的类型是 Vec<i32>
    println!("v = {:?}", v);

    // 别名
    // 可以用 type 语句给已有的类型缺个新的名字，类型的名称必须遵循驼峰命名法，否则编译器将给出警告，原生类型除外，如 usize、f32 等
    // 别名的主要用途就是避免写出冗长的模板化代码
    // type 新名字 = 原名字;
    type MyU64 = u64;
    type OtherU64 = u64;
    type ThirdU64 = u64;

    let MyU64: MyU64 = 1 as ThirdU64;
    let OtherU64: OtherU64 = 2 as ThirdU64;
    println!(
        "{} MyU64 + {} OtherU64 = {} unit",
        MyU64,
        OtherU64,
        MyU64 + OtherU64
    );

    // 类型转换
    // rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 Into 两个 trait
    // From 和 Into 两个 trait 是内部相互关联的，实际上这是它们实现的一部分，如果我们能够从类型 B 得到类型 A，那么很容易相信我们也能把类型 B 转换成类型 A

    // From trait 允许一种类型定义“怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中无数 From 的实现规定原生类型及其他常见类型的转换功能
    let s = "halo xxx";
    let s2 = String::from(s);
    println!("s = {}", s);
    println!("s2 = {}", s2);

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number { value: 10 };
    println!("num = {:?}", num);

    // into trait 就是把 From trait 倒过来而已，也就是说，如果你的类型实现了 From，那么同时你也就免费获得了 into
    // 使用 into trait 通常要求指明要转换到的类型，因为编译器大多时候不能推断出来
    let i = 5;
    let n: Number = i.into();
    println!("n = {:?}", n);

    // 解析字符串
    // 经常需要把字符串转成数字，完成这个任务的标准手段就是使用 parse 方法
    // 只要对目标类型实现了 FromStr trait，就可以使用 parse 方法，标准库中已经给无数种类型实现了 FromStr，如果要转换到用户定义类型，就要自己手动实现 FromStr
    let cons: i32 = "5".parse().unwrap();
    println!("cons = {}", cons);
}

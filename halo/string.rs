fn main() {
    // rust 提供了两种类型的字符串：
    //    1、&str 类型：在栈上分配内存，不可变，长度固定（字符串字面量）- 核心内置类型
    //    2、String 类型：在堆上分配内存，可变，长度不固定（字符串对象）- 标准库中的一个公开 pub 结构体
    // rust 中的字符串是不可变的，如果需要修改字符串，可以使用 String 类型
    // 【和 JS 不同，JS 中的字符串是可变的，可以直接修改字符串的内容】

    let s1 = "hello";
    let s2 = String::from(s1);
    println!("s2 = {}", s2);

    // &str 字符串字面量被称之为 字符串切片（string slice），因为它的底层实现是切片
    // String 字符串对象被称之为 字符串（string），是使用 UTF-8 作为底层数据编码格式，长度可变的集合
    // 【和 JS 不同，JS 是使用 UTF-16 作为底层数据编码格式，长度可变的集合】

    // 新建字符串对象
    let s3 = String::new(); // 创建一个新的空字符串，它是静态方法
    let s4 = String::from("xxx"); // 从具体字符串字面量创建字符串对象
    println!("s3 = {}, s4 = {}", s3, s4);

    // 在字符串末尾追加字符串
    let mut s5 = String::from("hello");
    s5.push_str(" world"); // push_str() 方法接收一个字符串切片作为参数
    println!("s5 = {}", s5);

    // 在原字符串后面追加一个字符
    s5.push('!');
    s5.push('?');
    s5.push('!');
    println!("s5 = {}", s5);

    // 指定字符串字串替换成另一个字符串
    let s6 = String::from("hello world");
    s6.replace("world", "rust"); // replace() 方法返回一个新的字符串，原字符串不变
    let s7 = s6.replace("world", "rust");
    println!("s6 = {}\ns7 = {}", s6, s7);
    //【和 JS 中的 replace() 方法相同，都是返回一个新的字符串，原字符串不变】

    // 获取字符串长度
    let s8 = String::from("hello world");
    let len = s8.len(); // len() 方法返回字符串的长度，单位是字节，包含制表符、换行符、空格等
    println!("s8 = {}, len = {}", s8, len);

    // 将字符串转为字符串对象
    let s9 = "hello world";
    let s10 = s9.to_string(); // to_string() 方法返回一个新的字符串对象
    println!("s9 = {}\ns10 = {}", s9, s10);

    // 返回一个字符串对象的 字符串字面量
    let s11 = String::from("hello world");
    let s12 = &s11; // s11 是一个字符串对象，s12 是一个字符串字面量
    let s13 = s11.as_str(); // as_str() 方法返回一个字符串字面量
    println!("s11 = {}\ns12 = {}\ns13 = {}", s11, s12, s13);

    // 去除字符串首尾的空格
    let s14 = String::from(" hello world ");
    let s15 = s14.trim(); // trim() 方法返回一个新的字符串对象，原字符串不变
    println!("s14 = {}\ns15 = {}", s14, s15);
    // 【和 JS 中的 trim() 方法相同，都是返回一个新的字符串，原字符串不变】

    // 根据指定的字符串字串将字符串分割，返回分割后的字符串字串组成的切片上的迭代器
    let s16 = "hello、world、halo、xxx";
    let s16p = s16.split('、');
    println!("s16 = {}\ns16p = {:?}", s16, s16p);
    for i in s16p {
        println!("item = {}", i);
    }

    // 将一个字符串打散为所有字符组成的数组
    let s17 = "halo xxx";
    for i in s17.chars() {
        println!("item = {}", i);
    }

    // 字符串拼接 +
    let s18 = "halo".to_string();
    // 发生移动是因为 s18 的类型为 String，它没有实现 Copy 特性
    let s19 = " xxx";
    let s20 = s18 + s19;
    //        --- 值移到这里
    // println!("s18 = {}", s18);
    println!("s19 = {}", s19);
    println!("s20 = {}", s20);
}

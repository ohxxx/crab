fn main() {
    // rust 通过 match 关键字来提供模式匹配功能，match 语句类似于 switch 语句，但是更加强大，它可以匹配任何类型的值
    // match 表达式的基本形式如下：
    // match 表达式 {
    //     模式 => 表达式,
    //     模式 => 表达式,
    //     模式 => 表达式,
    //     ...
    // }

    // 解构指针和引用
    // 对指针来说，解构 destructure 和解引用 dereference 要区分开，因为这是两个不同的概念
    // 解引用使用 * 操作符
    // 解构使用 &、ref、ref mut 操作符

    // 获得一个 i32 类型的引用，& 表示引用
    let num = &100;
    // 解构 num，& 表示解构
    match num {
        // 使用 &n 这个模式来匹配 num
        &n => println!("n = {}", n),
    }
    // 如果不想用 & 需要在匹配前解引用
    match *num {
        n => println!("n = {}", n),
    }
    // rust 对这种情况提供了 ref，它更改了赋值行为，从而可以对具体值创建引用，下面就得到了一个 i32 类型的引用
    let ref num2 = 666;
    // 相应的定义两个非引用的变量，通过 ref 和 ref mut 仍可获得其引用
    let num3 = 888;
    let mut num4 = 999;
    // 使用 ref 关键字来创建引用
    match num3 {
        ref n => println!("n = {}", n),
    }
    // ref mut 用于可变引用
    match num4 {
        ref mut n => {
            *n += 1;
            println!("n = {}", n);
        }
    }

    // 解构结构体
    struct Student {
        name: String,
        age: u8,
        score: u8,
    }
    let stu = Student {
        name: "张三".to_string(),
        age: 18,
        score: 100,
    };
    let Student {
        name: name,
        age: age,
        score: score,
    } = stu;
    println!("name = {}, age = {}, score = {}", name, age, score);

    let s2 = Student {
        name: "李四".to_string(),
        age: 20,
        score: 99,
    };
    let Student { name, .. } = s2;
    println!("name = {}", name);

    // 在一些场合使用 match 匹配枚举类型并不优雅
    // rust 提供了 if let 语法来简化这种场合的代码
    enum Color {
        Red,
        Green,
        Blue,
    }
    let color = Color::Red;
    if let Color::Red = color {
        println!("color is red");
    }

    let ss = Some("halo xxx");
    let ss1: Option<i32> = None;
    let ss2: Option<i32> = None;
    // 如果 let 将 ss 解构成 Some(i) 则执行 ({})
    if let Some(i) = ss {
        println!("ss i = {}", i);
    }
    if let Some(i) = ss1 {
        println!("ss1 i = {}", i);
    } else {
        // 如果没有匹配到 Some(i) 则执行 else
        println!("ss1 is None");
    }

    let flag = true;
    if let Some(i) = ss2 {
        println!("ss2 i = {}", i);
    } else if flag {
        println!("ss2 is None");
    } else {
        println!("ss2 is None");
    }

    // while let 语法
    // 将 optional 设为 Option<i32> 类型
    let mut num33 = Some(0);
    while let Some(i) = num33 {
        if i > 9 {
            println!("num33 is {}", i);
            num33 = None;
        } else {
            println!("num33 is {}", i);
            num33 = Some(i + 1);
        }
    }
}

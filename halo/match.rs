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
}

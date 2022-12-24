fn main() {
    // rust 中的三种循环
    // loop 语句
    // while 语句
    // for 语句

    // for 临时变量 in 左区间..右区间 {
    //   todo
    // }

    for n in 1..6 {
        println!("num = {}", n)
        // >>> 1, 2, 3, 4, 5
    }
    println!("=======");
    // 也可以用 a..=b 表示两端都包含内的范围
    for n in 1..=6 {
        println!("num = {}", n)
        // >>> 1, 2, 3, 4, 5, 6
    }

    // iter - 在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用
    let books = vec![
        "《Python 语言圣经》",
        "《Rust 语言圣经》",
        "《Go 语言圣经》",
    ];

    for name in books.iter() {
        match name {
            &"《Rust 语言圣经》" => println!("你终于学到了 rust 的书籍 {}", name),
            _ => println!("继续学习 {}", name),
        }
    }

    println!("=======");

    // into_iter - 会消耗集合。在每次迭代中。集合中的数据本身会被提供，一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 move 了
    let books2 = vec![
        "《Python 语言圣经》",
        "《Rust 语言圣经》",
        "《Go 语言圣经》",
    ];

    for name in books2.into_iter() {
        match name {
            "《Rust 语言圣经》" => println!("你终于学到了 rust 的书籍 {}", name),
            _ => println!("继续学习 {}", name),
        }
    }

    println!("=======");

    // iter_mut 可变的，借用集合中的每个元素，从而允许集合就地修改，就是停止本次执行剩余的语句，直接进入下一个循环
    let mut books3 = vec![
        "《Python 语言圣经》",
        "《Rust 语言圣经》",
        "《Go 语言圣经》",
    ];

    for name in books3.iter_mut() {
        *name = match name {
            &mut "《Rust 语言圣经》" => "你终于学到了 rust 的书籍《Rust 语言圣经》",
            _ => *name,
        };
    }
    println!("已经在学：{:?}", books3);

    let mut n1 = 1;
    while n1 < 20 {
        println!("n1 = {}", n1);
        n1 = n1 * 2;
    }

    println!("=========");

    let mut n2 = 1;
    loop {
        if n2 > 20 {
            break;
        }
        println!("n2 = {}", n2);
        n2 = n2 * 3;
    }
}

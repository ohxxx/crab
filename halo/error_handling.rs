fn main() {
    // rust 中错误分为两类：可恢复 和 不可恢复，相当于其他的语言的 异常 和 错误
    // Recoverable：可以被捕捉，相当于其他语言的异常 Exception
    // Unrecoverable：不可捕捉，会导致程序崩溃，相当于其他语言的错误 Error

    // panic! 不可恢复的错误
    // panic!() 程序会立即退出，退出时调用者抛出退出原因，一般情况下当遇到不可恢复的错误，程序会自动调用 panic!()

    // panic!("崩溃了");
    // println!("halo xxx")
    // thread 'main' panicked at '崩溃了', error_handling.rs:9:5

    // let v = vec![1, 2, 3];
    // v[4] = 5; // 超出了数组的长度，所以会触发不可恢复的错误
    // println!("halo xxx")

    // Result 枚举和可恢复错误
    // enum Result<T, E> {
    //     Ok(T),  // T 表示正常返回值的值的数据类型
    //     Err(E), // E 表示错误的值的数据类型
    // }
    // let f = std::fs::File::open("halo.txt");
    // println!("{:?}", f);
    // >>> Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })

    // unwrap() 是 Result<T, E> 的方法，在实例上调用此方法时，如果是 Ok 枚举值，就会返回 OK 中的对象
    // 如果是 Err 枚举值，在运行时会 panic，报错信息是 format!("{}", error)，其缺点是如果在不同地方都使用 unwrap 运行时出现 panic 的时候
    let res = is_even(6).unwrap();
    println!("res 的结果：{}", res);
    // let res2 = is_even(7).unwrap();
    // println!("res2 的结果：{}", res2);
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "当前值不是偶数"'

    // expect() 方法的作用和 unwrap() 类似，区别在于 expect 方法接受 msg: &str 作为参数，它在运行时的 panic 信息是 format!("{}", msg, error)
    // 使用 expect() 时，可以自定义报错信息，因此出现 panic 时，可以更好的定位错误
    std::fs::File::open("halo.txt").expect("文件打开失败");
    // thread 'main' panicked at '文件打开失败: Os { code: 2, kind: NotFound, message: "No such file or directory" }'
}

fn is_even(no: i32) -> Result<bool, String> {
    return if no % 2 == 0 {
        Ok(true)
    } else {
        Err("当前值不是偶数".to_string())
    };
}

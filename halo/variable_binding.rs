fn main() {
    // 变量绑定默认是不可变的，但在加上 mut 修饰语后，就可以变成可变的

    // 作用域和屏蔽
    // 变量绑定有一个作用域，它被限制只在一个代码块中生存。代码块是被 {} 包围的语句集合，另外也允许变量屏蔽 variable shadowing

    // 此绑定生于 main 函数的作用域中
    let spend = 1;

    // 这是一个代码块，比 main 函数拥有更小的作用域
    {
        // 此绑定只存着于此代码块中
        let target = "halo xxx";
        println!("代码块中的 tagert {}", target);

        // 此绑定屏蔽了外层的 spend 绑定
        let spend = 3;
        println!("代码块中的 spend {}", spend);
    } // 代码块结束

    println!("main 函数中的 spend {}", spend);
    // println!("main 函数中的 target {}", target); // 无法访问 target，因为它只在代码块中生存

    // 变量先声明
    // 可以先声明 declare 变量绑定，后面才将它们初始化 initialize，但是这种做法很少，因为可能导致使用未初始化的变量
    // 编译器禁止使用未初始化的变量，因为这会产生未定义的行为，这是一种不安全的行为

    let s;
    {
        let x = 666;
        s = x * x;
    }
    println!("s = {}", s);
    // let s2;
    // println!("s2 = {}", s2); // 无法访问 s2，因为它没有初始化

    // 冻结
    // 资源存在使用的引用时，在当前作用域中这一资源是不可被修改的，这种行为称之为冻结 freeze
    // let mut s3 = Box::new(1);
    // let s4 = &s3;
    // s3 = Box::new(2);
    // println!("s3 = {}", s3);
    // println!("s4 = {}", s4);
}

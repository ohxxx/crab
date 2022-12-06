fn main() {
    // 按照存储大小可以把浮点型分为
    // f32，单精度浮点型
    // f64，双精度浮点型，默认浮点类型

    // 注意：rust 中不能将 0.0 赋值给任意一个整型，也不能将 0 赋值给任意浮点型

    // 错误
    // let f1: f64 = 99
    //                ~ expected `f64`, found integer
    // println!("f1 = {}", f1)

    let f1 = 99.0;
    let f2: f32 = 8.88;
    // 双精度浮点型
    let f3: f64 = 168.125;
    // 单精度浮点型
    println!("f1 = {}, f2 = {}, f3 = {}", f1, f2, f3);
    // >>> f1 = 99, f2 = 8.88, f3 = 168.125

    // 可以使用 _ 来分隔数字，方便阅读，和 JS 类似
    let f4 = 1_000_000;
    let f5 = 1_000_000.0001;
    println!("f4 = {}, f5 = {}", f4, f5);
}

static mut AGE2: i32 = 0;

fn add_to_count(a: i32) {
    unsafe {
        AGE2 += a;
    }
}

fn main() {
    // 常量 就是哪些值不能被改变的变量，定义后再也没有任何方法可以改变常量的值
    //    - const：不可改变的值（常用）
    //    - static：具有 'static 生命周期的值。'static 生命周期是整个程序运行期间都有效的生命周期
    // 有个特例就是 “string” 字面量
    // 它可以不经改动就被赋给一个 static 变量，因为它 的类型标记：&’static str 就包含了所要求的生命周期 ‘static。
    // 其他的引用类型都 必须特地声明，使之拥有’static 生命周期。

    // const 常量名称: 数据类型 = 值;

    const NAME: &str = "xxx";
    // rust 中常量是不能被遮蔽的
    // const NAME: i32 = 666; // error: `NAME` is already defined in this scope
    const AGE: i32 = 18;
    add_to_count(AGE); // 第一次调用
    println!("name: {}，age: {}", NAME, AGE);

    // static 常量名称: 数据类型 = 值;
    static NAME2: &str = "xxx";
    // static NAME3: &'static str = "xxx";
    add_to_count(1); // 第二次调用
    println!("name: {}, age: {}", NAME2, unsafe { AGE2 });
}

fn main() {
    // rust 中的闭包 closure，也叫做 lambada 表达式或者 lambda，是一类能够捕获周围作用域中变量的函数
    // 调用一个闭包和调用一个函数完全相同，不过调用闭包时，输入和返回类型两者都可以自动推导，而输入变量名必须指明
    // 其他特点包括：
    // 1、声明时使用 || 代替 ()，将输入参数括起来
    // 2、函数体定界符 ({}) 对于单个表达式是可选的，其他情况必须加上
    // 3、有能力捕获外部环境的变量

    // 定义一个闭包
    // [ 普通函数 ]
    // fn 函数名(参数列表) { ... }
    // [ 闭包 ]
    // |参数列表| { ... }
    // || { ... }   >>> 无参数的闭包

    // let 闭包变量 = |参数列表| { ... };

    // let double = |x| x * 2;
    // let add = |x, y| x + y;
    // println!("double(2) = {}", double(2));
    // println!("add(2, 3) = {}", add(2, 3));
    // let v = 3;
    // let add_v = |x| x + v;
    // println!("add_v(2) = {}", add_v(2));

    // 捕获
    // 闭包本质上很灵活，闭包可以在没有类型标注的情况下运行，可以移动，借用
    // 闭包可以通过一下方式捕获变量：
    // 1、通过引用 &T
    // 2、通过可变引用 &mut T
    // 3、通过值 T

    // 总结：
    // 1、闭包就是在一个在函数内创建立即调用的另一个函数
    // 2、闭包是一个匿名函数
    // 3、闭包虽然没有函数名，但可以把整个闭包赋值一个变量，通过调用该变量来完成闭包的调用
    // 4、闭包不用声明返回值，但它可以有返回值，而且使用最后一条语句的执行结果作为返回值，闭包的返回值可以赋值给一个变量
    // 5、闭包又称为内联函数，可以让闭包访问外层函数里的变量

    // 闭包使用
    let add = |x, y| x + y;
    let res = add(1, 2);
    println!("res = {}", res);

    receives_closure(add);

    let y = 22;
    receives_closure2(|x| x + y);

    let clo = returns_closure();
    println!("返回闭包 = {}", clo(6));

    let rr_clo = rr_closure(add, 3);
    println!("rr_clo = {}", rr_clo(3));

    let rr_clo2 = rr_closure2(add, 6);
    println!("rr_clo2 = {}", rr_clo2(3));
}

fn receives_closure<F>(closure: F)
where
    F: Fn(i32, i32) -> i32,
{
    let res = closure(1, 2);
    println!("闭包作为参数执行的结果 = {}", res);
}

// 闭包捕获变量
fn receives_closure2<F>(closure: F)
where
    F: Fn(i32) -> i32,
{
    let res = closure(1);
    println!("闭包作为参数执行的结果 = {}", res);
}

// 返回闭包
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 6
}

// 参数和返回值都有闭包
fn rr_closure<F>(f: F, x: i32) -> impl Fn(i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    move |y| f(x, y)
}

fn rr_closure2<F, X, Y, Z>(f: F, x: X) -> impl Fn(Y) -> Z
where
    F: Fn(X, Y) -> Z,
    X: Copy,
{
    move |y| f(x, y)
}

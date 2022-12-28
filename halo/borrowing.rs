fn main() {
    // rust 中 borrowing 就是一个函数中的变量传递给另外一个函数作为参数暂时使用。
    // 也会要求函数参数离开自己作用域的时候将所有权还给当初传递给它的变量
    // &变量名   要定义参数的的时候这样定义
    let list = vec!["hello", "rust"];
    let list2 = list;
    show(&list2);
    println!("list2: {:?}", list2);

    // 可变的借用 主要还是 mut 关键字
    let mut list3 = list2;
    show2(&mut list3);

    // 如果我们要在 borrowing 的时候改变其中的值
    // 1、变量要用 mut
    // 2、函数参数为可变的，需要用 &mut 关键字
    // 3、传递参数的时候，也要用 &mut 关键字
}

fn show(v: &Vec<&str>) {
    println!("show v: {:?}", v)
}

fn show2(v: &mut Vec<&str>) {
    v[0] = "halo";
    println!("show2 v: {:?}", v)
}

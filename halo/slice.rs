fn main() {
    // 切片是指向一段连续内存的指针，在 rust 中连续内存的区间存储的数据结构：数组（array）、字符串（string）、向量（vector）
    // 切片可以和它们一起使用，切片也使用数字索引访问数据。下标索引从 0 开始，slice 可以指向数据的一部分，越界的下标会导致 panic
    // 切片是运行时才能确定的，并不像数组那样是编译时就确定的

    // let 切片值 = &变量名[起始位置..结束位置];
    // [起始位置..结束位置]，这是一个左闭右开的区间，起始位置包含在内，结束位置不包含在内
    // 起始位置最小值是 0
    // 结束位置是数组、字符串、向量的长度

    let mut v = Vec::new();
    v.push("hello");
    v.push("rust");
    v.push("xxx");

    println!("v: {:?}", v);

    v[0] = "halo";
    println!("v: {:?}", v);

    println!("v[1]: {:?}", v[1]);
    let v1 = &v[0..2];
    // v = ["halo", "rust", "xxx"]，而 v1 = ["halo", "rust"]
    println!("v1: {:?}", v1);

    // 切片当参数
    let v2 = ["hello", "rust", "xxx"];
    show_slice(&v2);

    // 可变切片
    let mut v3 = Vec::new();
    v3.push("hello");
    v3.push("rust");
    v3.push("xxx");
    show_slice2(&mut v3[0..2]);
}

fn show_slice(s: &[&str]) {
    println!("show_slice s: {:?}", s)
}

fn show_slice2(s: &mut [&str]) {
    s[0] = "halo";
    println!("show_slice2 s: {:?}", s)
}

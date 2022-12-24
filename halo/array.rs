fn main() {
    // 数组是存储以一系列数据，拥有相同类型 T 的对象的集合，在内存中是连续存储的。使用 [] 来创建，且它们的大小在编译时会被确定
    // 数组下标是从 0 开始，是在栈上分配的，数组可以自动被借用成为切片 slice

    let arr: [&str; 3] = ["halo", "xxx", "rust"];
    println!("{:?}", arr);

    // 数组长度
    println!("数组长度 {}", arr.len());

    // 遍历数组
    for item in arr {
        println!("{}", item);
    }

    for item in arr.iter() {
        println!("{}", item);
    }

    // arr[0] = "hello";
    // err >>> cannot assign to `arr[_]`, as `arr` is not declared as mutable

    // 数组作为参数(值传递)
    print_arr(arr);

    // 数组作为参数(引用传递)
    let mut arr2: [&str; 3] = ["halo", "xxx", "rust"];
    print_arr_ref(&mut arr2);
    println!("{:?}", arr2);
}

// 值传递
fn print_arr(a: [&str; 3]) {
    println!("{:?}", a)
}

// 引用传递
fn print_arr_ref(a: &mut [&str; 3]) {
    let l = a.len();
    for i in 0..l {
        a[i] = "hello";
    }
}

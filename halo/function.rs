fn halo() {
    println!("halo xxx")
}

fn say() -> String {
    return String::from("rust");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn change_val(mut v: i32) {
    v = v * 2;
    println!("值传递-函数内部 v = {}", v);
}

fn change_val2(v2: &mut i32) {
    *v2 = *v2 * 2;
    // * 号用于访问变量 v2 指向的内存位置上存储的变量的值，也称为解引用，* 号也称为解引用操作符
    println!("引用传递-函数内部 v2 = {}", v2);
}

fn print_name(name: String) {
    println!("halo {}", name)
}

fn main() {
    halo();
    println!("rust = {}", say());
    println!("1 + 2 = {}", add(1, 2));
    let v = 10;
    change_val(v);
    println!("值传递-函数外部 v = {}", v);
    let mut v2 = 10;
    change_val2(&mut v2);
    println!("引用传递-函数外部 v2 = {}", v2);
    let name: String = String::from("xxx");
    print_name(name);
    // println!("使用 print_name 函数后 {}", name)
    //  let name: String = String::from("xxx");
    //      ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
    //  print_name(name);
    //             ---- value moved here
    //  println!("使用 print_name 函数后 {}", name)
    //                                      ^^^^ value borrowed here after move
}

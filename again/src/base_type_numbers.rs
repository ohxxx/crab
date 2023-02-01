use num::complex::Complex;

pub fn base_type_numbers() {
    // [ 整数类型 ]
    // 长度	      有符号类型	  无符号类型            数字范围
    // 8 位	         i8	           u8        i8: -128 ~ 127, u8: 0 ~ 255
    // 16 位	     i16	       u16      i16: -32768 ~ 32767, u16: 0 ~ 65535
    // 32 位	     i32	       u32      i32: -2147483648 ~ 2147483647, u32: 0 ~ 4294967295
    // 64 位	     i64	       u64      i64: -9223372036854775808 ~ 9223372036854775807, u64: 0 ~ 18446744073709551615
    // 128 位	     i128	      u128      i128: -170141183460469231731687303715884105728 ~ 170141183460469231731687303715884105727, u128: 0 ~ 340282366920938463463374607431768211455
    // 视架构而定	  isize	       usize     isize: -2147483648 ~ 2147483647, usize: 0 ~ 4294967295

    // 有符号：表示数字既可以取正数又可以取负数，有符号数字以补码形式存储
    // 有符号类型规定的数字范围是 -(2^(n-1)) 到 2^(n-1)-1，其中 n 是类型的位数
    // 无符号类型规定的数字范围是 0 到 2^n-1，其中 n 是类型的位数

    // 整型字面量
    // 数字字面量   实例
    // 十进制	   98_222 = 98222
    // 十六进制	    0xff = 255
    // 八进制   	0o77 = 63
    // 二进制  	  0b1111_0000 = 240
    // 字节（u8）	  b'A' = 65

    // rs 默认使用 i32 类型

    wrapping_test();

    // [ 浮点数类型 ]
    // 浮点类型数字是带有小数点的数字，浮点类型有两种：f32 和 f64
    // f32 类型的浮点数占用 4 个字节，也就是 32 位，小数点后有 7 位有效数字
    // f64 类型的浮点数占用 8 个字节，也就是 64 位，小数点后有 15 位有效数字

    // 避免浮点数陷阱需要遵循以下规则：
    // 1. 不要比较两个浮点数是否相等
    // 2. 当结果在数学上可能存在未定式时，需要格外小心
    float_test();

    // [ NaN ]
    // rs 的浮点数类型使用 NaN(not a number)来处理对负数取平方根 -42.1.sqrt()
    // NaN 是一个特殊的浮点数，它不等于任何值，包括它自己

    // [ 数字运算 ]
    // + - * / % 逻辑运算符
    number_operation_test();

    // [ 位运算 ]
    // 运算符	      说明
    // & 位与	相同位置均为1时则为1，否则为0
    // | 位或	相同位置只要有1时则为1，否则为0
    // ^ 异或	相同位置不相同则为1，相同则为0
    // ! 位非	把位中的0和1相互取反，即0置为1，1置为0
    // << 左移	所有位向左移动指定位数，右位补0
    // >> 右移	所有位向右移动指定位数，带符号移动（正数补0，负数补1）
    bit_operation_test();

    // [ 序列 Range ]
    // 1..5 表示 1, 2, 3, 4
    // 1..=5 表示 1, 2, 3, 4, 5
    range_test();

    // [ 有理数和复数 ]
    // 有理数：Rational
    // 复数：Complex
    rational_and_complex_test();
}

fn wrapping_test() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("a: {}, b: {}", a, b);
}

fn float_test() {
    let x = 2.01;
    let y: f32 = 3.01;
    println!("x: {}, y: {}", x, y);

    // assert!(0.1 + 0.2 == 0.3) // false

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // let x1 = (-42.0_f32).sqrt();
    // assert_eq!(x1, x1);
}

fn number_operation_test() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}",
        sum, difference, product, quotient, remainder
    )
}

fn bit_operation_test() {
    let a: i32 = 2; // 二进制为 0000 0010
    let b: i32 = 3; // 二进制为 0000 0011
    let c = a & b; // 0000 0010
    let d = a | b; // 0000 0011
    let e = a ^ b; // 0000 0001
    let f = !a; // 1111 1101
    let g = a << 1; // 0000 0100
    let h = a >> 1; // 0000 0001
    println!(
        "a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}, h: {}",
        a, b, c, d, e, f, g, h
    );
}

fn range_test() {
    for i in 1..5 {
        println!("{}", i);
    }
}

fn rational_and_complex_test() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.11, 22.22);
    let res = a + b;
    println!("a: {}, b: {}, res: {}", a, b, res)
}

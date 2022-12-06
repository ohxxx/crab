fn main() {
    let n1 = 100;
    let n2: u32 = 200;
    let n3: i32 = -300; // i32 是默认的整型
    let n4: isize = 400;
    let n5: usize = 500;
    let n6: f32 = 600.0;
    let n7: f64 = 700.0;

    println!(
        "n1 = {}, n2 = {}, n3 = {}, n4 = {}, n5 = {}, n6 = {}, n7 = {}",
        n1, n2, n3, n4, n5, n6, n7
    );
    // >>> n1 = 100, n2 = 200, n3 = -300, n4 = 400, n5 = 500, n6 = 600, n7 = 700

    // 错误示例
    // let e1: i8 = 192;
    // println!("e1 = {}", e1);
    // = note: `#[deny(overflowing_literals)]` on by default
    // = note: the literal `192` does not fit into the type `i8` whose range is `-128..=127`
    // = help: consider using the type `u8` instead

    // 整数可以分为【有符号整型】和【无符号整型】
    // 【有符号整型】：
    //    英文 signed，既可以存储正数，也可以存储负数
    //    存储的最小值是 -2^(n-1)，最大值是 2^(n-1)-1
    // 【无符号整型】：
    //    英文 unsigned，只能存储正数
    //    存储的最小值是 0，最大值是 2^n-1

    // 大小     有符号      无符号
    // 8 bit     i8	        u8
    // 16 bit	   i16	      u16
    // 32 bit	   i32	      u32
    // 64 bit	   i64	      u64
    // 128 bit	 i128	      u128
    // Arch	     isize	    usize
}

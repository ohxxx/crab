fn main() {
    // 枚举(enum 或 enumeration)允许你通过列举可能的成员来定义一个枚举类型，例如扑克牌花色
    // enum 枚举名称 {
    //   variant 1
    //   variant 2
    // }

    let red = PokerSuit::红桃;
    println!("red: {:?}", red);

    // Option 枚举经常用在函数中的返回值，它可以表示有返回值，也可以表示没有返回值。如果有返回值
    // 可以使用返回 Some(value)，如果没有返回值可以使用 None
    let res = is_hearts(String::from("红桃"));
    let res2 = is_hearts(String::from("黑桃"));
    println!("当前 res 花色是红桃: {:?}", res);
    println!("当前 res2 花色是红桃: {:?}", res2);

    // match 语句
    // 判断一个枚举变量的值，唯一的操作符是 match
    let res3 = show_poker_suit(PokerSuit::红桃);
    println!("当前 res3 的花色是: {:?}", res3);

    //带数据类型的枚举
    let res4 = Student::Name(String::from("张三"));
    println!("res4: {:?}", res4);
}

// #[derive(Debug)] 注解的作用，就是让派生自 Debug

#[derive(Debug)]
enum PokerSuit {
    梅花,
    方块,
    红桃,
    黑桃,
}

// Option 枚举
#[derive(Debug)]
enum Option<T> {
    Some(T), //用于返回一个值
    None,    //用于返回 null,用 None 代替 null
}

fn is_hearts(s: String) -> std::option::Option<bool> {
    if s == "红桃" {
        Some(true)
    } else {
        None
    }
}

fn show_poker_suit(s: PokerSuit) -> std::option::Option<PokerSuit> {
    match s {
        PokerSuit::梅花 => Some(PokerSuit::梅花),
        PokerSuit::方块 => Some(PokerSuit::方块),
        PokerSuit::红桃 => Some(PokerSuit::红桃),
        PokerSuit::黑桃 => Some(PokerSuit::黑桃),
        _ => None,
    }
}

// 待数据类型的枚举
#[derive(Debug)]
enum Student {
    Name(String),
    Age(u8),
    Score(u8),
}

fn main() {
    // if 语句
    // if...else 语句
    // else...if 和嵌套 if 语句
    // match 语句

    if true {
        println!("halo xxx")
    }

    let v1 = 10;
    if v1 > 10 {
        println!("v1 > 10")
    } else {
        println!("v1 <= 10")
    }

    let v2 = 20;
    if v2 > 10 {
        println!("v2 > 10")
    } else if v2 > 20 {
        println!("v2 > 20")
    } else {
        println!("v2 <= 10")
    }

    // match 语句是有返回值的，它把 匹配值 后执行的最后一条语句的结果当做返回值
    // match variable_expression {
    //   constant_expr1 => {
    //     语句1;
    //   }
    //   constant_expr2 => {
    //     语句2;
    //   }
    //   _ => {
    //     默认
    //     其他语句
    //   }
    // }

    let code = 200;
    let res = match code {
        200 => "成功",
        400 => "服务端错误",
        500 => "服务器错误",
        _ => "未知错误",
    };
    println!("当前返回 {}", res)
}

fn main() {
    // 结构体（struct）可以由不同类型组成。使用 struct 关键字来创建。
    // struct 是 structure 的缩写，意思是结构体。结构体可以作为另一个结构体的字段，结构体可以嵌套。

    // 元组结构体 tuple struct，事实上就是具名元组而已
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // 经典的 C 语言风格的结构体
    // struct 结构体名称 {
    //     ...
    // }
    struct Point {
        x: i32,
        y: i32,
    }
    let origin = Point { x: 0, y: 0 };

    // 单元结构体 unit struct，不带字段，在泛型中很有用
    struct Unit;
    let unit = Unit;

    // 定义结构体
    // struct 结构体名称 {
    //     字段名称: 字段类型,
    //     字段名称: 字段类型,
    //     ...
    // }

    // 创建结构体实例
    // let 结构体实例名称 = 结构体名称 {
    //     字段名称: 字段值,
    //     字段名称: 字段值,
    //     ...
    // }

    let mut s = Student {
        name: String::from("xxx"),
        age: 18,
        score: 100,
    };
    println!("s: {:?}", s);

    // 访问实例属性
    // 实例名称.属性
    println!("s.name: {}", s.name);

    // 修改实例属性
    // 实例名称.属性 = 新值
    s.name = String::from("rust");
    println!("修改的 s.name: {}", s.name);
    println!("修改后的 s: {:?}", s);

    // 结构体做参数
    show(s);

    // 结构体作为函数的返回值
    let s2 = student_info(String::from("xxx"), 18, 100);
    println!("传入的学生信息: {:?}", s2);

    // 方法 method 是依附于对象的函数，这些方法通过关键字 self 来访问对象中的数据和其他。方法在 impl 代码块中定义。
    // 与函数的区别：
    // 函数：可以直接调用，同一个程序中不能出现两个相同函数签名的函数，应为函数不归属任何 owner
    // 方法：归属某一个 owner，不同的 owner 可以有相同的方法签名

    // impl 结构体 {
    //   fn 方法名(&self, 参数列表) -> 返回值类型 {
    //     ...
    //   }
    // }
    // impl 是 implement 的缩写，意识是”实现“的意思
    // self 是“自己“意思，&self 表示当前结构体的实例，& 表示引用，self 表示当前结构体的实例。&self 也是结构体普通方法固定的第一个参数，其他参数可选
    // 结构体方法的作用域仅限于结构体内部，结构体外部无法调用结构体方法
    impl Student {
        fn show_name(&self) -> String {
            return self.name.clone();
        }
    }
    println!("当前学生的名字: {}", s2.show_name());

    // 结构体静态方法
    // fn 方法名(参数: 数据类型...) -> 返回值类型 {
    // ...
    // }
    // 调用方法：结构体名称::方法名(参数列表)
    // 静态方法不需要 self 参数，不需要实例化就可以调用
    // 静态方法的作用域是全局的，结构体外部也可以调用

    impl Student {
        fn show_age(age: i32) -> i32 {
            return age;
        }
    }
    let s3 = Student::show_age(20);
    println!("当前学生的年龄: {}", s3);

    // 单元结构体
    // unit type 是一个类型，有且仅有一个值：()，单元类型() 类似于 c/c++/java 中的 void。当一个函数并不需要返回值的时候，rust 则返回()
    // 但语法层面上，void 仅仅只是一个类型，该类型没有任何值，而单元类型()即是一个类型，同时又是该类型的值

    // 实例化一个元组结构体
    let pair = (String::from("rust"), 100);
    println!("pair: {:?}", pair);
    // 访问元组结构体的属性
    println!("pair.0: {}", pair.0);
    // 解构一个元组结构体
    let (name, score) = pair;
    println!("name: {}, score: {}", name, score);
}

fn show(s: Student) {
    println!("需要显示的学生信息: {:?}", s);
}

fn student_info(name: String, age: i32, score: i32) -> Student {
    return Student { name, age, score };
}

// 结构体初始化，其实就是对结构体中的各个元素进行赋值
#[derive(Debug)]
struct Student {
    name: String,
    age: i32,
    score: i32,
}

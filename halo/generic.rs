fn main() {
    // 泛型是运行时指定数据类型的一种机制。好处是通过高度的抽象，使用一套代码应用多种数据类型。
    // 比如我们的向量可以使用数值类型，也可以使用字符串类型，泛型是可以保证数据安全和类型安全，同时也可以减少代码量。
    // rust 中的泛型主要包含：泛型集合、泛型结构体、泛型函数、泛型枚举和特质
    // 使用 <T> 语法来实现泛型，其中 T 可以是任意数据类型

    // 泛型集合
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // v.push("6");
    //   ---- ^^^ expected `i32`, found `&str`
    println!("v: {:?}", v);
    // 因为 Vec<i32> 指定了集合中的数据类型为 i32，所以我们不能将字符串类型的数据放进去，否则会报错，使我们的数据类型安全

    // 泛型结构体
    // struct 结构体名称<T> {
    //   元素: T
    // }
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    let p: Point<i32> = Point { x: 1, y: 2 };
    println!("p: {:?}", p);
    let p2: Point<f64> = Point { x: 1.1, y: 2.2 };
    println!("p2: {:?}", p2);

    // 特质 Trait
    // 可以把这个特质对标其他语言的接口，都是对行为的抽象。使用 trait 关键字来定义特质，可以包含具体的方法，也可以包含抽象的方法。使用 impl 关键字来实现特质
    // trait 特质名称 {
    //   fn 方法名(&self) -> 返回值类型;
    // }
    trait some_trait {
        // 没有任何实现的虚方法，称为抽象方法
        fn method1(&self);
        // 有实现的方法，称为具体方法
        fn method2(&self) {
            println!("method2");
        }
    }
    // impl 特质名称 for 结构体名称 {
    //   fn 方法名(&self) -> 返回值类型 {
    //     // 方法实现
    //   }
    // }
    impl some_trait for Point<i32> {
        fn method1(&self) {
            println!("method1!!!");
        }
        fn method2(&self) {
            println!("method2: x = {}, y = {}", self.x, self.y);
        }
    }
    let p: Point<i32> = Point { x: 1, y: 2 };
    p.method1();
    p.method2();

    // 泛型函数
    // 主要是指参数是泛型类型，不要求所有参数都必须是泛型参数
    // fn 方法名<T[:特质名称]>(参数1: T, ...) {
    //   函数实现代码
    // }
    struct Book {
        name: String,
        price: f64,
    }

    trait ShowBook {
        fn show(&self);
    }

    impl ShowBook for Book {
        fn show(&self) {
            println!("name: {}, price: {}", self.name, self.price);
        }
    }

    let book = Book {
        name: String::from("rust"),
        price: 100.2,
    };
    book.show();

    fn show_book<T: ShowBook>(book: T) {
        book.show();
    }

    show_book(book);
}

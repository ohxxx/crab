use std::ops::Deref;

fn main() {
    // rust 可以在堆上存储数据，rust 中某些数据类型就是默认存储在堆上的，比如 向量 Vector 和 字符串对象 String
    // rust 把指针封装在如下两个特质 Trait 中

    // 特质名	       包	                Description
    // Deref	std::ops::Deref	     用于创建一个只读智能指针，例如 *v
    // Drop	  std::ops::Drop	     智能指针超出它的作用域范围时会回调该特质的 drop() 方法。 类似于其它语言的 析构函数。

    // 当一个结构体实现了以上的接口后，它们就不再是普通的结构体了
    // rust 提供了在堆上存储数据的能力并把这个能力封装到了 Box 中
    // 把这种栈上数据搬到堆上的能力，称之为装箱

    // [ Box 指针 ]
    // Box 指针可以把数据存储在 堆(heap) 上，而不是 栈(stack) 上，这就是装箱 box
    // 栈还是包含指向堆上数据的指针

    let a = 6; // 默认保存在栈上
    let b = Box::new(a); // 使用 Box 后数据保存在堆上
    println!("a = {}, b = {}", a, b);

    // 访问 Box 存储的数据
    // 想要访问 Box 存储的数据，可以是用 * 访问，这个操作也叫 解引用，* 也叫做解引用符
    let p1 = 166; // 值类型数据
    let p2 = Box::new(p1); // p2 是一个智能指针，指向堆上存储的数据 166
    println!("{}", 166 == p1); // 这个是基础类型比较，只是比较值是否相等
    println!("{}", 166 == *p2); // p2 是一个智能指针，是引用类型，想访问到具体的值，就要对 p2 进行解引用操作
                                // 为了访问 p2 存储的具体数据，需要解引用

    // [ Deref ]
    // 实现 Deref 特质需要我们实现 deref() 方法，deref() 方法返回一个指向结构体内部数据的指针
    struct CustomBox<T> {
        value: T,
    }

    impl<T> CustomBox<T> {
        fn new(v: T) -> CustomBox<T> {
            CustomBox { value: v }
        }
    }

    impl<T> Deref for CustomBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.value
        }
    }

    let x = 666;
    let y = CustomBox::new(x); // 调用静态方法 new() 创建一个结构体实例
    println!("666 == x is {}", 666 == x);
    println!("666 == *y is {}", 666 == *y); // 解引用操作
    println!("x == *y is {}", x == *y); // 解引用操作

    // [ Drop ]
    // Drop Trait 只有一个方法 drop，当实现了 Drop Trait 的结构体，在超出它的作用域范围时会触发 drop 方法
    impl<T> Drop for CustomBox<T> {
        fn drop(&mut self) {
            println!("Dropping CustomBox!");
        }
    }
}

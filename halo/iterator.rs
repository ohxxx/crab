fn main() {
    // 迭代器就是把集合中所有元素按照顺序一个接一个的传递给处理逻辑
    // Iterator 特质有两个函数
    // 一个是 iter()，用于返回一个迭代器对象，也称之为项 items
    // 一个是 next()，用于返回迭代器中的下一个元素。如果已经迭代到集合的尾部（最后一个项后面），则返回 None

    let v = vec![1, 2, 3, 4, 5];
    let mut it = v.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());

    // 通过 for 循环来遍历集合中的元素
    let v = vec![1, 2, 3, 4, 5];
    for i in v.iter() {
        println!("{}", i);
    }

    // 方法	           描述
    // iter()	      返回一个只读可重入迭代器，迭代器元素的类型为 &T
    // into_iter()	返回一个只读不可重入迭代器，迭代器元素的类型为 T
    // iter_mut()	  返回一个可修改可重入迭代器，迭代器元素的类型为 &mut T
}

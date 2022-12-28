use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // rust 语言标准库提供了通用的数据结构的实现。包括：向量（Vector）、哈希表（HashMap）、哈希集合（HashSet）

    // 向量（Vector）
    // 在 rust 中定义了结构体 Vec 用于表示一个向量。向量和数组很相似
    // 数组长度是编译时确定的，定义后就不能改变了
    // 而向量是可以动态增长的，可以在运行时增加或者减少元素

    // 向量是在内存中开辟了一段连续的内存空间来存储元素的
    // 特点：
    // 1、向量中的元素都是相同类型元素的集合
    // 2、长度可变，运行时可以增加和减少
    // 3、使用索引查找元素，索引从 0 开始
    // 4、添加元素时，添加到向量的末尾
    // 5、向量的内存在堆上，长度可动态变化

    // 创建向量
    // 1、使用 new() 静态方法创建
    // let mut 向量的变量名称 = Vec::new();
    // 2、使用 vec! 宏创建
    // let 向量的变量名称 = vec![元素1, 元素2, 元素3, ...];

    // 向量的使用方法
    // 方法	             说明
    // new() 	     创建一个空的向量的实例
    // push()	     将某个值 T 添加到向量的末尾
    // remove()	   删除并返回指定的下标元素。
    // contains()	 判断向量是否包含某个值
    // len()	     返回向量中的元素个数
    // ...           更多方法请参考官方文档: https://doc.rust-lang.org/std/vec/struct.Vec.html
    let mut v = Vec::new();
    v.push("111");
    v.push("222");
    v.push("333");
    println!("原始向量 v: {:?}", v);
    println!("向量长度 v.len(): {:?}", v.len());
    println!("向量是否包含 v.contains(\"111\"): {:?}", v.contains(&"111"));
    println!("移除向量中某个值 v.remove(0): {:?}", v.remove(0));
    println!("现在的向量 v: {:?}", v);
    println!("向量第二个元素： {}", v[1]);
    // 遍历向量
    for i in v {
        println!("遍历向量： {}", i);
    }

    println!("----------------------------------------");

    // 哈希表（HashMap）
    // HashMap 就是键值对的结合，键是唯一的，值可以重复
    // 使用 HashMap 需要导入 std::collections 模块
    // 创建 HashMap：
    // let mut map = HashMap::new();

    // 方法	           说明
    // insert()	     插入/更新一个键值对到哈希表中，如果数据已经存在则返回旧值，如果不存在则返回 None
    // len()	       返回哈希表中键值对的个数
    // get()	       根据键从哈希表中获取相应的值
    // iter()	       返回哈希表键值对的无序迭代器，迭代器元素类型为 (&’a K, &’a V)
    // contains_key	 如果哈希表中存在指定的键则返回 true 否则返回 false
    // remove()	     从哈希表中删除并返回指定的键值对
    // ...           更多方法请参考官方文档: https://doc.rust-lang.org/std/collections/struct.HashMap.html

    let mut map = HashMap::new();
    map.insert("name", "张三");
    map.insert("age", "18");
    map.insert("score", "100");
    println!("原始哈希表 map: {:?}", map);
    println!("哈希表长度 map.len(): {:?}", map.len());
    println!(
        "哈希表是否包含 map.contains_key(\"name\"): {:?}",
        map.contains_key(&"name")
    );
    println!("哈希表中的值 map.get(\"name\"): {:?}", map.get(&"name"));
    println!(
        "移除哈希表中某个值 map.remove(\"name\"): {:?}",
        map.remove(&"name")
    );
    println!("现在的哈希表 map: {:?}", map);
    // 遍历哈希表
    for (k, v) in map.iter() {
        println!("遍历哈希表：{}: {}", k, v);
    }

    println!("----------------------------------------");

    // 哈希集合（HashSet）
    // HashSet 是相同数据类型的集合，它是没有重复值的，如果集合中已经存在相同的值值，则会插入失败

    // 创建 HashSet：
    // let mut set = HashSet::new();

    // 方法	              描述
    // insert()	       插入一个值到集合中 如果集合已经存在值则插入失败
    // len()	         返回集合中的元素个数
    // get()	         根据指定的值获取集合中相应值的一个引用
    // iter()	         返回集合中所有元素组成的无序迭代器 迭代器元素的类型为 &'a T，是无序的
    // contains    	   判断集合是否包含指定的值
    // remove()        从结合中删除指定的值
    // ...             更多方法请参考官方文档: https://doc.rust-lang.org/std/collections/struct.HashSet.html

    let mut set = HashSet::new();
    set.insert("111");
    set.insert("222");
    set.insert("333");
    println!("原始哈希集合 set: {:?}", set);
    println!("哈希集合长度 set.len(): {:?}", set.len());
    println!(
        "哈希集合是否包含 set.contains(\"111\"): {:?}",
        set.contains(&"111")
    );
    println!(
        "移除哈希集合中某个值 set.remove(\"111\"): {:?}",
        set.remove(&"111")
    );
    set.insert("666"); // 插入重复值
    set.insert("666"); // 插入重复值
    println!("现在的哈希集合 set: {:?}", set);
    // 遍历哈希集合
    for i in set.iter() {
        println!("遍历哈希集合：{}", i);
    }
}

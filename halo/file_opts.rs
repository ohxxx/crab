use std::io::Read;
use std::io::Write;

fn main() {
    // rust 使用结构体 File 来描述/展现一个文件
    // 所有结构体 File 的操作方法都会返回一个 Result 枚举

    // 以下是一些常用的文件方法
    // 模块	                         方法	            说明
    // std::fs::File	             open()	        静态方法，以 只读 模式打开文件
    // std::fs::File	             create()	      静态方法，以 可写 模式打开文件。 如果文件存在则清空旧内容 如果文件不存在则新建
    // std::fs::remove_file	      remove_file()	  从文件系统中删除某个文件
    // std::fs::OpenOptions	        append()	    设置文件模式为 追加
    // std::io::Writes	           write_all()	  将 buf 中的所有内容写入输出流
    // std::io::Read	           read_to_string()	读取所有内容转换为字符串后追加到 buf 中

    // 创建文件
    let file2 = std::fs::File::create("halo.txt").unwrap();
    std::fs::File::create("halo2.txt").unwrap();
    println!("文件创建成功\n:{:?}", file2);
    // 打开文件
    let file = std::fs::File::open("halo.txt").unwrap();
    println!("文件打开成功\n:{:?}", file);
    // 删除文件
    std::fs::remove_file("halo2.txt").unwrap();
    // 追加内容
    let mut file3 = std::fs::OpenOptions::new()
        .append(true) // 用于将文件的打开模式设置为追加模式
        .open("halo.txt")
        .expect("文件打开失败");
    file3.write("\nhalo xxx".as_bytes()).expect("写入失败");
    file3.write_all("halo xxx1".as_bytes()).expect("写入失败"); // write_all() 方法并不会在写入结束后自动写入换行符
    file3.write_all("\nhalo xxx2".as_bytes()).expect("写入失败"); // write_all() 方法并不会在写入结束后自动写入换行符
    println!("文件追加成功");
    // 读取内容
    let mut file4 = std::fs::File::open("halo.txt").unwrap();
    let mut buf = String::new();
    file4.read_to_string(&mut buf).unwrap();
    println!("文件读取成功\n{}", buf);
}

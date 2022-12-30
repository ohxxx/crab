use books::{books_info::show_books, mod1::mod2::mod3::mod3_func};

fn main() {
    // 在代码组织上，比模块更高级的是 crate，一个 crate 可以存放多个模块，在  rust 中 crate 是基本编译单元
    // 分为 可执行二进制文件（包含 main 函数作为程序入口）或者一个库

    // crates.io 是 rust 官方提供的第三方包地址，使用 cargo install 命令从 crates.io 安装包，有点类似 npm

    // 定义模块
    // mod 模块名 {
    //     // 模块内容z
    // }

    // 使用模块
    // use 公开的模块名::公开的方法名;

    show_books();
    mod3_func()
}

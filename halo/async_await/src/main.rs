// use async_std::task::{sleep, spawn};
use std::{future::Future, time::Duration};

use async_std::task::block_on;

// fn mod1() {
//     for i in 1..=5 {
//         println!("mod1 {}", i);
//         sleep(Duration::from_millis(500));
//     }
// }

// fn mod2() {
//     for i in 1..=5 {
//         println!("mod2 {}", i);
//         sleep(Duration::from_millis(1000));
//     }
// }

// fn main() {
//     // 这样会先执行mod1，然后再执行mod2
//     // mod1();
//     // mod2();

//     // 使用spawn创建线程，这样会同时执行mod1和mod2
//     let mod1_spawn = spawn(mod1);
//     let mod2_spawn = spawn(mod2);
//     mod1_spawn.join().unwrap();
//     mod2_spawn.join().unwrap();
// }

// async fn mod3() {
//     for i in 1..=5 {
//         println!("mod3 {}", i);
//         sleep(Duration::from_millis(500)).await;
//     }
// }

// async fn mod4() {
//     for i in 1..=5 {
//         println!("mod4 {}", i);
//         sleep(Duration::from_millis(1000)).await;
//     }
// }

// #[async_std::main]
// async fn main() {
//     let mod3_spawn = spawn(mod3());
//     mod4().await;
//     mod3_spawn.await;

//     // async fn 函数名称() -> 返回值类型 {} 实际上返回的就是 impl std::future::Future<Output = 返回值类型>
// }

async fn book() -> String {
    String::from("rust book")
}

fn book1() -> impl Future<Output = String> {
    async {
        let b = book().await;
        b + " 666"
    }
}

fn book2() -> impl Future<Output = String> {
    let b = |n: String| async move {
        let n2: String = book1().await;
        n2 + &*n
    };
    b(String::from(" 777"))
}

fn main() {
    let b = block_on(book2());
    println!("{}", b);
}

use std::time::Duration;

fn main() {
    // 现代的操作系统，是一个多任务操作系统，系统可以管理过个程序的运行，一个程序往往有一个或多个进程，而一个进程则有一个或多个线程
    // 让一个进程可以运行多个线程的机制叫做多线程
    // 一个进程一定有一个主线程，主线程之外创建出来的线程叫做子线程
    // 多线程（并发）编程的一个重要的思想就是：程序不同的部分可以同时独立运行互不干扰

    // 创建一个线程
    // std::thread::spawn()
    // spawn() 函数原型
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // 参数 f 是一个闭包，是线程要执行的代码

    // 子线程
    std::thread::spawn(|| {
        for i in 1..10 {
            println!("halo, 数字 {} 是来自[ 子线程 ]", i);
            std::thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程
    for i in 1..5 {
        println!("halo, 数字 {} 是来自[ 主线程 ]", i);
        std::thread::sleep(Duration::from_millis(1));
    }

    // 当主线程结束时，子线程也会结束

    // thread::sleep() 会让当前线程睡眠一段时间，某个线程睡眠的时候会让出 CPU，可以让不同的线程交替执行，要看操作系统的调度策略

    // join() 函数
    // 如果想让子线程结束后，主线程再结束，可以使用 join() 函数，把子线程加入到主线程中
    // spawn<F, T>(f: F) -> JoinHandle<T>

    // 子线程
    let handler = std::thread::spawn(|| {
        for i in 1..10 {
            println!("[ 子线程 ]：{}", i);
            std::thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程
    for i in 1..5 {
        println!("[ 主线程 ]：{}", i);
        std::thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}

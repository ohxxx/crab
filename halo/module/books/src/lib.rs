pub mod books_info {
    pub fn show_books() {
        println!("显示书籍");
    }
}

pub mod mod1 {
    pub mod mod2 {
        pub mod mod3 {
            pub fn mod3_func() {
                println!("哈哈哈哈哈哈");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

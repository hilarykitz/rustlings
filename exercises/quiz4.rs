// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

// mod gimme {
//     pub fn print(val: expr) {
//         println!("Hello {}", val);
//     }
// }

#[macro_export]
macro_rules! maccers {
    ($val: expr) => {
        concat!("Hello ", $val);
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_my_macro_world() {
        assert_eq!(maccers!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(maccers!("goodbye!"), "Hello goodbye!");
    }
}

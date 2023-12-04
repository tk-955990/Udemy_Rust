//! ライブラリクレートのドキュメント

/// say_helloの関数のドキュメント
pub fn say_hello() {
    println!("Hello!");
}

/// **say_good_by**関数のドキュメント
/// ### 使用例
/// ```
/// fn main(){
///     udemy_rust::say_good_by();
/// }
///```
pub fn say_goog_bye() {
    println!("Good bye!");
}

/// ２つの関数を足し合わせます
///
/// ```
/// let result = udemy_rust::add(1, 2);
/// assert_eq!(result, 2);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// cargo doc --no-deps --open

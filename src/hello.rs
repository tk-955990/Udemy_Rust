pub fn run() {
    println!("Hello neovim");
    println!("Hello, World!");
    print!("Hello");
    print!(", World!");
    println!("");
    println!("Hello, {}", "Student!");
    let a = 1;
    let b: i64 = 5;
    let c = 0.5;
    let d: f32 = 0.8;
    const A: i32 = 100;
    const B: &str = "Hello!";
    let e = a as f64 + c;
    let f = a + c as i32;
    let g = a as i32 + c as i32;
    println!("The A is: {}", A);
    println!("The B is: {}", B);
    println!("The a(i32) is: {}", a);
    println!("The b(i64) is: {}", b);
    println!("The c(f64) is: {}", c);
    println!("The d(f32) is: {}", d);
    println!("a(f64) + c(f64) is: {}", e);
    println!("a(i32) + c(i32) is: {}", f);
    println!("a(i32) + c(i32) is: {}", g);
}

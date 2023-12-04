pub fn run() {
    // トレイト、共通メソッドの定義
    trait Shape {
        // 面積の算出
        fn calc_area(&self) -> f64;
        // 周囲の算出
        fn calc_perimeter(&self) -> f64;
        // デフォルトメソッド
        fn default_something(&self) -> &str {
            "This is default method!"
        }
        // 型関連関数
        fn do_something();
    }
    // 構造体の定義
    struct Rectangle {
        // フィールドの定義
        pub width: f64,
        pub height: f64,
    }
    // 構造体 Rectangle に対する Shapeトレイトの実装
    impl Shape for Rectangle {
        fn calc_area(&self) -> f64 {
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64 {
            self.width * 2.0 + self.height * 2.0
        }
        // デフォルトメソッドのオーバーライド(上書き)
        fn default_something(&self) -> &str {
            "This is Rectangle default!"
        }
        fn do_something() {
            println!("This is Rectangle function");
        }
    }
    struct Circle {
        pub radius: f64,
    }
    impl Shape for Circle {
        fn calc_area(&self) -> f64 {
            // self.radius * self.radius * 3.14
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64 {
            // self.radius * 2.0 * 3.14
            self.radius * 2.0 * std::f64::consts::PI
        }
        fn do_something() {
            println!("This is Circle function");
        }
    }
    // トレイトを引数の型として実装
    fn double_area(shape: &impl Shape) -> f64 {
        shape.calc_area() * 2.0
    }

    let rect = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let circle = Circle { radius: 2.0 };

    println!("Rectangle area is: {}", rect.calc_area());
    println!("Rectangle perimeter is: {}", rect.calc_perimeter());
    Rectangle::do_something();

    println!("Circle area is: {}", circle.calc_area());
    println!("Circle perimeter is: {}", circle.calc_perimeter());
    Circle::do_something();

    println!("Rectangle default: {}", rect.default_something());
    println!("Circle default: {}", circle.default_something());

    println!("Rectangle double area: {}", double_area(&rect));
    println!("Circle double area: {}", double_area(&circle));
}

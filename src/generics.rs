use std::fmt::{Debug, Display};
pub fn run() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T: PartialOrd + Debug> Point<T> {
        fn max(&self) -> &T {
            if self.x >= self.y {
                &self.x
            } else {
                &self.y
            }
        }
        fn print_arg<U: Display>(&self, val: U) {
            println!("self.x : {:?}", self.x);
            println!("val : {}", val);
        }
    }
    impl Point<i32> {
        fn min(&self) -> i32 {
            if self.x <= self.y {
                self.x
            } else {
                self.y
            }
        }
    }
    let p1 = Point { x: 2, y: 1 };
    let p2 = Point { x: 1.1, y: 1.3 };
    let p3 = Point { x: "x", y: "a" };

    println!("p1.max : {:?}", p1.max());
    println!("p2.max : {:?}", p2.max());
    println!("p3.max : {:?}", p3.max());

    p1.print_arg("test");
    p2.print_arg(true);

    println!("p1.min : {}", p1.min());
    // println!("p2.min : {}", p2.min())
}

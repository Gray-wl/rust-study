fn main() {
    // 类型系统
    // let b = a as u64;
    // let a: u32 = 10;
    // let b = a as String; // 错误
    // println!("{b}");

    // let a = 1.0f32;
    // let b = 10;
    // let c = a * b; // no implementation for `f32 * {integer}`

    // let a = 1.0f32;
    // let b = 10 as f32;
    // let c = a * b;
    // println!("{c}");

    // let a = 9 + '1' as u8;
    // let b = 9.to_string() + "1";
    // println!("{a} {b}");

    // let a: u8 = 10;
    // let b: String = "123".to_string();
    // println!("{a} {b}");

    // 结构体中的类型参数
    struct Point1<T> {
        x: T,
        y: T,
    }
    let integer = Point1::<u32> { x: 5, y: 10 };
    let float = Point1::<f32> { x: 1.0, y: 4.0 };

    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point2::<u32, u32> { x: 5, y: 10 };
    let both_float = Point2::<f32, f32> { x: 1.0, y: 4.0 };
    let integer_and_float = Point2::<u32, f32> { x: 5, y: 4.0 };

    // 在泛型上做impl
    impl<T> Point1<T> {
        fn play(n: T) {}
    }

    // 枚举中的类型参数
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        OK(T),
        Err(E),
    }
    enum Aaa<T, U> {
        V1(Point1<T>),
        V2(Vec<U>),
    }

    // 函数中的类型参数
    struct PointU32 {
        x: u32,
        y: u32,
    }
    struct PointF32 {
        x: f32,
        y: f32,
    }
    fn print_u32(p: PointU32) {
        println!("Point {}, {}", p.x, p.y);
    }
    fn print_f32(p: PointF32) {
        println!("Point {}, {}", p.x, p.y);
    }
    fn print<T: std::fmt::Display>(p: Point1<T>) {
        println!("Point {} {}", p.x, p.y);
    }
    let p = PointU32 { x: 10, y: 20 };
    print_u32(p);
    let p = PointF32 { x: 10.2, y: 20.4 };
    print_f32(p);
    let p = Point1 { x: 10, y: 20 };
    print(p);
    let p = Point1 { x: 10.2, y: 20.4 };
    print(p);

    // 方法中的类型参数
    impl<T> Point1<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point1 { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    struct Point3<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X3, Y3> Point3<X3, Y3> {
        fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X3, Y2> {
            Point3 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // struct 和 enum
    struct Point(u32, u32);
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    struct Triangle(Point, Point, Point);
    struct Circle(Point, u32);
    enum Shape {
        Rectangle(Rectangle),
        Triangle(Triangle),
        Circle(Circle),
    }
    struct Axes;
    struct Geometry {
        shape: Shape,
        axes: Axes,
    }
    struct Algebra;
    enum Course {
        Geometry(Geometry),
        Algebra(Algebra),
    }
    enum Level {
        Elementary,
        Secondary,
        High,
    }
    struct MathLesson {
        math: Course,
        level: Level,
    }
    // newtype
    struct List<T>(Vec<T>);

    use std::collections::HashMap;
    type AAA = HashMap<String, Vec<u8>>;
    type BBB = Vec<AAA>;
    type CCC = HashMap<String, BBB>;
    type DDD = Vec<CCC>;
    type EEE = HashMap<String, DDD>;
    type MyType<T> = HashMap<String, Vec<HashMap<String, Vec<HashMap<String, Vec<T>>>>>>;
}

fn main() {
    // 枚举
    // enum Shape {
    //     Rectangle,
    //     Triangle,
    //     Circle,
    // }

    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    // enum Shape {
    //     Rectangle(Rectangle),
    //     // Rectangle { width: u32, height: u32 },
    //     Triangle((u32, u32), (u32, u32), (u32, u32)),
    //     Circle { origin: (u32, u32), radius: u32 },
    // }

    // 枚举实例化
    // #[derive(Debug)]
    // enum WebEvent {
    //     PageLoad,
    //     PageUnLoad,
    //     KeyPress(char),
    //     Paste(String),
    //     Click { x: i64, y: i64 },
    // }
    // let a = WebEvent::PageLoad;
    // let b = WebEvent::PageUnLoad;
    // let c = WebEvent::KeyPress('c');
    // let d = WebEvent::Paste(String::from("batman"));
    // let e = WebEvent::Click { x: 320, y: 240 };
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
    // println!("{:?}", d);
    // println!("{:?}", e);

    // 类 C 枚举
    // enum Number {
    //     Zero = 0,
    //     One,
    //     Two,
    // }
    // enum Color {
    //     Red = 0xff0000,
    //     Green = 0x00ff00,
    //     Blue = 0x0000ff,
    // }
    // println!("zero is {}", Number::Zero as i32);
    // println!("one is {}", Number::One as i32);
    // println!("roses are #{:06x}", Color::Red as i32);
    // println!("violets are #{:06x}", Color::Blue as i32);

    // 空枚举
    // enum Foo {}
    // let a = Foo {} // not a struct, variant or union type 报错

    // impl 枚举
    // enum MyEnum {
    //     Add,
    //     Subtract,
    // }
    // impl MyEnum {
    //     fn run(&self, x: i32, y: i32) -> i32 {
    //         match self {
    //             Self::Add => x + y,
    //             Self::Subtract => x - y,
    //         }
    //     }
    // }
    // let add = MyEnum::Add;
    // println!("{}", add.run(100, 200));

    // enum Foo {
    //     AAA,
    //     BBB,
    //     CCC,
    // }
    // impl Foo::AAA {} // not a type报错

    // match
    // #[derive(Debug)]
    // enum Shape {
    //     Rectangle,
    //     Triangle,
    //     Circle,
    // }

    // let shape_a = Shape::Rectangle;
    // let ret = match shape_a {
    //     // 所有分支都必须处理
    //     Shape::Rectangle => {
    //         println!("{:?}", Shape::Rectangle);
    //         1
    //     }
    //     Shape::Triangle => {
    //         println!("{:?}", Shape::Triangle);
    //         2
    //     }
    //     Shape::Circle => {
    //         println!("{:?}", Shape::Circle);
    //         3
    //     }
    // };
    // println!("{}", ret);

    // _ 占位符
    // let ret1 = match shape_a {
    //     // 所有分支都必须处理
    //     Shape::Rectangle => {
    //         1
    //     }
    //     _ => {
    //         10
    //     }
    // };
    // println!("{}", ret1);

    // 更广泛的分支
    // let number = 13;
    // println!("Tell me about {}", number);
    // match number {
    //     1 => println!("One!"),
    //     2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    //     13..=19 => println!("A teen"),
    //     _ => println!("Ain't special"),
    // }

    // 模式匹配
    // if match
    // let shape_a = Shape::Rectangle;
    // if let Shape::Rectangle = shape_a {
    //     println!("1");
    // } else {
    //     println!("2");
    // }

    // // while match
    // let mut shape_a = Shape::Rectangle;
    // let mut i = 0;
    // while let Shape::Rectangle = shape_a {
    //     if i > 9 {
    //         println!("Greater than 9, quit!");
    //         shape_a = Shape::Triangle;
    //     } else {
    //         println!(" `i` is `{:?}`. Try again.", i);
    //         i += 1;
    //     }
    // }

    // let
    // #[derive(Debug)]
    // enum Shape {
    //     Rectangle { width: u32, height: u32 },
    //     Triangle,
    //     Circle,
    // }
    // let shape_a = Shape::Rectangle { width: 10, height: 20 };
    // let Shape::Rectangle { width, height } = shape_a else { // 这种语法是匹配结构体负载，获取字段值的方式
    //     panic!("Can't extract rectangle.");
    // };
    // println!("width: {}, height: {}", width, height);

    // 匹配元组
    // let a = (1, 2, 'a');
    // let (b, c, d) = a;
    // println!("{:?}", a);
    // println!("{}", b);
    // println!("{}", c);
    // println!("{}", d);
    // fn foo() -> (u32, u32, char) {
    //     (1, 2, 'a')
    // }
    // let (b, c, d) = foo();
    // println!("{}", b);
    // println!("{}", c);
    // println!("{}", d);

    // 匹配枚举
    // struct Rectangle {
    //     width: u32,
    //     height: u32,
    // }
    // enum Shape {
    //     Rectangle(Rectangle),
    //     Triangle((u32, u32), (u32, u32), (u32, u32)),
    //     Circle { origin: (u32, u32), radius: u32 },
    // }
    // let a_rec = Rectangle {
    //     width: 10,
    //     height: 20,
    // };
    // let shape_a = Shape::Rectangle(a_rec);
    // let shape_a = Shape::Triangle((0, 1), (3, 4), (3, 0));
    // let shape_a = Shape::Circle {
    //     origin: (0, 0),
    //     radius: 5,
    // };
    // match shape_a {
    //     Shape::Rectangle(a_rec) => {
    //         println!("Rectangle {}, {}", a_rec.width, a_rec.height);
    //     }
    //     Shape::Triangle(x, y, z) => {
    //         println!("Triangle {:?} {:?} {:?}", x, y, z);
    //     }
    //     Shape::Circle { origin, radius } => {
    //         println!("Circle {:?}, {:?}", origin, radius);
    //     }
    // }

    // 匹配结构体
    // #[derive(Debug)]
    // struct User {
    //     name: String,
    //     age: u32,
    //     student: bool,
    // }
    // let mut a = User {
    //     name: String::from("mike"),
    //     age: 20,
    //     student: false,
    // };
    // let User { ref mut name, age, student } = a;
    // *name = String::from("Gray");
    // println!("{}", name);
    // println!("{}", age);
    // println!("{}", student);
    // println!("{:?}", a); // could not compile `demo6` (bin "demo6") due to previous error 需要加上ref

    // 函数参数中的模式匹配
    // fn foo((a, b, c): (u32, u32, char)) {
    //     println!("{}", a);
    //     println!("{}", b);
    //     println!("{}", c);
    // }
    // let a = (1, 2, 'a');
    // foo(a);
    #[derive(Debug)]
    struct User {
        name: String,
        age: u32,
        student: bool,
    }
    fn foo(User { name, age, student }: User) {
        println!("{}", name);
        println!("{}", age);
        println!("{}", student);
    }
    let a = User {
        name: String::from("Mike"),
        age: 20,
        student: false,
    };
    foo(a);
}

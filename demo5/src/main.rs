fn main() {
    // 结构体
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    struct Class {
        serial_number: u32,
        grade_number: u32,
        entry_year: String,
        members: Vec<User>,
    }

    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    // 命名结构体
    // let active = true;
    // let username = String::from("someusername123");
    // let email = String::from("someone@example.com");
    // let mut user1 = User {
    //     active,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("anotheremail@example.com");

    // println!("{:?}", user1);

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // println!("{:?}", user2);

    // 元组结构体
    // #[derive(Debug)]
    // struct Color(i32, i32, i32);
    // #[derive(Debug)]
    // struct Point(i32, i32, i32);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    // println!("{:?}", black);
    // println!("{:?}", origin);

    // 单元结构体
    // struct ArticleModule;
    // let module = ArticleModule;

    // 结构体中的所有权问题
    // 部分移动
    // let active = true;
    // let username = String::from("someusername123");
    // let email = String::from("someone@example.com");
    // let user1 = User {
    //     active,
    //     username,
    //     email,
    //     sign_in_count: 1,
    // };
    // // let email = user1.email;
    // // println!("{:?}", user1); // value borrowed here after partial move
    // println!("{}", user1.username);
    // println!("{}", user1.active);
    // println!("{}", user1.sign_in_count);
    // println!("{:?}", user1);

    // 面向对象特性
    #[derive(Debug, Default)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        // fn area(self) -> u32 {
        //     self.width * self.height
        // }
        fn area1(self: Self) -> u32 {
            self.width * self.height
        }
        fn area2(self: &Self) -> u32 {
            self.width * self.height
        }
        // fn area3(self: &mut Self) -> u32 {
        //     self.width * self.height
        // }
    }

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("{}", rect1.area1());

    let r1 = &rect1;

    println!("{}", r1.area2());

    // 静态方法
    impl Rectangle {
        fn numbers(rows: u32, cols: u32) -> u32 {
            rows * cols
        }
    }
    println!("{}", Rectangle::numbers(10, 10));

    // 构造函数
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }
    }
    let rect2 = Rectangle::new(30, 50);

    // default
    let rect1: Rectangle = Default::default();
    let rect2 = Rectangle::default();
    println!("{:?}", rect1);
    println!("{:?}", rect2);

    const INITWIDTH: u32 = 50;
    const INITHEIGHT: u32 = 30;
    let rect1 = Rectangle::new(INITWIDTH, INITHEIGHT);
}

fn main() {
    // 整数
    // let a: u32 = 1;
    // let b = 10u32;
    // println!("{} {}", a, b);

    // 浮点数
    // let a = 10.0f32;
    // println!("{}", a);

    // 布尔类型
    // let a = true;
    // let b:bool = false;
    // println!("{} {}", a, b);

    // 字符
    // let c = 'z';
    // let z: char = 'Z';
    // let t = '中';
    // println!("{} {} {}", c, z, t);

    // 字符串
    // let hello = String::from("Hello");
    // println!("{}", hello);
    // let a = hello[0]; // `String` cannot be indexed by `{integer}`
    // println!("{}", a);

    // 转义
    // let byte_escape = "Im saying \"Hello\" \n 你好 \r\n Ok \\ 是的 \0";
    // println!("{}", byte_escape);
    // let byte_escape1 = "Im saying hello \x7f";
    // println!("{}", byte_escape1);
    // let byte_escape2 = "Im saying hello \u{0065}";
    // println!("{}", byte_escape2);

    // 禁止转义
    // let byte_escape = r"Im saying \n 你好 \r\n Ok \\ 是的 \0 \u{0065}";
    // println!("{}", byte_escape);

    // 字节串
    // let bytestring = b"this is a nyye string";
    // println!("A byte string: {:?}", bytestring);
    // let escaped = b"\x52\x75\x73\x74 as bytes";
    // println!("Some escaped bytes: {:?}", escaped);
    // let raw_bytestring = br"\u{211D} is not escaped here";
    // println!("{:?}", raw_bytestring);

    // 数组
    // let a = [1, 2, 3, 4, 5];
    // println!("{:?}", a);
    // let b = a[5]; // index out of bounds: the length is 5 but the index is 5
    // let b = a[0];
    // println!("{}", b);

    // 动态数组
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(3);
    // println!("{:?}", v);

    // let s = vec![1, 2, 3];
    // println!("{:?}", s);

    // let s1 = String::from("superman s1");
    // let s2 = String::from("superman s2");
    // let l = vec![s1, s2];
    // println!("{:?}", l);
    // thread 'main' panicked at src/main.rs:70:21:
    // index out of bounds: the len is 2 but the index is 4
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // println!("{}", l[4]);

    // 哈希表
    // use std::collections::HashMap;
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores);

    // 元组
    // let tup = (500, 6.4, 1, "好的");
    // println!("{:?}", tup);
    // println!("{}", tup.0);

    // 结构体
    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     age: u64,
    // }
    // let user1 = User {
    //     active: true,
    //     username: String::from("张三"),
    //     email: String::from("xxx@example.com"),
    //     age: 1,
    // };
    // println!("{} {} {} {}", user1.username, user1.active, user1.age, user1.email);

    // 枚举
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // 分支语句
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else {
    //     println!("number is divisible by xxx");
    // }
    // let x = 1;
    // let y = if x == 0 {
    //     100
    // } else {
    //     101
    // };
    // println!("y is {}", y);

    // 循环语句
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");
    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }
    // println!("LIFTOFF!");
    // let a = [10, 20, 30, 40, 50];
    // for ele in a {
    //     println!("the value is {ele}!");
    // }
    // // 左闭右开
    // for number in 1..4 {
    //     println!("{number}");
    // }
    // // 左闭右闭
    // for number in 1..=4 {
    //     println!("{number}");
    // }
    // // 反向
    // for number in (1..4).rev() {
    //     println!("{number}");
    // }
    // for ch in 'a'..'z' {
    //     println!("{ch}");
    // }

    // 函数
    // fn print_a_b(a: i32, b: char) {
    //     println!("The value of a b is: {a} {b}");
    // }
    // print_a_b(12, 'h')

    // 闭包
    // let a = 10u32;
    // let add = |x: u32| -> u32 { x + a };
    // let res = add(20);
    // println!("{}", res);
}

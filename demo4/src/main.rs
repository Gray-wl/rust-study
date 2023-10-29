fn main() {
    // 不同类型的字符串
    // let s1: &'static str = "I am a superman.";
    // let s2: String = s1.to_string();
    // let s3: &String = &s2;
    // let s4: &str = &s2[..];
    // let s5: &str = &s2[..6];
    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");
    // println!("{s4}");
    // println!("{s5}");

    // let s: String = "I am a superman.".to_string();
    // let a_slice: &str = &s[..];
    // let another_string: String = a_slice.to_string();
    // println!("{another_string}");

    // 切片
    // let s = String::from("abcdefg");
    // let s1 = &s[..];
    // let s2 = &s[0..4];
    // let s3 = &s[2..5];
    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");

    // 切片转所有权字符串
    // let s: &str = "I am a superman.";
    // let s1: String = String::from(s);
    // let s2: String = s.to_string();
    // let s3: String = s.to_owned();
    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");

    // let a_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let a_slice: &[u8] = &a_vec[0..5];
    // let another_vec1 = a_slice.to_vec();
    // let another_vec2 = a_slice.to_owned();
    // println!("{:?}", a_vec);
    // println!("{:?}", a_slice);
    // println!("{:?}", another_vec1);
    // println!("{:?}", another_vec2);

    // as_str() 等价于 &a_string[..]
    // let s = String::from("foo");
    // assert_eq!("foo", s.as_str());

    // as_bytes() 返回&[u8]
    // let s = String::from("hello");
    // println!("{:?}", s.as_bytes());
    // assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());

    // let bytes = "bors".as_bytes();
    // println!("{:?}", bytes);
    // assert_eq!(b"bors", bytes);

    // let a_vec = vec![1, 2, 3, 4, 5, 8];
    // println!("{:?}", a_vec.as_slice());
    // assert_eq!(&[1, 2, 3, 4, 5, 8], a_vec.as_slice());

    // fn foo(s: &str) { // 用String 字符串字面量传入会报错
    //     println!("{s}");
    // }
    // let s = String::from("I am a spuerman.");
    // foo(&s);
    // let s1 = "I am a superman.";
    // foo(s1);

    // fn foo(s: &[u32]) {
    //     println!("{:?}", s);
    // }
    // let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    // foo(&v);
    // let a_slice = v.as_slice();
    // foo(a_slice);

    // 字符串切割成字符数组
    // let s = String::from("中国你好");
    // let char_vec: Vec<char> = s.chars().collect();
    // println!("{:?}", char_vec);
    
    // for ch in s.chars() {
    //     println!("{ch}");
    // }

    // Parse 方法
    // let a = "10".parse::<u32>();
    // let aa: u32 = "10".parse().unwrap();
    // println!("{:?}", a);
    // println!("{:?}", aa);

    // let a = "10".parse::<f32>();
    // println!("{:?}", a);

    // let a = "4.2".parse::<f32>();
    // println!("{:?}", a);

    // let a = "true".parse::<bool>();
    // println!("{:?}", a);

    // let a = "a".parse::<char>();
    // println!("{:?}", a);

    // let a = "192.168.1.100".parse::<std::net::IpAddr>();
    // println!("{:?}", a);
}

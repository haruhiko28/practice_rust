// use std::io::Write; // write, writelnマクロを使うため

fn main() {
    println!("Hello, world!");
    // 文字列 convert str and String
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;              // String --> &str
    let s3: String = s2.to_string(); // &str --> String
    println!("{}", type_of(&s1));
    println!("{}", type_of(&s2));
    println!("{}", type_of(&s3));
    
    // タプル
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}",t);   

    // 配列
    let mut a: [i32; 3] = [0,1,2];
    let b: [i32; 4] =[0; 4];
    a[1] = b[1];     
    a[2] = b[2];
    println!("{:?}",&a[0..3]);

    // ユーザ定義型
    #[derive(Debug)] //構造体をPrintlnする https://qiita.com/Kashiwara/items/9cec0d4940a8c92e85a5
    struct Person{
        name: String,
        age: u32,
    };
    let p = Person{
        name: String::from("John"),
        age: 8,
    };
    println!("{:?}", p);

    // 列挙型
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown {x: i32, y:i32},
    }
    let e1 = Event::Quit;
    let e2 = Event::KeyDown(1);
    let e3 = Event::MouseDown{x: 10,y: 10};

    println!("{:?}", e1);
    println!("{:?}", e2);
    println!("{:?}", e3);

    // 標準ライブラリの型
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code {}", code),
        Err(err) => println!("Err {}", err),
    };
    
    let result: Result<i32, String> = Ok(200);
    if let Ok(code) = result {
        println!("code {}", code);
    }
    println!("code {}", result.unwrap_or(-1));
    
    let result: Result<i32, String> = Err("error".to_string());
    println!("code {}", result.unwrap_or(-1));

    fn func(code: i32) -> Result<i32, String>{
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let _next_result = result.and_then(func); //実行される
    
    let result: Result<i32, String> = Err("error".to_string());
    let _next_result = result.and_then(func); //実行されない

    // fn error_handle(result: Result<i32,String>) -> Result<i32,String>{
    //     let code = result?;
    //     println!("code: {}",code);
    //     Ok(100)
    // }

    // Vec
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![0; 5];
    println!("{}", v1[0]);
    println!("{}", v2[0]);

    let v = vec![1,2,3,4,5];
    for element in &v {
        println!("{}", element);
    }
    
    // Box
    let byte_array = [b'h',b'h',b'h',b'h',b'h'];
    print(Box::new(byte_array));

    // letとmut
    let immut_val = 10;
    let mut mut_val = 20;

    mut_val += immut_val;
    println!("{}", mut_val);

    let v1: u64 = 10;
    let v2 = 10u64;
    println!("{}", v1);
    println!("{}", v2);

    //if
    let number = 1;
    if 0 < number {
        println!("0 < number");
    }else if number < 0{
        println!("number < 0");
    }else{
        println!("0==number");
    }

    let number = 1;
    let result = if 0 <= number{
        number
    }else{
        -number
    };
    println!("{}",result);

    // ループ
    let mut count = 0;
    let result = loop {
        println!("conut: {}",count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{}",result);

    let mut count = 0;
    while count < 10{
        println!("conut: {}",count);
        count += 1;
    }

    for count in 0..10{
        println!("conut: {}",count);
    }

    let array = [0,1,2,3,4,5,6,7,8,9];

    for element in &array{
        println!("element {}",element);
    }

    'main: loop{
        println!("main loop start");
        // 'sub: loop{ //使ってないラベルなので、これはコメントアウト
            println!("sub loop start");

            break 'main;
            // println!("sub loop end");
        // }
        // println!("main loop end");
    }

    // match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("_"),
    }

    enum Color {
        Red,
        Bule,
        Green,
    }

    let c1 = Color::Red;
    let c2 = Color::Bule;
    let c3 = Color::Green;
    match c1 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }
    match c2 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }
    match c3 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }

    let result : Result<i32, String> = Ok(100);
    let result_number = match result{
        Ok(number) => number,
        Err(message) => {
            println!("Error {}",message);
            -1
        },
    };
    println!("{}",result_number);

    // range
    for number in 1..5{
        println!("{}", number);
    }

    // iterator
    let it = Iter{
        current: 0,
        max: 10,
    };

    // Iterトレイト
    for num in it{
        println!("{}",num);
    }

    // 関数
    let x = add(1,2);
    println!("x = {}",x);

    println!("abs {}",abs(-110));

    //impl
    let p = Person2{
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name();
    p.say_age();

    let p = Person3{
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name().say_age();

    let p = Person3::new("Taro", 20);
    p.say_name().say_age();

    //マクロ
    let s = concat!("A","B","C");
    println!("{}",s);
    let s = format!("{}--{:?}",s,("D",5));
    println!("{}",s);
    let s = format!("{}-{}","abc","cdf");
    println!("{}",s);

    print!("hello");
    println!("hello {}","world");
    print!("hello {}","error");
    eprint!("hello");

    // let mut w = Vec::new();
    // write!(&mut w, "{}", "ABC");
    // writeln!(&mut w, " is 123");
    // dbg!(w);
}
// ======================================================================================================================================================

//impl
struct Person3{
    name: String,
    age: u32,
}

impl Person3 {
    fn new(name: &str, age: u32) -> Person3 {
        Person3 {
            name: String::from(name),
            age: age,
        }
    }
    fn say_name(&self) -> &Self{
        println!("I am {}.",self.name);
        self
    }
    fn say_age(&self)-> &Self {
        print!("I am {} years old.",self.age);
        self
    }
}


struct Person2{
    name: String,
    age: u32,
}

impl Person2{
    fn say_name(&self){
        println!("I am {}.",self.name);
    }

    fn say_age(&self){
        print!("I am {} years old.",self.age);
    }
}

// 型を表示する関数 http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn print(s: Box<[u8]>){
    println!("{:?}",s);
}
// Iterトレイト
struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter{
    type Item = usize;

    fn next(&mut self) -> Option<usize>{
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        }else{
            None
        }
    }
}

// 関数
fn add(a: i32, b: i32) -> i32{
    a + b
}

fn abs(number: i32) -> i32{
    if number < 0{
        return -number;
    }
    number
}

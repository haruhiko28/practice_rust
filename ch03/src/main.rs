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
        Ok(100);
    }

    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func);
    println!("{}", next_result);
    
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);
}

// 型を表示する関数 http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

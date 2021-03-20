fn main() {
    let x = 5;
    
    let x = x + 1;
    
    let x = x*2;

    println!("The value of x is {}", x); // x = 12

    // 手数はconstと全て大文字でアンダースコアで単語区切り
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINT is {}", MAX_POINTS);
}

// use std::env;
// use clap::{App, Arg};

use anyhow::{bail, ensure, Context, Result};
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>(); //スペースで切って、逆順にして、Vecにする
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos  = 0;

        while let Some(token) = tokens.pop(){
            pos += 1;
            
            if let Ok(x) = token.parse::<i32>() { //数字ならstackに格納
                stack.push(x);
            } else { // 演算子ならstackから取り出して計算
                let y = stack.pop().context(format!("invalid syntax at {}",pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}",pos))?;
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    _ => bail!("invalid token at {}", pos) ,
                };
            stack.push(res); // 出した答えをstackにいれる
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        
        ensure!(stack.len() == 1, "invalid syntax");

        Ok(stack[0])
    }
}

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN program",
    version = "1.0.0",
    author = "Your Name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    /// Sets the lebel of verbosity
    #[clap(short, long)]
    verbose: bool,

    /// Formulas writeen in RPN
    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    // 4−1
    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is versity specified?: {}",opts.verbose);

    if let Some(path) = opts.formula_file {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
        // for line in reader.lines() {
        //     let line = line.unwrap();
        //     println!("{}", line);
        // }
    } else {
        //ファイル指定がなかった場合
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
        // println!("No file is specified!");
    }
}

fn run<R: BufRead> (reader: R, verbose: bool) -> Result<()>{
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines(){
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}",answer),
            Err(e) => eprintln!("{:#?}",e),
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5").unwrap(), 5);
        assert_eq!(calc.eval("50").unwrap(), 50);
        assert_eq!(calc.eval("-50").unwrap(), -50);

        assert_eq!(calc.eval("2 3 +").unwrap(), 5);
        assert_eq!(calc.eval("2 3 *").unwrap(), 6);
        assert_eq!(calc.eval("2 3 -").unwrap(), -1);
        assert_eq!(calc.eval("2 3 /").unwrap(), 0);
        assert_eq!(calc.eval("2 3 %").unwrap(), 2);
    }
}

#[test]
fn test_ng() {
    let calc = RpnCalculator::new(false);
    assert!(calc.eval("").is_err());
    assert!(calc.eval("1 1 1 +").is_err());
    assert!(calc.eval("+ 1 1").is_err());
}

// 4-1
// fn main() {
//     // let args: Vec<String> = env::args().collect();
//     // println!("{:?}", args);
//     let matches = App::new("My RPN program")
//         .version("1.0.0")
//         .author("Your name")
//         .about("Super awesom sample RPN calculator")
//         .arg(
//             Arg::with_name("formula_file")
//             // .about("Formulas written in RPN")
//             .value_name("FILE")
//             .index(1)
//             .required(false),
//         )
//         .arg(
//             Arg::with_name("verbose")
//             // .about("Sets the level of verbosity")
//             .short("v")
//             .long("verbose")
//             .required(false),
//         )
//         .get_matches();
    
//         match matches.value_of("formula_file") {
//             Some(file) => println!("File specified: {}", file),
//             None => println!("No file specified."),
//         }

//         let verbose = matches.is_present("verbose");
//         println!("Is verbosity specified?: {}", verbose);
    
// }


// use std::env;
// use clap::{App, Arg};
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
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
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    // 4−1
    // match opts.formula_file {
    //     Some(file) => println!("File specified: {}", file),
    //     None => println!("No file specified."),
    // }
    // println!("Is versity specified?: {}",opts.verbose);

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
        // for line in reader.lines() {
        //     let line = line.unwrap();
        //     println!("{}", line);
        // }
    } else {
        //ファイル指定がなかった場合
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
        // println!("No file is specified!");
    }
}

fn run<R: BufRead> (reader: R, verbose: bool){
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines(){
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop(){
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalid syntax");
                let x = stack.pop().expect("invalid syntax");
                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    _ => panic!("invalid token") ,
                };
            stack.push(res);
            }

            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        }else {
            panic!("invalid syntax")
        }
    }
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


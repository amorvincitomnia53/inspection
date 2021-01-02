// #[macro_use] extern crate scan_fmt;

use std::env;
use rand::{rngs::StdRng, SeedableRng, distributions::{Distribution, Uniform}, Rng};
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::{create_dir_all, canonicalize, File};
use std::iter::Iterator;



macro_rules! catch {
    ($x:expr) => {
        (||-> Result<_, Box<dyn std::error::Error>>{Ok($x)})();
    }
}
macro_rules! try_input{
    ($($r:tt)*) => {input_basic!{try, $($r)*}}
}
macro_rules! input{
    ($($r:tt)*) => {input_basic!{unwrap, $($r)*}}
}
macro_rules! input_basic {
    ($mode:ident, str = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || -> Result<_, Box<dyn std::error::Error>>{ Ok(iter.next().ok_or("Insufficient input")?) };
        input_inner!{$mode, next, $($r)*}
    };
    ($mode:ident, tokens = $s:expr, $($r:tt)*) => {
        let mut iter = $s;
        let mut next = || -> Result<_, Box<dyn std::error::Error>>{ Ok(iter.next().ok_or("Insufficient input")?) };
        input_inner!{$mode, next, $($r)*}
    };

    ($mode: ident, source = $s:expr, $($r:tt)*) => {
        let mut bytes = std::io::Read::bytes($s);
        let is_whitespace_or_err = |x: &Result<char, _>| match x { Ok(c) => {c.is_whitespace()}, Err(_) => {true} };
        let mut next = move || -> Result<String, Box<dyn std::error::Error>>{
            bytes
                .by_ref()
                .map(|r: Result<u8, _>| {let c = r? as char; Ok(c)})
                .skip_while(|x|is_whitespace_or_err(x))
                .take_while(|x|!is_whitespace_or_err(x))
                .collect()
        };
        input_inner!{$mode, next, $($r)*}
        std::mem::drop(next);
    };
    ($mode:ident, $($r:tt)*) => {
        let stdin = std::io::stdin();
        input_basic!{$mode, source = stdin.lock(), $($r)*}
        std::mem::drop(stdin);
    };
}



macro_rules! input_inner {
    (try, $next:expr) => {};
    (try, $next:expr, ) => {};
    (unwrap, $next:expr) => {};
    (unwrap, $next:expr, ) => {};

    (try, $next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t)?;
        input_inner!{try, $next $($r)*}
    };
    (unwrap, $next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t).unwrap();
        input_inner!{unwrap, $next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        catch!(( $((read_value!($next, $t))?),* ))
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Result<Vec<_>, _>>()
    };

    ($next:expr, chars) => {
        catch!(read_value!($next, String)?.chars().collect::<Vec<char>>())
    };

    ($next:expr, usize1) => {
        catch!(read_value!($next, usize)? - 1)
    };

    ($next:expr, $t:ty) => {
        catch!($next()?.parse::<$t>()?);
    };
}



fn arg<T: FromStr>(i: usize) -> Result<T, Box<dyn Error>> where <T as FromStr>::Err: std::error::Error + 'static {
    return Ok(env::args().nth(i).ok_or("No arg")?.parse::<T>()?);
}

fn main() {


    let program : String = 
        match arg(1) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Usage: judge <program> [<n>=1000] [<m>=1000] [<p_min>=0] [<p_max>=0.05] [<seed>=random]");
                eprintln!("\twhere n: 患者数, m: テストケース数, p: 陽性率 ~ Uniform(p_min, p_max)");
                return;
            }
        };
    
    let n : usize = arg(2).unwrap_or(1000);
    let m : u64 = arg(3).unwrap_or(1000);
    let p_min : f64 = arg(4).unwrap_or(0.0);
    let p_max : f64 = arg(5).unwrap_or(0.05);
    let seed: u64 = arg(6).unwrap_or(rand::random());

    let mut rng = StdRng::seed_from_u64(seed);
    println!("program: {}", &program);
    println!("n: {}, m: {}, seed: {}", n, m, seed);

    let dirname = "test_cases";
    create_dir_all(&dirname).unwrap();
    println!("dir: {}", canonicalize(&dirname).unwrap().display());

    let p_dist = Uniform::new_inclusive(p_min, p_max);


    let mut total_inspections = 0.0;
    let mut total_inspections_lowerbound = 0.0;
    for i in 0..m{
        let p: f64 = p_dist.sample(&mut rng);
        let lb = -(p * p.log2() + (1.0-p) * (1.0-p).log2());
        let ans: Vec<bool> = (0..n).map(|_i| rng.gen_bool(p)).collect();
        {
            let mut file = File::create(format!("{}/test{:>05}_ans.txt", &dirname, i)).unwrap();
            for &is_positive in &ans{
                write!(file, "{} ", if is_positive {1} else {0}).unwrap();
            }
        }
        
        let mut in_file = File::create(format!("{}/test{:>05}_in.txt", &dirname, i)).unwrap();
        let mut out_file = File::create(format!("{}/test{:>05}_out.txt", &dirname, i)).unwrap();
        

        let process = Command::new(&program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().unwrap();
        
        let exec = || -> Result<_, Box<dyn Error>>{
            let out = BufReader::new(process.stdout.unwrap());
            let mut inp = process.stdin.unwrap();
            let mut o_iter = out.lines();
            
            // split(b' ')
            //                 .map(|x|->Result<_, Box<dyn Error>>{
            //                         Ok(String::from_utf8(x?)?) 
            //                     });

            let mut write_input = |s: &str| -> Result<_, Box<dyn Error>>{
                let b = s.as_bytes();
                in_file.write_all(&b)?;
                inp.write_all(&b)?;
                inp.flush()?;
                return Ok(());
            };

            let mut read_output = || -> Result<_, Box<dyn Error>>{
                match o_iter.next() {
                    Some(x) => {
                        let content = x?;
                        out_file.write_all(format!("{}\n",content).as_bytes())?;
                        Ok(content)
                    },
                    None => Err("Unexpected EOF")?,
                }
            };

            
            write_input(&format!("{} {}\n", n, p))?;
            let mut inspections = 0;
            let correct = loop {
                let ln = read_output()?;
                let mut tokens = ln.split_whitespace();
                match tokens.next() {
                    Some("INS") => {
                        inspections+=1;
                        try_input!{ tokens = tokens,
                            k: usize,
                            v: [usize; k],
                        };
                        if !(v.iter().all(|&i| i<n )) {Err("Invalid index")?;}
                        let positive = v.iter().any(|&i|ans[i]);
                        write_input(if positive {"1 \n"} else {"0 \n"})?;
                    },
                    Some("ANS") => {
                        try_input!{tokens = tokens,
                            v: [u64; n],
                        };
                        break v.iter().zip(ans.iter())
                                .all(|(a,b)| match (a,b){
                                    (0, false) => true,
                                    (1, true) => true,
                                    _ => false,
                                });
                    },
                    _ => Err("Presentation Error")?
                }
            };

            return if correct{ Ok(inspections)} else {Err("Wrong Answer")?}
        };
        match exec() {
            Ok(inspections) => {
                println!("[{}] p:{} AC! cost:{} (lb:{})", i, p, inspections as f64 / n as f64, lb);
                total_inspections += inspections as f64 / n as f64;
                total_inspections_lowerbound += lb;
            },
            Err(why) => {
                println!("[{}] p:{} Error! Reason: {}", i, p, why);
                return;
            },
        }

    }

    println!("[Summary] average ins: {} (lb: {})", total_inspections / m as f64, total_inspections_lowerbound / m as f64);


}

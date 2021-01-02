
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


fn golden_minimize_int_by<F, T, C>(f: F, l_: i32, r_: i32, cmp: C) -> (i32, T) 
		where F: Fn(i32) -> T, C: Fn(&T, &T)->std::cmp::Ordering, T: Clone
{

    assert!(r_ >= l_);
    
    let mut l = (l_, f(l_));
    if r_ == l_ {return l};
    let mut r = (r_, f(r_));

    const PHI: f64 = 1.618033988749895;

    let getl = |a, b| {let x=a + ((b-a) as f64 / (PHI + 1.0)).ceil() as i32; (x, f(x))};
    let getr = |a, b| {let x=b - ((b-a) as f64 / (PHI + 1.0)).ceil() as i32; (x, f(x))};
    let mut ml = getl(l.0, r.0);
    let mut mr = getr(l.0, r.0);
    while r.0 - l.0 >= 4 {
        // eprintln!("{} {} {} {}", l.0, ml.0, mr.0, r.0);
        if cmp(&ml.1, &mr.1) == std::cmp::Ordering::Less {
            r = mr;
            mr = ml;
            ml = getr(l.0, mr.0);
        } else{
            l = ml;
            ml = mr;
            mr = getl(ml.0, r.0);
        }
    }
    let mut val: [Option<T>; 4] = [None, None, None, None];
    let l0 = l.0;
    let sz = r.0 - l.0 + 1;
    val[0] = Some(l.1);
    val[(ml.0 - l.0) as usize] = Some(ml.1);
    val[(mr.0 - l.0) as usize] = Some(mr.1);
    val[(r.0 - l.0) as usize] = Some(r.1);

    val.iter()
        .take((sz) as usize)
        .enumerate()
        .map(|(i,x)| (l0 + i as i32, x.clone().unwrap_or_else(||f(l0 + i as i32))))
        .min_by(|(_, x0), (_, x1)|cmp(x0, x1))
        .unwrap()
}
fn analyze(p: f64) -> (f64, i32, f64, f64, Vec<(i32, f64, f64)>){
    let mut ret = Vec::new();
    ret.push((-1, 0.0, 0.0));
    ret.push((-1, 0.0, 1.0));

    let (mut lc, mut la) = (1.0 / p, 1.0 / p);

    for i in 2..{
        let f = |j: i32|{
            let p0 = (1.0 - (1.0 - p).powi(j)) / (1.0 - (1.0 - p).powi(i));
            let (_, c0, a0) = &ret[j as usize];
            let (_, c1, a1) = &ret[(i-j) as usize];
            let c = 1.0 + c0 * p0 + c1 * (1.0 - p0);
            let a = a0 * p0 + (j as f64 + a1) * (1.0 - p0);
            (c, a)
        };
        let cmp = |(c0, a0): &_, (c1, a1): &_| ((c0 / a0) as f64).partial_cmp(&(c1 / a1)).unwrap();
        let (j, (c, a)) = golden_minimize_int_by(f, 1, i as i32 - 1, cmp);
        
        let pi = 1.0 - (1.0 - p).powi(i as i32);
        let c_inf = c + 1.0 / pi;
        let a_inf = a + i as f64 * (1.0 - pi) / pi;
        
        ret.push((j,c,a));
        if c_inf/a_inf > lc/la {return (lc/la, i-1, lc, la, ret);}
        else { 
            lc = c_inf;
            la = a_inf; 
        }   
    }
    unreachable!();
}

use std::cmp::{max, min};

fn h(x:f64) -> f64{
    -(x * x.ln() + (1.0-x) * (1.0 - x).ln())
}
fn main() {

    let stdin_raw = std::io::stdin();
    let mut stdin = std::io::BufReader::new(stdin_raw.lock());

    input!{source = &mut stdin, 
        n : usize,
        p : f64,
    };
    let (_mean_cost, m_inf, _, _, dp) = analyze(p);

    let mut ans: Vec<bool> = vec![false;n];
    let mut l : usize = 0;
    let mut r : Option<usize> = None;
    let mut inspect = |l, r| {
        print!("INS {}", r-l);
        for i in l..r {
            print!(" {}", i);
        }
        println!("");
        input!{source=&mut stdin, res: u64};
        return res == 1;
    };

    while l < n{
        let m = r.map(|x|x-l);
        if m == Some(1){
            ans[l] = true;
            l = r.unwrap();
            r = None;
            continue;
        }
        let m1 = match m{
            Some(x) => dp[x].0 as usize,
            None => m_inf as usize
        };
        let r1 = min(l + m1, n);
        if inspect(l, r1){
            r = Some(r1);
        } else{
            l = r1;
        }
    }
    

    print!("ANS");
    for &r in &ans{
        print!(" {}", if r{1} else {0});
    }
    println!("");

}

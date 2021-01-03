
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


fn find_fib_after(n: i32) -> (i32, i32){
    if n <= 3 {return (n-1, n);}
    let mut s = (3,5);
    loop{
        if s.1 >= n {return s;}
        s = (s.1, s.0+s.1);
    }

}
fn fibonacci_minimize_by<F, T, C>(f: F, l_: i32, r_: i32, cmp_: C) -> (i32, T) 
		where F: Fn(i32) -> T, C: Fn(&T, &T)->std::cmp::Ordering, T: Clone
{
    use std::cmp::Ordering;
    assert!(r_ >= l_);

    let ev = |x: i32| -> (i32, Option<T>) {assert!( x >= l_);if x > r_ {(x, None)} else {(x, Some(f(x)))}};
    let cmp = |xv0: &(i32, Option<T>), xv1: &(i32, Option<T>)| {
        match (xv0, xv1) {
            ((_, Some(a)), (_, Some(b))) => cmp_(&a, &b),
            ((_, Some(_)), (_, None)) => Ordering::Less,
            ((_, None), (_, Some(_))) => Ordering::Greater,
            ((a, None), (b, None)) => a.cmp(&b)
        }
    };

    // eprintln!("============");

    let mut l = ev(l_);
    if r_ == l_ {return (l.0, l.1.unwrap())};
    let (f0, f1): (i32, i32) = find_fib_after(r_ - l_);
    let mut r = ev(l_ + f1);
    let mut mr = ev(l_ + f0);
    let mut ml = ev(l_ + f1 - f0);
    // eprintln!("{} {} {} {}", l.0, ml.0, mr.0, r.0);
    while r.0 - l.0 >= 4 {
        if cmp(&ml, &mr) == std::cmp::Ordering::Less {
            r = mr;
            mr = ml;
            ml = ev(l.0 + r.0 - mr.0);
        } else{
            l = ml;
            ml = mr;
            mr = ev(l.0 + r.0 - ml.0);
        }
        // eprintln!("{} {} {} {}", l.0, ml.0, mr.0, r.0);
    }
    let lst = [l, ml, mr, r];
    let (a, b) = lst.iter()
                    .min_by(|x, y|cmp(*x, *y))
                    .unwrap();
    return (*a, b.clone().unwrap())          
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
        let (j, (c, a)) = fibonacci_minimize_by(f, 1, i as i32 - 1, cmp);
        
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


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




fn main() {

    let stdin_raw = std::io::stdin();
    let mut stdin = std::io::BufReader::new(stdin_raw.lock());

    input!{source = &mut stdin, 
        n : usize,
        p : f64,
    };
    let ans: Vec<bool> = (0..n).map(|i|{
        println!("INS 1 {}", i);

        input!{source= &mut stdin,
            res : u64
        };
        return res == 1;
    }).collect();
    
    print!("ANS");
    for &r in &ans{
        print!(" {}", if r{1} else {0});
    }
    println!("");
}

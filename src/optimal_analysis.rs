fn kitamasa_fib((a0,b0): &(i64, i64), (a1,b1): &(i64, i64)) -> (i64, i64){
    ((a0 * a1 + b0 * b1), (b0 * b1 + a0 * b1 + a1 * b0))
}
fn find_fib_near(n: i64) -> (i64,(i64, i64)) {
    let mut arr : [(i64, i64); 9]= [(0,0); 9];
    arr[0] = (0, 1);
    let u = {
        let mut i = 0;
        while i < 8{
            i += 1;
            arr[i as usize] = kitamasa_fib(&arr[i - 1], &arr[i - 1]);
            if arr[i as usize].0 > n{
                i -= 1;
                break;
            }
        }
        i
    };

    let mut k = 1_i64 << u;
    let mut v = arr[u as usize].clone();
    for i in (0..u).rev(){
        let v2 = kitamasa_fib(&v, &arr[i as usize]);
        if v2.0 <= n {
            v = v2;
            k += 1_i64 << i;
        }
    }
    (k-1, v)
}
fn find_fib_after(n: i32) -> (i32, i32){
    if n <= 3 {return (n-1, n);}
    let mut s = (3,5);
    loop{
        if s.1 >= n {return s;}
        s = (s.1, s.0+s.1);
    }

}


fn golden_minimize_by<F, T, C>(f: F, l_: f64, r_: f64, tol: f64, cmp: C) -> (f64, T) 
        where F: Fn(f64) -> T, C: Fn(&T, &T)->std::cmp::Ordering
{

    assert!(r_ >= l_);
    let mut l = (l_, f(l_));
    if r_ == l_  {return l};
    let mut r = (r_, f(r_));
    const PHI: f64 = 1.618033988749895;

    let getl = |a, b| {let x=a + (b-a) / (PHI + 1.0); (x, f(x))};
    let getr = |a, b| {let x=b - (b-a) / (PHI + 1.0); (x, f(x))};
    let mut ml = getl(l.0, r.0);
    let mut mr = getr(l.0, r.0);
    while r.0 - l.0 > tol{
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
    if cmp(&ml.1, &mr.1) == std::cmp::Ordering::Less {ml} else {mr}
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
fn minimize_int_by<F, T, C>(f: F, l_: i32, r_: i32, cmp: C) -> (i32, T) 
		where F: Fn(i32) -> T, C: Fn(&T, &T)->std::cmp::Ordering, T: Clone
{
    (l_ .. (r_+1)).map(|i|(i, f(i))).min_by(|(_, x0), (_, x1)|cmp(&x0, &x1)).unwrap()
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
        // let (j1, (c1, a1)) = minimize_int_by(f, 1, i as i32 - 1, cmp);
        //assert!(c / a <= c1 / a1);
        // if /*c/a > c1 / a1*/ cmp (&(c, a), &(c1, a1)) == std::cmp::Ordering::Greater{
        //     eprintln!("!!! {} {}  |  {} {}", j, c/a, j1, c1/a1);
        // }
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



fn dp(p: f64, n: usize) -> Vec<(i32, f64, f64)>{
    let mut ret = Vec::new();
    ret.push((-1, 0.0, 0.0));
    ret.push((-1, 0.0, 1.0));
    for i in 2..n{
        // eprintln!("============");
        ret.push((1..i).map(|j| {
            let p0 = (1.0 - (1.0 - p).powi(j as i32)) / (1.0 - (1.0 - p).powi(i as i32));
            let (_, c0, a0) = &ret[j];
            let (_, c1, a1) = &ret[i-j];
            let c = 1.0 + c0 * p0 + c1 * (1.0 - p0);
            let a = a0 * p0 + (j as f64 + a1) * (1.0 - p0);
            // eprintln!("{}", c/a);
            (j as i32, c, a)
        }).min_by(|(_, c0, a0): &_, (_, c1, a1): &_| (c0 / a0).partial_cmp(&(c1 / a1)).unwrap())
            .unwrap());
    }
    ret
}


// fn main() {
//     for p in itertools_num::linspace(0.01, 0.99, 99){
//         let (m, k, c, a, _dp) = analyze(p);
//         println!("{} {} {} {} {} {} {}", p, m, k, c, a, -p * p.log2() - (1.-p) * (1.-p).log2(), m-(-p * p.log2() - (1.-p) * (1.-p).log2()));
//     }
// }
fn main() {
    for lp in itertools_num::linspace(-5., -0.01, 500){
        let p = 10.0_f64.powf(lp);
        let (m, k, c, a, _dp) = analyze(p);
        println!("{} {} {} {} {} {} {}", p, m, k, c, a, -p * p.log2() - (1.-p) * (1.-p).log2(), m-(-p * p.log2() - (1.-p) * (1.-p).log2()));
    }
}
// fn main() {
//     let p = 0.245858;
//     let (m, k, c, a, dp) = analyze(p);
//     println!("{} {} {} {} {} {}", p, m, k, c, a, -p * p.log2() - (1.-p) * (1.-p).log2());
//     for (j, (k,c,a)) in dp.iter().enumerate(){
//         let pj = 1.0 - (1.0 - p).powi(j as i32);
//         let c_inf = c + 1.0 / pj;
//         let a_inf = a + j as f64 * (1.0-pj) / pj;
//         println!("{} {} {}", k, c / a, c_inf / a_inf);
//     }
//     println!("");
// }
// fn main() {
//     let n = 1000;
//     let p = 0.001;
//     let a = dp(p,n);
//     // let a : Vec<_> = [0.01].iter().map(|&p|dp(p, n)).collect();
//     // for i in 1..a[0].len(){
//     for j in 1..a.len(){
//         match a[j] {
//             (k, c, a) => {
//                 let pj = 1.0 - (1.0 - p).powi(j as i32);
//                 let c_inf = c + 1.0 / pj;
//                 let a_inf = a + j as f64 * (1.0-pj) / pj;
//                 println!("{} {} {} {}", k, c, a, c_inf / a_inf);
//             }
//         };
//     }
//     println!("");
// }

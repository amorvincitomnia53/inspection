


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


fn main() {
    for lp in itertools_num::linspace(-4., -0.1, 500){
        let p = 10.0_f64.powf(lp);
        let (m, k, c, a, _dp) = analyze(p);
        println!("{} {} {} {} {}", p, m, k, c, a);
    }
}
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

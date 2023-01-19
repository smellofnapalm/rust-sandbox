use std::*;

fn input<T: str::FromStr>() -> T where <T as str::FromStr>::Err: fmt::Debug {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.trim().parse::<T>().expect("")
}

fn input_array<T: str::FromStr>(flag: bool) -> Vec<T> where <T as str::FromStr>::Err: fmt::Debug {
    let n = input::<usize>();
    let mut a = Vec::<T>::new();
    if flag {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("");
        a = line.trim().split(' ').map(|x| x.parse::<T>().expect("")).collect();
    }
    else {
        for _ in 0..n {
            a.push(input::<T>());
        }
    }
    return a;
}

fn solve() {
    let x: i32 = input();
    let mut pow10 = 10;
    let mut ans = 0;
    while pow10 < x {
        ans += 9;
        pow10 *= 10;
    }
    ans += (x / (if pow10 > 1 {pow10 / 10} else {1}));
    println!("{ans}");
}

fn main() {
    let n: i32 = input();
    for _ in 0..n {
        solve();
    }
}

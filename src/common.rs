use std;
use std::io;
use std::cmp;

// read a line(without line feed)
pub fn read_line() -> String {
    let mut in_str = String::new();
    io::stdin().read_line(&mut in_str).unwrap();
    return in_str.trim().to_string();
}

// read wods on a line
pub fn read_words_in_line() -> Vec<String> {
    let s = read_line();
    return s.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
}

// read type t on a line
pub fn read_t_in_line<T : std::str::FromStr>() -> Vec<T> {
    let mut v : Vec<T> = Vec::new();
    for e in read_words_in_line().iter().map(|x| x.parse::<T>()) {
        if let Ok(r) = e {
            v.push(r);
        }
        else{
            return Vec::new();
        }
    }
    return v;
}

// sort slice
pub fn sort<T : std::cmp::Ord>(slice : &mut [T]) {
    slice.sort();
}
// sort slice by reverse order
pub fn sort_rev<T : std::cmp::Ord>(slice : &mut [T]) {
    slice.sort_by(|x, y| y.cmp(x));
}

pub fn min<T : std::cmp::Ord>(a : T, b : T) -> T{
    return cmp::min(a, b);
}

pub fn max<T : std::cmp::Ord>(a : T, b : T) -> T{
    return cmp::max(a, b);
}

use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn solution(){

    let mut _vec: Vec<i64> = Vec::new();
    let mut input: Vec<i64> = Vec::new();
    let mut max: i64 = 0;
   if let Ok(lines) = read_lines("input/1.txt") {
    for line in lines{
        if let Ok(ip) = line {
            if ip.eq(""){
                if input.len() > 0 {
                    let s:i64 = input.iter().sum();
                    if s > max {
                        max = s;
                    }
                    _vec.push(s);
                }
                input.clear();
            }
            else {
                let i:i64 = ip.parse().unwrap_or(0);
                input.push(i);
            }
        }
    }

   }
    _vec.sort_by(|a, b| b.cmp(a));
    println!("{}", max);
    println!("{}", _vec[0]+_vec[1]+_vec[2]);
}


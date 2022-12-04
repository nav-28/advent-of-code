


macro_rules! scan {
    ($string:expr, $sep:expr, $($x:ty), +) => {{
       let mut iter = $string.split($sep);
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()),)*)
    }}
}



fn check_range(i1: i32, i2: i32, i3: i32, i4: i32) -> bool{
    if i3 >= i1 && i4 <= i2 {
        return true;
    }
    else if i1 >= i3 && i2 <= i4 {
        return true;
    }
    else {
        return false;
    }
}

fn check_overlap(i1: i32, i2: i32, i3: i32, i4: i32) -> bool{
    if (i3 >= i2 && i3 <= i1) || (i1 >= i3 && i1 <= i4) {
        return true;
    }
    else if (i2 <= i4 && i2 >= i3) || (i4 <= i2 && i4 >= i1) {
        return true;
    }
    else {
        return false;
    }


}


pub fn solution() {
    let mut input = include_str!("../input/4.txt").split("\n").collect::<Vec<&str>>();
    input.pop().unwrap(); // remove extra line
    let mut sum = 0;
    let mut sum2 = 0;
    for line in input {
        let mut line_iter = line.split(",");
        let u1 = line_iter.next().unwrap();
        let u2 = line_iter.next().unwrap();
        let i1 = scan!(u1, "-", i32, i32);
        let i2 = scan!(u2, "-", i32, i32);
        let i1 = (i1.0.unwrap(), i1.1.unwrap());
        let i2 = (i2.0.unwrap(), i2.1.unwrap());
        if check_range(i1.0, i1.1, i2.0, i2.1){
            sum += 1
        }
        if check_overlap(i1.0, i1.1, i2.0, i2.1) {
            sum2 += 1;
        }
    }
    println!("{sum} {sum2}");
}

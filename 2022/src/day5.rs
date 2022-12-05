fn parse_line(line : &str) -> Option<(i32, i32, i32)> {
    let mut iter = line.split_ascii_whitespace();
    iter.next();
    let a = iter.next()?.parse().ok()?;
    iter.next();
    let b = iter.next()?.parse().ok()?;
    iter.next();
    let c = iter.next()?.parse().ok()?;
    Some((a,b,c))
}

fn create_stacks(stack : &mut Vec<Vec<char>>){
    let s = vec!['Q', 'S', 'W', 'C', 'Z', 'V', 'F', 'T'];
    let s2 = vec!['Q', 'R', 'B'];
    let s3 = vec!['B', 'Z', 'T', 'Q', 'P', 'M', 'S'];
    let s4 = vec!['D', 'V', 'F', 'R', 'Q', 'H'];
    let s5 = vec!['J', 'G', 'L', 'D', 'B', 'S', 'T', 'P'];
    let s6 = vec!['W', 'R', 'T', 'Z'];
    let s7 = vec!['H', 'Q', 'M', 'N', 'S', 'F', 'R', 'J'];
    let s8 = vec!['R', 'N', 'F', 'H', 'W'];
    let s9 = vec!['J', 'Z', 'T', 'Q', 'P', 'R', 'B'];
    stack.push(s);
    stack.push(s2);
    stack.push(s3);
    stack.push(s4);
    stack.push(s5);
    stack.push(s6);
    stack.push(s7);
    stack.push(s8);
    stack.push(s9);
}

pub fn solution() {
    let mut stacks : Vec<Vec<char>>= Vec::new();
    create_stacks(&mut stacks);
    let mut stacks2 = stacks.clone();


    let input : Vec<(i32, i32, i32)> = include_str!("../input/5_moves.txt")
        .split("\n")
        .map(|line|  {if line.len() > 0 {parse_line(&line).unwrap()} else {(-1,-1,-1)}})
        .collect();

    for m in input.clone() {
        if m.0 > 0 {
            for _ in 0..m.0 {
                let popped = stacks[(m.1 - 1) as usize].pop().unwrap();
                stacks[(m.2 - 1) as usize].push(popped);
            }
        }
    }

    let part1 = stacks.iter()
        .map(|l| l.last().unwrap()).collect::<String>();

    for m in input {
        if m.0 > 0 {
            let mut temp : Vec<char> = Vec::new();
            for _ in 0..m.0 {
                temp.push(stacks2[(m.1 - 1) as usize].pop().unwrap());
            }
            for _ in 0..m.0{
                let popped = temp.pop().unwrap();
                stacks2[(m.2 - 1) as usize].push(popped);
            }
        }
    }
    let part2 = stacks2.iter()
        .map(|l| l.last().unwrap()).collect::<String>();
    println!("{part1} {part2}");
}

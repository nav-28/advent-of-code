use std::collections::HashSet;


fn intersect<I, Set>(sets: I) -> u8
where
    I: IntoIterator<Item = Set>,
    Set: IntoIterator<Item = u8>,
{
    let common = sets
        .into_iter()
        .map(|set| {
            let set : HashSet<u8> = HashSet::from_iter(set);
            set
        })
        .reduce(|l,r| &l & &r);

    *common.unwrap().iter().next().unwrap_or(&0)

}


pub fn solution() {

    let sum = include_bytes!("../input/3.txt")
        .split(|b| *b == b'\n')
        .map(|line| line.split_at(line.len()/2))
        .map(|(l, r)| intersect([l.iter().copied(), r.iter().copied()]))
        .map(|b| (if b >= b'a' {
            b - b'a' + 1
        } else if b >= b'A' {
            b - b'A' + 27
        } else {
            0
        }) as i16)
        .sum::<i16>();


    let sum2 = include_bytes!("../input/3.txt")
        .split(|b| *b == b'\n')
        .collect::<Vec<&[u8]>>()
        .chunks(3)
        .map(|s| intersect(s.iter().map(|c| c.iter().copied())))
        .map(|b| (if b >= b'a' {
            b - b'a' + 1
        } else if b >= b'A'{
            b - b'A' + 27
        } else {
            0
        }) as i16 )
        .sum::<i16>();
    println!("{sum} {sum2}");
}

use std::fs;
// A = X - Rock 1
// B = Y - Paper 2
// C = Z - Scissors 3
// win - 6, draw - 3, lose - 0
// X - lose
// Y - draw
// Z - win
pub fn solution() {
    let input = fs::read_to_string("input/2.txt").unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    let mut max = 0;
    let mut part2 = 0;

    for line in input {
        let x: Vec<&str> = line.split(" ").collect();
        if x[0] == "A" {
            if x[1] == "Y" {
                max += 8;
                part2 += 4;
            } else if x[1] == "X" {
                max += 4;
                part2 += 3;
            } else {
                max += 3;
                part2 += 8;
            }
        } else if x[0] == "B" {
            if x[1] == "Z" {
                max += 9;
                part2 += 9;
            } else if x[1] == "Y" {
                max += 5;
                part2 += 5;
            } else {
                max += 1;
                part2 += 1;
            }
        } else if x[0] == "C" {
            if x[1] == "X" {
                max += 7;
                part2 += 2;
            } else if x[1] == "Z" {
                max += 6;
                part2 += 7;
            } else {
                max += 2;
                part2 += 6;
            }
        }
    }

    println!("{max} {part2}");
}

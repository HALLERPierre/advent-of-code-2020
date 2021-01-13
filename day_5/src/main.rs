const ROWS: usize = 127;
const COLUMNS: usize = 7;

fn main() {
    println!(
        "{}",
        include_str!("../input")
            .split("\n")
            .map(|l| l.chars().collect::<Vec<char>>())
            .filter(|l| l.len() >= 10)
            .map(|binary| {
                let column = (0..7).fold([0, ROWS], |acc, index| {
                    let half: usize = (acc[1] + acc[0]) / 2 as usize;
                    if binary[index] == 'F' {
                        [acc[0], half]
                    } else {
                        [half + 1, acc[1]]
                    }
                })[0];
                let row = (0..3).fold([0, COLUMNS], |acc, index| {
                    let half: usize = (acc[1] + acc[0]) / 2 as usize;
                    if binary[index + 7] == 'L' {
                        [acc[0], half]
                    } else {
                        [half + 1, acc[1]]
                    }
                })[0];
                column * 8 + row
            })
            .fold(0, |max, value| if value > max { value } else { max })
    );
}

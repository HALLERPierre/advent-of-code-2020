pub fn main() {
    let items: Vec<usize> = include_str!("../input")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();
    let part_1 = get_result_of_two_numbers_that_sum_up(items.clone());
    let part_2 = get_result_of_three_numbers_that_sum_up(items.clone());

    println!("{}", part_1);
    println!("{}", part_2);
}

fn get_result_of_two_numbers_that_sum_up(items: Vec<usize>) -> usize {
    for a in &items {
        for b in &items {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    return 0;
}

fn get_result_of_three_numbers_that_sum_up(items: Vec<usize>) -> usize {
    for a in &items {
        for b in &items {
            for c in &items {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    return 0;
}

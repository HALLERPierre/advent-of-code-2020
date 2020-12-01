mod read_input;

fn main() {
    let mut numbers : Vec<i32> = Vec::new();
    if let Ok(lines) = read_input::read_lines("./inputs/input_1") {
        for line in lines {
            if let Ok(number) = line {
                numbers.push(number.parse::<i32>().unwrap())
            }
        }
    }

    let result_two = get_result_of_two_numbers_that_sum_up(numbers.as_mut_slice());
    let result_three = get_result_of_three_numbers_that_sum_up(numbers.as_mut_slice());

    println!("{}", result_two);
    println!("{}", result_three);

}

fn get_result_of_two_numbers_that_sum_up(numbers: &mut [i32]) -> i32 {
    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            let result = number1 + number2;
            if result == 2020 {
                return number1 * number2
            }
        }
    }
    return 0;
}

fn get_result_of_three_numbers_that_sum_up(numbers: &mut [i32]) -> i32 {
    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            for number3 in numbers.iter() {
                let result = number1 + number2 + number3;
                if result == 2020 {
                    return number1 * number2 * number3;
                }
            }
        }
    }
    return 0;
}

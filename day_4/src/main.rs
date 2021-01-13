const EYES: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
const FIELDS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    // let test = include_str!("../input").split("\n\n").map(|fields| {
    //     fields
    //         .split_ascii_whitespace()
    //         .map(|field| field.split(':').next().unwrap())

    // });
    println!(
        "{}",
        include_str!("../input")
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').next().unwrap())
                .collect::<Vec<&str>>())
            .filter(|passport| FIELDS.iter().all(|item| passport.contains(item)))
            .count(),
    );

    println!(
        "{}",
        include_str!("../input")
            .split("\n\n")
            .map(|fields| fields
                .split_ascii_whitespace()
                .map(|field| field.split(':').collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>())
            .filter(|passport| {
                let fields: Vec<&str> = passport.iter().map(|passport| passport[0]).collect();
                FIELDS.iter().all(|item| fields.contains(item))
                    && passport.iter().all(|passport| {
                        let field: &str = passport[0];
                        let value: &str = passport[1];
                        validate_field(field, value)
                    })
            })
            .count(),
    );
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => value.parse::<usize>().unwrap() >= 1920 && value.parse::<usize>().unwrap() <= 2002,
        "iyr" => value.parse::<usize>().unwrap() >= 2010 && value.parse::<usize>().unwrap() <= 2020,
        "eyr" => value.parse::<usize>().unwrap() >= 2020 && value.parse::<usize>().unwrap() <= 2030,
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                value[0..3].parse::<usize>().unwrap() >= 150
                    && value[0..3].parse::<usize>().unwrap() <= 193
            } else if value.ends_with("in") && value.len() == 4 {
                value[0..2].parse::<usize>().unwrap() >= 59
                    && value[0..2].parse::<usize>().unwrap() <= 76
            } else {
                false
            }
        }
        "hcl" => value.len() == 7, // check that it's 0-9 or a-f
        "ecl" => EYES.iter().any(|v| v == &value),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => panic!("unknown field"),
    }
}

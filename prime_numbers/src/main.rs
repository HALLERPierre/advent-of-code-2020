use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    (1i32..).step_by(2).fold(0, |acc, x| {
        if start.elapsed().unwrap().as_secs() >= 10 {
            println!(
                "Found {} primes numbers on {} in {}s",
                acc,
                x,
                start.elapsed().unwrap().as_secs()
            );
            std::process::exit(0);
        }

        let mut y = 3;
        let mut is_prime = true;
        while is_prime && y <= (x as f64).sqrt() as i32 {
            if x % y == 0 {
                is_prime = false;
            }
            y += 2;
        }
        if is_prime {
            acc + 1
        } else {
            acc
        }
    });
}

// if (3..=(x as f64).sqrt() as i32)
//             .step_by(2)
//             .any(|y| x % y == 0)
//         {
//             acc
//         } else {
//             acc + 1
//         }

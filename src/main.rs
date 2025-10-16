use colored::*;
use rand::prelude::*;
use std::time::Duration;
mod utils;

fn draw_box(box_numbers: &[String]) {
    for number in 0..box_numbers.len() {
        if number % 3 == 0 {
            if box_numbers[number] == "X" {
                print!(
                    "{} {} {}",
                    "|".on_truecolor(50, 50, 50).white().bold(),
                    format!("{}", box_numbers[number]).red().bold(),
                    "|".on_truecolor(50, 50, 50).white().bold()
                );
            } else if box_numbers[number] == "Y" {
                print!(
                    "{} {} {}",
                    "|".on_truecolor(50, 50, 50).white().bold(),
                    format!("{}", box_numbers[number]).blue().bold(),
                    "|".on_truecolor(50, 50, 50).white().bold()
                );
            } else {
                print!(
                    "{}",
                    format!("| {} |", box_numbers[number])
                        .on_truecolor(50, 50, 50)
                        .white()
                        .bold()
                );
            }
        } else {
            if box_numbers[number] == "X" {
                print!(
                    " {} {}",
                    format!("{}", box_numbers[number]).red().bold(),
                    "|".on_truecolor(50, 50, 50).white().bold()
                );
            } else if box_numbers[number] == "Y" {
                print!(
                    " {} {}",
                    format!("{}", box_numbers[number]).blue().bold(),
                    "|".on_truecolor(50, 50, 50).white().bold()
                );
            } else {
                print!(
                    "{}",
                    format!(" {} |", box_numbers[number])
                        .on_truecolor(50, 50, 50)
                        .white()
                        .bold()
                );
            }
        }
        if number == 2 || number == 5 {
            print!("\n");
        }
    }
    print!("\n");
}

fn check_winner(box_numbers: &[String]) -> bool {
    let winning_lines = [
        (&box_numbers[0], &box_numbers[1], &box_numbers[2]),
        (&box_numbers[3], &box_numbers[4], &box_numbers[5]),
        (&box_numbers[6], &box_numbers[7], &box_numbers[8]),
        (&box_numbers[0], &box_numbers[3], &box_numbers[6]),
        (&box_numbers[1], &box_numbers[4], &box_numbers[7]),
        (&box_numbers[2], &box_numbers[5], &box_numbers[8]),
        (&box_numbers[0], &box_numbers[4], &box_numbers[8]),
        (&box_numbers[2], &box_numbers[4], &box_numbers[6]),
    ];
    for line in winning_lines {
        if line.0 == line.1 && line.1 == line.2 {
            return true;
        }
    }

    false
}

fn is_any_number_left(box_numbers: &[String]) -> bool {
    if box_numbers.contains(&"0".to_string())
        || box_numbers.contains(&"1".to_string())
        || box_numbers.contains(&"2".to_string())
        || box_numbers.contains(&"3".to_string())
        || box_numbers.contains(&"4".to_string())
        || box_numbers.contains(&"5".to_string())
        || box_numbers.contains(&"6".to_string())
        || box_numbers.contains(&"7".to_string())
        || box_numbers.contains(&"8".to_string())
    {
        return true;
    }

    return false;
}
fn main() {
    let mut rng = rand::rng();
    let mut nums: Vec<u8> = (0..9).collect();
    nums.shuffle(&mut rng);
    let mut box_numbers: Vec<String> = nums.iter().map(|num| num.to_string()).collect();
    let mut is_player_won = false;
    let mut is_player_x_afk = false;
    let mut is_player_y_afk = false;

    let mut players_array = ["x", "y"];
    players_array.shuffle(&mut rng);

    while is_any_number_left(&box_numbers) == true {
        if players_array[0] == "x" {
            player_x(&mut box_numbers, &mut is_player_won, &mut is_player_x_afk);
            if is_player_won || !is_any_number_left(&box_numbers) {
                break;
            }

            player_y(&mut box_numbers, &mut is_player_won, &mut is_player_y_afk);
            // println!("{}", is_any_number_left(&box_numbers));
            if is_player_won || !is_any_number_left(&box_numbers) {
                break;
            }
        } else {
            player_y(&mut box_numbers, &mut is_player_won, &mut is_player_y_afk);
            // println!("{}", is_any_number_left(&box_numbers));
            if is_player_won || !is_any_number_left(&box_numbers) {
                break;
            }

            player_x(&mut box_numbers, &mut is_player_won, &mut is_player_x_afk);
            if is_player_won || !is_any_number_left(&box_numbers) {
                break;
            }
        }
    }
    if is_any_number_left(&box_numbers) == false && is_player_won == false {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        println!(
            "{}",
            format!("It's a Draw!").white().on_black().bold().italic()
        );

        utils::play_audio("assets/draw.mp3");
        std::thread::sleep(Duration::from_secs(2));
    }
    if is_player_y_afk && is_player_x_afk {
        panic!("AFK DETECTED!")
    }
}

fn player_x(box_numbers: &mut [String], is_player_won: &mut bool, is_player_x_afk: &mut bool) {
    while is_any_number_left(&box_numbers) == true {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);
        let user_input: String = match utils::threaded_x_input(is_player_x_afk) {
            Some(input) => input,
            _ => break,
        };

        let parsed_input: u32 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
            continue;
        }

        match box_numbers.iter().position(|i| i == user_input.trim()) {
            Some(index) => box_numbers[index] = "X".to_string(),
            None => {
                println!("Entered number place is already taken!");
                continue;
            }
        }

        if check_winner(&box_numbers) {
            clearscreen::clear().expect("failed to clear screen");
            draw_box(&box_numbers);
            println!("{}", format!("Winner: PLayer X!").red().bold().italic());
            // std::thread::sleep(Duration::from_secs(1));
            utils::play_audio("assets/winner_x.mp3");
            *is_player_won = true;
            std::thread::sleep(Duration::from_secs(2));
            break;
        }
        break;
    }
}

fn player_y(box_numbers: &mut [String], is_player_won: &mut bool, is_player_y_afk: &mut bool) {
    while is_any_number_left(&box_numbers) == true {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        let user_input: String = match utils::threaded_y_input(is_player_y_afk) {
            Some(input) => input,
            _ => break,
        };
        let parsed_input: u32 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => continue,
        };

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
            continue;
        }

        match box_numbers.iter().position(|i| i == user_input.trim()) {
            Some(index) => box_numbers[index] = "Y".to_string(),
            None => {
                println!("Entered number place is already taken!");
                continue;
            }
        }

        if check_winner(&box_numbers) {
            clearscreen::clear().expect("failed to clear screen");
            draw_box(&box_numbers);
            println!("{}", format!("Winner: PLayer Y!").blue().bold().italic());

            // std::thread::sleep(Duration::from_secs(1));
            utils::play_audio("assets/winner_y.mp3");
            *is_player_won = true;
            std::thread::sleep(Duration::from_secs(2));
            break;
        }

        break;
    }
}

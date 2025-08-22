use colored::*;
use std::io::{self};

fn draw_box(box_numbers: &Vec<&str>) {
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

fn check_winner<'a>(box_numbers: &'a Vec<&'a str>) -> Option<&'a str> {
    let winning_lines = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    for &(a, b, c) in &winning_lines {
        if box_numbers[a] == box_numbers[b]
            && box_numbers[b] == box_numbers[c]
            && (box_numbers[a] == "X" || box_numbers[a] == "Y")
        {
            return Some(box_numbers[a]);
        }
    }

    None
}

fn check_box_filled(box_numbers: &Vec<&str>) -> bool {
    return box_numbers.iter().any(|&num| num >= "0" && num <= "8");
}

fn main() {
    let mut box_numbers: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8"];
    clearscreen::clear().expect("failed to clear screen");

    while check_box_filled(&box_numbers) == true {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        //get player X input;
        println!("{}", format!("Player X: Enter a number: ").italic());
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: u32 = input.trim().parse().expect("failed to parse the input");

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
        }

        // change number to X
        match box_numbers.iter().position(|&i| i == input.trim()) {
            Some(index) => box_numbers[index] = "X",
            None => {
                println!("Entered number place is already taken!");
                continue;
            }
        }

        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);
        if check_box_filled(&box_numbers) == false {
            println!("Draw!");
            break;
        }

        match check_winner(&box_numbers) {
            Some(player) => {
                println!(
                    "{}",
                    format!("PLayer {} Won", player).red().bold().italic()
                );
                break;
            }
            _ => (),
        }

        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);
        //get player Y input;
        println!("Player Y: Enter a number: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: u32 = input.trim().parse().expect("failed to parse the input");

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
        }

        // change number to Y
        match box_numbers.iter().position(|&i| i == input.trim()) {
            Some(index) => box_numbers[index] = "Y",
            None => {
                println!("Entered number place is already taken!");
                continue;
            }
        }

        //select Y as winner
        match check_winner(&box_numbers) {
            Some(player) => {
                println!(
                    "{}",
                    format!("PLayer {} Won", player).blue().bold().italic()
                );
                break;
            }
            _ => (),
        }
    }
}

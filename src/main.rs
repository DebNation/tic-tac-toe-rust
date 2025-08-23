use colored::*;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};
use rand::prelude::*;
use std::{
    io::{self},
    time::Duration,
};

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

    while is_any_number_left(&box_numbers) == true {
        player_x(&mut box_numbers, &mut is_player_won);
        if is_player_won || !is_any_number_left(&box_numbers) {
            break;
        }

        player_y(&mut box_numbers, &mut is_player_won);
        println!("{}", is_any_number_left(&box_numbers));
        if is_player_won || !is_any_number_left(&box_numbers) {
            break;
        }
    }
    if is_any_number_left(&box_numbers) == false && is_player_won == false {
        let mut manager =
            AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let draw_audio = StaticSoundData::from_file("assets/draw.mp3").unwrap();
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        println!(
            "{}",
            format!("It's a Draw!").white().on_black().bold().italic()
        );

        manager.play(draw_audio.clone()).unwrap();
        std::thread::sleep(Duration::from_secs(2));
    }
}

fn player_x(box_numbers: &mut [String], is_player_won: &mut bool) {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    // let enter_x_audio = StaticSoundData::from_file("assets/player_x_enter.mp3").unwrap();
    let winner_x = StaticSoundData::from_file("assets/winner_x.mp3").unwrap();
    while is_any_number_left(&box_numbers) == true {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        println!("{}", format!("Player X: Enter a number: ").italic());
        // std::thread::sleep(Duration::from_secs(1));
        // manager.play(enter_x_audio.clone()).unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: u32 = input.trim().parse().expect("failed to parse the input");

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
            continue;
        }

        match box_numbers.iter().position(|i| i == input.trim()) {
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
            manager.play(winner_x.clone()).unwrap();
            *is_player_won = true;
            std::thread::sleep(Duration::from_secs(2));
            break;
        }
        break;
    }
}

fn player_y(box_numbers: &mut [String], is_player_won: &mut bool) {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    // let enter_y_audio = StaticSoundData::from_file("assets/player_y_enter.mp3").unwrap();
    let winner_y = StaticSoundData::from_file("assets/winner_y.mp3").unwrap();
    while is_any_number_left(&box_numbers) == true {
        clearscreen::clear().expect("failed to clear screen");
        draw_box(&box_numbers);

        println!("{}", format!("Player Y: Enter a number: ").italic());
        // std::thread::sleep(Duration::from_secs(1));
        // manager.play(enter_y_audio.clone()).unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: u32 = input.trim().parse().expect("failed to parse the input");

        if parsed_input > 8 {
            println!("You have entered an invalid number!");
            continue;
        }

        match box_numbers.iter().position(|i| i == input.trim()) {
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
            manager.play(winner_y.clone()).unwrap();
            *is_player_won = true;
            std::thread::sleep(Duration::from_secs(2));
            break;
        }

        break;
    }
}

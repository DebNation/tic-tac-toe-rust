use colored::Colorize;
use std::io;

fn draw_box(box_numbers: &Vec<&str>) {
    for number in 0..box_numbers.len() {
        if number == 0 || number == 3 || number == 6 {
            print!("| {} |", box_numbers[number].yellow().bold());
        } else {
            print!(" {} |", box_numbers[number].yellow().bold())
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
            && (box_numbers[a] == "X" || box_numbers[a] == "O")
        {
            return Some(box_numbers[a]);
        }
    }

    None
}

fn check_box_filled(box_numbers: &Vec<&str>) -> bool {
    if box_numbers.contains(&"0")
        || box_numbers.contains(&"1")
        || box_numbers.contains(&"2")
        || box_numbers.contains(&"3")
        || box_numbers.contains(&"4")
        || box_numbers.contains(&"5")
        || box_numbers.contains(&"6")
        || box_numbers.contains(&"7")
        || box_numbers.contains(&"8")
    {
        return true;
    }
    return false;
}

fn main() {
    let mut box_numbers: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8"];
    //draw the box
    draw_box(&box_numbers);

    while check_box_filled(&box_numbers) == true {
        //get player X input;
        println!("Player X: Enter a number: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: &str = input.trim();
        // change number to X
        match box_numbers.iter().position(|&i| i == parsed_input) {
            Some(index) => box_numbers[index] = "X",
            None => {
                println!("sorry, Enter number is not found or place is already taken");
                continue;
            }
        }
        if check_box_filled(&box_numbers) == false {
            println!("Box is Filled, No winner yet!");
            break;
        }
        draw_box(&box_numbers);

        //choose winner
        match check_winner(&box_numbers) {
            Some(player) => {
                println!("Player {} won!", player);
                break;
            }
            _ => (),
        }

        //get player O input;
        println!("Player O: Enter a number: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parsed_input: &str = input.trim();

        // change number to O
        match box_numbers.iter().position(|&i| i == parsed_input) {
            Some(index) => box_numbers[index] = "O",
            None => {
                println!("sorry, Enter number is not found or place is already taken");
                continue;
            }
        }

        draw_box(&box_numbers);

        //choose winner
        match check_winner(&box_numbers) {
            Some(player) => {
                println!("Player {} won!", player);
                break;
            }
            _ => (),
        }
    }
}

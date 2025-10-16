use colored::*;
use std::{io::stdin, sync::mpsc, thread, time::Duration};

pub fn threaded_x_input(is_player_x_afk: &mut bool) -> Option<String> {
    let (sender, receiver) = mpsc::channel::<String>();

    let input_sender_fn = move || {
        let mut input: String = String::new();
        println!("{}", format!("Player X, Enter a number: ").italic());
        if stdin().read_line(&mut input).is_ok() {
            sender.send(input).unwrap()
        }
    };

    let _handle = thread::spawn(input_sender_fn);

    match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(input) => {
            return Some(input);
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            play_audio("assets/player_x_enter.mp3");
            thread::sleep(Duration::from_secs(2));
        }
        Err(_) => return None,
    }

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(input) => return Some(input),
        Err(_) => {
            play_audio("assets/timeout.mp3");
            thread::sleep(Duration::from_secs(1));
            *is_player_x_afk = true;
            return None;
        }
    };
}

pub fn threaded_y_input(is_player_y_afk: &mut bool) -> Option<String> {
    let (sender, receiver) = mpsc::channel::<String>();

    let input_sender_fn = move || {
        let mut input: String = String::new();
        println!("{}", format!("Player Y, Enter a number: ").italic());
        if stdin().read_line(&mut input).is_ok() {
            sender.send(input).unwrap()
        }
    };

    let _handle = thread::spawn(input_sender_fn);

    match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(input) => {
            return Some(input);
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            play_audio("assets/player_y_enter.mp3");
            thread::sleep(Duration::from_secs(2));
        }
        Err(_) => return None,
    }

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(input) => return Some(input),
        Err(_) => {
            play_audio("assets/timeout_audio.mp3");
            thread::sleep(Duration::from_secs(1));
            *is_player_y_afk = true;
            return None;
        }
    };
}

pub fn play_audio(audio_path: &str) {
    std::process::Command::new("mpv")
        .arg("--ao=pulse")
        .arg(audio_path)
        .output()
        .expect("mpv is not found");
}

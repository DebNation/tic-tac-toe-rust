use colored::*;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, sound::static_sound::StaticSoundData,
};
use std::{io::stdin, sync::mpsc, thread, time::Duration};

pub fn threaded_x_input() -> Option<String> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let enter_x_audio = StaticSoundData::from_file("assets/player_x_enter.mp3").unwrap();
    let timeout_audio = StaticSoundData::from_file("assets/timeout.mp3").unwrap();
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
            manager.play(enter_x_audio.clone()).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        Err(_) => return None,
    }

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(input) => return Some(input),
        Err(_) => {
            manager.play(timeout_audio).unwrap();
            thread::sleep(Duration::from_secs(1));
            return None;
        }
    };
}

pub fn threaded_y_input() -> Option<String> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let enter_y_audio = StaticSoundData::from_file("assets/player_y_enter.mp3").unwrap();
    let timeout_audio = StaticSoundData::from_file("assets/timeout.mp3").unwrap();
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
            manager.play(enter_y_audio.clone()).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
        Err(_) => return None,
    }

    match receiver.recv_timeout(Duration::from_secs(5)) {
        Ok(input) => return Some(input),
        Err(_) => {
            manager.play(timeout_audio).unwrap();
            thread::sleep(Duration::from_secs(1));
            return None;
        }
    };
}

mod utils;

use std::fs::File;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType, Key as RdevKey};
use std::{thread};
use std::io::{BufRead, BufReader};
use std::time::Duration;
use std::option::Option;
use std::sync::mpsc;
use utils::str_helper::str_to_key;
use utils::log::log;
use crate::utils::log::LogType;
use utils::stratagems::{StratagemMap,ReversedStratagemMap};

const W: u16 = 0x11;
const A: u16 = 0x1E;
const S: u16 = 0x1F;
const D: u16 = 0x20;

fn main() {
    let stratagems_cont = extract_stratagem_calls();
    if stratagems_cont.is_none() {
        log("stratagems not provided", LogType::Error);
    }
    let stratagems = stratagems_cont.unwrap();
    if stratagems.len() == 0 {
        log("stratagems not found", LogType::Error);
    }
    log("Stratagems imported successfully", LogType::Info);
    for stratagem in stratagems.iter().clone() {
        log(format!(
            "trigger: {:?}, stratagem: {:?}, call: {:?}",
            stratagem.trigger, stratagem.call_label, stratagem.human_call()
        ), LogType::Info);
    }
    let (tx, rx) = mpsc::channel::<StratagemCall>();


    let handle = thread::spawn(move || {
        key_handler(rx);
    });


    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            for strat in &stratagems {
                if strat.trigger == key {
                    let _ = tx.send(StratagemCall::new(strat.trigger, strat.call.clone(), strat.call_label.clone()));
                }
            }
        }
    };

    if let Err(error) = listen(callback) {
        log(format!("Ошибка при прослушивании событий: {:?}", error), LogType::Error);
    }

    handle.join().unwrap();
}


fn key_handler(rx: mpsc::Receiver<StratagemCall>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    while let Ok(strat) = rx.recv() {
        log(format!(
            "event received, trigger: {:?}, call: {:?}, directions: {:?}",
            strat.trigger, strat.call_label, strat.call
        ), LogType::Info);
        enigo.key(Key::Control, Direction::Press).unwrap();
        thread::sleep(Duration::from_millis(60));

        for key in strat.call {
            enigo.raw(key, Direction::Press).unwrap();
            thread::sleep(Duration::from_millis(20));
            enigo.raw(key, Direction::Release).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
        thread::sleep(Duration::from_millis(60));
        enigo.key(Key::Control, Direction::Release).unwrap();
    }
}


struct StratagemCall {
    trigger: RdevKey,
    call: Vec<u16>,
    call_label: String,
}

impl StratagemCall {
    fn new(
        trigger: RdevKey,
        call: Vec<u16>,
        mut call_label: String,
    ) -> StratagemCall {
        if call_label == "" {
            call_label = String::from("no name");
        }
        StratagemCall {
            trigger,
            call,
            call_label,
        }
    }

    fn human_call(&self) -> String {
        let mut s = String::new();
        for ch in self.call.iter() {
            let key = match ch {
                &W => "w", &S => "s", &A => "a", &D => "d",
                _ => "",
            };
            s.push_str(key);
        }
        s
    }
}

fn extract_stratagem_calls() -> Option<Vec<StratagemCall>> {
    let file = File::open("config.txt").ok()?;
    let mut stratagem_calls: Vec<StratagemCall>= Vec::new();
    let reader = BufReader::new(file);


    for line in reader.lines() {
        let line_str = line.ok()?;
        let parts: Vec<_> = line_str.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        let trigger = str_to_key(parts[0]);
        if trigger.is_none() {
            continue;
        }
        let trigger:RdevKey = trigger.unwrap();
        let mut call_str = parts[1];
        let mut call_label = String::from("no name");

        let call_from_map = StratagemMap[call_str];
        if call_from_map != "" {
            call_label = String::from(call_str);
            call_str = call_from_map;
        }
        if call_label == "no name" {
            let reversed_call_from_map = ReversedStratagemMap[call_str];
            if reversed_call_from_map != "" {
                call_label = String::from(reversed_call_from_map);
            }
        }

        let mut call: Vec<u16> = Vec::new();
        for ch in call_str.to_lowercase().chars() {
            let key = match ch {
                'w' => W, 's' => S, 'a' => A, 'd' => D,
                _ => return None,
            };
            call.push(key);
        }
        stratagem_calls.push(StratagemCall::new(trigger, call, call_label));
    }

    Some(stratagem_calls)
}

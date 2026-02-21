mod utils;

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType};
use std::{thread};
use std::time::Duration;
use std::sync::mpsc;
use utils::log::log;
use crate::utils::log::LogType;
use std::collections::HashSet;
use utils::stratagems::{extract_stratagem_calls,StratagemCall};

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

    let mut pressed_keys = HashSet::new();
    let mut active_triggers = HashSet::new();

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                pressed_keys.insert(key);

                for (idx, strat) in stratagems.iter().enumerate() {
                    let is_active = strat.trigger.iter().all(|k| pressed_keys.contains(k));

                    if is_active && !active_triggers.contains(&idx) {
                        let _ = tx.send(StratagemCall::new(
                            strat.trigger.clone(),
                            strat.call.clone(),
                            strat.call_label.clone(),
                        ));
                        active_triggers.insert(idx);
                    }
                }
            }

            EventType::KeyRelease(key) => {
                pressed_keys.remove(&key);

                active_triggers.retain(|idx| {
                    stratagems[*idx]
                        .trigger
                        .iter()
                        .all(|k| pressed_keys.contains(k))
                });
            }

            _ => {}
        }
    };

    if let Err(error) = listen(callback) {
        log(format!("Error when listen events: {:?}", error), LogType::Error);
    }

    handle.join().unwrap();
}


fn key_handler(rx: mpsc::Receiver<StratagemCall>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    while let Ok(strat) = rx.recv() {
        log(format!(
            "event received, trigger: {:?}, call: {:?}, directions: {:?}",
            strat.trigger, strat.call_label, strat.human_call()
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
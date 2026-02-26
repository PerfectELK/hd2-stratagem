mod utils;

use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use rdev::{listen, Event, EventType};
use std::{env, fs, thread};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use utils::log::log;
use crate::utils::log::LogType;
use crate::utils::windows::{get_active_window_info,is_hd2};
use std::collections::HashSet;
use utils::stratagems::{extract_stratagem_calls, StratagemCall};
use dialoguer::{Select, console::Term};
use dialoguer::theme::ColorfulTheme;

fn main() {
    let filename = get_config_file_name();
    if filename.is_none() {
        log("error when get config file name", LogType::Error);
    }
    let filename = filename.unwrap();
    let stratagems_cont = extract_stratagem_calls(filename.as_str());
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
            stratagem.trigger_in_order, stratagem.call_label, stratagem.human_call()
        ), LogType::Info);
    }
    let (tx, rx) = mpsc::channel::<StratagemCall>();


    let handle = thread::spawn(move || {
        key_handler(rx);
    });

    let mut pressed_keys = HashSet::new();
    let mut active_triggers = HashSet::new();

    let is_hd2_current_app: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let monitor = is_hd2_current_app.clone();
    monitor_current_app(monitor);

    let callback = move |event: Event| {
        if !*is_hd2_current_app.lock().unwrap() {
            pressed_keys.clear();
            active_triggers.clear();
            return;
        }
        match event.event_type {
            EventType::KeyPress(key) => {
                pressed_keys.insert(key);

                for (idx, strat) in stratagems.iter().enumerate() {
                    let is_active = strat.trigger.iter().all(|k| pressed_keys.contains(k));

                    if is_active && !active_triggers.contains(&idx) {
                        let _ = tx.send(StratagemCall::new(
                            strat.trigger.clone(),
                            strat.trigger_in_order.clone(),
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
            strat.trigger_in_order, strat.call_label, strat.human_call()
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

fn get_config_file_name() -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let entries = fs::read_dir(current_dir).ok()?;

    let txt_files: Vec<_> = entries.filter_map(|entry|{
        let entry = entry.ok()?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "txt" {
                    return Some(entry.file_name().into_string().ok()?);
                }
            }
        }
        None
    }).collect();

    if txt_files.is_empty() {
        return None
    }

    if txt_files.len() == 1 {
        return Some(txt_files[0].clone())
    }

    let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt(
        "Please select a config file"
    ).items(&txt_files).interact_on_opt(&Term::stderr()).ok()?;

    match selection {
        Some(index) => {
            Some(txt_files[index].clone())
        }
        None => {
            log("Select was canceled", LogType::Error);
            None
        }
    }
}

fn monitor_current_app(monitor: Arc<Mutex<bool>>) {
    thread::spawn(move || {
        loop {
            if let Some(name) = get_active_window_info() {
                log(format!("App name: {:?}", name.to_lowercase()), LogType::Debug);
                let is_hd2 = is_hd2(name);
                let mut data = monitor.lock().unwrap();
                *data = is_hd2;
            }

            thread::sleep(Duration::from_millis(1000));
        }
    });
}
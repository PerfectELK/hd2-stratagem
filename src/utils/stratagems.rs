
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use once_cell::sync::Lazy;
use crate::utils::str_helper::str_to_key;
use rdev::{Key as RdevKey};

const W: u16 = 0x11;
const A: u16 = 0x1E;
const S: u16 = 0x1F;
const D: u16 = 0x20;

pub static STRATAGEM_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("femoid", "wdsss");
    m.insert("supply", "sswd");
    m.insert("res", "wsdaw");
    m.insert("sos", "wsdw");
    m.insert("gas", "ddsd");
    m.insert("laser", "dswds");
    m.insert("380", "dswwass");
    m.insert("120", "ddsads");
    m
});

pub static REVERSED_STRATAGEM_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (k, v) in STRATAGEM_MAP.iter() {
        m.insert(*v, *k);
    }
    m
});

pub struct StratagemCall {
    pub trigger: HashSet<RdevKey>,
    pub trigger_in_order: Vec<RdevKey>,
    pub call: Vec<u16>,
    pub call_label: String,
}

impl StratagemCall {
    pub(crate) fn new(
        trigger: HashSet<RdevKey>,
        trigger_in_order: Vec<RdevKey>,
        call: Vec<u16>,
        mut call_label: String,
    ) -> StratagemCall {
        if call_label == "" {
            call_label = String::from("no name");
        }
        StratagemCall {
            trigger,
            trigger_in_order,
            call,
            call_label,
        }
    }

    pub(crate) fn human_call(&self) -> String {
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

pub fn extract_stratagem_calls(file_name: &str) -> Option<Vec<StratagemCall>> {
    let file = File::open(file_name).ok()?;
    let mut stratagem_calls: Vec<StratagemCall>= Vec::new();
    let reader = BufReader::new(file);


    for line in reader.lines() {
        let line_str = line.ok()?;
        let parts: Vec<_> = line_str.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        let trigger = extract_trigger(parts[0]);
        if trigger.is_none() {
            continue;
        }
        let trigger = trigger.unwrap();
        let call = extract_call(parts[1]);
        if call.is_none() {
            continue;
        }
        let call = call.unwrap();
        stratagem_calls.push(StratagemCall::new(trigger.0, trigger.1, call.0, call.1));
    }

    Some(stratagem_calls)
}


fn extract_trigger(mut trigger_str: &str) -> Option<Box<(HashSet<RdevKey>, Vec<RdevKey>)>> {
    trigger_str = trigger_str.trim();
    if trigger_str.is_empty() {
        return None
    }
    let mut hash: HashSet<RdevKey> = HashSet::new();
    let mut trigger_arr: Vec<RdevKey> = Vec::new();
    let trigger = str_to_key(trigger_str);
    if trigger.is_some() {
        let trigger:RdevKey = trigger.unwrap();
        hash.insert(trigger);
        trigger_arr.push(trigger);
        return Some(Box::new((hash, trigger_arr)))
    }
    let first = trigger_str.chars().next().unwrap();
    let last = trigger_str.chars().last().unwrap();
    if first != '(' && last != ')' {
        return None;
    }
    let trigger_str: String = trigger_str.chars().skip(1).take(trigger_str.chars().count() - 2).collect();
    let triggers: Vec<&str> = trigger_str.split(',').filter(|s| !s.is_empty()).collect();

    for t in triggers {
        let trigger = str_to_key(t);
        if trigger.is_none() {
            return None
        }
        hash.insert(trigger.unwrap());
        trigger_arr.push(trigger.unwrap());
    }
    Some(Box::new((hash, trigger_arr)))
}


fn extract_call(mut call_str: &str) -> Option<Box<(Vec<u16>, String)>> {
    let mut call_label = String::from("no name");
    let call_from_map = STRATAGEM_MAP.get(call_str).unwrap_or(&"");
    if call_from_map != &"" {
        call_label = String::from(call_str);
        call_str = call_from_map;
    }
    if call_label == "no name" {
        let reversed_call_from_map = REVERSED_STRATAGEM_MAP.get(call_str).unwrap_or(&"");
        if reversed_call_from_map != &"" {
            call_label = String::from(*reversed_call_from_map);
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
    Some(Box::new((call, call_label)))
}
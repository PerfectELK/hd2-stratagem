
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static StratagemMap: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("feminism", "wdsss");
    m.insert("supply", "sswd");
    m.insert("res", "wsdaw");
    m.insert("sos", "wsdw");
    m.insert("gas", "ddsd");
    m.insert("laser", "dswds");
    m.insert("380", "dswwass");
    m.insert("120", "ddsads");
    m
});

pub static  ReversedStratagemMap: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (k, v) in StratagemMap.iter() {
        m.insert(*v, *k);
    }
    m
});
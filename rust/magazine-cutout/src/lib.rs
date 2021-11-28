// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_map = HashMap::new();

    for key in magazine.iter() {
        let val = mag_map.entry(key).or_insert(0);
        *val += 1;
    }

    for key in note.iter() {
        let val = mag_map.entry(key).or_insert(0);
        if *val == 0 {
            return false;
        }
        *val -= 1;
    }

    true
}

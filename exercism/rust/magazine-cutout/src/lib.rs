// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazines = HashMap::new();
    let mut notes = HashMap::new();
    for word in magazine {
        magazines.entry(word).and_modify(|e| { *e += 1 }).or_insert(1);
    }
    for word in note {
        notes.entry(word).and_modify(|e| { *e += 1 }).or_insert(1);
    }
    for (word, value) in &notes {
        if magazines.get(word).unwrap_or(&0) < value {
            return false;
        }
    }
    true
}

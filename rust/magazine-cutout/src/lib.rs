// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_dic = HashMap::new();
    let mut note_dic = HashMap::new();

    for word in magazine {
        let v = magazine_dic.get(word);
        magazine_dic.insert(word, v.map_or(0, |v| v + 1));
    }

    for word in note {
        let v = note_dic.get(word);
        note_dic.insert(word, v.map_or(0, |v| v + 1));
    }

    note.iter().all(|word| {
        let m_count = magazine_dic.get(word);

        match m_count {
            Some(m_count) => {
                let n_count = note_dic.get(word).unwrap_or(&0);
                m_count >= n_count
            }
            None => false,
        }
    })
}

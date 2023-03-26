#![feature(map_many_mut)]
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TeamData<'a> {
    pub name: &'a str,
    pub mp: i32,
    pub w: i32,
    pub d: i32,
    pub l: i32,
    pub p: i32,
}

impl<'a> TeamData<'a> {
    fn new(name: &'a str) -> Self {
        TeamData {
            name,
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        }
    }
}

impl<'a> PartialOrd for TeamData<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.p.partial_cmp(&self.p)
    }
}

impl<'a> Ord for TeamData<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.p.cmp(&self.p)
    }
}

type Teams<'a> = HashMap<&'a str, TeamData<'a>>;

pub fn tally(match_results: &str) -> String {
    let mut header = "Team                           | MP |  W |  D |  L |  P".to_string();
    if match_results.is_empty() {
        return header.to_string();
    };

    let mut list: Teams = HashMap::new();

    match_results.lines().for_each(|line| {
        let t: Vec<&str> = line.split(';').collect();
        let (subj, target, action) = (t[0], t[1], t[2]);

        [subj, target].into_iter().for_each(|v| {
            let data = list.get_mut(v);
            if let None = data {
                list.insert(v, TeamData::new(v));
            }
        });

        let [subj_data, target_data] = list.get_many_mut([subj, target]).unwrap();

        match action {
            "win" => {
                subj_data.w += 1;
                subj_data.p += 3;
                target_data.l += 1;
            }
            "loss" => {
                target_data.w += 1;
                target_data.p += 3;
                subj_data.l += 1;
            }
            "draw" => {
                target_data.p += 1;
                subj_data.p += 1;
                target_data.d += 1;
                subj_data.d += 1;
            }
            _ => {}
        };

        subj_data.mp += 1;
        target_data.mp += 1;
    });

    let mut sorted = list.clone().into_iter().map(|d| d.1).collect::<Vec<_>>();
    sorted.sort();

    for data in sorted {
        println!("I am first {}", data.name);
        let mut name = [" "; 31].join("");
        name.replace_range(..data.name.len(), data.name);

        header = header
            + "\n"
            + name.as_str()
            + format!(
                "|  {} |  {} |  {} |  {} |  {}",
                data.mp, data.w, data.d, data.l, data.p
            )
            .as_str();
    }

    header
}

// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale(Vec<String>);

const CHROMATIC_SHARP: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

const CHROMATIC_FLAT: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let chromatic = match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "C" | "a" => {
                CHROMATIC_SHARP
            }
            _ => CHROMATIC_FLAT,
        };

        let tonic = capitalize(&tonic.to_string());
        let mut scale = vec![tonic.clone()];
        let tonic_i = chromatic.into_iter().position(|v| v == tonic).unwrap();

        intervals.chars().fold(tonic_i, |acc, v| {
            let interval = match v {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => 0,
            };
            let note_i = (acc + interval) % chromatic.len();
            scale.push(chromatic[note_i].to_string());
            return note_i;
        });

        Ok(Self(scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        let chromatic = match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "C" | "a" => {
                CHROMATIC_SHARP
            }
            _ => CHROMATIC_FLAT,
        };
        let tonic_i = chromatic.into_iter().position(|v| v == tonic).unwrap_or(0);
        let mut scale = vec!["".to_string(); 13];

        chromatic
            .into_iter()
            .skip(tonic_i)
            .chain(chromatic.into_iter().take(tonic_i + 1))
            .enumerate()
            .for_each(|(i, v)| scale[i] = v.to_string());

        Ok(Self(scale))
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}

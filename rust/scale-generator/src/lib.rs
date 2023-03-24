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

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let chromatic = match tonic {
            "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "C" | "a" => {
                CHROMATIC_SHARP
            }
            _ => CHROMATIC_FLAT,
        };

        let mut pos = chromatic
            .iter()
            .position(|&n| n.to_uppercase() == tonic.to_uppercase())
            .unwrap();
        let mut scale = vec![chromatic[pos].to_string()];

        for interval in intervals.chars() {
            pos += match interval {
                'm' => 1,
                'M' => 2,
                'A' => 3,
                _ => 0,
            };
            scale.push(chromatic[pos % chromatic.len()].to_string());
        }

        Ok(Self(scale))
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Self::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.0.clone()
    }
}

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

pub struct Scale {
    tonic: String,
    scale: Vec<String>,
}

const CHROMATIC: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let tonic_i = CHROMATIC.into_iter().position(|v| v == tonic).unwrap();
        let intervals: Vec<usize> = intervals
            .chars()
            .map(|c| match c {
                'm' => 1,
                'M' => 2,
                _ => 0,
            })
            .collect();

        let scale: Vec<String> = intervals
            .into_iter()
            .map(|n| {
                let note_i = (tonic_i + n) % CHROMATIC.len();
                CHROMATIC.get(note_i).unwrap().to_string()
            })
            .collect();

        Ok(Scale {
            tonic: tonic.to_string(),
            scale,
        })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        unimplemented!("Construct a new chromatic scale with tonic {tonic}")
    }

    pub fn enumerate(&self) -> Vec<String> {
        unimplemented!()
    }
}

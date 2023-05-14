use serde::{Deserialize, Serialize};

/// Holds all the question data. `a` and `b` are the possible answers, and `correct` is a char
/// that's either `a` or `b`.
#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub a: String,
    pub b: String,
    pub correct: char,
    pub number: u8,
}

/// Returns a question based on the question ID provided
pub fn get_question(number: u8) -> Question {
    match number {
        1 => Question {
            question: String::from("What is the name of Rome's largest amphitheatre?"),
            a: String::from("The Colosseum"),
            b: String::from("The Globe Theatre"),
            correct: 'a',
            number: 1,
        },
        2 => Question {
            question: String::from("What river runs through Rome?"),
            a: String::from("The Thames"),
            b: String::from("The Tiber"),
            correct: 'b',
            number: 2,
        },
        3 => Question {
            question: String::from("Which famous artist painted the ceiling of the Sistine Chapel in Vatican City?"),
            a: String::from("Van Gogh"),
            b: String::from("Michelangelo"),
            correct: 'b',
            number: 3,
        },
        4 => Question {
            question: String::from("What is the traditional Roman dish of spaghetti with cheese and pepper?"),
            a: String::from("Cacio e Pepe"),
            b: String::from("Carbonara"),
            correct: 'a',
            number: 4,
        },
        5 => Question {
            question: String::from("Which of Rome's hills is home to the Roman Forum?"),
            a: String::from("Palatine Hill"),
            b: String::from("Capitoline Hill"),
            correct: 'a',
            number: 5,
        },
        6 => Question {
            question: String::from("Which iconic Rome building is known for its oculus?"),
            a: String::from("The Pantheon"),
            b: String::from("The Colosseum"),
            correct: 'a',
            number: 6,
        },
        7 => Question {
            question: String::from("What style of architecture is Rome's Trevi Fountain?"),
            a: String::from("Gothic"),
            b: String::from("Baroque"),
            correct: 'b',
            number: 7,
        },
        8 => Question {
            question: String::from("How many hills is Rome famously built on?"),
            a: String::from("Seven (7)"),
            b: String::from("Five (5)"),
            correct: 'a',
            number: 8,
        },
        9 => Question {
            question: String::from("Which museum in Rome houses a vast collection of Roman sculptures and mosaics?"),
            a: String::from("Capitoline Museums"),
            b: String::from("Uffizi Gallery"),
            correct: 'a',
            number: 9,
        },
        10 => Question {
            question: String::from("In Roman mythology, who are the twin founders of Rome?"),
            a: String::from("Castor and Pollux"),
            b: String::from("Romulus and Remus"),
            correct: 'b',
            number: 10,
        },
        _ => Question {
            question: String::from(""),
            a: String::from(""),
            b: String::from(""),
            correct: ' ',
            number: 0,
        },
    }
}

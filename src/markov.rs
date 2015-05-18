// markov - does markov stuff like creating a markov chain and
// generating sentences.

extern crate rand;

use self::rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io::BufRead;


static START : &'static str = "START";

static END : &'static str = "END";

//Storage for Markov states
pub struct MarkovChain {   
    states : HashMap<Vec<String>, HashMap<String, u32>>,
    degree : u32,
}

impl MarkovChain {

    pub fn new(n : u32) -> MarkovChain {
        MarkovChain {
            states: HashMap::new(),
            degree: n,
        }
    }

    // update_states: given a key and a value to transition to, adds
    // said transition to the set of states.
    // NOTE: This assumes key is len (degree)
    pub fn update_states(&mut self, key : Vec<String>, val : &str) {
        match self.states.entry(key) {
            Entry::Occupied(mut state) => {
                match state.get_mut().entry(val.to_string()) {
                    Entry::Occupied(mut transition) => {
                        *transition.get_mut() += 1;
                    },
                    Entry::Vacant(transition) => {
                        transition.insert(1);
                    },
                }
            }
            Entry::Vacant(entry) => {
                let mut transitions = HashMap::new();
                transitions.insert(val.to_string(), 1);
                entry.insert(transitions);
            }
        }
    }

    // choose_transition: chooses a transition from the weighted set
    // of transitions for this key.
    // NOTE: assumes key is len (degree)
    pub fn choose_transition(&self, key : &Vec<String>) -> String{
        let transitions = self.states.get(key).unwrap();
        let mut weights = vec![];
        for (key, val) in transitions.iter() {
            weights.push(Weighted { item: key, weight: *val as u32 });
        }
        let wc = WeightedChoice::new(&mut weights);
        let mut rng = rand::thread_rng();
        wc.ind_sample(&mut rng).to_string()
    }

    // create_sentence
    pub fn create_sentence(&self) -> Vec<String> {
        let mut sentence  = vec![];

        let mut key = self.start_state();

        let mut current_word;

        // NOTE: Loop assumes chain is well formatted with end states
        loop {
            current_word = self.choose_transition(&key);
            if current_word == END {
                break;
            }
            sentence.push(current_word.clone());
            key.remove(0);
            key.push(current_word);
        }

        sentence
    }

    // process_line: Processes a line of input from training data into
    // a more usable form.
    pub fn process_training_line(s : String) -> String {
        let mut out = String::with_capacity(s.len());
        let charz = s.chars().map(|c| c.to_lowercase().next().unwrap());

        for c in charz {
            if c.is_alphabetic() {
                out.push(c);
            } else {
                out.push(' ');
                out.push(c);
                out.push(' ');
            }
        }
        out
    }

    // add_training_data: given a buffer to read from, this function
    // turns all the text in that buffer into additional
    // training data.
    pub fn add_training_data<R : BufRead> (&mut self, reader : &mut R) {
        let mut key = self.start_state();

        for line in reader.lines().map(|l| l.unwrap()) {
            if line.trim().is_empty() {
                self.update_states(key, END);
                key = self.start_state();
                continue;
            }
            let processed_line = MarkovChain::process_training_line(line);

            let words = processed_line.split(' ');
            for word in words {                
                self.update_states(key.clone(), &word);
                key.remove(0);
                key.push(word.to_string());

                if word == "." || word == "?" || word == "!" {
                    self.update_states(key, END);
                    key = self.start_state();
                }
            }
        }

        //Add an END to the final state if we didn't end on a terminal
        if key != self.start_state() {
            self.update_states(key, END);
        }

    }

    // start_state: creates the start state for this chain
    pub fn start_state(&self) -> Vec<String> {
        let mut key = vec![];
        for _ in 0..self.degree {
            key.push(START.to_string());
        }
        key
    }
        
}


#[cfg(test)]
mod tests {
    use super::MarkovChain;
    use super::{START, END};
    use std::io::Cursor;


    #[test]
    fn test_markov_chain() {
        let mut ex_mc = MarkovChain::new(2);
        let e_string = START.to_string();
        ex_mc.update_states(vec![e_string.clone(), e_string.clone()], "Once");
        assert_eq!(ex_mc.choose_transition(&vec![e_string.clone(), e_string.clone()]), "Once");
    }

    #[test]
    fn test_create_sentence() {
        let mut markov = MarkovChain::new(2);
        markov.update_states(string_vec(vec![START, START]), "the");
        markov.update_states(string_vec(vec![START, "the"]), "cat");
        markov.update_states(string_vec(vec!["the", "cat"]), "is");
        markov.update_states(string_vec(vec!["cat", "is"]), "nice");
        markov.update_states(string_vec(vec!["is", "nice"]), ".");
        markov.update_states(string_vec(vec!["nice", "."]), END);
        let sentence = markov.create_sentence();
        assert_eq!(sentence, string_vec(vec!["the", "cat", "is", "nice", "."]));
    }

    #[test]
    fn test_add_training_data() {
        let mut markov = MarkovChain::new(2);
        markov.add_training_data(&mut string_to_buffer("The cat is nice."));
        let sentence = markov.create_sentence();
        assert_eq!(sentence, string_vec(vec!["the", "cat", "is", "nice", "."]));
    }

    #[test]
    fn test_add_training_data_harder_sentence() {
        let mut markov = MarkovChain::new(1);
        markov.add_training_data(&mut string_to_buffer("The cat is nice to the dog."));
        let sentence = markov.create_sentence();
        assert_eq!(sentence, string_vec(vec!["the", "cat", "is", "nice", "to", "the", "dog", "."]));
    }

    fn string_vec(input : Vec<&str>) -> Vec<String>{
        let mut output = vec![];
        for i in input.iter() {
            output.push(i.to_string());
        }
        output
    }

    fn string_to_buffer(s : &str) -> Cursor<Vec<u8>> {
         Cursor::new(s.to_string().into_bytes())
    }

}

use std::collections::HashMap;
use std::hash::Hash;

pub type DataType = String;
pub type Number = f64;
pub type ProbabilityMap<T> = HashMap<T, Number>;

pub trait Tokenize {
    fn tokenize(&self) -> Vec<DataType>;
}

pub struct Counts<T: Hash + Eq + Copy> {
    total: f64,
    per_class: HashMap<T, Number>,
}

impl<T: Hash + Eq + Copy> Counts<T> {
    pub fn new() -> Counts<T> {
        Counts {
            total: 0.0,
            per_class: HashMap::new(),
        }
    }
}

pub struct NaiveBayesClassifier<T: Hash + Eq + Copy> {
    pub classes: Vec<T>,
    class_counts: Counts<T>,
    datum_counts: HashMap<DataType, Counts<T>>,
}

impl <T: Hash + Eq + Copy>  NaiveBayesClassifier<T> {
    pub fn new() -> NaiveBayesClassifier<T> {
        NaiveBayesClassifier {
            classes: Vec::new(),
            class_counts: Counts::new(),
            datum_counts: HashMap::new(),
        }
    }

    pub fn train<U: Tokenize>(
        &mut self,
        class: T,
        data: U
    ) {
        let iterator: Number = 1.0;

        for datum in data.tokenize() {
            let recorded_word_counts = self.datum_counts
                .entry(datum)
                .or_insert(
                    Counts::new()
                );

            recorded_word_counts.total += iterator;
            recorded_word_counts.per_class
                .entry(class)
                .and_modify(|entry| *entry += iterator)
                .or_insert(iterator);

            self.class_counts.total += iterator;
            self.class_counts.per_class
                .entry(class)
                .and_modify(|entry| *entry += iterator)
                .or_insert(iterator);

            if !self.classes.contains(&class) {
                self.classes.push(class);
            }
        }
    }

    pub fn analyze<U: Tokenize>(&self, data: U) -> ProbabilityMap<T> {
        let mut probability_map: ProbabilityMap<T> = ProbabilityMap::new();

        for datum in data.tokenize() {
            if let Some(datum_counts) = self.datum_counts.get(&datum) {
                let total_datum_count = datum_counts.total;

                for class in self.classes.iter() {
                    let mut new_probability: Number = 0.01;

                    if let Some(datum_count_in_class) = datum_counts.per_class.get(&class) {
                        let mut probability_of_datum_given_class: Number = *datum_count_in_class / total_datum_count;

                        if probability_of_datum_given_class == 1.0 {
                            probability_of_datum_given_class = 0.99;
                        }

                        new_probability = match probability_map.get(&class) {
                            Some(existing_probability) => existing_probability * probability_of_datum_given_class,
                            None => probability_of_datum_given_class,
                        };

                        // TODO: Compensate for rarer datum here.
                    }

                    probability_map.insert(*class, new_probability);
                }
            }
        }

        for class in self.classes.iter() {
            if let Some(&existing_probability) = probability_map.get(&class) {
                if let Some(class_count) = self.class_counts.per_class.get(&class) {
                    let probability_of_class: Number = *class_count / self.class_counts.total;

                    probability_map.insert(*class, existing_probability * probability_of_class);
                }
            }
        }

        probability_map
    }
}

use enum_iterator::*;

pub struct Allergies {
    score: u32,
}

#[repr(u32)]
#[derive(Clone, Debug, PartialEq, Eq, Sequence)]
pub enum Allergen {
    Eggs = 0b00000001,
    Peanuts = 0b00000010,
    Shellfish = 0b00000100,
    Strawberries = 0b00001000,
    Tomatoes = 0b00010000,
    Chocolate = 0b00100000,
    Pollen = 0b01000000,
    Cats = 0b10000000,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let mask = allergen.clone() as u32;
        self.score & mask == mask
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        all::<Allergen>().into_iter().for_each(|allergen| {
            if self.is_allergic_to(&allergen) {
                allergies.push(allergen);
            }
        });
        allergies
    }
}

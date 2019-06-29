use super::Settlement;
use fxhash::FxHashSet;
use rand::Rng;
use vek::*;

#[derive(Clone, Debug)]
pub struct Location {
    pub(crate) name: String,
    pub(crate) center: Vec2<i32>,
    pub(crate) kingdom: Option<Kingdom>,
    pub(crate) neighbours: FxHashSet<usize>,
    pub(crate) settlement: Settlement,
}

impl Location {
    pub fn generate(center: Vec2<i32>, rng: &mut impl Rng) -> Self {
        Self {
            name: generate_name(rng),
            center,
            kingdom: None,
            neighbours: FxHashSet::default(),
            settlement: Settlement::generate(rng),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kingdom(&self) -> Option<&Kingdom> {
        self.kingdom.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct Kingdom {
    region_name: String,
}

fn generate_name<R: Rng>(rng: &mut R) -> String {
    let firstsyl = [
        "Eri", "Val", "Gla", "Wilde", "Cold", "Deep", "Dura", "Ester", "Fay", "Dark", "West",
        "East", "North", "South", "Ray", "Eri", "Dal", "Som", "Sommer", "Black", "Iron", "Grey",
        "Hel", "Gal", "Mor", "Lo", "Nil", "Bel", "Lor", "Gold", "Red", "Marble", "Mana", "Gar",
        "Mountain", "Red", "Cheo", "Far", "High",
    ];
    let mid = ["ka", "se", "au", "da", "di"];
    let tails = [
        /*"mill",*/ "ben", "sel", "dori", "theas", "dar", "bur", "to", "vis", "ten", "stone",
        "tiva", "id", "and", "or", "el", "ond", "ia", "eld", "ald", "aft", "ift", "ity", "well",
        "oll", "ill", "all", "wyn", "light", " Hill", "lin", "mont", "mor", "cliff", "rok", "den",
        "mi", "rock", "glenn", "rovi", "lea", "gate", "view", "ley", "wood", "ovia", "cliff",
        "marsh", "kor", "ice", /*"river",*/ "acre", "venn", "crest", "field", "vale",
        "spring", " Vale", "grasp", "fel", "fall", "grove", "wyn", "edge",
    ];

    let mut name = String::new();
    if rng.gen() {
        name += rng.choose(&firstsyl).unwrap();
        name += rng.choose(&mid).unwrap();
        name += rng.choose(&tails).unwrap();
        name
    } else {
        name += rng.choose(&firstsyl).unwrap();
        name += rng.choose(&tails).unwrap();
        name
    }
}
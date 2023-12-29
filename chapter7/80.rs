struct SeaCreature {
    pub name: String,
    noise: String,
}

// impl SeaCreature {
//     pub fn get_sound(&self) -> &str {
//         &self.noise
//     }
// }

trait NoiseMaker {
    fn make_noise(&self);
}

trait LoudNoiseMaker: NoiseMaker {
    fn make_alot_of_noise(&self) {
        self.make_noise();
        self.make_noise();
        self.make_noise();
    }
}

impl NoiseMaker for SeaCreature {
    fn make_noise(&self) {
        // println!("{}", &self.get_sound());
        println!("{}", self.noise);
    }
}

impl LoudNoiseMaker for SeaCreature {}

fn main() {
    let creature = SeaCreature {
        name: String::from("Ferris"),
        noise: String::from("blub"),
    };
    println!("{}", creature.name);
    creature.make_alot_of_noise();
}

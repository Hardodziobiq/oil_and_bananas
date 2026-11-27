/// USES & IMPORTS
use crate::{
    domain::society::household::HouseholdId,
    utils::society::{education::Education, sex::Sex, state::State, wealth::Wealth},
};

/// PERSON CONSTRAINTS
const HEALTH_MIN: u8 = 0;
const HEALTH_MAX: u8 = 100;

/// PERSON ID STRUCT
#[derive(Clone, Copy)]
pub struct PersonId(pub u64);

/// PERSON STRUCT
pub struct Person {
    /// IDENTITY
    first_name: String,
    last_name: String,
    age: u8,
    sex: Sex,

    /// CONDITION
    health: u8,
    wealth: Wealth,
    state: State,
    education: Education,

    /// FAMILY
    parents: [Option<PersonId>; 2],
    household_id: HouseholdId,
}

/// PERSON MAIN CONSTRUCTOR
impl Person {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        first_name: String,
        last_name: String,
        age: u8,
        sex: Sex,
        health: u8,
        wealth: Wealth,
        state: State,
        education: Education,
        parents: [Option<PersonId>; 2],
        household_id: HouseholdId,
    ) -> Self {
        Self {
            first_name,
            last_name,
            age,
            sex,
            health,
            wealth,
            state,
            education,
            parents,
            household_id,
        }
    }
}

/// PERSON MAIN FUNCTIONS
impl Person {
    /// UPDATING FUNCTIONS
    pub fn update_age(&mut self) {
        self.age += 1;
    }
    pub fn update_health(&mut self, new_health: u8) {
        self.health = new_health.clamp(HEALTH_MIN, HEALTH_MAX);
    }
    pub fn update_wealth(&mut self, new_wealth: Wealth) {
        self.wealth = new_wealth;
    }
    pub fn update_state(&mut self, new_state: State) {
        self.state = new_state;
    }
    pub fn update_education(&mut self, new_education: Education) {
        self.education = new_education;
    }
    pub fn update_household_id(&mut self, new_household_id: HouseholdId) {
        self.household_id = new_household_id;
    }

    /// GETTING FUNCTIONS
    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }
    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }
    pub fn get_age(&self) -> u8 {
        self.age
    }
    pub fn get_health(&self) -> u8 {
        self.health
    }
    pub fn get_wealth(&self) -> Wealth {
        self.wealth
    }
    pub fn get_sex(&self) -> Sex {
        self.sex
    }
    pub fn get_state(&self) -> State {
        self.state
    }
    pub fn get_education(&self) -> Education {
        self.education
    }
    pub fn get_parents(&self) -> &[Option<PersonId>; 2] {
        &self.parents
    }
    pub fn get_household_id(&self) -> HouseholdId {
        self.household_id
    }
}

// ---------- ---------- ---------- ---------- ---------- ---------- ---------- ---------- ---------- ---------- //

// /// USES & IMPORTS
// use rand::{RngExt, rngs::ThreadRng, seq::IndexedRandom};
// /// MALE FIRST NAMES ARRAY
// pub const MALE_FIRST_NAMES: &[&str] = &[
//     "Alejandro",
//     "Alessandro",
//     "Anders",
//     "Antonio",
//     "Arturo",
//     "Benedikt",
//     "Boris",
//     "Bruno",
//     "Carlos",
//     "Cesar",
//     "Conrad",
//     "Dario",
//     "Desmond",
//     "Diego",
//     "Dimitri",
//     "Domingo",
//     "Eduardo",
//     "Emilio",
//     "Enzo",
//     "Erik",
//     "Esteban",
//     "Fabian",
//     "Felipe",
//     "Fidel",
//     "Gabriel",
//     "Giorgio",
//     "Gonzalo",
//     "Gustavo",
//     "Hector",
//     "Hugo",
//     "Ignacio",
//     "Ivan",
//     "Jacques",
//     "Javier",
//     "Jorge",
//     "Karl",
//     "Klaus",
//     "Lars",
//     "Leonardo",
//     "Lorenzo",
//     "Luis",
//     "Magnus",
//     "Manuel",
//     "Marco",
//     "Mateo",
//     "Mikhail",
//     "Nico",
//     "Nikolai",
//     "Oscar",
//     "Pablo",
//     "Paolo",
//     "Pedro",
//     "Pierre",
//     "Rafael",
//     "Ramon",
//     "Raul",
//     "Ricardo",
//     "Ruben",
//     "Salvador",
//     "Sandro",
//     "Santiago",
//     "Sebastian",
//     "Sergio",
//     "Stefan",
//     "Tomas",
//     "Ulrich",
//     "Victor",
//     "Vincent",
//     "Walter",
//     "Yuri",
// ];

// /// FEMALE FIRST NAMES ARRAY
// pub const FEMALE_FIRST_NAMES: &[&str] = &[
//     "Adriana",
//     "Agnes",
//     "Alicia",
//     "Alma",
//     "Amelia",
//     "Anastasia",
//     "Astrid",
//     "Beatriz",
//     "Bianca",
//     "Camila",
//     "Carla",
//     "Carmen",
//     "Chiara",
//     "Claudia",
//     "Cristina",
//     "Dagmar",
//     "Dalia",
//     "Daniela",
//     "Dolores",
//     "Elena",
//     "Elsa",
//     "Esperanza",
//     "Eva",
//     "Fernanda",
//     "Florence",
//     "Francesca",
//     "Freya",
//     "Gabriela",
//     "Gloria",
//     "Greta",
//     "Hanna",
//     "Helga",
//     "Ilse",
//     "Ines",
//     "Ingrid",
//     "Irina",
//     "Isabel",
//     "Josefina",
//     "Juanita",
//     "Julia",
//     "Katarina",
//     "Laura",
//     "Leticia",
//     "Liesel",
//     "Lorena",
//     "Lucia",
//     "Margarita",
//     "Maria",
//     "Marta",
//     "Mercedes",
//     "Milena",
//     "Monique",
//     "Natalia",
//     "Nina",
//     "Noemi",
//     "Olga",
//     "Paloma",
//     "Pilar",
//     "Renata",
//     "Rosa",
//     "Simone",
//     "Sofia",
//     "Svetlana",
//     "Tatiana",
//     "Teresa",
//     "Ursula",
//     "Valentina",
//     "Vera",
//     "Ximena",
//     "Yolanda",
//     "Zofia",
// ];

// /// LAST NAMES ARRAY
// pub const LAST_NAMES: &[&str] = &[
//     "Abramov",
//     "Acosta",
//     "Aguilar",
//     "Albrecht",
//     "Alvarez",
//     "Andersen",
//     "Aranda",
//     "Arriaga",
//     "Baptiste",
//     "Barros",
//     "Becker",
//     "Bellini",
//     "Beltran",
//     "Bianchi",
//     "Blanco",
//     "Bogdanov",
//     "Brandt",
//     "Bustamante",
//     "Caballero",
//     "Cabrera",
//     "Calderon",
//     "Campos",
//     "Castillo",
//     "Cordero",
//     "Costa",
//     "Cruz",
//     "Dahl",
//     "De Luca",
//     "Delgado",
//     "Dimitrov",
//     "Dominguez",
//     "Duarte",
//     "Dubois",
//     "Echeverria",
//     "Eriksson",
//     "Espinoza",
//     "Esposito",
//     "Falcone",
//     "Ferrari",
//     "Figueroa",
//     "Fischer",
//     "Fontaine",
//     "Fuentes",
//     "Galindo",
//     "Garcia",
//     "Gomez",
//     "Grassi",
//     "Guerrero",
//     "Gutierrez",
//     "Haas",
//     "Hansen",
//     "Herrera",
//     "Hoffmann",
//     "Iglesias",
//     "Iturbe",
//     "Ivanov",
//     "Jansen",
//     "Jimenez",
//     "Kaminski",
//     "Keller",
//     "Kowalski",
//     "Kraus",
//     "Lambert",
//     "Larsen",
//     "Lindqvist",
//     "Lombardi",
//     "Lopez",
//     "Lund",
//     "Machado",
//     "Mancini",
//     "Marchetti",
//     "Medina",
//     "Mendoza",
//     "Molina",
//     "Morales",
//     "Moreau",
//     "Nascimento",
//     "Navarro",
//     "Nielsen",
//     "Novak",
//     "Orlov",
//     "Ortega",
//     "Ostrowski",
//     "Pacheco",
//     "Paredes",
//     "Pereira",
//     "Petrov",
//     "Quintero",
//     "Ramirez",
//     "Reyes",
//     "Ricci",
//     "Rios",
//     "Rivera",
//     "Romano",
//     "Rossi",
//     "Ruiz",
//     "Salazar",
//     "Sandoval",
//     "Santos",
//     "Schneider",
//     "Silva",
//     "Sokolov",
//     "Soto",
//     "Tavares",
//     "Torres",
//     "Uribe",
//     "Valdez",
//     "Varga",
//     "Vargas",
//     "Vega",
//     "Volkov",
//     "Weber",
//     "Wolff",
//     "Yanez",
//     "Zamora",
//     "Zapata",
//     "Zielinski",
// ];

// /// DEFAULT NAMES
// const MALE_FIRST_NAME_DEFAULT: &str = "John";
// const FEMALE_FIRST_NAME_DEFAULT: &str = "Jane";
// const LAST_NAME_DEFAULT: &str = "Doe";

// /// IMMIGRANT AGE CLAMP
// const ARRIVED_AGE_MIN: u8 = 20;
// const ARRIVED_AGE_MAX: u8 = 30;

// /// NAMES ALLOCATION ENGINE
// pub struct PersonDataAllocationEngine;

// /// NAMES ALLOCATION ENGINE MAIN FUNCTION
// impl PersonDataAllocationEngine {
//     pub fn new_random(rng: &mut ThreadRng) -> (String, String, u8, Sex) {
//         let sex: Sex = Self::random_sex(rng);
//         let first_name: String = match sex {
//             Sex::Male => Self::random_male_first_name(rng),
//             Sex::Female => Self::random_female_first_name(rng),
//         };
//         let last_name: String = Self::random_last_name(rng);
//         let age: u8 = Self::random_age(rng);
//         (first_name, last_name, age, sex)
//     }
//     pub fn new_born(rng: &mut ThreadRng) -> (String, Sex) {
//         let sex: Sex = Self::random_sex(rng);
//         let first_name: String = match sex {
//             Sex::Male => Self::random_male_first_name(rng),
//             Sex::Female => Self::random_female_first_name(rng),
//         };
//         (first_name, sex)
//     }
// }

// /// NAMES ALLOCATION ENGINE UTILITY FUNCTIONS
// impl PersonDataAllocationEngine {
//     /// RANDOM UTILITY FUNCTIONS
//     fn random_sex(rng: &mut ThreadRng) -> Sex {
//         if rng.random_bool(0.5) {
//             Sex::Male
//         } else {
//             Sex::Female
//         }
//     }
//     fn random_male_first_name(rng: &mut ThreadRng) -> String {
//         MALE_FIRST_NAMES
//             .choose(rng)
//             .copied()
//             .unwrap_or(MALE_FIRST_NAME_DEFAULT)
//             .to_string()
//     }
//     fn random_female_first_name(rng: &mut ThreadRng) -> String {
//         FEMALE_FIRST_NAMES
//             .choose(rng)
//             .copied()
//             .unwrap_or(FEMALE_FIRST_NAME_DEFAULT)
//             .to_string()
//     }
//     fn random_last_name(rng: &mut ThreadRng) -> String {
//         LAST_NAMES
//             .choose(rng)
//             .copied()
//             .unwrap_or(LAST_NAME_DEFAULT)
//             .to_string()
//     }
//     fn random_age(rng: &mut ThreadRng) -> u8 {
//         rng.random_range(ARRIVED_AGE_MIN..=ARRIVED_AGE_MAX)
//     }
// }

/// USES & IMPORTS
use crate::{domain::society::person::PersonId, utils::society::wealth::Wealth};

/// HOUSEHOLD ID STRUCT
#[derive(Clone, Copy)]
pub struct HouseholdId(pub u64);

/// HOUSEHOLD STRUCT
pub struct Household {
    adults: Vec<PersonId>,
    children: Vec<PersonId>,
    average_wealth: Wealth,
}

/// HOUSEHOLD MAIN CONSTRUCTOR
impl Household {
    pub fn new(adults: Vec<PersonId>, children: Vec<PersonId>, average_wealth: Wealth) -> Self {
        Self {
            adults,
            children,
            average_wealth,
        }
    }
}

/// MAIN HOUSEHOLD FUNCTIONS
impl Household {
    /// UPDATING FUNCTIONS
    pub fn update_adults(&mut self, new_adults: Vec<PersonId>) {
        self.adults = new_adults;
    }
    pub fn update_children(&mut self, new_children: Vec<PersonId>) {
        self.children = new_children;
    }
    pub fn update_average_health(&mut self, new_average_wealth: Wealth) {
        self.average_wealth = new_average_wealth;
    }

    /// GETTING FUNCTIONS
    pub fn get_adults(&self) -> &[PersonId] {
        &self.adults
    }
    pub fn get_children(&self) -> &[PersonId] {
        &self.children
    }
    pub fn get_average_wealth(&self) -> Wealth {
        self.average_wealth
    }
}

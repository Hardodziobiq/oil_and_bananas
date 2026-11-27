/// USES & IMPORTS
use crate::domain::society::{household::{Household, HouseholdId}, person::{Person, PersonId}};
use std::collections::BTreeMap;

/// SOCIETY SYSTEM STRUCT
pub struct SocietySystem {
    people: BTreeMap<PersonId, Person>,
    households: BTreeMap<HouseholdId, Household>,
}


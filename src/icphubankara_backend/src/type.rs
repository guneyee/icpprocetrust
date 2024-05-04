use candid::{CandidType, Deserialize, Principal};


#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]

pub struct TodoStorage {
    pub description: String,
    pub done: bool
}

#[derive(Clone, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]

pub struct Todo {
    pub description: String,
    pub done: bool
}
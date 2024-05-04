// types.rs

// Donation Place data structure
#[derive(Debug)]
pub struct DonationPlace {
    pub id: usize,
    pub name: String,
    pub location: String,
}

// User data structure
#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub donated_amount: usize,
}

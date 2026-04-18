#[derive(Debug)] 
pub struct Pubkey(pub String); 

#[derive(Debug)]
pub struct UserAccount {
    pub owner: Pubkey,
    pub balance: u64,
}
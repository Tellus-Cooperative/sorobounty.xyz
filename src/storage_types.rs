use soroban_sdk::{ contracttype, Address, BytesN };


pub(crate) const FEE_DECIMALS: u32 = 4;     // fee is described with the unit of 0.01%
pub(crate) const DEF_FEE_RATE: u32 = 30;    // default fee is 0.3%
pub(crate) const TOKEN_DECIMALS: u32 = 4;

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days


#[derive(Clone)]
#[contracttype]
pub enum ERROR {
    SUCCESS = 0,

    GET_ERROR_FAILED = 100,

    // Fee
    FEE_NOT_SET = 110,
    
    // Participance
    ALREADY_PARTICIPATED = 120,
    
    // Work
    WORK_NOT_FOUND = 130,
    
    // Bounty
    BOUNTY_NOT_FOUND = 140,
    INVALID_BOUNTY_STATUS = 141,
    INVALID_NAME = 142,
    INVALID_REWARD = 143,
    INVALID_DEADLINE = 144,
    INSUFF_CREATOR_BALANCE = 145,
    INSUFF_CREATOR_ALLOWANCE = 146,
    CREATOR_BOUNTY_MISMATCH = 147
}

pub struct FeeInfo {                                                                                                                                                                                                                                                                                                                                                                              
    pub fee_rate: u32,
    pub fee_wallet: Address,
}

#[derive(Clone, Copy, PartialEq)]
#[contracttype]
pub enum BountyType {
    NONE = 0,
    COMPETITIVE = 1,
    COOPERATIVE = 2,
    HACKATHON = 3,
}

pub enum BountyDifficulty {
    NONE = 0, 
    BEGINNER = 1, 
    INTERMEDIATE = 2, 
    ADVANCED = 3
}

pub enum BountyStatus {
    INIT = 0,
    CREATED = 1,
    FUNDED = 2,
    APPLIED = 3,
    SUBMITTED = 4,
    APPROVED = 5,
    REJECTED = 6,
    CANCELLED = 7,
    CLOSED = 8
}

pub enum WorkStatus {
    INIT = 0,
    APPROVED = 1,
    REJECTED = 2
}


#[derive(Clone)]
#[contracttype]
pub struct BountyInfo {
    pub creator: Address;
    
    pub name: String;
    // pub description: String;
    // pub repo_link: String;
    pub reward_amount: u64;
    pub end_date: u32;
    // pub bounty_type: BountyType;
    // pub difficulty: BountyDifficulty;

    pub status: BountyStatus;
}

#[derive(Clone)]
#[contracttype]
pub struct WorkInfo {
    pub participant: Address;
    pub bounty_id: u32;
    pub repo_link: String; // Worked url
}


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    ERROR_CODE,
    FEE,
    Participance(Address, u32),
    BOUNTY_COUNT,
    RegBounties(u32),
    WORK_COUNT,
    RegWorks(u32)
}

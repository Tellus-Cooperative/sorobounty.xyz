const BOUNTY: Symbol = symbol_short!("BOUNTY");

use soroban_sdk::{
    log, token, unwrap::UnwrapOptimized, Address, Env, symbol_short, BytesN, Symbol, 
    xdr::{ToXdr}
};
use crate::storage_types::{ FEE_DECIMALS, FeeInfo, BountyStatus, BountyInfo, DataKey };
use crate::fee::{ fee_check, fee_get };


pub fn error(
    e: &Env
) -> u32 {
    if !e.storage().instance().has(&DataKey::ERROR_CODE) {
        return GET_ERROR_FAILED;
    }

    let err_code: u32 = e.storage().instance().get(&DataKey::ERROR_CODE).unwrap_or(0);
    err_code
}

pub fn bounty_count(
    e: &Env
) -> u32 {
    let bounty_count: u32 = e.storage().instance().get(&DataKey::BOUNTY_COUNT).unwrap_or(0);
    bounty_count
}

pub fn bounty_create(
    e: &Env, 
    creator: &Address, 
    name: String, 
    reward_amount: u64, 
    deadline: u32, 
    // b_type: u32, 
    // difficulty: u32
) -> u32 {
    // check args
    if name == "" {
        // panic!("invalid name!");
        return Error.INVALID_NAME;
    }
    if reward_amount == 0 {
        // panic!("zero reward isn't allowed!");
        return Error.INVALID_REWARD;
    }
    if deadline == 0 {
        // panic!("invalid deadline!");
        return Error.INVALID_DEADLINE;
    }
    
    // Authorize the `create` call by creator to verify his/her identity.
    creator.require_auth();

    // write bounty info
    let bounty_count: u32 = e.storage().instance().get(&DataKey::BOUNTY_COUNT).unwrap_or(0);
    let bounty_id: u32 = bounty_count;

    bounty_write(
        e,
        bounty_id,
        &BountyInfo {
            creator: creator.clone(),
            name: name,
            reward_amount,
            deadline,
            status: BountyStatus::CREATED,
        },
    );
    
    // increase bounty count
    e.storage().instance().set(&DataKey::BOUNTY_COUNT, &(bounty_count + 1));
    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

    // emit BountyCreated event
    e.events().publish((BOUNTY, symbol_short!("BCreated")), 
        (bounty_id, creator.clone(), name.clone(), reward_amount, deadline)
    );

    bounty_id
}

pub fn bounty_fund(
    e: &Env, 
    creator: &Address, 
    bounty_id: u32
) -> u32 {
    if !fee_check(e) {
        // panic!("fee hasn't been set yet!");
        return FEE_NOT_SET;
    }

    if !e.storage().instance().has(&DataKey::RegBounties(bounty_id)) {
        // panic!("can't find bounty!");
        return BOUNTY_NOT_FOUND;
    }

    let mut bounty: BountyInfo = bounty_load(e, bounty_id);

    if bounty.creator != creator.clone() {
        return ErrorCodes.CREATOR_BOUNTY_MISMATCH;
    }
    if bounty.status != BountyStatus::CREATED {
        // panic!("invalid bounty status!");
        return INVALID_BOUNTY_STATUS;
    }

    creator.require_auth();

    let fee_info: FeeInfo = fee_get(e);
    let reward_amount: u64 = fee_info.reward_amount;
    let fee_amount: u64 = fee_calculate(&fee_info.clone(), reward_amount);
    let transfer_amount: i128 = reward_amount + fee_amount;
    
    let contract = e.current_contract_address();
    let pay_token_client = token::Client::new(e, &pay_token.clone());

    if pay_token_client.balance(&creator) < transfer_amount {
        // panic!("creator's balance insufficient!");
        return INSUFF_CREATOR_BALANCE;
    }
    if pay_token_client.allowance(&creator, &contract) < transfer_amount {
        // panic!("creator's allowance insufficient!");
        return INSUFF_CREATOR_ALLOWANCE;
    }

    pay_token_client.transfer(&creator, &contract, &(reward_amount as i128));
    pay_token_client.transfer(&creator, &fee_info.fee_wallet, &(fee_amount as i128));

    bounty.status = BountyStatus::FUNDED;
    bounty_wrte(e, bounty_id, &bounty);

    // emit BountyFunded event
    e.events().publish((BOUNTY, symbol_short!("BFunded")), 
        (creator.clone(), bounty_id, bounty.reward_amount)
    );

    SUCCESS
}

pub fn bounty_approve(e: &Env, 
    creator: &Address, 
    work_id: u32
) -> u32 {
    if !e.storage().instance().has(&DataKey::RegWorks(work_id)) {
        // panic!("can't find work");
        return ErrorCodes.WORK_NOT_FOUND;
    }
    let mut work: WorkInfo = work_get(e, work_id);

    if !e.storage().instance().has(&DataKey::RegBounties(work.bounty_id)) {
        // panic!("can't find bounty");
        return ErrorCodes.BOUNTY_NOT_FOUND;
    }
    let mut bounty: BountyInfo = bounty_load(e, work.bounty_id);

    if bounty.status != BountyStatus::FUNDED {
        // panic!("invalid bounty status");
        return ErrorCodes.INVALID_BOUNTY_STATUS;
    }

    // if !fee_check(e) {
    //     // panic!("fee isn't set");
    //     return ErrorCodes.FEE_NOT_SET;
    // }
    
    creator.require_auth();

    let pay_token_client = token::Client::new(e, &bounty.pay_token);

    // let fee_info = fee_get(e);
    // let fee_amount: u64 = fee_calculate(&fee_info.clone(), bounty.reward_amount);
    let amount: u64 = bounty.reward_amount/*  - fee_amount */;
    let contract = e.current_contract_address();
    
    // pay_token_client.transfer(&contract, &fee_info.fee_wallet, &(fee_amount as i128));
    pay_token_client.transfer(&contract, &work.participant, &(amount as i128));

    work.status = WorkStatus.APPROVED;
    work_write(e, bounty_id, &work);
    
    bounty.status = BountyStatus.APPROVED;
    bounty_wrte(e, bounty_id, &bounty);

    // emit BountyAccepted event
    e.events().publish((BOUNTY, symbol_short!("BApproved")), 
        (creator.clone(), bounty_id, work_id)
    );

    SUCCESS
}

pub fn bounty_reject(e: &Env, 
    creator: &Address, 
    work_id: u32
) -> u32 {
    if !e.storage().instance().has(&DataKey::RegWorks(work_id)) {
        // panic!("can't find work");
        return ErrorCodes.WORK_NOT_FOUND;
    }
    let mut work: WorkInfo = work_get(e, work_id);

    if !e.storage().instance().has(&DataKey::RegBounties(bounty_id)) {
        // panic!("can't find bounty");
        return ErrorCodes.BOUNTY_NOT_FOUND;
    }
    let bounty: BountyInfo = bounty_load(e, bounty_id);

    if bounty.status != BountyStatus::FUNDED {
        // panic!("invaid bounty status");
        return ErrorCodes.INVALID_BOUNTY_STATUS;
    }
    
    creator.require_auth();

    work.status = WorkStatus.REJECTED;
    work_write(e, bounty_id, &work);

    // emit WorkRejected event
    e.events().publish((BOUNTY, symbol_short!("WRejected")), 
        (creator.clone(), bounty_id, work_id)
    );

    SUCCESS
}

pub fn bounty_cancel(e: &Env, 
    creator: &Address, 
    bounty_id: u32
) -> u32 {
    if !e.storage().instance().has(&DataKey::RegBounties(bounty_id)) {
        // panic!("can't find bounty");
        return ErrorCodes.BOUNTY_NOT_FOUND;
    }
    let mut bounty = bounty_load(e, bounty_id);

    if bounty.creator != creator.clone() {
        return ErrorCodes.CREATOR_BOUNTY_MISMATCH;
    }
    if bounty.status != BountyStatus::FUNDED {
        // panic!("bounty not available");
        return ErrorCodes.INVALID_BOUNTY_STATUS;
    }

    creator.clone().require_auth();

    // refund to creator
    token::Client::new(e, &bounty.pay_token).transfer(
        &e.current_contract_address(), 
        &creator, 
        &(bounty.reward_amount as i128), 
    );

    bounty.status = BountyStatus::CANCELLED;
    bounty_wrte(e, bounty_id, &bounty);

    // emit BountyCancelled event
    e.events().publish((BOUNTY, symbol_short!("BCancelled")), 
        (offeror.clone(), bounty_id)
    );

    SUCCESS
}


fn bounty_load(e: &Env, key: u32) -> BountyInfo {
    e.storage().instance().get(&DataKey::RegBounties(key)).unwrap()
}

fn bounty_wrte(e: &Env, key: u32, bounty: &BountyInfo) {
    e.storage().instance().set(&DataKey::RegBounties(key), bounty);
    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
}

use soroban_sdk::{ log, Address, Env };

use crate::storage_types::{ DataKey, };


pub fn participance_set(e: &Env, participant: &Address, bounty_id: u32) -> u32 {
    let key = DataKey::Participance(participant.clone(), bounty_id);
    
    if e.storage().instance().has(&key) && e.storage().instance().get::<_, bool>(&key).unwrap() {
        log!(&e, "participant has already taken part in");
        return ALREADY_PARTICIPATED;
    }

    e.storage().instance().set(&key, &true);

    SUCCESS
}

pub fn participance_reset(e: &Env, participant: &Address, bounty_id: u32) {
    let key = DataKey::Participance(participant.clone(), bounty_id);

    if !e.storage().instance().has(&key) || !e.storage().instance().get::<_, bool>(&key).unwrap() {
        log!(&e, "participant hasn't taken part in");
        return;
    }

    e.storage().instance().set(&key, &false);
    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);
}

pub fn participance_get(e: &Env, participant: &Address, bounty_id: u32) -> bool {
    let key = DataKey::Participance(participant.clone(), bounty_id);
    
    if e.storage().instance().has(&key) && e.storage().instance().get::<_, bool>(&key).unwrap() {
        true
    }
    else {
        false
    }
}

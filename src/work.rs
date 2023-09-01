
pub fn work_create(
    e: &Env, 
    participant: &Address, 
    bounty_id: u32, 
    work_repo: String
) -> u32 {
    // check args
    if bounty_id == 0 {
        // panic!("bounty_id is zero!");
        return Error.INVALID_BOUNTY_ID;
    }
    if !bounty_check(e, bounty_id) {
        // panic!("bounty not found!");
        return Error.BOUNTY_NOT_FOUND;
    }
    if work_repo == "" {
        // panic!("invalid repo!");
        return Error.INVALID_WORK_REPO;
    }
    
    // Authorize the `create` call by participant to verify his/her identity.
    participant.require_auth();

    // write bounty info
    let work_count: u32 = e.storage().instance().get(&DataKey::WORK_COUNT).unwrap_or(0);
    let work_id: u32 = work_count;

    work_write(
        e,
        work_id,
        &WorkInfo {
            participant: participant.clone(), 
            bounty_id, 
            work_repo: work_repo.clone(), 
            status: WorkStatus::CREATED,
        },
    );
    
    // increase bounty count
    e.storage().instance().set(&DataKey::WORK_COUNT, &(work_count + 1));
    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT);

    // emit BountyCreated event
    e.events().publish((WORK, symbol_short!("WCreate")), 
        (bounty_id, creator.clone(), name.clone(), reward_amount, deadline)
    );

    work_id
}

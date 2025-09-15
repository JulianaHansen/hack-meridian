use soroban_sdk::{Env, Symbol, Address, BytesN, Vec};

pub fn get_user_badges(env: &Env, user: &Address) -> Vec<BytesN<32>> {
    env.storage()
        .persistent()
        .get(&(Symbol::new(env, "ub"), user.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn set_user_badges(env: &Env, user: &Address, badges: &Vec<BytesN<32>>) {
    env.storage()
        .persistent()
        .set(&(Symbol::new(env, "ub"), user.clone()), badges);
}

pub fn get_event_owners(env: &Env, event_id: &BytesN<32>) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&(Symbol::new(env, "eo"), event_id.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn set_event_owners(env: &Env, event_id: &BytesN<32>, owners: &Vec<Address>) {
    env.storage()
        .persistent()
        .set(&(Symbol::new(env, "eo"), event_id.clone()), owners);
}

// eo = event_owners
// ub = user_badges
// em = event_metadata
// ev = events_list
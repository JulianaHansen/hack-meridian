use soroban_sdk::{Env, Symbol, Address, BytesN, Vec};

pub fn get_user_badges(env: &Env, user: &Address) -> Vec<BytesN<32>> {
    env.storage()
        .persistent()
        .get(&(Symbol::new(env, "user_badges"), user.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn set_user_badges(env: &Env, user: &Address, badges: &Vec<BytesN<32>>) {
    env.storage()
        .persistent()
        .set(&(Symbol::new(env, "user_badges"), user.clone()), badges);
}

pub fn get_event_owners(env: &Env, event_id: &BytesN<32>) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&(Symbol::new(env, "event_owners"), event_id.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn set_event_owners(env: &Env, event_id: &BytesN<32>, owners: &Vec<Address>) {
    env.storage()
        .persistent()
        .set(&(Symbol::new(env, "event_owners"), event_id.clone()), owners);
}

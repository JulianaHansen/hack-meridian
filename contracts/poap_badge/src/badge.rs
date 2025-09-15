use soroban_sdk::{Env, Address, BytesN, Vec};
use crate::storage;

pub fn mint_badge(env: Env, event_id: BytesN<32>, recipient: Address) {
    // lista de badges do usuário
    let mut badges = storage::get_user_badges(&env, &recipient);
    if !badges.contains(&event_id) {
        badges.push_back(event_id.clone());
        storage::set_user_badges(&env, &recipient, &badges);
    }

    // lista de owners do evento
    let mut owners = storage::get_event_owners(&env, &event_id);
    if !owners.contains(&recipient) {
        owners.push_back(recipient.clone());
        storage::set_event_owners(&env, &event_id, &owners);
    }
}

pub fn list_user_badges(env: Env, user: Address) -> Vec<BytesN<32>> {
    storage::get_user_badges(&env, &user)
}

pub fn list_event_owners(env: Env, event_id: BytesN<32>) -> Vec<Address> {
    storage::get_event_owners(&env, &event_id)
}

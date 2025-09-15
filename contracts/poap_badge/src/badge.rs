use soroban_sdk::{Env, Address, BytesN, Vec, symbol_short};

use crate::storage;

pub fn mint_badge(env: Env, event_id: BytesN<32>, recipient: Address) {
    // adicionar badge ao usuário
    let mut user_badges: Vec<BytesN<32>> =
        env.storage().persistent()
            .get(&(&symbol_short!("ub"), &recipient))
            .unwrap_or(Vec::new(&env));

    if !user_badges.contains(&event_id) {
        user_badges.push_back(event_id.clone());
        env.storage().persistent().set(
            &(&symbol_short!("ub"), &recipient),
            &user_badges,
        );
    }

    // adicionar usuário na lista de owners do evento
    let mut owners: Vec<Address> =
        env.storage().persistent()
            .get(&(&symbol_short!("eo"), &event_id))
            .unwrap_or(Vec::new(&env));

    if !owners.contains(&recipient) {
        owners.push_back(recipient.clone());
        env.storage().persistent().set(
            &(&symbol_short!("eo"), &event_id),
            &owners,
        );
    }
}

pub fn list_user_badges(env: Env, user: Address) -> Vec<BytesN<32>> {
    storage::get_user_badges(&env, &user)
}

pub fn list_event_owners(env: Env, event_id: BytesN<32>) -> Vec<Address> {
    storage::get_event_owners(&env, &event_id)
}

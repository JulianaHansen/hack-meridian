pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#![no_std]
use soroban_sdk::{contractimpl, Address, Env, BytesN, Vec, Map, Symbol};

pub struct PoapContract;

#[contractimpl]
impl PoapContract {
    // cria evento — só organizador registrado pode alterar
    pub fn create_event(env: Env, event_id: BytesN<32>, organizer: Address, metadata: Symbol) { ... }

    // mint do badge para um endereço (somente se não existir)
    pub fn mint_badge(env: Env, event_id: BytesN<32>, recipient: Address) { ... }

    // consulta se recipient tem badge do event
    pub fn has_badge(env: Env, event_id: BytesN<32>, recipient: Address) -> bool { ... }

    // lista todos event_ids que um usuário possui (ou lista owners do event)
    pub fn list_user_badges(env: Env, user: Address) -> Vec<BytesN<32>> { ... }
    pub fn list_event_owners(env: Env, event_id: BytesN<32>) -> Vec<Address> { ... }
}

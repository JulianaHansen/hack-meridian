#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env, BytesN, Vec, Symbol};

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

#[contract]
pub struct PoapBadge;

#[contractimpl]
impl PoapBadge {
    // cria evento — só organizador registrado pode alterar
    pub fn create_event(env: Env, event_id: BytesN<32>, organizer: Address, metadata: Symbol) { 
        // TODO: salvar evento no storage
        let _ = (env, event_id, organizer, metadata);
     }

    // mint do badge para um endereço (somente se não existir)
    pub fn mint_badge(env: Env, event_id: BytesN<32>, recipient: Address) { 
        // TODO: salvar badge do usuário
        let _ = (env, event_id, recipient);
     }

    // consulta se recipient tem badge do event
    pub fn has_badge(env: Env, event_id: BytesN<32>, recipient: Address) -> bool { 
        // TODO: checar storage
        let _ = (env.clone(), event_id, recipient);
        false
     }

    // lista todos event_ids que um usuário possui (ou lista owners do event)
    pub fn list_user_badges(env: Env, user: Address) -> Vec<BytesN<32>> { 
        // TODO: retornar lista de badges
        let _ = (env.clone(), user);
        Vec::new(&env)
     }
    pub fn list_event_owners(env: Env, event_id: BytesN<32>) -> Vec<Address> { 
            // TODO: retornar lista de donos do badge
            let _ = (env.clone(), event_id);
            Vec::new(&env)
     }
}
use soroban_sdk::{Env, Address, BytesN, Symbol, Vec, symbol_short};

#[derive(Clone)]
pub struct EventMetadata {
    pub name: Symbol,
    pub description: Symbol,
    pub image: Symbol, // pode ser URL/IPFS hash
}

pub fn create_event(
        env: Env,
        event_id: BytesN<32>,
        organizer: Address,
        name: Symbol,
        description: Symbol,
        image: Symbol,
    ) {
        // salvar metadados no storage
        let metadata = EventMetadata { name, description, image };
        env.storage().persistent().set(
            &(&symbol_short!("event_metadata"), &event_id),
            &metadata,
        );


        // manter lista global de eventos
        let mut events: Vec<BytesN<32>> =
            env.storage().persistent().get(&Symbol::short("events")).unwrap_or(Vec::new(&env));
        events.push_back(event_id.clone());
        env.storage().set(&Symbol::short("events"), &events);

        // associar organizador
        env.storage().set(([symbol_short!("organizer"), &event_id]), organizer);
    }

pub fn list_events(env: Env) -> Vec<BytesN<32>> {
        env.storage().get(&Symbol::short("events"))
            .unwrap_or(Vec::new(&env))
    }

pub fn get_event_metadata(env: Env, event_id: BytesN<32>) -> EventMetadata {
    env.storage().get((&Symbol::short("event_metadata"), &event_id))
        .unwrap()
}

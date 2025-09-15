use soroban_sdk::{Env, Address, BytesN, Symbol};

pub fn create_event(env: Env, event_id: BytesN<32>, organizer: Address, metadata: Symbol) {
    // TODO: salvar informações do evento
    let _ = (env, event_id, organizer, metadata);
}

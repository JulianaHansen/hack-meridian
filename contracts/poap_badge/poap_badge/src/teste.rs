#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Env as _, Env, Address};

    #[test]
    fn test_create_event() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PoapBadge);
        let client = PoapBadgeClient::new(&env, &contract_id);

        let organizer = Address::random(&env);
        client.create_event(&organizer, &"ipfs://hash123".into());

        // assert que storage contém o evento
        // (depende de como você salvará em storage)
    }

    #[test]
    fn test_mint_and_duplicate() {
        let env = Env::default();
        let contract_id = env.register_contract(None, PoapBadge);
        let client = PoapBadgeClient::new(&env, &contract_id);

        let organizer = Address::random(&env);
        let user = Address::random(&env);

        client.create_event(&organizer, &"ipfs://hash123".into());
        client.mint_badge(&1u32, &user);

        assert!(client.has_badge(&1u32, &user));

        // tentativa duplicada deve falhar (pode usar Result ou panic catch)
    }
}

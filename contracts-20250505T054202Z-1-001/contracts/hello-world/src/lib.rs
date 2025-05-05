#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Address, String, symbol_short, log};

// Data types
#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub id: u64,
    pub owner: Address,
    pub location: String,
    pub area_sqft: u32,
    pub registered_at: u64,
}

// Storage key
#[contracttype]
pub enum PropertyKey {
    Property(u64),
    Count,
}

#[contract]
pub struct RealEstateRegistry;

#[contractimpl]
impl RealEstateRegistry {
    // Register a new property
    pub fn register_property(
        env: Env,
        owner: Address,
        location: String,
        area_sqft: u32,
    ) -> u64 {
        owner.require_auth();

        let mut count: u64 = env.storage().instance().get(&PropertyKey::Count).unwrap_or(0);
        count += 1;

        let property = Property {
            id: count,
            owner: owner.clone(),
            location,
            area_sqft,
            registered_at: env.ledger().timestamp(),
        };

        env.storage().instance().set(&PropertyKey::Property(count), &property);
        env.storage().instance().set(&PropertyKey::Count, &count);

        log!(&env, "Property #{} registered to {}", count, owner);
        count
    }

    // Transfer ownership of a property
    pub fn transfer_property(env: Env, property_id: u64, new_owner: Address, current_owner: Address) -> bool {
        current_owner.require_auth();

        let mut property: Property = env
            .storage()
            .instance()
            .get(&PropertyKey::Property(property_id))
            .expect("Property not found");

        if property.owner != current_owner {
            log!(&env, "Only the current owner can transfer this property");
            return false;
        }

        property.owner = new_owner.clone();
        env.storage().instance().set(&PropertyKey::Property(property_id), &property);

        log!(&env, "Property #{} transferred to {}", property_id, new_owner);
        true
    }

    // View property details
    pub fn get_property(env: Env, property_id: u64) -> Property {
        env.storage().instance()
            .get(&PropertyKey::Property(property_id))
            .expect("Property not found")
    }
}

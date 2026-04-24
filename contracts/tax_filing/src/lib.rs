#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

#[contract]
pub struct TaxFiling;

#[contractimpl]
impl TaxFiling {
    /// Initialize the contract with the tax authority address.
    pub fn init(env: Env, tax_authority: Address) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
        env.storage().instance().set(&"tax_authority", &tax_authority);
    }

    /// Tax authority creates a tax year with a deadline (Unix timestamp).
    pub fn create_tax_year(env: Env, year: u32, deadline: u64) {
        let tax_authority: Address = env
            .storage()
            .instance()
            .get(&"tax_authority")
            .expect("Not initialized");
        tax_authority.require_auth();

        let mut years: Map<u32, u64> = env
            .storage()
            .instance()
            .get(&"years")
            .unwrap_or(Map::new(&env));

        years.set(year, deadline);
        env.storage().instance().set(&"years", &years);
    }

    /// Citizen submits a tax filing for a given year.
    pub fn submit_filing(env: Env, citizen: Address, year: u32, amount: u64, hash: String) {
        citizen.require_auth();

        let years: Map<u32, u64> = env
            .storage()
            .instance()
            .get(&"years")
            .unwrap_or(Map::new(&env));
        if !years.contains_key(year) {
            panic!("Tax year not found");
        }

        let key = (citizen.clone(), year);
        let mut filings: Map<(Address, u32), (u64, String, String)> = env
            .storage()
            .instance()
            .get(&"filings")
            .unwrap_or(Map::new(&env));

        if filings.contains_key(key.clone()) {
            panic!("Filing already submitted");
        }

        filings.set(key, (amount, hash, String::from_str(&env, "pending")));
        env.storage().instance().set(&"filings", &filings);
    }

    /// Tax authority approves a filing.
    pub fn approve_filing(env: Env, citizen: Address, year: u32) {
        let tax_authority: Address = env
            .storage()
            .instance()
            .get(&"tax_authority")
            .expect("Not initialized");
        tax_authority.require_auth();

        let key = (citizen.clone(), year);
        let mut filings: Map<(Address, u32), (u64, String, String)> = env
            .storage()
            .instance()
            .get(&"filings")
            .unwrap_or(Map::new(&env));

        let entry = filings.get(key.clone()).expect("Filing not found");
        let (amount, hash, _) = entry;
        filings.set(key, (amount, hash, String::from_str(&env, "approved")));
        env.storage().instance().set(&"filings", &filings);
    }

    /// Tax authority rejects a filing.
    pub fn reject_filing(env: Env, citizen: Address, year: u32) {
        let tax_authority: Address = env
            .storage()
            .instance()
            .get(&"tax_authority")
            .expect("Not initialized");
        tax_authority.require_auth();

        let key = (citizen.clone(), year);
        let mut filings: Map<(Address, u32), (u64, String, String)> = env
            .storage()
            .instance()
            .get(&"filings")
            .unwrap_or(Map::new(&env));

        let entry = filings.get(key.clone()).expect("Filing not found");
        let (amount, hash, _) = entry;
        filings.set(key, (amount, hash, String::from_str(&env, "rejected")));
        env.storage().instance().set(&"filings", &filings);
    }

    /// Get a filing. Returns (amount, hash, status).
    pub fn get_filing(env: Env, citizen: Address, year: u32) -> (u64, String, String) {
        let filings: Map<(Address, u32), (u64, String, String)> = env
            .storage()
            .instance()
            .get(&"filings")
            .unwrap_or(Map::new(&env));

        let key = (citizen, year);
        filings.get(key).expect("Filing not found")
    }
}

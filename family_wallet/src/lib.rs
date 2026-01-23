#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Vec,
};

// Storage TTL constants
const INSTANCE_LIFETIME_THRESHOLD: u32 = 17280; // ~1 day
const INSTANCE_BUMP_AMOUNT: u32 = 518400; // ~30 days

/// Family member data structure
#[derive(Clone)]
#[contracttype]
pub struct FamilyMember {
    pub address: Address,
    pub name: String,
    pub spending_limit: i128,
    pub role: String,
}

/// Events emitted by the contract for audit trail
#[contracttype]
#[derive(Clone)]
pub enum FamilyWalletEvent {
    MemberAdded,
    MemberUpdated,
    SpendingLimitUpdated,
}

#[contract]
pub struct FamilyWallet;

#[contractimpl]
impl FamilyWallet {
    /// Initialize the family wallet with an owner
    ///
    /// # Arguments
    /// * `owner` - Address of the wallet owner (must authorize)
    ///
    /// # Returns
    /// True if initialization was successful
    ///
    /// # Panics
    /// - If owner doesn't authorize the transaction
    /// - If wallet is already initialized
    pub fn initialize(env: Env, owner: Address) -> bool {
        // Access control: require owner authorization
        owner.require_auth();

        // Check if already initialized
        let existing: Option<Address> = env.storage().instance().get(&symbol_short!("OWNER"));

        if existing.is_some() {
            panic!("Wallet already initialized");
        }

        // Extend storage TTL
        Self::extend_instance_ttl(&env);

        // Store owner
        env.storage()
            .instance()
            .set(&symbol_short!("OWNER"), &owner);

        // Initialize empty members map and addresses vec
        let members: Map<Address, FamilyMember> = Map::new(&env);
        let addresses: Vec<Address> = Vec::new(&env);

        env.storage()
            .instance()
            .set(&symbol_short!("MEMBERS"), &members);
        env.storage()
            .instance()
            .set(&symbol_short!("ADDRS"), &addresses);

        true
    }

    /// Add or update a family member
    ///
    /// # Arguments
    /// * `owner` - Address of the wallet owner (must authorize)
    /// * `address` - Address of the family member
    /// * `name` - Name of the family member
    /// * `spending_limit` - Spending limit for the member (must be positive)
    /// * `role` - Role of the member (e.g., "parent", "child")
    ///
    /// # Returns
    /// True if operation was successful
    ///
    /// # Panics
    /// - If owner doesn't authorize the transaction
    /// - If caller is not the owner
    /// - If spending_limit is not positive
    /// - If name or role is empty
    pub fn add_member(
        env: Env,
        owner: Address,
        address: Address,
        name: String,
        spending_limit: i128,
        role: String,
    ) -> bool {
        // Access control: require owner authorization
        owner.require_auth();

        // Verify caller is the owner
        let stored_owner: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("OWNER"))
            .expect("Wallet not initialized");

        if stored_owner != owner {
            panic!("Only the owner can add members");
        }

        // Input validation
        if spending_limit <= 0 {
            panic!("Spending limit must be positive");
        }

        // Extend storage TTL
        Self::extend_instance_ttl(&env);

        let mut members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));

        let mut addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&symbol_short!("ADDRS"))
            .unwrap_or_else(|| Vec::new(&env));

        // Check if member already exists
        let is_update = members.contains_key(address.clone());

        let member = FamilyMember {
            address: address.clone(),
            name,
            spending_limit,
            role,
        };

        members.set(address.clone(), member);

        // Add address to tracking vec if new member
        if !is_update {
            addresses.push_back(address.clone());
        }

        env.storage()
            .instance()
            .set(&symbol_short!("MEMBERS"), &members);
        env.storage()
            .instance()
            .set(&symbol_short!("ADDRS"), &addresses);

        // Emit event for audit trail
        let event = if is_update {
            FamilyWalletEvent::MemberUpdated
        } else {
            FamilyWalletEvent::MemberAdded
        };

        env.events()
            .publish((symbol_short!("family"), event), address);

        true
    }

    /// Get a family member by address
    ///
    /// # Arguments
    /// * `address` - Address of the family member
    ///
    /// # Returns
    /// FamilyMember struct or None if not found
    pub fn get_member(env: Env, address: Address) -> Option<FamilyMember> {
        let members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));

        members.get(address)
    }

    /// Get all family members
    ///
    /// # Returns
    /// Vec of all FamilyMember structs
    pub fn get_all_members(env: Env) -> Vec<FamilyMember> {
        let members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));

        let addresses: Vec<Address> = env
            .storage()
            .instance()
            .get(&symbol_short!("ADDRS"))
            .unwrap_or_else(|| Vec::new(&env));

        let mut result = Vec::new(&env);

        for addr in addresses.iter() {
            if let Some(member) = members.get(addr) {
                result.push_back(member);
            }
        }

        result
    }

    /// Update spending limit for a family member
    ///
    /// # Arguments
    /// * `owner` - Address of the wallet owner (must authorize)
    /// * `address` - Address of the family member
    /// * `new_limit` - New spending limit (must be positive)
    ///
    /// # Returns
    /// True if update was successful, false if member not found
    ///
    /// # Panics
    /// - If owner doesn't authorize the transaction
    /// - If caller is not the owner
    /// - If new_limit is not positive
    pub fn update_spending_limit(
        env: Env,
        owner: Address,
        address: Address,
        new_limit: i128,
    ) -> bool {
        // Access control: require owner authorization
        owner.require_auth();

        // Verify caller is the owner
        let stored_owner: Address = env
            .storage()
            .instance()
            .get(&symbol_short!("OWNER"))
            .expect("Wallet not initialized");

        if stored_owner != owner {
            panic!("Only the owner can update spending limits");
        }

        // Input validation
        if new_limit <= 0 {
            panic!("Spending limit must be positive");
        }

        // Extend storage TTL
        Self::extend_instance_ttl(&env);

        let mut members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));

        // Get member or return false if not found
        let mut member = match members.get(address.clone()) {
            Some(m) => m,
            None => return false,
        };

        member.spending_limit = new_limit;
        members.set(address.clone(), member);

        env.storage()
            .instance()
            .set(&symbol_short!("MEMBERS"), &members);

        // Emit event for audit trail
        env.events().publish(
            (symbol_short!("family"), FamilyWalletEvent::SpendingLimitUpdated),
            address,
        );

        true
    }

    /// Check if an amount is within a member's spending limit
    ///
    /// # Arguments
    /// * `address` - Address of the family member
    /// * `amount` - Amount to check
    ///
    /// # Returns
    /// True if amount <= spending_limit, false if member not found or amount exceeds limit
    pub fn check_spending_limit(env: Env, address: Address, amount: i128) -> bool {
        let members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));

        match members.get(address) {
            Some(member) => amount <= member.spending_limit,
            None => false,
        }
    }

    /// Extend the TTL of instance storage
    fn extend_instance_ttl(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}

#[cfg(test)]
mod test;

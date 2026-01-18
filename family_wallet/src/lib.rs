#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, vec, Address, Env, Map, Symbol, Vec, String,
};

#[derive(Clone)]
#[contracttype]
pub struct FamilyMember {
    pub address: Address,
    pub name: String,
    pub spending_limit: i128, // Daily or monthly limit
    pub role: String, // "sender", "recipient", "admin"
}

#[contract]
pub struct FamilyWallet;

#[contractimpl]
impl FamilyWallet {
    /// Add a family member to the wallet
    /// 
    /// # Arguments
    /// * `address` - Stellar address of the family member
    /// * `name` - Name of the family member
    /// * `spending_limit` - Spending limit for this member
    /// * `role` - Role: "sender", "recipient", or "admin"
    /// 
    /// # Returns
    /// True if member was added successfully
    pub fn add_member(
        env: Env,
        address: Address,
        name: String,
        spending_limit: i128,
        role: String,
    ) -> bool {
        let mut members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));
        
        let member = FamilyMember {
            address: address.clone(),
            name: name.clone(),
            spending_limit,
            role: role.clone(),
        };
        
        members.set(address, member);
        env.storage().instance().set(&symbol_short!("MEMBERS"), &members);
        
        true
    }
    
    /// Get a family member by address
    /// 
    /// # Arguments
    /// * `address` - Stellar address of the family member
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
        
        let mut result = Vec::new(&env);
        // Note: In a real implementation, you'd need to track member addresses
        // For now, this is a placeholder
        result
    }
    
    /// Update spending limit for a family member
    /// 
    /// # Arguments
    /// * `address` - Stellar address of the family member
    /// * `new_limit` - New spending limit
    /// 
    /// # Returns
    /// True if update was successful
    pub fn update_spending_limit(env: Env, address: Address, new_limit: i128) -> bool {
        let mut members: Map<Address, FamilyMember> = env
            .storage()
            .instance()
            .get(&symbol_short!("MEMBERS"))
            .unwrap_or_else(|| Map::new(&env));
        
        if let Some(mut member) = members.get(address.clone()) {
            member.spending_limit = new_limit;
            members.set(address, member);
            env.storage().instance().set(&symbol_short!("MEMBERS"), &members);
            true
        } else {
            false
        }
    }
    
    /// Check if a spending amount is within limit
    /// 
    /// # Arguments
    /// * `address` - Stellar address of the family member
    /// * `amount` - Amount to check
    /// 
    /// # Returns
    /// True if amount is within limit
    pub fn check_spending_limit(env: Env, address: Address, amount: i128) -> bool {
        if let Some(member) = Self::get_member(env, address) {
            amount <= member.spending_limit
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test;


use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();

    let result = client.initialize(&owner);
    assert_eq!(result, true);
}

#[test]
#[should_panic(expected = "Wallet already initialized")]
fn test_initialize_twice_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);
    client.initialize(&owner); // Should panic
}

#[test]
fn test_add_member_creates_new() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    let result = client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    assert_eq!(result, true);

    // Verify member was added
    let member = client.get_member(&member_addr).unwrap();
    assert_eq!(member.address, member_addr);
    assert_eq!(member.name, String::from_str(&env, "Alice"));
    assert_eq!(member.spending_limit, 1000);
    assert_eq!(member.role, String::from_str(&env, "parent"));
}

#[test]
fn test_add_member_updates_existing() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    // Add member first time
    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    // Update same member
    let result = client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice Updated"),
        &2000,
        &String::from_str(&env, "admin"),
    );

    assert_eq!(result, true);

    // Verify member was updated
    let member = client.get_member(&member_addr).unwrap();
    assert_eq!(member.name, String::from_str(&env, "Alice Updated"));
    assert_eq!(member.spending_limit, 2000);
    assert_eq!(member.role, String::from_str(&env, "admin"));

    // Verify we still have only one member
    let all_members = client.get_all_members();
    assert_eq!(all_members.len(), 1);
}

#[test]
#[should_panic(expected = "Spending limit must be positive")]
fn test_add_member_zero_limit_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &0, // Zero limit should fail
        &String::from_str(&env, "parent"),
    );
}

#[test]
#[should_panic(expected = "Spending limit must be positive")]
fn test_add_member_negative_limit_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &-100, // Negative limit should fail
        &String::from_str(&env, "parent"),
    );
}

#[test]
fn test_get_member_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Bob"),
        &500,
        &String::from_str(&env, "child"),
    );

    let member = client.get_member(&member_addr);
    assert!(member.is_some());

    let member = member.unwrap();
    assert_eq!(member.address, member_addr);
    assert_eq!(member.name, String::from_str(&env, "Bob"));
    assert_eq!(member.spending_limit, 500);
    assert_eq!(member.role, String::from_str(&env, "child"));
}

#[test]
fn test_get_member_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let non_existent = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    let member = client.get_member(&non_existent);
    assert!(member.is_none());
}

#[test]
fn test_get_all_members() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);
    let member3 = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    // Add multiple members
    client.add_member(
        &owner,
        &member1,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    client.add_member(
        &owner,
        &member2,
        &String::from_str(&env, "Bob"),
        &500,
        &String::from_str(&env, "child"),
    );

    client.add_member(
        &owner,
        &member3,
        &String::from_str(&env, "Charlie"),
        &750,
        &String::from_str(&env, "child"),
    );

    let all_members = client.get_all_members();
    assert_eq!(all_members.len(), 3);

    // Verify all members are present
    let mut found_member1 = false;
    let mut found_member2 = false;
    let mut found_member3 = false;

    for member in all_members.iter() {
        if member.address == member1 {
            found_member1 = true;
        }
        if member.address == member2 {
            found_member2 = true;
        }
        if member.address == member3 {
            found_member3 = true;
        }
    }

    assert!(found_member1);
    assert!(found_member2);
    assert!(found_member3);
}

#[test]
fn test_get_all_members_empty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    let all_members = client.get_all_members();
    assert_eq!(all_members.len(), 0);
}

#[test]
fn test_update_spending_limit_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    let result = client.update_spending_limit(&owner, &member_addr, &2500);
    assert_eq!(result, true);

    // Verify limit was updated
    let member = client.get_member(&member_addr).unwrap();
    assert_eq!(member.spending_limit, 2500);
}

#[test]
fn test_update_spending_limit_member_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let non_existent = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    let result = client.update_spending_limit(&owner, &non_existent, &1000);
    assert_eq!(result, false);
}

#[test]
#[should_panic(expected = "Spending limit must be positive")]
fn test_update_spending_limit_zero_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    client.update_spending_limit(&owner, &member_addr, &0); // Should panic
}

#[test]
#[should_panic(expected = "Spending limit must be positive")]
fn test_update_spending_limit_negative_fails() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    client.update_spending_limit(&owner, &member_addr, &-500); // Should panic
}

#[test]
fn test_check_spending_limit_within_limit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    // Test amounts within limit
    assert_eq!(client.check_spending_limit(&member_addr, &500), true);
    assert_eq!(client.check_spending_limit(&member_addr, &1000), true);
    assert_eq!(client.check_spending_limit(&member_addr, &1), true);
}

#[test]
fn test_check_spending_limit_exceeds_limit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    // Test amounts exceeding limit
    assert_eq!(client.check_spending_limit(&member_addr, &1001), false);
    assert_eq!(client.check_spending_limit(&member_addr, &5000), false);
}

#[test]
fn test_check_spending_limit_member_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let non_existent = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    // Should return false for non-existent member
    assert_eq!(client.check_spending_limit(&non_existent, &100), false);
}

#[test]
fn test_large_spending_limit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    // Test with very large limit
    let large_limit = i128::MAX;
    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &large_limit,
        &String::from_str(&env, "admin"),
    );

    let member = client.get_member(&member_addr).unwrap();
    assert_eq!(member.spending_limit, large_limit);

    assert_eq!(
        client.check_spending_limit(&member_addr, &(large_limit - 1)),
        true
    );
    assert_eq!(client.check_spending_limit(&member_addr, &large_limit), true);
}

#[test]
#[should_panic(expected = "Only the owner can add members")]
fn test_non_owner_cannot_add_member() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let non_owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    // Non-owner tries to add member
    client.add_member(
        &non_owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );
}

#[test]
#[should_panic(expected = "Only the owner can update spending limits")]
fn test_non_owner_cannot_update_limit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FamilyWallet);
    let client = FamilyWalletClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let non_owner = Address::generate(&env);
    let member_addr = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&owner);

    client.add_member(
        &owner,
        &member_addr,
        &String::from_str(&env, "Alice"),
        &1000,
        &String::from_str(&env, "parent"),
    );

    // Non-owner tries to update limit
    client.update_spending_limit(&non_owner, &member_addr, &2000);
}

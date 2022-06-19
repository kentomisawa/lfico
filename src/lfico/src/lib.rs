extern crate core;

use std::borrow::{Borrow, BorrowMut};
use ic_cdk::{
    api::call::ManualReply,
    export::{
        candid::{CandidType, Deserialize},
        Principal,
    },
    storage,
};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Edge {
    to: String,
    labels: Vec<String>
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Account {
    name: String,
    verifications: Vec<String>,
    edges: Vec<Edge>
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Regulation {
    name: String,
    owner: String,
    edges: Vec<Edge>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Verification {
    name: String,
    verified: bool,
    verifier: String,
    regulation: String,
    edges: Vec<Edge>
}

type Accounts = BTreeMap<String, Account>;
type Regulations = BTreeMap<String, Regulation>;
type Verifications = BTreeMap<String, Verification>;
type Balance = BTreeMap<String, BTreeMap<String, f64>>;

thread_local! {
    static ACCOUNTS: RefCell<Accounts> = RefCell::default();
    static REGULATIONS: RefCell<Regulations> = RefCell::default();
    static VERIFICATIONS: RefCell<Verifications> = RefCell::default();
    static BALANCE: RefCell<Balance> = RefCell::default();
}

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
fn create_regulation(id: String, name: String, owner: String) {
    REGULATIONS.with(|regulations| regulations.borrow_mut().insert(
        id, Regulation {
            name,
            owner,
            edges: Vec::new()
        }
    ));
}

#[update]
fn create_account(id: String, name: String) {
    ACCOUNTS.with(|accounts| accounts.borrow_mut().insert(
        id.clone(), Account {
            name,
            verifications: Vec::new(),
            edges: Vec::new()
        }
    ));
    BALANCE.with(|balance| balance.borrow_mut().insert(
        id, BTreeMap::new()
    ));
}

#[update]
fn create_verification(
    id: String,
    name: String,
    verified: bool,
    verifier: String,
    regulation: String,
    target: String
) {
    VERIFICATIONS.with(|verifications| verifications.borrow_mut().insert(
        id, Verification {
            name,
            verified,
            verifier,
            regulation,
            edges: Vec::new()
        }
    ));
    ACCOUNTS.with(|accounts| match accounts.clone().borrow().get(&target) {
        Some(user) => {
            let mut new_user = user.clone();
            new_user.verifications.push(target.clone());
            accounts.borrow_mut().insert(target, new_user)
        },
        None => panic!("User not exist")
    });
}

#[update]
fn offer_token_to(to: String, id: String, amount: f64) {
    BALANCE.with(|balance| match balance.clone().borrow().get(&to) {
        Some(tokens) => {
            let mut new_tokens = tokens.clone();
            match new_tokens.borrow_mut().insert(id.clone(), amount) {
                Some(_) => {
                    balance.borrow_mut().insert(to, new_tokens);
                },
                None => {
                    balance.borrow_mut().insert(to, new_tokens);
                }
            };
        },
        None => panic!("No account")
    })
}

#[query(manual_reply = true)]
fn get_account(id: String) -> ManualReply<Account> {
    ACCOUNTS.with(|accounts| match accounts.borrow().get(&id) {
        Some(account) => ManualReply::one(account),
        None => panic!("No account")
    })
}

#[query(manual_reply = true)]
fn get_regulation(id: String) -> ManualReply<Regulation> {
    REGULATIONS.with(|regulations| match regulations.borrow().get(&id) {
        Some(regulation) => ManualReply::one(regulation),
        None => panic!("No account")
    })
}

#[query(manual_reply = true)]
fn get_verification(id: String) -> ManualReply<Verification> {
    VERIFICATIONS.with(|verifications| match verifications.borrow().get(&id) {
        Some(verification) => ManualReply::one(verification),
        None => panic!("No account")
    })
}

#[query(manual_reply = true)]
fn get_balance(account_id: String) -> ManualReply<BTreeMap<String, f64>> {
    BALANCE.with(|balance| match balance.borrow().get(&account_id) {
        Some(tokens) => ManualReply::one(tokens),
        None => panic!("No account")
    })
}
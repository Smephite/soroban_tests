#![no_std]

use soroban_sdk::{contracttype};

pub struct TestContract;

#[contracttype]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum MyCoolType {
    ThisTypeIsSadBecauseTheNameIsTooLong = 0,
    Type = 1
}


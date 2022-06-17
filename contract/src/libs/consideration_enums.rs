use casper_types::{ContractHash, U256, U512};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderType {
    Buy,
    Sell,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderRouteType {
    NativeToCEP47,
    ERC20ToCEP47,
    CEP47ToERC20,
    CEP47ToNative,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderStatus {
    Pending,
    Cancelled,
    Succeed,
}

pub enum Asset {
    Native {
        amount: U512,
    },
    ERC20 {
        contract_hash: ContractHash,
        amount: U256,
    },
    CEP47 {
        contract_hash: ContractHash,
        token_id: U256,
        start_amount: U256,
        end_amount: U256,
    },
}

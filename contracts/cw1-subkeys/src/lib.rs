pub mod contract;
mod cw1;
pub mod error;
#[cfg(any(test, feature = "tests"))]
pub mod multitest;
pub mod responses;
pub mod state;
mod whitelist;

#[cfg(not(any(feature = "library", tarpaulin_include)))]
mod entry_points {
    use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};

    use crate::contract::{ContractExecMsg, ContractQueryMsg, Cw1SubkeysContract, InstantiateMsg};
    use crate::error::ContractError;

    const CONTRACT: Cw1SubkeysContract = Cw1SubkeysContract::new();

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        msg.dispatch(&CONTRACT, (deps, env, info))
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ContractExecMsg,
    ) -> Result<Response, ContractError> {
        msg.dispatch(&CONTRACT, (deps, env, info))
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: ContractQueryMsg) -> Result<Binary, ContractError> {
        msg.dispatch(&CONTRACT, (deps, env))
    }
}

#[cfg(not(any(feature = "library", tarpaulin_include)))]
pub use crate::entry_points::*;

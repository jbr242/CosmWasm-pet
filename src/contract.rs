use cosmwasm_std::{
    entry_point, to_binary, Addr, BankMsg, Binary, BlockInfo, CanonicalAddr, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage, Timestamp, Uint128, Uint64
};
use secret_toolkit::permit::Permit;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryAnswer, QueryMsg, QueryWithPermit};
use crate::state::{add_to_bid, Auction, HighestBid, ADMIN, AUCTION, AUCTION_STARTED, BIDS, HIGHEST_BID, SALE_COMPLETED};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admin = msg.admin.unwrap_or(info.sender);
    let admin = deps.api.addr_canonicalize(admin.as_str())?;
    ADMIN.save(deps.storage, &admin)?;

    AUCTION_STARTED.save(deps.storage, &false)?;
    SALE_COMPLETED.save(deps.storage, &false)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetAuction { secret, minimum_bid, end_time } => try_set_auction(
            deps, 
            env, 
            info.sender, 
            secret, 
            minimum_bid, 
            end_time
        ),
        ExecuteMsg::StartAuction { } => try_start_auction(deps, info.sender),
        ExecuteMsg::Bid { } => try_bid(deps, env, info),
        ExecuteMsg::Withdraw { } => try_withdraw(deps, env, info.sender),
    }
}

// pub fn try_increment(deps: DepsMut, _env: Env) -> StdResult<Response> {
//     CONFIG.update(deps.storage, |mut state| {
//         state.count += 1;
//         Ok(state)
//     })?;

//     Ok(Response::default())
// }

// pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> StdResult<Response> {
//     let sender_address = info.sender.clone();
//     CONFIG.update(deps.storage, |mut state| {
//         if sender_address != state.owner {
//             return Err(StdError::generic_err("Only the owner can reset count"));
//         }
//         state.count = count;
//         Ok(state)
//     })?;

//     Ok(Response::default())
// }

// #[entry_point]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
//     }
// }

// fn query_count(deps: Deps) -> StdResult<CountResponse> {
//     let state = CONFIG.load(deps.storage)?;
//     Ok(CountResponse { count: state.count })
// }

pub fn try_set_auction(
    deps: DepsMut, 
    _env: Env, 
    sender: Addr, 
    secret: String, 
    minimum_bid: Uint128, 
    end_time: Uint64
) -> StdResult<Response> {
    // check if sender is admin
    let admin = deps.api.addr_humanize(&ADMIN.load(deps.storage)?)?;
    if sender != admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // make sure the auction has not started
    if AUCTION_STARTED.load(deps.storage)? {
        return Err(StdError::generic_err("Auction has already started"));
    }

    let end_time = Timestamp::from_seconds(end_time.u64());
    let minimum_bid = minimum_bid.u128();
    AUCTION.save(deps.storage, &Auction{
        secret,
        end_time,
        minimum_bid,
    })?;

    Ok(Response::default())
}

pub fn try_start_auction(deps: DepsMut, sender: Addr) -> StdResult<Response> {
    // check if sender is admin
    let admin = deps.api.addr_humanize(&ADMIN.load(deps.storage)?)?;
    if sender != admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // make sure the auction was not already started
    if AUCTION_STARTED.load(deps.storage)? {
        return Err(StdError::generic_err("Auction has already started"));
    }

    // make sure the auction is set
    if AUCTION.may_load(deps.storage)?.is_none() {
        return Err(StdError::generic_err("Auction has not been set"));
    }

    AUCTION_STARTED.save(deps.storage, &true)?;

    Ok(Response::default())
}

pub fn try_bid(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    // make sure the auction has started
    if !AUCTION_STARTED.load(deps.storage)? {
        return Err(StdError::generic_err("Auction has not started"));
    }

    // make sure auction has not finished
    if env.block.time > AUCTION.load(deps.storage)?.end_time {
        return Err(StdError::generic_err("Auction has ended"));
    }

    // make sure some funds were sent
    if info.funds.len() < 1 {
        return Err(StdError::generic_err("No funds sent"));
    }

    // make sure the funds sent were SCRT (`uscrt` stands for micro-SCRT)
    if info.funds[0].denom != "uscrt" {
        return Err(StdError::generic_err("Bid not SCRT"));
    }

    let sender_address = deps.api.addr_canonicalize(info.sender.as_str())?;

    // make sure the bidder is not the admin
    if sender_address == ADMIN.load(deps.storage)? {
        return Err(StdError::generic_err("Admin cannot bid"));
    }
    
    // add the sent funds to the bidder
    let total_bid = add_to_bid(deps.storage, &sender_address, info.funds[0].amount.u128())?;

    // check if this is the highest bid and update if so
    if let Some(highest_bid) = HIGHEST_BID.may_load(deps.storage)? {
        if total_bid > highest_bid.amount {
            HIGHEST_BID.save(deps.storage, &HighestBid {
                bidder: sender_address,
                amount: total_bid,
            })?;
        }
    } else { // no highest bid, yet
        HIGHEST_BID.save(deps.storage, &HighestBid {
            bidder: sender_address,
            amount: total_bid,
        })?;
    }
    
    Ok(Response::default())
}

fn verify_auction_finished(
    storage: &dyn Storage,
    block: &BlockInfo,
) -> StdResult<()> {
    // make sure the auction has started
    if !AUCTION_STARTED.load(storage)? {
        return Err(StdError::generic_err("Auction has not started"));
    }

    // make sure the auction has finished
    if block.time <= AUCTION.load(storage)?.end_time {
        return Err(StdError::generic_err("Auction has not finished"));
    }

    Ok(())
}

pub fn try_withdraw(deps: DepsMut, env: Env, sender: Addr) -> StdResult<Response> {
    verify_auction_finished(deps.storage, &env.block)?;

    let message;

    // withdraw completes sale for admin or withdraws failed bid for others (except winner)
    let admin = deps.api.addr_humanize(&ADMIN.load(deps.storage)?)?;
    let highest_bid = HIGHEST_BID.load(deps.storage)?;
    if sender == admin {
        if SALE_COMPLETED.load(deps.storage)? {
            return Err(StdError::generic_err("Already completed sale"));
        }
        
        let auction = AUCTION.load(deps.storage)?;
        if highest_bid.amount < auction.minimum_bid {
            return Err(StdError::generic_err("Auction ended without minimum bid reached"));
        }

        // send highest bid coins to the admin
        let coins_to_send: Vec<Coin> = vec![Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::from(highest_bid.amount),
        }];
        
        message = CosmosMsg::Bank(BankMsg::Send {
            to_address: sender.clone().into_string(),
            amount: coins_to_send,
        });

        SALE_COMPLETED.save(deps.storage, &true)?;
    } else if sender == deps.api.addr_humanize(&highest_bid.bidder)? {
        return Err(StdError::generic_err("You won the auction, cannot withdraw"));
    } else {
        let sender_canonical = deps.api.addr_canonicalize(sender.as_str())?;
        let bid = BIDS.get(deps.storage, &sender_canonical).unwrap_or_default();
        if bid == 0 {
            return Err(StdError::generic_err("Nothing to withdraw"));
        }

        let coins_to_send: Vec<Coin> = vec![Coin {
            denom: "uscrt".to_string(),
            amount: Uint128::from(bid),
        }];
        
        message = CosmosMsg::Bank(BankMsg::Send {
            to_address: sender.clone().into_string(),
            amount: coins_to_send,
        });

        BIDS.remove(deps.storage, &sender_canonical)?;
    }

    let res = Response::new().add_message(message);
    Ok(res)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AuctionInfo{ } => query_auction_info(deps),
        QueryMsg::WithPermit { permit, query } => permit_queries(deps, env, permit, query),
    }
}

pub fn permit_queries(
    deps: Deps,
    env: Env,
    permit: Permit,
    query: QueryWithPermit,
) -> StdResult<Binary> {
    let account = secret_toolkit::permit::validate(
        deps,
        "revoked_permits",
        &permit,
        env.contract.address.to_string(),
        None,
    )?;

    // permit validated, process query
    match query {
        QueryWithPermit::GetSecret { } => query_secret(deps, &env.block, &deps.api.addr_canonicalize(&account)?)
    }
}

fn query_auction_info(deps: Deps) -> StdResult<Binary> {
    let started;
    let mut minimum_bid = None;
    let mut end_time = None;
    if !AUCTION_STARTED.load(deps.storage)? {
        started = false;
    } else {
        started = true;
        let auction = AUCTION.load(deps.storage)?;
        minimum_bid = Some(Uint128::from(auction.minimum_bid));
        end_time = Some(Uint64::from(auction.end_time.seconds()));
    }
    Ok(to_binary(&QueryAnswer::AuctionInfo { started, minimum_bid, end_time })?)
}

fn query_secret(deps: Deps, block: &BlockInfo, account: &CanonicalAddr) -> StdResult<Binary> {
    verify_auction_finished(deps.storage, block)?;
    let highest_bid = HIGHEST_BID.load(deps.storage)?;

    let auction = AUCTION.load(deps.storage)?;
    if highest_bid.amount < auction.minimum_bid {
        return Err(StdError::generic_err("Unauthorized"));
    }
    
    if highest_bid.bidder != *account {
        return Err(StdError::generic_err("Only the winner of the auction can see the secret"));
    }

    Ok(
        to_binary(&QueryAnswer::GetSecret { secret: auction.secret })?
    )
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::*;
//     use cosmwasm_std::{from_binary, Coin, StdError, Uint128};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "earth".to_string(),
//                 amount: Uint128::new(1000),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }

//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // anyone can increment
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );

//         let exec_msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }

//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&[Coin {
//             denom: "token".to_string(),
//             amount: Uint128::new(2),
//         }]);
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let init_msg = InstantiateMsg { count: 17 };

//         let _res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();

//         // not anyone can reset
//         let info = mock_info(
//             "anyone",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let res = execute(deps.as_mut(), mock_env(), info, exec_msg);

//         match res {
//             Err(StdError::GenericErr { .. }) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let info = mock_info(
//             "creator",
//             &[Coin {
//                 denom: "token".to_string(),
//                 amount: Uint128::new(2),
//             }],
//         );
//         let exec_msg = ExecuteMsg::Reset { count: 5 };

//         let _res = execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use cw2::set_contract_version;
use crate::state::{Config, CONFIG, Poll, POLLS, Ballot, BALLOTS};

use crate::contract::{instantiate, execute};
use crate::msg::{InstantiateMsg, ExecuteMsg};


const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
 

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
     deps: DepsMut, //dependencies for storage, query and apis.
    _env: Env,      //environment contains contract info, i.e, addr, block, height, timestamp.
    info: MessageInfo, //metadata
    msg: InstantiateMsg, //contains instatiate message from msg.rs
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage,CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string()); //this and the following line help in setting admin address.
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: Deps, 
    env: Env, 
    info: MessageInfo,
    msg: QueryMsg
    ) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreatePoll {
            poll_id,
            question,
            options,
        } => execute_create_poll(deps, env, info, poll_id, question, options),
        ExecuteMsg::Vote { poll_id, vote } => unimplemented!(),
    }

    fn execute_create_poll (
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        poll_id: String,
        question: String,
        options: Vec<String>,
    ) -> Result<Response, ContractError> {
        if options.len() > 10 {
            return Err(ContractError::TooManyOptions {} );
        }

        let mut opts: Vec<(String, u64)> = vec![];
        for option in options {
            opts.push((option, 0));
        }

        let poll = Poll {
            creator: info.sender,
            question,
            options: opts
        };

        POLLS.save(deps.storage, poll_id, &poll)?;

        Ok(Response::new());
        unimplemented!()
    }

    fn execute_vote (
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        poll_id: String,
        vote: String,
    ) -> Result<Response, ContractError> {
        let poll = POLLS.may_load(deps.storage, poll_id.clone())?;

        match poll {
            Some(mut poll) => {
                BALLOTS.update(
                    deps.storage,
                    (info.sender, poll_id.clone()),
                    |ballot| -> StdResult<Ballot> {
                        match ballot {
                            Some(ballot) => {
                                let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                                poll.options[position_of_old_vote].1 -=1;
                                Ok(Ballot {option: vote.clone() })
                            }
                            None => {
                                Ok(Ballot { option: vote.clone() })
                            }
                        }
                    },
                )?;

                let position = poll
                    .option
                    .iter()
                    .position(|option| option.0 == vote);
                if position.is_none() {
                    return Err(ContractError::Unauthorized {});
                }    
                let position = position.unwrap();
                poll.options[position].1 +=1;

                POLLS.save(deps.storage, poll_id, &poll)?;
                Ok(Response::new())
            },
            None => Err(ContractError::Unauthorized {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::attr;  //helps in contructing an attribute. eg: action, instantiate)
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // to mock an env.
    use crate::contract::instantiate; //contract instantiate function.
    use crate::msg::InstantiateMsg; //our instantiate method.

    //mock addresses for testing.
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let env = mock_env;
        let info = mock_info(ADDR1, &vec![]);
        let msg = IntantiateMsg { admin: None};
        let res  = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)],
        )
    }

    #[test]
    fn test_execute_create_poll_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();


        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your fav chain maker?".to_string(),
            options: vec![
                "Cosmos".to_string(),
                "Substrate".to_string(),
                "Polkadot".to_string(),
                ],
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        let msg = InstantiateMsg {admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();


        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id.to_string(),
            question: What's your favourtie number?".to_string(),
            option: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],

            
        };
        let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
    }
    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None};
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        let msg =  ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "Whats your fav chain maker? ".to_string(),
            options: vec![
                "Cosmos".to_string(),
                "Substrate".to_string(),
                "Polkadot".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Cosmos".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Substrate".to_string(),
        };
        let _res = execute(deps.as_mut(), env, info, msg).unwrap();
        }


        #[test]
        fn test_execute_vote_invalid() {
            let mut deps = mock_dependencies();
            let env = mock_env();
            let info = mock_info(ADDR1, &vec![]);
    
            let msg = InstantiateMsg { admin: None };
            let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    
            let msg = ExecuteMsg::Vote {
                poll_id: "some_id".to_string(),
                vote: "Polkadot".to_string(),
            };
    
            let _err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

            // Create the poll
            let msg = ExecuteMsg::CreatePoll {
                poll_id: "some_id".to_string(),
                question: "What's your fav chain maker?".to_string(),
                options: vec![
                "Cosmos".to_string(),
                "Substrate".to_string(),
                "Polkadot".to_string(),
                ],
            };
            let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

             // Vote on a now existing poll but the option "DVPN" does not exist
            let msg = ExecuteMsg::Vote {
                poll_id: "some_id".to_string(),
                vote: "DVPN".to_string(),
                };
                let _err = execute(deps.as_mut(), env, info, msg).unwrap_err();
            }
    }


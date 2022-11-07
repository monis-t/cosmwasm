/* Let's start by importing some functions from other packages. 
*/
use schemars::JsonSchema;            //Json schema allows serialization and deserialization to and from JSON.
use serde::{Deserialize, Serialize}; //deserialize and serialize help with the above function.

use cosmwasm_std::Addr;                 //Addr stands for Address,as in contract address. It's a string.
use cw_storage_plus::{Item,Map};        //Item and Map are used for storing our structs and the multiple values within them. 
                                        //The state variable is an Item that stores a singular state struct.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] //the derive attribute helps us borrow some traits for certain specific functions as mentioned in the singature.

pub struct Config {                      //declaring the state Config.
    pub admin: Addr,                     //declaring a global poll admin as Addr.
           
}

pub struct Poll {
    pub creator: Addr,
    pub question: String,
    pub options: Vec<(String, u64)>,    //a global options list that houses our poll options and the votes each option gets.
}

pub struct Ballot {
    pub options: String,                //This struct is for storing the options a person has voted for. It's to prevent, the double voting problem, silly !!
}


pub const CONFIG: Item<Config> = Item::new("config");

//A map with a string key and poll value.
//the key will be a UUID generated clientside. *make sense of that*
pub const POLLS: Map<String, Poll> = Map::new("polls");
pub const BALLOTS: Map<(Addr, String), Ballot> = Map::new("ballots");

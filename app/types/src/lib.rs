use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account_name: String,
    pub balance: u32,
}

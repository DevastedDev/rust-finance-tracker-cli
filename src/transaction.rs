use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize,Serialize)]
pub struct Transaction{
    pub amount:f64,
    pub description: String
}
impl Transaction {
    pub fn new(amount:f64,description:String) -> Self {
        Transaction{amount,description}
    }
}
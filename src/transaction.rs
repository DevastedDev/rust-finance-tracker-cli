use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction{
    pub amount:f64,
    pub description: String
}
impl Transaction {
    pub fn new(amount:f64,description:String){
        Transaction{amount,description};
    }
}
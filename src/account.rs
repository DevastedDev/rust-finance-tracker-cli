use std::fs;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::io::Error;
#[derive(Debug,Deserialize)]
pub struct Account{
    transactions: Vec<Transaction>
}
impl Account{
    pub fn init()  -> Self{
        Account{
            transactions: vec![]
        }
    }
    pub fn load_transactions(&self, file_path:String) -> Result<Account,Error>{
        let contents = fs::read_to_string(file_path)?;
        match serde_json::from_str::<Vec<Transaction>>(&contents){
            Ok(transactions) =>{
                Ok(Account{transactions})
            },
            Err(reason) => Err(Error::other("Dataparsing Failed")),
        }
    }
    pub fn add_transaction(&mut self, trxn: Transaction){
        self.transactions.push(trxn);
        fs::write("data/data.json",serde_json::to_string_pretty(&self.transactions).unwrap()).unwrap()
    }
    pub fn get_transactions(&self)-> &Vec<Transaction>{
        &self.transactions
    }

    pub fn remove_transaction(&mut self,index : &usize){
        self.transactions.remove(index-1);
        fs::write("data/data.json",serde_json::to_string_pretty(&self.transactions).unwrap()).unwrap()
    }
}
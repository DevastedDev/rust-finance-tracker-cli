use std::fs;
use crate::transaction::Transaction;
use serde::{Deserialize};
use serde_json::Error;
use crate::errors::{CommonError, FileErrors};
use crate::errors::FileErrors::FileNotFound;

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
    pub fn load_transactions(&self, file_path:&String) -> Result<Account,FileErrors>{
        let contents = fs::read_to_string(file_path).map_err(|e| FileNotFound)?;
        match serde_json::from_str::<Vec<Transaction>>(&contents){
            Ok(transactions) =>{
                Ok(Account{transactions})
            },
            Err(reason) => Err(FileErrors::FileParsingFailed),
        }
    }
    pub fn add_transaction(&mut self, trxn: Transaction,datafile:&String)->Result<bool,CommonError>{
        self.transactions.push(trxn);
        match fs::write(datafile,serde_json::to_string_pretty(&self.transactions).map_err(|e| CommonError::ToPrettyStringFailed)?) {
            Ok(..) => Ok(true),
            Err(..) => Err(CommonError::RemovingTransactionFailed)
        }
    }
    pub fn get_transactions(&self)-> &Vec<Transaction>{
        &self.transactions
    }

    pub fn remove_transaction(&mut self,index : &usize,datafile:&String)-> Result<bool,CommonError> {
        self.transactions.remove(index - 1);
        match fs::write(datafile, serde_json::to_string_pretty(&self.transactions).map_err(|_| CommonError::ToPrettyStringFailed)?) {
            Ok(..) => Ok(true),
            Err(..) => Err(CommonError::RemovingTransactionFailed)
        }
    }

    pub fn find_transactions(&mut self ,keywords : Vec<&str>) -> Vec<&Transaction>{

        let mut records : Vec<&Transaction> = vec![];
        for keyword in keywords {
            self.transactions.iter().filter(|s| s.description.to_lowercase().contains(&keyword.to_lowercase())).for_each(|s| records.push(s));
        }
        records
    }
    pub fn get_stats(&self) -> (f64,usize) {
        let amt:f64 = self.transactions.iter().map(|el| el.amount).sum();
         (amt,self.transactions.iter().count() )
    }
}
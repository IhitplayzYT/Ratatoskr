pub mod Finance{
    use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;
    use crate::model::{meta::Meta::{Frequency, Txn_Type}};
    use uuid::Uuid;
    #[derive(Debug)]
    pub struct Finance_task{
        id:Uuid,
        item:String,
        desc:Option<String>,
        txn_type: Txn_Type,
        amnt: Decimal,
        freq:Frequency,
        pub txn_time: DateTime<Utc>
    }

    impl Finance_task{
       pub fn get_id(&self) ->Uuid{
        self.id
       }
        pub fn get_item(&self) -> String{
        self.item.clone()
       }
       pub fn get_desc(&self) -> String{
        self.desc.clone().unwrap_or_default()
       }

       pub fn get_txn_type(&self) -> Txn_Type{self.txn_type}

       pub fn get_amnt(&self) -> Decimal{self.amnt}
       pub fn get_freq(&self) -> Frequency {self.freq}
       pub fn get_txn_time(&self) -> DateTime<Utc> {self.txn_time}
    
    }


    impl Finance_task {

        pub fn new(item:String,desc:Option<String>,txn_type:Txn_Type,amnt:Decimal,freq:Option<Frequency>) -> Self{
            Self {id:Uuid::new_v4(),item, desc, txn_type, amnt, freq: freq.unwrap_or(Frequency::Once),txn_time:Utc::now()}
        }

    }

    pub struct Ledger{
        txns: Vec<Finance_task>,
    }

    impl Ledger{
        pub fn new() ->Self{
            Self { txns: vec![] }
        }
        pub fn add_txn(&mut self,item:String,desc:Option<String>,txn_type:Txn_Type,amnt:Decimal,freq:Option<Frequency>){
            self.txns.push(Finance_task::new(item, desc, txn_type, amnt, freq));
        }

        pub fn from_txn(&mut self,txn:&Finance_task){
            self.add_txn(txn.item.clone(), txn.desc.clone(), txn.txn_type, txn.amnt, Some(txn.freq));
        }


        pub fn from_txns(txns:Vec<Finance_task>)-> Self{
            let mut ret = Self::new();
            txns.iter().for_each(|x| ret.from_txn(x));
            return ret;            
        }

        pub fn unblock(id:Uuid) -> Decimal{
            Decimal::ZERO
        }
        
        pub fn retrive_txn(&self) -> &Vec<Finance_task> {
            return &self.txns;
        }

        pub fn balance(&self) -> Decimal{
            let mut ret = Decimal::zero();
            self.txns.iter().for_each(|x| {
                match x.txn_type{
                    Txn_Type::CREDIT => {ret += x.amnt;},
                    Txn_Type::DEBIT => {ret -= x.amnt;},
                    Txn_Type::BLOCKED => {}
                }
            });
            ret
        }

        pub fn get_blocked(&self) -> Decimal{
            self.txns.iter().filter_map(|x| if x.txn_type == Txn_Type::BLOCKED {Some(x.amnt)}else{None}).sum()
        }

        pub fn get_txns_between(&self,start:DateTime<Utc>,end:DateTime<Utc>) -> Vec<&Finance_task>{
            self.txns.iter().filter(|x| x.txn_time.ge(&start) && x.txn_time.le(&end)).collect()
        }

    }


}
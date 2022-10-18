use crate::trx::Trx;
use std::string::ToString;
use sha256::digest;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::network::NetworkSetting;

pub struct Block<'a> {
    pub time: String,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u32,
    pub data: Vec<Trx>,
    net_cfg: &'a NetworkSetting,
}

impl<'a> Block<'a> {
    pub fn new(conf: &'a NetworkSetting) -> Self{
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Block {
            time: since_the_epoch.as_millis().to_string(),
            hash: String::from(""),
            prev_hash: String::from(""),
            nonce: 0,
            data: Vec::<Trx>::new(),
            net_cfg: conf,
        }
    }

    pub fn add_trx(&mut self, data: Trx){
        self.data.push(data)
    }

    fn get_hash(&self)->String{
        digest(self.to_string())
    }

    pub fn is_valid(&self) -> bool{
        let sub_patt = &(self.hash)[..self.net_cfg.difficulty];
        let mut pattern: String = String::new();
        for _ in 0..self.net_cfg.difficulty{
            pattern.push_str("0");
        }
        sub_patt == pattern
    }

    pub fn mine(&mut self){
        loop {
            let val = self.get_hash();

            let mut pattern: String = String::new();
            for _ in 0..self.net_cfg.difficulty{
                pattern.push_str("0");
            }

            let sub_patt = &val[..self.net_cfg.difficulty];
            if sub_patt == pattern{
                self.hash = val;
                break;
            }
            self.nonce += 1;
        }
    }
}

impl<'a> ToString for Block<'a> {
    fn to_string(&self) -> String {
        let mut str_data: String = String::new();
        for rec in self.data.iter(){
            str_data.push_str(&rec.to_string())
        }
        return String::from(
            format!("{},{},{},{}",
                self.time,
                self.prev_hash,
                self.nonce,
                str_data,
            )
        )
    }
}
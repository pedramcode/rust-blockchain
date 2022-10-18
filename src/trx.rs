use std::string::ToString;

pub struct Trx {
    from: String,
    to: String,
    amount: f32,
    fee: f32,
}

impl Trx {
    pub fn new(from: String, to: String, amount: f32, fee: f32) -> Self {
        Trx {
            from,
            to,
            amount,
            fee,
        }
    }
}

impl ToString for Trx {
    fn to_string(&self) -> String {
        return String::from(
            format!("{},{},{},{}",
                self.from,
                self.to,
                self.amount,
                self.fee,
            )
        )
    }
}
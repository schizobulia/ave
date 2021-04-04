//控制台消息
#[derive(Debug, Clone)]
pub struct ReceiveMsg {
    key: String,
    msg: String,
    index: i32,
}

impl ReceiveMsg {
    pub fn new(key: String, msg: String, index: i32) -> Self {
        Self { key, msg, index }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res.push_str(self.index.to_string().as_str());
        res.push_str(self.key.as_str());
        res.push_str("：");
        res.push_str(self.msg.as_str());
        res
    }
}

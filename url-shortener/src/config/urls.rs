use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub  struct  Url{
    long_url: String,
    short_url: String,
}

impl Url{
    pub fn new(long_url:String, short_url: String) -> Self{
        Self { long_url, short_url }
    }
    pub fn long_url(&self) -> String{
        self.long_url.clone()
    }
    pub fn short_url(&self) -> String{
        self.short_url.clone()
    }
}
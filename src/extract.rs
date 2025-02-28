pub struct Extract;

impl Extract {
    pub fn from(val: &str) -> Vec<u128> {
       // let txt: String = String::from("h3110 23 cat 444 45 bf5  rabbit 11 2 dog");

        let txt_vec: Vec<String> = val.split_whitespace().map(String::from).collect();

        let mut result: Vec<u128> = Vec::new();

        for i in txt_vec {
            if i.parse::<u128>().is_ok() {
                
                result.push(i.parse().unwrap());
            }
        }
        result
    }
}

pub struct Extract;

impl Extract {
    pub fn from(val: &str) -> Vec<u128> {
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

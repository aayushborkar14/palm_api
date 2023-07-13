pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod palm;
pub use palm::PalmClient;

pub fn create_client(api_key: String) -> PalmClient {
    palm::create_client(api_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let my_client = create_client(String::from(""));
        my_client.list_models().expect("err");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

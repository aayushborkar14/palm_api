pub mod palm;
pub use palm::PalmClient;

pub fn create_client(api_key: String) -> PalmClient {
    palm::create_client(api_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_models_works() {
        let my_client = create_client(String::from(""));
        let models_list = my_client.list_models().expect("err");
        assert_eq!(true, models_list.len() > 0);
    }

    #[test]
    fn get_model_works() {
        let my_client = create_client(String::from(""));
        let model = my_client.get_model(String::from("text-bison-001")).expect("err");
        println!("{}",model.display_name);
    }
}

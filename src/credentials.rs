use crate::*;


/// Get the variable from the .env
fn get_env_var(name: &str) -> String {
    env::var(name).expect(&format!("{} NOT FOUND", name))
}

pub fn endpoint_mongodb() -> String {
    get_env_var("ENDPOINT_MONGODB")
}


#[cfg(test)]
mod tests {
    
    use super::*;
    
    /// Test all the environment variables, if they are present or not
    #[test]
    fn all_envs() {
        dotenv().ok();
        
        let envs = [
            "ENDPOINT_MONGODB",
        ];

        for env in envs {
            assert_ne!(String::default(), get_env_var(env));
        }
    }
}
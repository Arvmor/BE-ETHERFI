use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct APIResponse<T>
    where
    T: Serialize, // Ensure T is Serialize
{
    pub status: i16,
    pub message: T,
}

impl<T> APIResponse<T> 
where
    T: Serialize, // Ensure T is Serialize
{
    pub fn new(status: i16, message: T) -> Self {
        APIResponse {
            status,
            message,
        }
    }

    pub fn success(message: T) -> Self {
        APIResponse {
            status: 0,
            message,
        }
    }

    pub fn fail(message: T) -> Self {
        APIResponse {
            status: -1,
            message,
        }
    }

    // Function to serialize the struct to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let serialized_json = serde_json::to_string(&self).unwrap();
        serialized_json.into_bytes()
    }
}
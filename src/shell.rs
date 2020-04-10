//! # Logging Data Types
//!
//! This module contains data types for the `wascc:logging` capability provider

pub const OP_INVOKE: &str = "Invoke";

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InvokeRequest{
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InvokeResponse{
    pub token: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use codec::{deserialize, serialize};

    // A quick test to certify that the enum round trip
    // works fine in message pack
    #[test]
    fn round_trip() {
        let req1 = InvokeRequest{
            command: "sh".to_string(),
            args: vec!["-c".to_string(),"echo hello".to_string()]
        };
        let buf = serialize(&req1).unwrap();
        let req2: InvokeRequest= deserialize(&buf).unwrap();
        assert_eq!(req1, req2);
    }
}

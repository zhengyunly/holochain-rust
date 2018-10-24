use serde_json;

/// Struct for input data received when Call API function is invoked
#[derive(Deserialize, Serialize)]
pub struct CallArgs {
    pub zome_name: String,
    pub cap_name: String,
    pub fn_name: String,
    pub fn_args: String,
}

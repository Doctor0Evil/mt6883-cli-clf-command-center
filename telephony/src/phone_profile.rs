use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneProfile {
    pub id: String,
    pub label: String,
    pub imei: String,
    pub carrier: String,
}

pub fn list_profiles() -> Vec<PhoneProfile> {
    vec![PhoneProfile {
        id: "profile_default".into(),
        label: "MT6883-Default".into(),
        imei: "000000000000000".into(),
        carrier: "virtual-carrier".into(),
    }]
}

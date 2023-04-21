use std::option::Option;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse {
    pub r#type: String,
    pub version: String,
    pub total_results: i32,
    pub results_from: u32,
    pub previous_results_uri: Option<String>,
    pub next_results_uri: Option<String>,
    pub exception_notice_uri: Option<String>,
    pub results: Vec<Business>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub business_id: String,
    pub name: String,
    pub registration_date: String,
    pub company_form: String,
    pub details_uri: Option<String>,
    pub liquidations: Vec<String>,
    pub names: Vec<Name>,
    pub auxiliary_names: Vec<AuxiliaryName>,
    pub addresses: Vec<Address>,
    pub company_forms: Vec<CompanyForm>,
    pub business_lines: Vec<BusinessLine>,
    pub languages: Vec<Language>,
    pub registed_offices: Vec<RegistedOffice>,
    pub contact_details: Vec<ContactDetail>,
    pub registered_entries: Vec<RegisteredEntry>,
    pub business_id_changes: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    order: u32,
    version: u32,
    name: String,
    registration_date: String,
    end_date: Option<String>,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuxiliaryName {
    order: u32,
    version: u32,
    name: String,
    registration_date: String,
    end_date: Option<String>,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    care_of: Option<String>,
    street: String,
    post_code: String,
    r#type: u32,
    version: u32,
    city: String,
    country: Option<String>,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompanyForm {
    version: u32,
    name: String,
    r#type: Option<String>,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BusinessLine {
    order: u32,
    version: u32,
    code: String,
    name: String,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    version: u32,
    name: String,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistedOffice {
    order: u32,
    version: u32,
    name: String,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetail {
    version: u32,
    value: String,
    r#type: String,
    registration_date: String,
    end_date: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredEntry {
    authority: u32,
    register: u32,
    status: u32,
    registration_date: String,
    end_date: Option<String>,
    status_date: String,
    language: String,
    description: String,
}

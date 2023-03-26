use std::option::Option;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse {
    pub r#type: String,
    pub version: String,
    pub totalResults: i32,
    pub resultsFrom: u32,
    pub previousResultsUri: Option<String>,
    pub nextResultsUri: Option<String>,
    pub exceptionNoticeUri: Option<String>,
    pub results: Vec<Business>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Business {
    pub businessId: String,
    pub name: String,
    pub registrationDate: String,
    pub companyForm: String,
    pub detailsUri: Option<String>,
    pub liquidations: Vec<String>,
    pub names: Vec<Name>,
    pub auxiliaryNames: Vec<AuxiliaryName>,
    pub addresses: Vec<Address>,
    pub companyForms: Vec<CompanyForm>,
    pub businessLines: Vec<BusinessLine>,
    pub languages: Vec<Language>,
    pub registedOffices: Vec<RegistedOffice>,
    pub contactDetails: Vec<ContactDetail>,
    pub registeredEntries: Vec<RegisteredEntry>,
    pub businessIdChanges: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Name {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuxiliaryName {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Address {
    careOf: Option<String>,
    street: String,
    postCode: String,
    r#type: u32,
    version: u32,
    city: String,
    country: Option<String>,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompanyForm {
    version: u32,
    name: String,
    r#type: Option<String>,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BusinessLine {
    order: u32,
    version: u32,
    code: String,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Language {
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegistedOffice {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContactDetail {
    version: u32,
    value: String,
    r#type: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisteredEntry {
    authority: u32,
    register: u32,
    status: u32,
    registrationDate: String,
    endDate: Option<String>,
    statusDate: String,
    language: String,
    description: String,
}

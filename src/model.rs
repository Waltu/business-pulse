use std::option::Option;

#[derive(Debug)]
pub struct ApiResponse {
    r#type: String,
    version: String,
    totalResults: i32,
    resultsFrom: u32,
    previousResultsUri: Option<String>,
    nextResultsUri: Option<String>,
    exceptionNoticeUri: Option<String>,
    results: Vec<Business>,
}

#[derive(Debug)]
struct Business {
    businessId: String,
    name: String,
    registrationDate: String,
    companyForm: String,
    detailsUri: Option<String>,
    liquidations: Vec<String>,
    names: Vec<Name>,
    auxiliaryNames: Vec<AuxiliaryName>,
    addresses: Vec<Address>,
    companyForms: Vec<CompanyForm>,
    businessLines: Vec<BusinessLine>,
    languages: Vec<Language>,
    registedOffices: Vec<RegistedOffice>,
    contactDetails: Vec<ContactDetail>,
    registeredEntries: Vec<RegisteredEntry>,
    businessIdChanges: Vec<String>,
}

#[derive(Debug)]
struct Name {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    source: u32,
}

#[derive(Debug)]
struct AuxiliaryName {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    source: u32,
}

#[derive(Debug)]
struct Address {
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

#[derive(Debug)]
struct CompanyForm {
    version: u32,
    name: String,
    r#type: Option<String>,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug)]
struct BusinessLine {
    order: u32,
    version: u32,
    code: String,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug)]
struct Language {
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug)]
struct RegistedOffice {
    order: u32,
    version: u32,
    name: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug)]
struct ContactDetail {
    version: u32,
    value: String,
    r#type: String,
    registrationDate: String,
    endDate: Option<String>,
    language: String,
    source: u32,
}

#[derive(Debug)]
struct RegisteredEntry {
    authority: u32,
    register: u32,
    status: u32,
    registrationDate: String,
    endDate: Option<String>,
    statusDate: String,
    language: String,
    description: String,
}

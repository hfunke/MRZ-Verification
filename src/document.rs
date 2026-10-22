use chrono::{NaiveDate, ParseError};


#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Gender {
    Female,
    Male,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Document {
    pub numeric_check_digits: bool,
    /// `true` if every entry in `check_digits` is valid.
    pub valid_check_digits: bool,
    /// Result per check digit, in MRZ order, keyed by the `CHECK_DIGIT_*` names
    /// (e.g. `("date_of_birth", false)`). Only the check digits the format
    /// carries are listed: MRV-A/B have no composite check digit, and the TD3
    /// personal number is listed only when that field is not empty.
    pub check_digits: Vec<(&'static str, bool)>,
    pub valid_issuing_country_code: bool,
    pub valid_nationality_country_code: bool,
    pub issuing_country_code_type: String,
    pub nationality_country_code_type: String,
    pub valid_characters: bool,
    pub valid_document_type: bool,
    pub document_type: String,
    /// Human-readable type per the harmonized secondary document codes of
    /// Doc 9303-4 Section 4.4 (e.g. "Diplomatic passport" for `PD`);
    /// empty if the code is not one of the harmonized MRP codes.
    pub document_type_name: String,
    /// `true` if the document code is one of the harmonized MRP codes of
    /// Doc 9303-4 Section 4.4. `false` is not an error: codes issued before
    /// 2028 may use other second letters and remain valid until 2038, and
    /// non-passport documents (ID cards, visas) are not covered by 4.4.
    pub harmonized_document_code: bool,
    pub issuing_country_code: String,
    pub issuing_country_name: String,
    pub surname: String,
    pub given_name: String,
    pub document_number: String,
    pub nationality_code: String,
    pub nationality_name: String,
    pub date_of_birth: String,
    pub gender: Gender,
    /// Serialized as `"YYYY-MM-DD"`, or `null` if the date could not be parsed.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_expiry_date"))]
    pub expiry_date: Result<NaiveDate, ParseError>,
    pub optional_data: String,
}

#[cfg(feature = "serde")]
fn serialize_expiry_date<S: serde::Serializer>(
    date: &Result<NaiveDate, ParseError>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match date {
        Ok(d) => serializer.collect_str(&d.format("%Y-%m-%d")),
        Err(_) => serializer.serialize_none(),
    }
}

/// Generates the nine field-getter methods for a `MachineReadableZone` impl.
/// Parameters: byte ranges for name field, doc_no, nationality, dob, sex, doe.
/// `get_document_type` and `get_issuing_state` are always at [0..2] and [2..5].
#[macro_export]
#[doc(hidden)]
macro_rules! impl_mrz_getters {
    // Full form: document number is a fixed field at [doc_no].
    (
        name:   ($ns:literal, $ne:literal),
        doc_no: ($dns:literal, $dne:literal),
        nat:    ($nats:literal, $nate:literal),
        dob:    ($dobs:literal, $dobe:literal),
        sex:    ($sxs:literal, $sxe:literal),
        doe:    ($does:literal, $doee:literal)
    ) => {
        $crate::impl_mrz_getters!(
            name: ($ns, $ne),
            nat:  ($nats, $nate),
            dob:  ($dobs, $dobe),
            sex:  ($sxs, $sxe),
            doe:  ($does, $doee)
        );
        fn get_document_number(&self) -> String {
            $crate::utils::trim(&self.raw_mrz[$dns..$dne])
        }
    };
    // Reduced form: caller implements `get_document_number` itself (e.g. TD1, which
    // supports document numbers longer than the fixed field).
    (
        name:   ($ns:literal, $ne:literal),
        nat:    ($nats:literal, $nate:literal),
        dob:    ($dobs:literal, $dobe:literal),
        sex:    ($sxs:literal, $sxe:literal),
        doe:    ($does:literal, $doee:literal)
    ) => {
        fn get_document_type(&self) -> String {
            $crate::utils::trim(&self.raw_mrz[0..2])
        }
        fn get_issuing_state(&self) -> String {
            $crate::utils::trim(&self.raw_mrz[2..5])
        }
        fn get_surname(&self) -> String {
            let field = &self.raw_mrz[$ns..$ne];
            let pos = field.find("<<").unwrap_or(field.len());
            $crate::utils::trim(&field[..pos])
        }
        fn get_given_name(&self) -> String {
            let field = &self.raw_mrz[$ns..$ne];
            let Some(pos) = field.find("<<") else { return String::new(); };
            $crate::utils::trim(&field[pos..])
        }
        fn get_nationality(&self) -> String {
            $crate::utils::trim(&self.raw_mrz[$nats..$nate])
        }
        fn get_date_of_birth(&self) -> String {
            self.raw_mrz[$dobs..$dobe].to_string()
        }
        fn get_sex(&self) -> String {
            self.raw_mrz[$sxs..$sxe].to_string()
        }
        fn get_date_of_expiry(&self) -> String {
            self.raw_mrz[$does..$doee].to_string()
        }
    };
}

use chrono::NaiveDate;
use countrycodes::{is_country_code_valid, get_country_name, get_country_code_type};
use documentcodes::{is_document_code_harmonized, get_document_type_name};
pub use document::{Document, Gender};
pub use error::Error;
use mrv_a::{MRV_A_LENGTH, MrvA};
use mrv_b::{MRV_B_LENGTH, MrvB};
use regex::Regex;
use std::sync::OnceLock;
use td1::{TD1_LENGTH, TD1};
use td2::{TD2_LENGTH, TD2};
use td3::{TD3_LENGTH, TD3};
use utils::*;

static RE_TD3:   OnceLock<Regex> = OnceLock::new();
static RE_TD2:   OnceLock<Regex> = OnceLock::new();
static RE_TD1:   OnceLock<Regex> = OnceLock::new();
static RE_MRV_A: OnceLock<Regex> = OnceLock::new();
static RE_MRV_B: OnceLock<Regex> = OnceLock::new();

mod document;
mod documentcodes;
mod td3;
mod td2;
mod td1;
mod mrv_a;
mod mrv_b;
mod utils;
mod countrycodes;
mod error;

pub trait MachineReadableZone {
    fn get_raw_mrz(self) -> String;
    fn get_document_type(&self) -> String;
    fn get_issuing_state(&self) -> String;
    fn get_document_number(&self) -> String;
    fn get_optional_data(&self) -> String;
    fn get_date_of_birth(&self) -> String;
    fn get_sex(&self) -> String;
    fn get_date_of_expiry(&self) -> String;
    fn get_nationality(&self) -> String;
    fn get_surname(&self) -> String;
    fn get_given_name(&self) -> String;
    fn are_check_digits_numeric(&self) -> bool;
    /// Result of every check digit this format carries, in MRZ order, keyed by
    /// one of the `CHECK_DIGIT_*` names.
    fn check_digit_results(&self) -> Vec<(&'static str, bool)>;
    fn are_check_digits_valid(&self) -> bool {
        self.check_digit_results().iter().all(|&(_, valid)| valid)
    }
}

// Field names used in `Document::check_digits`.
pub const CHECK_DIGIT_DOCUMENT_NUMBER: &str = "document_number";
pub const CHECK_DIGIT_DATE_OF_BIRTH: &str = "date_of_birth";
pub const CHECK_DIGIT_EXPIRY_DATE: &str = "expiry_date";
/// TD3 only, and only when the personal number field is not empty.
pub const CHECK_DIGIT_PERSONAL_NUMBER: &str = "personal_number";
/// TD1, TD2 and TD3 only; MRV-A and MRV-B carry no composite check digit.
pub const CHECK_DIGIT_COMPOSITE: &str = "composite";

const DATE_FORMAT_YYMMDD: &str = "%y%m%d";
//const DATE_FORMAT_YYMM: &str = "%y%m";
//const DATE_FORMAT_YY: &str = "%y";

/*ToDo: Validity checken:
// 1) Valid Length
// 2) Valid chars
// 3) Valid dates (DOB, DOE...), Gültigkeit
// 4) Valid check digits
// 5) Valid States (Issuing state...)
// 6) Complete information
// 7) Optional data: German Serial information
// 8) Document type
// 9) Verify that check digits are numerical 
// 10) Regulärer Ausdruck über die gesamte MRZ, inkl. Datum Zahlen 0-3

// Samples of Doc9303:
TD1:   I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<
TD2:   I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6
TD3:   P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10
        
MRV-A: V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<
MRV-B: V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226
ETD: TD2 format



TD1: 3x30
TD2: 2x36
TD3: 2x44
MRV-A: 2x44
MRV-B: 2x36


Check-Reihenfolge:
- Prüfung der MRZ-Länge
- Prüfung der MRZ-Struktur (RegEx)
-> Bestimmung ob TD1, TD2 oder TD3
- Prüfung, ob Checkdigits numerisch sind
- Prüfung der Checkdigits 
- Prüfung der verwendeten Zeichen
- Prüfung des DocTypes
- Prüfung des Issuing Countries
- Prüfung der Nationalität
- Prüfung des DoB
- Prüfung des DoE
- Prüfung des Geschlechts
-> Bestimmung des Typs: MRP, MRTD, MROTD...
-> Extrahieren der einzelnen Infos in Datenstruktur (struct) und Rückgabe


*/


fn extract_data<T: MachineReadableZone>(m: T) -> Document {

    //let t = &m.get_raw_mrz();

    let numeric_check_digits = m.are_check_digits_numeric();
    let check_digits = m.check_digit_results();
    let valid_check_digits = check_digits.iter().all(|&(_, valid)| valid);

    let document_type = m.get_document_type();
    let valid_document_type = is_document_type_valid(&document_type);
    let document_type_name = get_document_type_name(&document_type);
    let harmonized_document_code = is_document_code_harmonized(&document_type);

    let issuing_country_code = m.get_issuing_state();
    let valid_issuing_country_code = is_country_code_valid(&issuing_country_code);
    let issuing_country_name = get_country_name(&issuing_country_code);
    let issuing_country_code_type = get_country_code_type(&issuing_country_code);

    let surname = m.get_surname();
    let given_name = m.get_given_name();
    let document_number = m.get_document_number();

    let nationality_country_code = m.get_nationality();
    let valid_nationality_country_code = is_country_code_valid(&nationality_country_code);
    let nationality_country_name = get_country_name(&nationality_country_code);
    let nationality_country_code_type = get_country_code_type(&nationality_country_code);

    let doe = m.get_date_of_expiry();
    let date_of_expiry = NaiveDate::parse_from_str(&doe, DATE_FORMAT_YYMMDD);

    // ICAO allows incomplete date of birth:
    let dob = m.get_date_of_birth();
    let date_of_birth = extract_date(&dob);

    let gender = match m.get_sex().as_str() {
        "F" => Gender::Female,
        "M" => Gender::Male,
        _   => Gender::Other,
    };

    let optional_data = m.get_optional_data();
    let valid_characters = are_characters_valid(&m.get_raw_mrz());

    

    Document{
        numeric_check_digits,
        valid_check_digits,
        check_digits,
        valid_issuing_country_code,
        valid_nationality_country_code,
        issuing_country_code_type,
        nationality_country_code_type,
        valid_characters,
        valid_document_type,
        document_type,
        document_type_name,
        harmonized_document_code,
        issuing_country_code,
        issuing_country_name,
        surname,
        given_name,
        document_number,
        nationality_code: nationality_country_code,
        nationality_name: nationality_country_name,
        date_of_birth,
        gender,
        expiry_date: date_of_expiry,
        optional_data,
    }

}

fn extract_date(date: &str) -> String {
    let mut date_string = String::new();
    if date.contains('<') {
        let date = date.replace('<', "X");
        date_string.push_str(&date[..2]);
        date_string.push('-');
        date_string.push_str(&date[2..4]);
        date_string.push('-');
        date_string.push_str(&date[4..6]);
    } else {
        if let Ok(d) = NaiveDate::parse_from_str(date, DATE_FORMAT_YYMMDD) {
            date_string = d.format("%y-%m-%d").to_string();
        }
    }
    date_string
}


pub fn parse_mrz(str: String) -> Result<Document, Error> {
    let len = str.len();
    if len != TD1_LENGTH && len != TD2_LENGTH && len != TD3_LENGTH
            && len != MRV_A_LENGTH && len != MRV_B_LENGTH {
        return Err(Error::InvalidLength);
    }

    // Field extraction slices the raw MRZ at fixed byte offsets. Every MRZ format is
    // ASCII, so rejecting non-ASCII up front keeps those offsets on char boundaries
    // and prevents a slice-boundary panic if a structure regex ever admits non-ASCII.
    if !str.is_ascii() {
        return Err(Error::InvalidStructure);
    }

    // Dispatch by length: a format struct is only built when the string has that
    // format's exact length, so its fixed field offsets are always in bounds.
    // Lengths collide (88 = TD3/MRV-A, 72 = TD2/MRV-B); order preserves the original
    // precedence within each group.
    if len == TD3_LENGTH {
        if verify_structure_td3(&str) {
            return Ok(extract_data(TD3 { raw_mrz: str }));
        }
        if verify_structure_mrv_a(&str) {
            return Ok(extract_data(MrvA { raw_mrz: str }));
        }
    } else if len == TD2_LENGTH {
        if verify_structure_td2(&str) {
            return Ok(extract_data(TD2 { raw_mrz: str }));
        }
        if verify_structure_mrv_b(&str) {
            return Ok(extract_data(MrvB { raw_mrz: str }));
        }
    } else if len == TD1_LENGTH
        && verify_structure_td1(&str) {
            return Ok(extract_data(TD1 { raw_mrz: str }));
        }

    Err(Error::InvalidStructure)
}


// ToDo: Add new function get_document_type() Visa, Passport, Crew Member Certificate, MROTD, MRTD, TDx...

// ToDo: Add new function to assure that encoding of sex is valid

// ToDo: Add handling for NaiveData with incomplete data

// ToDo: Add improved and more detailed error handling

/// ICAO Doc 9303 name field: primary identifier components separated by single
/// fillers, optionally followed by `<<` and the secondary identifier components,
/// then filler padding. The `<<` separator is absent when a single-name holder's
/// (possibly truncated) primary identifier fills the field up to at most one
/// trailing filler (Doc 9303-4 Sections 4.2.3 and 4.2.3.1 f).
const NAME_FIELD: &str = r"[[:upper:]]+(?:<[[:upper:]]+)*(?:<<[[:upper:]]+(?:<[[:upper:]]+)*)?<*";

pub fn verify_structure_td3(mrz: &str) -> bool {
    let re = RE_TD3.get_or_init(|| Regex::new(&[r"^P[[:upper:]<][[:upper:]]{1}[[:upper:]<]{2}", NAME_FIELD, r"[[:upper:][:digit:]<]{9}[[:digit:]][[:upper:]]{1}[[:upper:]<]{2}[[:digit:]<]{7}[MF<][[:digit:]]{7}[[:upper:][:digit:]<]{14}[[:digit:]<][[:digit:]]\z"].concat()).unwrap());
    re.is_match(mrz)
}

pub fn verify_structure_td2(mrz: &str) -> bool {
    let re = RE_TD2.get_or_init(|| Regex::new(&[r"^[ACI][[:upper:]<][[:upper:]]{1}[[:upper:]<]{2}", NAME_FIELD, r"[[:upper:][:digit:]<]{9}[[:digit:]<][[:upper:]]{1}[[:upper:]<]{2}[[:digit:]<]{7}[MF<][[:digit:]]{7}[[:upper:][:digit:]<]{6}[[:digit:]<][[:digit:]]\z"].concat()).unwrap());
    re.is_match(mrz)
}

pub fn verify_structure_td1(mrz: &str) -> bool {
    let re = RE_TD1.get_or_init(|| Regex::new(&[r"^[ACI][[:upper:]<][[:upper:]]{1}[[:upper:]<]{2}[[:upper:][:digit:]<]{9}[[:digit:]<][[:upper:][:digit:]<]{14}[[:digit:]<][[:digit:]<]{7}[MF<][[:digit:]]{7}[[:upper:]<]{3}[[:upper:][:digit:]<]{10}[[:digit:]<][[:digit:]]", NAME_FIELD, r"\z"].concat()).unwrap());
    re.is_match(mrz)
}

pub fn verify_structure_mrv_a(mrz: &str) -> bool {
    let re = RE_MRV_A.get_or_init(|| Regex::new(&[r"^V[[:upper:]<][[:upper:]]{1}[[:upper:]<]{2}", NAME_FIELD, r"[[:upper:][:digit:]<]{9}[[:digit:]][[:upper:]]{1}[[:upper:]<]{2}[[:digit:]<]{7}[MF<][[:digit:]]{7}[[:upper:][:digit:]<]{16}\z"].concat()).unwrap());
    re.is_match(mrz)
}

pub fn verify_structure_mrv_b(mrz: &str) -> bool {
    let re = RE_MRV_B.get_or_init(|| Regex::new(&[r"^V[[:upper:]<][[:upper:]]{1}[[:upper:]<]{2}", NAME_FIELD, r"[[:upper:][:digit:]<]{9}[[:digit:]][[:upper:]]{1}[[:upper:]<]{2}[[:digit:]<]{7}[MF<][[:digit:]]{7}[[:upper:][:digit:]<]{8}\z"].concat()).unwrap());
    re.is_match(mrz)
}


pub fn is_document_type_valid(doc_type: &str) -> bool {
    if doc_type.len() > 2 || doc_type.is_empty() {
        // ToDo: Throw an error in this case
        return false;
    }
    
    // Crew Member Certificate
    if doc_type == "AC" {
        return true;
    }
    
    if doc_type.starts_with('P') || doc_type.starts_with('A') || doc_type.starts_with('C') || doc_type.starts_with('V') {
        return true;
    }
    
    if doc_type.starts_with('I') {
        !doc_type.ends_with('C') && !doc_type.ends_with('V')

    } else {
        false
    }
    
}


pub fn are_characters_valid(str: &str) -> bool {
    str.chars().all(is_valid_char)
}


#[cfg(test)]
mod tests {
    use crate::countrycodes::COUNTRY_TYPE_ICAO;
    use super::*;

    #[test]
    fn test_char_validity_icao_sample() {
        let dummy = are_characters_valid("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<");
        assert!(dummy);
    }   
    
    #[test]
    fn test_char_validity_icao_sample_lower_char() {
        let dummy = are_characters_valid("I<uTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<");
        assert!(!dummy);
    }   
       
    #[test]
    fn test_char_validity_icao_sample_space() {
        let dummy = are_characters_valid("I<UTOD231458907 <<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<");
        assert!(!dummy);
    }   
 
    #[test]
    fn test_document_type_validity_p() {
        let dummy =  is_document_type_valid("P<");
        assert!(dummy);
    }   

    #[test]
    fn test_document_type_validity_v() {
        let dummy =  is_document_type_valid("V<");
        assert!(dummy);
    }   

    #[test]
    fn test_document_type_validity_ac() {
        let dummy =  is_document_type_valid("AC");
        assert!(dummy);
    }   

    #[test]
    fn test_document_type_validity_ic() {
        let dummy =  is_document_type_valid("IC");
        assert!(!dummy);
    }   

    #[test]
    fn test_document_type_validity_ppp() {
        let dummy =  is_document_type_valid("PPP");
        assert!(!dummy);
    }   

    #[test]
    fn test_document_type_validity_pa() {
        let dummy =  is_document_type_valid("PA");
        assert!(dummy);
    }   

    #[test]
    fn test_document_type_validity_di() {
        let dummy =  is_document_type_valid("DI");
        assert!(!dummy);
    }   

    #[test]
    fn test_document_type_validity_empty() {
        let dummy =  is_document_type_valid("");
        assert!(!dummy);
    }   

    #[test]
    fn test_structure_validity_td3_optional_data1() {
        let dummy = verify_structure_td3("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }   

    #[test]
    fn test_structure_validity_td3_optional_data2() {
        let dummy = verify_structure_td3("P<UTOIVANOVA<<ANNA<<<<<<<<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F2504155184226<<<<<<<<18");
        assert!(dummy);
    }   

    #[test]
    fn test_structure_validity_td3_no_optional_data() {
        let dummy = verify_structure_td3("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122<2310314<<<<<<<<<<<<<<<4");
        assert!(dummy);
    }   

    #[test]
    fn test_structure_validity_td3_long_name() {
        let dummy = verify_structure_td3("P<D<<MUSTERMANN<<ERIKA<ANASTASIA<LUDMILLA<KAC11T002JM4D<<9608122<2310314<<<<<<<<<<<<<<<4");
        assert!(dummy);
    }   


    #[test]
    fn test_structure_validity_td3_invalid_sex() {
        let dummy = verify_structure_td3("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122A2310314<<<<<<<<<<<<<<<4");
        assert!(!dummy);
    }       
    
    #[test]
    fn test_structure_validity_td3_invalid_given_name() {
        let dummy = verify_structure_td3("P<D<<MUSTERMANN<<<<<<<<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122A2310314<<<<<<<<<<<<<<<4");
        assert!(!dummy);
    }       

    #[test]
    fn test_structure_validity_td3_incomplete_dob() {
        let dummy = verify_structure_td3("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408<<2F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }       
    
    #[test]
    fn test_structure_validity_td3_single_name_padded() {
        // Doc 9303-4 4.2.3.1 f): no secondary identifier, field padded with fillers
        let dummy = verify_structure_td3("P<UTOARKFREITH<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td3_single_name_full_field() {
        // Truncated single name occupying all 39 positions: no `<<` separator at all
        let dummy = verify_structure_td3("P<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLML898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td3_single_name_one_trailing_filler() {
        // 38-character single name leaves room for only one filler: no `<<` either
        let dummy = verify_structure_td3("P<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKL<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td3_multi_component_surname() {
        // Doc 9303-4 4.2.3.1 c): hyphenated surname encoded with single filler
        let dummy = verify_structure_td3("P<UTOSMITH<JONES<<SUSIE<MARGARET<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td3_triple_filler_in_name_invalid() {
        // A secondary identifier must follow `<<` directly; `<<<` before letters is invalid
        let dummy = verify_structure_td3("P<UTOERIKSSON<<<ANNA<MARIA<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_td1_single_name_full_field() {
        let dummy = verify_structure_td1("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ABCDEFGHIJKLMNOPQRSTUVWXYZABCD");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td2_single_name_full_field() {
        let dummy = verify_structure_td2("I<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDED231458907UTO7408122F1204159<<<<<<<6");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_mrv_a_single_name_full_field() {
        let dummy = verify_structure_mrv_a("V<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLML898902C<3UTO6908061F9406236ZE184226B<<<<<<<");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_mrv_b_single_name_full_field() {
        let dummy = verify_structure_mrv_b("V<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEL898902C<3UTO6908061F9406236ZE184226");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td3_no_dob() {
        let dummy = verify_structure_td3("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO<<<<<<2F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }       

    #[test]
    fn test_structure_validity_td2_no_optional_data() {
        let dummy = verify_structure_td2("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6");
        assert!(dummy);
    }       
    
    #[test]
    fn test_structure_validity_td2_optional_data() {
        let dummy = verify_structure_td2("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226");
        assert!(dummy);
    }       
    
    #[test]
    fn test_structure_validity_td1_no_optional_data() {
        let dummy = verify_structure_td1("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<");
        assert!(dummy);
    }
    
    #[test]
    fn test_structure_validity_mrv_a() {
        let dummy = verify_structure_mrv_a("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<");
        assert!(dummy);
    }       

    #[test]
    fn test_structure_validity_mrv_b() {
        let dummy = verify_structure_mrv_b("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226");
        assert!(dummy);
    }       

    #[test]
    fn test_parse_mrz_td3() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122M1204159ZE184226B<<<<<10".to_string());
        assert!(dummy.as_ref().unwrap().numeric_check_digits);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
        assert!(dummy.as_ref().unwrap().valid_issuing_country_code);
        assert!(dummy.as_ref().unwrap().valid_nationality_country_code);
        assert!(dummy.as_ref().unwrap().valid_characters);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().nationality_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().document_type, "P");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_name, "Utopia");
        assert_eq!(dummy.as_ref().unwrap().surname, "ERIKSSON");
        assert_eq!(dummy.as_ref().unwrap().given_name, "ANNA MARIA");
        assert_eq!(dummy.as_ref().unwrap().document_number, "L898902C3");
        assert_eq!(dummy.as_ref().unwrap().nationality_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().nationality_name, "Utopia");
        //assert_eq!(dummy.as_ref().unwrap().date_of_birth, NaiveDate::from_ymd_opt(1974, 8, 12).unwrap());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-12");
        assert_eq!(dummy.as_ref().unwrap().gender, Gender::Male);
        assert_eq!(dummy.as_ref().unwrap().expiry_date.unwrap(), NaiveDate::from_ymd_opt(2012, 4, 15).unwrap());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "ZE184226B");

    }       
    
    #[test]
    fn test_parse_mrz_td2() {
        let dummy = parse_mrz("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6".to_string());
        assert!(dummy.as_ref().unwrap().numeric_check_digits);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
        assert!(dummy.as_ref().unwrap().valid_issuing_country_code);
        assert!(dummy.as_ref().unwrap().valid_nationality_country_code);
        assert!(dummy.as_ref().unwrap().valid_characters);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().nationality_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().document_type, "I");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_name, "Utopia");
        assert_eq!(dummy.as_ref().unwrap().surname, "ERIKSSON");
        assert_eq!(dummy.as_ref().unwrap().given_name, "ANNA MARIA");
        assert_eq!(dummy.as_ref().unwrap().document_number, "D23145890");
        assert_eq!(dummy.as_ref().unwrap().nationality_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().nationality_name, "Utopia");
        //assert_eq!(dummy.as_ref().unwrap().date_of_birth, NaiveDate::from_ymd_opt(1974, 8, 12).unwrap());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-12");
        assert_eq!(dummy.as_ref().unwrap().gender, Gender::Female);
        assert_eq!(dummy.as_ref().unwrap().expiry_date.unwrap(), NaiveDate::from_ymd_opt(2012, 4, 15).unwrap());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "");
    }       
  
    #[test]
    fn test_parse_mrz_td1() {
        let dummy = parse_mrz("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert!(dummy.as_ref().unwrap().numeric_check_digits);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
        assert!(dummy.as_ref().unwrap().valid_issuing_country_code);
        assert!(dummy.as_ref().unwrap().valid_nationality_country_code);
        assert!(dummy.as_ref().unwrap().valid_characters);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().nationality_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().document_type, "I");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_name, "Utopia");
        assert_eq!(dummy.as_ref().unwrap().surname, "ERIKSSON");
        assert_eq!(dummy.as_ref().unwrap().given_name, "ANNA MARIA");
        assert_eq!(dummy.as_ref().unwrap().document_number, "D23145890");
        assert_eq!(dummy.as_ref().unwrap().nationality_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().nationality_name, "Utopia");
        //assert_eq!(dummy.as_ref().unwrap().date_of_birth, NaiveDate::from_ymd_opt(1974, 8, 12).unwrap());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-12");
        assert_eq!(dummy.as_ref().unwrap().gender, Gender::Female);
        assert_eq!(dummy.as_ref().unwrap().expiry_date.unwrap(), NaiveDate::from_ymd_opt(2012, 4, 15).unwrap());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "");
    }       
  
    #[test]
    fn test_parse_mrz_mrv_a() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3D<<6908061F9406236ZE184226B<<<<<<<".to_string());
        assert!(dummy.as_ref().unwrap().numeric_check_digits);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
        assert!(dummy.as_ref().unwrap().valid_issuing_country_code);
        assert!(dummy.as_ref().unwrap().valid_nationality_country_code);
        assert!(dummy.as_ref().unwrap().valid_characters);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().nationality_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().document_type, "V");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_name, "Utopia");
        assert_eq!(dummy.as_ref().unwrap().surname, "ERIKSSON");
        assert_eq!(dummy.as_ref().unwrap().given_name, "ANNA MARIA");
        assert_eq!(dummy.as_ref().unwrap().document_number, "L898902C");
        assert_eq!(dummy.as_ref().unwrap().nationality_code, "D");
        assert_eq!(dummy.as_ref().unwrap().nationality_name, "Germany");
        //assert_eq!(dummy.as_ref().unwrap().date_of_birth, NaiveDate::from_ymd_opt(2069, 8, 6).unwrap());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "69-08-06");
        assert_eq!(dummy.as_ref().unwrap().gender, Gender::Female);
        assert_eq!(dummy.as_ref().unwrap().expiry_date.unwrap(), NaiveDate::from_ymd_opt(1994, 6, 23).unwrap());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "ZE184226B");
    }       
 
    #[test]
    fn test_parse_mrz_mrv_b() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061<9406236ZE184226".to_string());
        assert!(dummy.as_ref().unwrap().numeric_check_digits);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
        assert!(dummy.as_ref().unwrap().valid_issuing_country_code);
        assert!(dummy.as_ref().unwrap().valid_nationality_country_code);
        assert!(dummy.as_ref().unwrap().valid_characters);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().nationality_country_code_type, COUNTRY_TYPE_ICAO);
        assert_eq!(dummy.as_ref().unwrap().document_type, "V");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().issuing_country_name, "Utopia");
        assert_eq!(dummy.as_ref().unwrap().surname, "ERIKSSON");
        assert_eq!(dummy.as_ref().unwrap().given_name, "ANNA MARIA");
        assert_eq!(dummy.as_ref().unwrap().document_number, "L898902C");
        assert_eq!(dummy.as_ref().unwrap().nationality_code, "UTO");
        assert_eq!(dummy.as_ref().unwrap().nationality_name, "Utopia");
        //assert_eq!(dummy.as_ref().unwrap().date_of_birth, NaiveDate::from_ymd_opt(2069, 8, 6).unwrap());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "69-08-06");
        assert_eq!(dummy.as_ref().unwrap().gender, Gender::Other);
        assert_eq!(dummy.as_ref().unwrap().expiry_date.unwrap(), NaiveDate::from_ymd_opt(1994, 6, 23).unwrap());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "ZE184226");
    }       

    #[test]
    fn test_parse_mrz_short() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidLength);
        
    }       

    #[test]
    fn test_parse_mrz_long() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidLength);
    }       

    #[test]
    fn test_parse_mrz_empty() {
        let dummy = parse_mrz("".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidLength);
    }       
    
    #[test]
    fn test_parse_mrz_empty_lt() {
        let dummy = parse_mrz("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       

    #[test]
    fn test_parse_mrz_invalid() {
        let dummy = parse_mrz("ERICSSON".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidLength);
    }       

    #[test]
    fn test_parse_mrz_invalid_structure_issuing_state_td1() {
        let dummy = parse_mrz("I<<<<D231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       

    #[test]
    fn test_parse_mrz_invalid_structure_issuing_state_td2() {
        let dummy = parse_mrz("I<<<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       
    
    #[test]
    fn test_parse_mrz_invalid_structure_issuing_state_td3() {
        let dummy = parse_mrz("P<<<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       

    #[test]
    fn test_parse_mrz_germany_issuing() {
        let dummy = parse_mrz("I<D<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6".to_string());
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "D");
    }       
        
    #[test]
    fn test_parse_mrz_germany_nationality() {
        let dummy = parse_mrz("I<D<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907D<<7408122F1204159<<<<<<<6".to_string());
        assert_eq!(dummy.as_ref().unwrap().issuing_country_code, "D");
    }       
    
    #[test]
    fn test_parse_mrz_bt() {                               
        let dummy = parse_mrz("V>UTOERIKSSON>>ANNA>MARIA>>>>>>>>>>>L898902C>3UTO6908061F9406236ZE184226".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       

    #[test]
    fn test_parse_mrz_invalid_visa_type() {                               
        let dummy = parse_mrz("D>UTOERIKSSON>>ANNA>MARIA>>>>>>>>>>>L898902C>3UTO6908061F9406236ZE184226".to_string());
        assert_eq!(dummy.unwrap_err(), Error::InvalidStructure);
    }       

    #[test]
    fn test_parse_mrz_invalid_doe_month_zero() {                               
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408062M1200159ZE184226B<<<<<10".to_string());
        assert!(dummy.as_ref().unwrap().expiry_date.is_err());
    }       

    #[test]
    fn test_parse_mrz_incomplete_dob() {                               
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408<<2M1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-XX");
    }       

    #[test]
    fn test_extract_date_incomplete_dob_day() {                               
        let dummy = extract_date("7408<<");
        assert_eq!(dummy, "74-08-XX");
    }       

    #[test]
    fn test_extract_date_incomplete_dob_month() {                               
        let dummy = extract_date("74<<06");
        assert_eq!(dummy, "74-XX-06");
    }       

    #[test]
    fn test_extract_date_incomplete_dob_year() {                               
        let dummy = extract_date("<<0806");
        assert_eq!(dummy, "XX-08-06");
    }       

    #[test]
    fn test_extract_date_incomplete_dob_all() {
        let dummy = extract_date("<<<<<<");
        assert_eq!(dummy, "XX-XX-XX");
    }

    #[test]
    fn test_extract_date_complete_valid() {
        let dummy = extract_date("740812");
        assert_eq!(dummy, "74-08-12");
    }

    #[test]
    fn test_extract_date_invalid_complete() {
        let dummy = extract_date("740032");
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_document_type_validity_i_single() {
        let dummy = is_document_type_valid("I");
        assert!(dummy);
    }

    #[test]
    fn test_document_type_validity_c_single() {
        let dummy = is_document_type_valid("C");
        assert!(dummy);
    }

    #[test]
    fn test_document_type_validity_a_single() {
        let dummy = is_document_type_valid("A");
        assert!(dummy);
    }

    #[test]
    fn test_document_type_validity_iv() {
        let dummy = is_document_type_valid("IV");
        assert!(!dummy);
    }

    #[test]
    fn test_document_type_validity_ia() {
        let dummy = is_document_type_valid("IA");
        assert!(dummy);
    }

    #[test]
    fn test_structure_validity_td1_invalid_doc_type() {
        let dummy = verify_structure_td1("P<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_td2_invalid_sex() {
        let dummy = verify_structure_td2("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122A1204159<<<<<<<6");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_td2_invalid_issuing_state() {
        let dummy = verify_structure_td2("I<<<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_mrv_a_invalid_sex() {
        let dummy = verify_structure_mrv_a("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061A9406236ZE184226B<<<<<<<");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_mrv_a_invalid_issuing_state() {
        let dummy = verify_structure_mrv_a("V<<<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_mrv_b_invalid_sex() {
        let dummy = verify_structure_mrv_b("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061A9406236ZE184226");
        assert!(!dummy);
    }

    #[test]
    fn test_structure_validity_mrv_b_invalid_issuing_state() {
        let dummy = verify_structure_mrv_b("V<<<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226");
        assert!(!dummy);
    }

    #[test]
    fn test_parse_mrz_td1_with_optional_data() {
        let dummy = parse_mrz("I<UTOD231458907AA<<<<<<<<<<<<<7408122F1204159UTOBB<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "AABB");
        assert_eq!(dummy.as_ref().unwrap().document_number, "D23145890");
    }

    #[test]
    fn test_parse_mrz_td3_without_optional_data() {
        let dummy = parse_mrz("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122F2310314<<<<<<<<<<<<<<<4".to_string());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "");
        assert!(dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_td1_incomplete_dob() {
        let dummy = parse_mrz("I<UTOD231458907<<<<<<<<<<<<<<<7408<<2F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-XX");
    }

    #[test]
    fn test_parse_mrz_td2_incomplete_dob() {
        let dummy = parse_mrz("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408<<2F1204159<<<<<<<6".to_string());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "74-08-XX");
    }

    #[test]
    fn test_parse_mrz_mrv_a_incomplete_dob() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908<<2F9406236ZE184226B<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "69-08-XX");
    }

    #[test]
    fn test_parse_mrz_mrv_b_incomplete_dob() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908<<2F9406236ZE184226".to_string());
        assert_eq!(dummy.as_ref().unwrap().date_of_birth, "69-08-XX");
    }

    #[test]
    fn test_parse_mrz_td3_invalid_check_digits() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C35UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert!(!dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_td1_invalid_check_digits() {
        let dummy = parse_mrz("I<UTOD231458908<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert!(!dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_td3_check_digits_per_field() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.unwrap().check_digits, vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, true),
            (CHECK_DIGIT_DATE_OF_BIRTH, true),
            (CHECK_DIGIT_EXPIRY_DATE, true),
            (CHECK_DIGIT_PERSONAL_NUMBER, true),
            (CHECK_DIGIT_COMPOSITE, true),
        ]);
    }

    #[test]
    fn test_parse_mrz_td3_check_digits_invalid_document_number() {
        // Wrong document-number check digit (5 instead of 6) also breaks the composite.
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C35UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.unwrap().check_digits, vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, false),
            (CHECK_DIGIT_DATE_OF_BIRTH, true),
            (CHECK_DIGIT_EXPIRY_DATE, true),
            (CHECK_DIGIT_PERSONAL_NUMBER, true),
            (CHECK_DIGIT_COMPOSITE, false),
        ]);
    }

    #[test]
    fn test_parse_mrz_td3_check_digits_without_personal_number() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159<<<<<<<<<<<<<<<8".to_string());
        let doc = dummy.unwrap();
        assert!(doc.valid_check_digits);
        assert!(!doc.check_digits.iter().any(|&(name, _)| name == CHECK_DIGIT_PERSONAL_NUMBER));
    }

    #[test]
    fn test_parse_mrz_td1_check_digits_invalid_document_number() {
        let dummy = parse_mrz("I<UTOD231458908<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        let doc = dummy.unwrap();
        let failed: Vec<_> = doc.check_digits.iter().filter(|&&(_, ok)| !ok).map(|&(name, _)| name).collect();
        assert_eq!(failed, vec![CHECK_DIGIT_DOCUMENT_NUMBER, CHECK_DIGIT_COMPOSITE]);
    }

    #[test]
    fn test_parse_mrz_mrv_b_check_digits_invalid_date_of_birth() {
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908062F9406236ZE184226".to_string());
        assert_eq!(dummy.unwrap().check_digits, vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, true),
            (CHECK_DIGIT_DATE_OF_BIRTH, false),
            (CHECK_DIGIT_EXPIRY_DATE, true),
        ]);
    }

    #[test]
    fn test_document_clone() {
        let doc = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string()).unwrap();
        assert_eq!(doc.clone(), doc);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_document_serialize() {
        let doc = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C35UTO7408122F1204159ZE184226B<<<<<10".to_string()).unwrap();
        let json = serde_json::to_value(&doc).unwrap();
        assert_eq!(json["check_digits"][0], serde_json::json!(["document_number", false]));
        assert_eq!(json["gender"], "Female");
        assert_eq!(json["expiry_date"], "2012-04-15");
    }

    #[test]
    fn test_parse_mrz_td3_invalid_characters() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert!(dummy.as_ref().unwrap().valid_characters);
    }

    #[test]
    fn test_parse_mrz_td3_harmonized_document_code() {
        // Doc 9303-4 Section 4.4: PD = Diplomatic passport. The composite check
        // digit does not cover positions 0-1, so the ICAO sample stays valid.
        let dummy = parse_mrz("PDUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_type, "PD");
        assert_eq!(dummy.as_ref().unwrap().document_type_name, "Diplomatic passport");
        assert!(dummy.as_ref().unwrap().harmonized_document_code);
        assert!(dummy.as_ref().unwrap().valid_document_type);
        assert!(dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_td3_non_harmonized_document_code() {
        // A second letter outside the Section 4.4 table still parses (documents
        // issued before 2028 remain valid until 2038), but is not harmonized.
        let dummy = parse_mrz("PXUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_type, "PX");
        assert_eq!(dummy.as_ref().unwrap().document_type_name, "");
        assert!(!dummy.as_ref().unwrap().harmonized_document_code);
        assert!(dummy.as_ref().unwrap().valid_document_type);
    }

    #[test]
    fn test_parse_mrz_td3_without_secondary_document_code() {
        let dummy = parse_mrz("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_type, "P");
        assert_eq!(dummy.as_ref().unwrap().document_type_name, "");
        assert!(!dummy.as_ref().unwrap().harmonized_document_code);
        assert!(dummy.as_ref().unwrap().valid_document_type);
    }

    #[test]
    fn test_parse_mrz_td1_document_code_not_harmonized() {
        // Section 4.4 covers MRPs only; ID cards are out of scope.
        let dummy = parse_mrz("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_type_name, "");
        assert!(!dummy.as_ref().unwrap().harmonized_document_code);
    }

    #[test]
    fn test_parse_mrz_td3_surname_only() {
        // Doc 9303-4 4.2.3.1 f): a holder with a single name gets it as primary
        // identifier; the given-name part stays empty. The name field is not part
        // of any check digit, so the sample's check digits remain valid.
        let dummy = parse_mrz("P<UTOARKFREITH<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().surname, "ARKFREITH");
        assert_eq!(dummy.as_ref().unwrap().given_name, "");
        assert!(dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_td3_single_name_full_field() {
        // Truncated single name filling the whole field: no `<<` separator present
        let dummy = parse_mrz("P<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLML898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().surname, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLM");
        assert_eq!(dummy.as_ref().unwrap().given_name, "");
    }

    #[test]
    fn test_parse_mrz_td3_multi_component_surname() {
        // Doc 9303-4 4.2.3.1 c): SMITH-JONES, SUSIE MARGARET
        let dummy = parse_mrz("P<UTOSMITH<JONES<<SUSIE<MARGARET<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string());
        assert_eq!(dummy.as_ref().unwrap().surname, "SMITH JONES");
        assert_eq!(dummy.as_ref().unwrap().given_name, "SUSIE MARGARET");
    }

    #[test]
    fn test_parse_mrz_td1_single_name_full_field() {
        let dummy = parse_mrz("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ABCDEFGHIJKLMNOPQRSTUVWXYZABCD".to_string());
        assert_eq!(dummy.as_ref().unwrap().surname, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCD");
        assert_eq!(dummy.as_ref().unwrap().given_name, "");
    }

    #[test]
    fn test_parse_mrz_mrv_a_full_optional_data() {
        // Doc 9303-7, 4.2.2.2: optional data spans positions 29-44 (16 characters)
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B1234567".to_string());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "ZE184226B1234567");
        assert!(dummy.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_parse_mrz_mrv_a_without_optional_data() {
        // 88 chars: positions 72-87 are all filler (no optional data)
        let dummy = parse_mrz("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3D<<6908061F9406236<<<<<<<<<<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().optional_data, "");
        assert_eq!(dummy.as_ref().unwrap().document_type, "V");
    }

    #[test]
    fn test_parse_mrz_td3_shape_at_td2_length_does_not_panic() {
        // 72-byte string that satisfies the TD3 structure regex. Must not be parsed
        // as TD3 (whose fields extend to byte 87) and must not panic.
        let s = "P<UTOERIKSSON<<ANNA<MARIA<XXL898902C36UTO7408122F1204159ZE184226B<<<<<10";
        assert_eq!(s.len(), 72);
        assert_eq!(parse_mrz(s.to_string()).unwrap_err(), Error::InvalidStructure);
    }

    #[test]
    fn test_parse_mrz_rejects_non_ascii() {
        // 88 bytes but contains a multibyte char -> must be rejected, not panic.
        let mut s = String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B");
        while s.len() < 86 { s.push('<'); }
        s.push('é'); // 2 bytes -> total 88 bytes
        assert_eq!(s.len(), 88);
        assert_eq!(parse_mrz(s).unwrap_err(), Error::InvalidStructure);
    }

    #[test]
    fn test_parse_mrz_td2_document_number_longer_than_nine() {
        // Overflow document number "D2314589071": field holds "D23145890", position 45
        // is a filler, remainder "71" + check digit continue in the optional-data field.
        let dummy = parse_mrz("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159712<<<<6".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_number, "D2314589071");
    }

    #[test]
    fn test_parse_mrz_td1_document_number_longer_than_nine() {
        let dummy = parse_mrz("I<UTOD23145890<712<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(dummy.as_ref().unwrap().document_number, "D2314589071");
    }

    #[test]
    fn test_parse_mrz_overflow_document_number_valid_check_digits() {
        // Fully valid overflow documents: the document-number check digit lives in the
        // optional-data field and must still validate.
        let td2 = parse_mrz("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159719<<<<8".to_string());
        assert_eq!(td2.as_ref().unwrap().document_number, "D2314589071");
        assert!(td2.as_ref().unwrap().valid_check_digits);

        let td1 = parse_mrz("I<UTOD23145890<719<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<2ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string());
        assert_eq!(td1.as_ref().unwrap().document_number, "D2314589071");
        assert!(td1.as_ref().unwrap().valid_check_digits);
    }

    #[test]
    fn test_verify_structure_td3_accepts_letter_doc_type_second_char() {
        // Second document-type character may be any A-Z (here 'B'), not only the
        // few letters the previous `[A..Z<]` typo allowed.
        let dummy = verify_structure_td3("PBUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10");
        assert!(dummy);
    }

    #[test]
    fn test_verify_structure_td1_accepts_letter_doc_type_second_char() {
        let dummy = verify_structure_td1("IBUTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<");
        assert!(dummy);
    }

    #[test]
    fn test_verify_structure_td2_accepts_letter_doc_type_second_char() {
        let dummy = verify_structure_td2("IBUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6");
        assert!(dummy);
    }

    #[test]
    fn test_verify_structure_mrv_a_accepts_letter_doc_type_second_char() {
        let dummy = verify_structure_mrv_a("VBUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<");
        assert!(dummy);
    }

    #[test]
    fn test_verify_structure_mrv_b_accepts_letter_doc_type_second_char() {
        let dummy = verify_structure_mrv_b("VBUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226");
        assert!(dummy);
    }

    #[test]
    fn test_verify_structure_td3_rejects_trailing_newline() {
        // 87 structural chars + '\n'. With the `\z` anchor this must not match.
        let s = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10\n";
        assert_eq!(s.len(), 88);
        assert!(!verify_structure_td3(s));
    }

}

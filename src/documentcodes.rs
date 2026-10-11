use phf::phf_map;

// Harmonized secondary document codes for MRPs per ICAO Doc 9303-4 Section 4.4
// (Amendment 2, effective 2026). Only applies to TD3 passports (first letter P);
// issuing states may still use other second letters on documents issued before
// 2028, which remain valid until 2038 — such codes are parseable but not harmonized.
static PASSPORT_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "PP" => "National/ordinary passport",
    "PE" => "Emergency passport",
    "PD" => "Diplomatic passport",
    "PO" => "Official/service passport",
    "PR" => "Refugee passport",
    "PT" => "Alien/Non-citizen passport",
    "PS" => "Stateless passport",
    "PL" => "Laissez-passer passport",
    "PM" => "Military passport",
    "PU" => "Single sheet travel document", // Note 1: per Doc 9303-8
};

pub fn is_document_code_harmonized(doc_type: &str) -> bool {
    PASSPORT_CODES.contains_key(doc_type)
}

pub fn get_document_type_name(doc_type: &str) -> String {
    match PASSPORT_CODES.get(doc_type) {
        Some(name) => name.to_string(),
        None => String::new(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_harmonized_codes_are_known() {
        for code in ["PP", "PE", "PD", "PO", "PR", "PT", "PS", "PL", "PM", "PU"] {
            assert!(is_document_code_harmonized(code));
        }
    }

    #[test]
    fn test_non_harmonized_code_px() {
        assert!(!is_document_code_harmonized("PX"));
    }

    #[test]
    fn test_plain_p_without_secondary_code() {
        assert!(!is_document_code_harmonized("P"));
    }

    #[test]
    fn test_non_passport_codes() {
        assert!(!is_document_code_harmonized("ID"));
        assert!(!is_document_code_harmonized("V"));
        assert!(!is_document_code_harmonized("AC"));
    }

    #[test]
    fn test_get_document_type_name_diplomatic() {
        assert_eq!(get_document_type_name("PD"), "Diplomatic passport");
    }

    #[test]
    fn test_get_document_type_name_unknown() {
        assert_eq!(get_document_type_name("PX"), "");
    }
}

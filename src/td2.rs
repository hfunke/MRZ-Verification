use crate::{utils::{check_digits_numeric, trim, validate_check_digit}, MachineReadableZone,
    CHECK_DIGIT_DOCUMENT_NUMBER, CHECK_DIGIT_DATE_OF_BIRTH, CHECK_DIGIT_EXPIRY_DATE, CHECK_DIGIT_COMPOSITE};

pub const TD2_LENGTH: usize = 72;

pub struct TD2 {
    pub(crate) raw_mrz: String,
}

impl TD2 {
    /// ICAO Doc 9303 long document number: when the check-digit position (index 45)
    /// holds a filler, the document number continues at the start of the optional-data
    /// field (index 64..), followed by its check digit and a filler. Returns the
    /// continuation characters and the absolute position of the check digit, or `None`
    /// in the normal (<= 9 chars) case.
    fn document_number_overflow(&self) -> Option<(String, usize)> {
        if self.raw_mrz.as_bytes()[45] != b'<' {
            return None;
        }
        let overflow = &self.raw_mrz[64..71];
        let end = overflow.find('<')?;
        if end == 0 {
            return None;
        }
        Some((overflow[..end - 1].to_string(), 64 + end - 1))
    }
}

impl MachineReadableZone for TD2 {

    fn get_raw_mrz(self) -> String {
        self.raw_mrz
    }

    impl_mrz_getters!(
        name:   (5, 36),
        nat:    (46, 49),
        dob:    (49, 55),
        sex:    (56, 57),
        doe:    (57, 63)
    );

    fn get_document_number(&self) -> String {
        let base = trim(&self.raw_mrz[36..45]);
        match self.document_number_overflow() {
            Some((extra, _)) => format!("{}{}", base, extra),
            None => base,
        }
    }

    fn get_optional_data(&self) -> String {
        trim(&self.raw_mrz[64..71])
    }

    fn are_check_digits_numeric(&self) -> bool {
        check_digits_numeric(&self.raw_mrz, &[45, 55, 63, 71])
    }

    fn check_digit_results(&self) -> Vec<(&'static str, bool)> {
        let mrz = &self.raw_mrz;
        // The document-number check digit is at the fixed position 45 normally, or
        // inside the optional-data field for an overflowing (>9 chars) document number.
        // It is computed over the whole document number in both cases.
        let doc_cd_pos = self.document_number_overflow().map_or(45, |(_, pos)| pos);
        let composite = format!("{}{}{}", &mrz[36..46], &mrz[49..56], &mrz[57..71]);
        vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, validate_check_digit(mrz, &self.get_document_number(), doc_cd_pos)),
            (CHECK_DIGIT_DATE_OF_BIRTH, validate_check_digit(mrz, &self.get_date_of_birth(), 55)),
            (CHECK_DIGIT_EXPIRY_DATE, validate_check_digit(mrz, &self.get_date_of_expiry(), 63)),
            (CHECK_DIGIT_COMPOSITE, validate_check_digit(mrz, &composite, 71)),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_check_digits_numeric() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_invalid_checksum() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890AUTO7408122F1204159<<<<<<<6") };
        let dummy = td2.are_check_digits_numeric();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_type_i() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_document_type();
        assert_eq!(dummy, "I");
    }

    #[test]
    fn test_get_issuing_state() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_issuing_state();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_surname() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_surname();
        assert_eq!(dummy, "ERIKSSON");
    }

    #[test]
    fn test_get_given_name() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_given_name();
        assert_eq!(dummy, "ANNA MARIA");
    }

    #[test]
    fn test_get_document_number() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_get_document_number_longer_than_nine() {
        // Document number "D2314589071" (11 chars): the 9-char field holds "D23145890",
        // position 45 is a filler, and the remainder "71" plus its check digit ("2")
        // continues at the start of the optional-data field.
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159712<<<<6") };
        let dummy = td2.get_document_number();
        assert_eq!(dummy, "D2314589071");
    }

    #[test]
    fn test_get_document_number_longer_than_nine_max_length() {
        // 14-char number: 9 in the field, "9071X" continuation, check digit "2", filler.
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F12041599071X2<6") };
        let dummy = td2.get_document_number();
        assert_eq!(dummy, "D231458909071X");
    }

    #[test]
    fn test_get_document_number_overflow_marker_without_continuation() {
        // Position 45 is a filler but the optional-data field starts with a filler too
        // (no continuation): fall back to the 9-char base field.
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_get_document_number_overflow_without_terminator() {
        // Position 45 is a filler and the optional-data field has no terminating filler:
        // no check-digit boundary can be found, so fall back to the base field.
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F120415912345676") };
        let dummy = td2.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_get_nationality() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_nationality();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_dob() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_date_of_birth();
        assert_eq!(dummy, "740812");
    }

    #[test]
    fn test_get_sex() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_sex();
        assert_eq!(dummy, "F");
    }

    #[test]
    fn test_get_sex_male() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122M1204159<<<<<<<6") };
        let dummy = td2.get_sex();
        assert_eq!(dummy, "M");
    }

    #[test]
    fn test_get_sex_unspecified() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122<1204159<<<<<<<6") };
        let dummy = td2.get_sex();
        assert_eq!(dummy, "<");
    }

    #[test]
    fn test_get_doe() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_date_of_expiry();
        assert_eq!(dummy, "120415");
    }

    #[test]
    fn test_get_optional_data() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.get_optional_data();
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_get_optional_data_with_content() {
        // TD2 optional data is [64..71] = 7 chars; position 71 is the composite check digit
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = td2.get_optional_data();
        assert_eq!(dummy, "ZE18422");
    }

    #[test]
    fn test_validity_of_checkdigits() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_overflow_document_number() {
        // Document number "D2314589071": check digit ("9") sits in the optional-data
        // field after the continuation "71", not at the filler position 45.
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159719<<<<8") };
        let dummy = td2.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_overflow_invalid_document_number_check_digit() {
        // Same as above but the overflow check digit is wrong ("8" instead of "9").
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D23145890<UTO7408122F1204159718<<<<8") };
        let dummy = td2.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_invalid_checkdigit_doc_no() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458908UTO7408122F1204159<<<<<<<6") };
        let dummy = td2.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_invalid_checkdigit_doe() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204150<<<<<<<6") };
        let dummy = td2.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_invalid_checkdigit_dob() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408123F1204159<<<<<<<6") };
        let dummy = td2.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_validity_of_checkdigits_invalid_checkdigit_composite() {
        let td2 = TD2{raw_mrz: String::from("I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<D231458907UTO7408122F1204159<<<<<<<7") };
        let dummy = td2.are_check_digits_valid();
        assert!(!dummy);
    }

}

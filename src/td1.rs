use crate::{utils::{check_digits_numeric, trim, validate_check_digit}, MachineReadableZone,
    CHECK_DIGIT_DOCUMENT_NUMBER, CHECK_DIGIT_DATE_OF_BIRTH, CHECK_DIGIT_EXPIRY_DATE, CHECK_DIGIT_COMPOSITE};

pub const TD1_LENGTH: usize = 90;

pub struct TD1 {
    pub raw_mrz: String,
}

impl TD1 {
    /// ICAO Doc 9303 long document number: when the check-digit position (index 14)
    /// holds a filler, the document number continues at the start of the optional-data
    /// field (index 15..), followed by its check digit and a filler. Returns the
    /// continuation characters and the absolute position of the check digit, or `None`
    /// in the normal (<= 9 chars) case.
    fn document_number_overflow(&self) -> Option<(String, usize)> {
        if self.raw_mrz.as_bytes()[14] != b'<' {
            return None;
        }
        let overflow = &self.raw_mrz[15..30];
        let end = overflow.find('<')?;
        if end == 0 {
            return None;
        }
        Some((overflow[..end - 1].to_string(), 15 + end - 1))
    }
}

impl MachineReadableZone for TD1 {

    fn get_raw_mrz(self) -> String {
        self.raw_mrz
    }

    impl_mrz_getters!(
        name:   (60, 90),
        nat:    (45, 48),
        dob:    (30, 36),
        sex:    (37, 38),
        doe:    (38, 44)
    );

    fn get_document_number(&self) -> String {
        let base = trim(&self.raw_mrz[5..14]);
        match self.document_number_overflow() {
            Some((extra, _)) => format!("{}{}", base, extra),
            None => base,
        }
    }

    fn get_optional_data(&self) -> String {
        // TD1 optional data spans two separate regions; each part must be trimmed before joining
        let part1 = trim(&self.raw_mrz[15..30]);
        let part2 = trim(&self.raw_mrz[48..59]);
        trim(&format!("{}{}", part1, part2))
    }

    fn are_check_digits_numeric(&self) -> bool {
        check_digits_numeric(&self.raw_mrz, &[14, 36, 44, 59])
    }

    fn check_digit_results(&self) -> Vec<(&'static str, bool)> {
        let mrz = &self.raw_mrz;
        // The document-number check digit is at the fixed position 14 normally, or
        // inside the optional-data field for an overflowing (>9 chars) document number.
        // It is computed over the whole document number in both cases.
        let doc_cd_pos = self.document_number_overflow().map_or(14, |(_, pos)| pos);
        let composite = format!("{}{}{}{}", &mrz[5..30], &mrz[30..37], &mrz[38..45], &mrz[48..59]);
        vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, validate_check_digit(mrz, &self.get_document_number(), doc_cd_pos)),
            (CHECK_DIGIT_DATE_OF_BIRTH, validate_check_digit(mrz, &self.get_date_of_birth(), 36)),
            (CHECK_DIGIT_EXPIRY_DATE, validate_check_digit(mrz, &self.get_date_of_expiry(), 44)),
            (CHECK_DIGIT_COMPOSITE, validate_check_digit(mrz, &composite, 59)),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_check_digits_numeric() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_invalid_checksum() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<740812AF1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_numeric();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_valid_overflow_document_number() {
        // Document number "D2314589071": check digit ("9") sits in the optional-data
        // field after the continuation "71", not at the filler position 14.
        let td1 = TD1{raw_mrz: String::from("I<UTOD23145890<719<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<2ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_valid_overflow_invalid_document_number_check_digit() {
        // Same as above but the overflow check digit is wrong ("8" instead of "9").
        let td1 = TD1{raw_mrz: String::from("I<UTOD23145890<718<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<2ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_number_overflow_marker_without_continuation() {
        // Position 14 is a filler but the optional-data field starts with a filler too
        // (no continuation): fall back to the 9-char base field.
        let td1 = TD1{raw_mrz: String::from("I<UTOD23145890<<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_get_document_number_overflow_without_terminator() {
        // Position 14 is a filler and the optional-data field has no terminating filler:
        // no check-digit boundary can be found, so fall back to the base field.
        let td1 = TD1{raw_mrz: String::from("I<UTOD23145890<1234567890123457408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_doc_no() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458908<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_doe() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204150UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_dob() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408123F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_composite() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<7ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_type() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_type();
        assert_eq!(dummy, "I");
    }

    #[test]
    fn test_get_document_type_two_chars() {
        let td1 = TD1{raw_mrz: String::from("IPUTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_type();
        assert_eq!(dummy, "IP");
    }

    #[test]
    fn test_get_issuing_state() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_issuing_state();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_document_number() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_number();
        assert_eq!(dummy, "D23145890");
    }

    #[test]
    fn test_get_optional_data_empty() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_optional_data();
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_get_optional_data_over_two_lines() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907AA<<<<<<<<<<<<<7408122F1204159UTOBB<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_optional_data();
        assert_eq!(dummy, "AABB");
    }

    #[test]
    fn test_get_dob() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_date_of_birth();
        assert_eq!(dummy, "740812");
    }

    #[test]
    fn test_get_sex() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_sex();
        assert_eq!(dummy, "F");
    }

    #[test]
    fn test_get_sex_male() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122M1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_sex();
        assert_eq!(dummy, "M");
    }

    #[test]
    fn test_get_sex_unspecified() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122<1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_sex();
        assert_eq!(dummy, "<");
    }

    #[test]
    fn test_get_doe() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_date_of_expiry();
        assert_eq!(dummy, "120415");
    }

    #[test]
    fn test_get_nationality() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_nationality();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_surname() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_surname();
        assert_eq!(dummy, "ERIKSSON");
    }

    #[test]
    fn test_get_given_name() {
        let td1 = TD1{raw_mrz: String::from("I<UTOD231458907<<<<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_given_name();
        assert_eq!(dummy, "ANNA MARIA");
    }

    #[test]
    fn test_get_document_number_longer_than_nine() {
        // Document number "D2314589071" (11 chars): the 9-char field holds "D23145890",
        // position 14 is a filler, and the remainder "71" plus its check digit ("2")
        // continues at the start of the optional-data field.
        let td1 = TD1{raw_mrz: String::from("I<UTOD23145890<712<<<<<<<<<<<<7408122F1204159UTO<<<<<<<<<<<6ERIKSSON<<ANNA<MARIA<<<<<<<<<<") };
        let dummy = td1.get_document_number();
        assert_eq!(dummy, "D2314589071");
    }

}

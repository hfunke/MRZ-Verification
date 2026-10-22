use crate::{utils::{check_digits_numeric, trim, validate_check_digit}, MachineReadableZone,
    CHECK_DIGIT_DOCUMENT_NUMBER, CHECK_DIGIT_DATE_OF_BIRTH, CHECK_DIGIT_EXPIRY_DATE};

pub const MRV_A_LENGTH: usize = 88;

pub struct MrvA {
    pub(crate) raw_mrz: String,
}

impl MachineReadableZone for MrvA {

    fn get_raw_mrz(self) -> String {
        self.raw_mrz
    }

    impl_mrz_getters!(
        name:   (5, 44),
        doc_no: (44, 53),
        nat:    (54, 57),
        dob:    (57, 63),
        sex:    (64, 65),
        doe:    (65, 71)
    );

    fn get_optional_data(&self) -> String {
        // Doc 9303-7, 4.2.2.2: positions 29-44 of the lower line (16 characters).
        // Unlike TD3 there is no optional-data or composite check digit at the end.
        trim(&self.raw_mrz[72..88])
    }

    fn are_check_digits_numeric(&self) -> bool {
        check_digits_numeric(&self.raw_mrz, &[53, 63, 71])
    }

    fn check_digit_results(&self) -> Vec<(&'static str, bool)> {
        let mrz = &self.raw_mrz;
        vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, validate_check_digit(mrz, &self.get_document_number(), 53)),
            (CHECK_DIGIT_DATE_OF_BIRTH, validate_check_digit(mrz, &self.get_date_of_birth(), 63)),
            (CHECK_DIGIT_EXPIRY_DATE, validate_check_digit(mrz, &self.get_date_of_expiry(), 71)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_check_digits_numeric_with_optional_data() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_invalid_checksum() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<AUTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_numeric();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_icao_sample_with_optional_data() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_doc_no() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<4UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_doe() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406237ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_dob() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908062F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_type_v() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_document_type();
        assert_eq!(dummy, "V");
    }

    #[test]
    fn test_get_document_type_va() {
        let mrv_a = MrvA{raw_mrz: String::from("VAUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_document_type();
        assert_eq!(dummy, "VA");
    }

    #[test]
    fn test_get_issuing_state_uto() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_issuing_state();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_surname() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_surname();
        assert_eq!(dummy, "ERIKSSON");
    }

    #[test]
    fn test_get_given_name() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_given_name();
        assert_eq!(dummy, "ANNA MARIA");
    }

    #[test]
    fn test_get_document_number() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_document_number();
        assert_eq!(dummy, "L898902C");
    }

    #[test]
    fn test_get_nationality() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_nationality();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_dob() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_date_of_birth();
        assert_eq!(dummy, "690806");
    }

    #[test]
    fn test_get_sex() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_sex();
        assert_eq!(dummy, "F");
    }

    #[test]
    fn test_get_sex_male() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061M9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_sex();
        assert_eq!(dummy, "M");
    }

    #[test]
    fn test_get_sex_unspecified() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061<9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_sex();
        assert_eq!(dummy, "<");
    }

    #[test]
    fn test_get_date_of_expiry() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_date_of_expiry();
        assert_eq!(dummy, "940623");
    }

    #[test]
    fn test_get_optional_data() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B<<<<<<<") };
        let dummy = mrv_a.get_optional_data();
        assert_eq!(dummy, "ZE184226B");
    }

    #[test]
    fn test_get_optional_data_full_sixteen_characters() {
        // Optional data occupies all positions 29-44; the last two characters
        // must not be cut off (they are not check digits, unlike in TD3).
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226B1234567") };
        let dummy = mrv_a.get_optional_data();
        assert_eq!(dummy, "ZE184226B1234567");
    }

    #[test]
    fn test_get_optional_data_empty() {
        let mrv_a = MrvA{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C<3UTO6908061F9406236<<<<<<<<<<<<<<<<") };
        let dummy = mrv_a.get_optional_data();
        assert_eq!(dummy, "");
    }

}

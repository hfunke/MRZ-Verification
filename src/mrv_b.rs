use crate::{utils::{check_digits_numeric, trim, validate_check_digit}, MachineReadableZone,
    CHECK_DIGIT_DOCUMENT_NUMBER, CHECK_DIGIT_DATE_OF_BIRTH, CHECK_DIGIT_EXPIRY_DATE};

pub const MRV_B_LENGTH: usize = 72;

pub struct MrvB {
    pub(crate) raw_mrz: String,
}

impl MachineReadableZone for MrvB {

    fn get_raw_mrz(self) -> String {
        self.raw_mrz
    }

    impl_mrz_getters!(
        name:   (5, 36),
        doc_no: (36, 45),
        nat:    (46, 49),
        dob:    (49, 55),
        sex:    (56, 57),
        doe:    (57, 63)
    );

    fn get_optional_data(&self) -> String {
        trim(&self.raw_mrz[64..72])
    }

    fn are_check_digits_numeric(&self) -> bool {
        check_digits_numeric(&self.raw_mrz, &[45, 55, 63])
    }

    fn check_digit_results(&self) -> Vec<(&'static str, bool)> {
        let mrz = &self.raw_mrz;
        vec![
            (CHECK_DIGIT_DOCUMENT_NUMBER, validate_check_digit(mrz, &self.get_document_number(), 45)),
            (CHECK_DIGIT_DATE_OF_BIRTH, validate_check_digit(mrz, &self.get_date_of_birth(), 55)),
            (CHECK_DIGIT_EXPIRY_DATE, validate_check_digit(mrz, &self.get_date_of_expiry(), 63)),
        ]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_check_digits_numeric() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_invalid_checksum() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO690806AF9406236ZE184226") };
        let dummy = mrv_b.are_check_digits_numeric();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_doc_no() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<4UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_doe() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406237ZE184226") };
        let dummy = mrv_b.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_are_check_digits_valid_invalid_checksum_dob() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908062F9406236ZE184226") };
        let dummy = mrv_b.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_type_v() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_document_type();
        assert_eq!(dummy, "V");
    }

    #[test]
    fn test_get_issuing_state() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_issuing_state();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_surname() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_surname();
        assert_eq!(dummy, "ERIKSSON");
    }

    #[test]
    fn test_get_given_name() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_given_name();
        assert_eq!(dummy, "ANNA MARIA");
    }

    #[test]
    fn test_get_document_number() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_document_number();
        assert_eq!(dummy, "L898902C");
    }

    #[test]
    fn test_get_nationality() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_nationality();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_dob() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_date_of_birth();
        assert_eq!(dummy, "690806");
    }

    #[test]
    fn test_get_sex() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_sex();
        assert_eq!(dummy, "F");
    }

    #[test]
    fn test_get_sex_male() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061M9406236ZE184226") };
        let dummy = mrv_b.get_sex();
        assert_eq!(dummy, "M");
    }

    #[test]
    fn test_get_sex_unspecified() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061<9406236ZE184226") };
        let dummy = mrv_b.get_sex();
        assert_eq!(dummy, "<");
    }

    #[test]
    fn test_get_doe() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_date_of_expiry();
        assert_eq!(dummy, "940623");
    }

    #[test]
    fn test_get_optional_data() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.get_optional_data();
        assert_eq!(dummy, "ZE184226");
    }

    #[test]
    fn test_get_optional_data_empty() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236<<<<<<<<") };
        let dummy = mrv_b.get_optional_data();
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_validity_of_checkdigits() {
        let mrv_b = MrvB{raw_mrz: String::from("V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<L898902C<3UTO6908061F9406236ZE184226") };
        let dummy = mrv_b.are_check_digits_valid();
        assert!(dummy);
    }

}

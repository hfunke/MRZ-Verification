use crate::{utils::{check_digits_numeric, trim, validate_check_digit}, MachineReadableZone};

pub const TD3_LENGTH: usize = 88;

pub struct TD3 {
    pub(crate) raw_mrz: String,
}

impl MachineReadableZone for TD3 {

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
        trim(&self.raw_mrz[72..86])
    }

    fn are_check_digits_numeric(&self) -> bool {
        check_digits_numeric(&self.raw_mrz, &[53, 63, 71, 86, 87])
    }

    fn are_check_digits_valid(&self) -> bool {
        let mrz = &self.raw_mrz;
        if !validate_check_digit(mrz, &self.get_document_number(), 53) { return false; }
        if !validate_check_digit(mrz, &self.get_date_of_birth(), 63) { return false; }
        if !validate_check_digit(mrz, &self.get_date_of_expiry(), 71) { return false; }
        let opt = self.get_optional_data();
        if !opt.is_empty() && !validate_check_digit(mrz, &opt, 86) { return false; }
        let composite = format!("{}{}{}{}", &mrz[44..54], &mrz[57..64], &mrz[65..72], &mrz[72..87]);
        validate_check_digit(mrz, &composite, 87)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_check_digits_numeric_td3_without_optional_data() {
        let td3 = TD3{raw_mrz: String::from("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122F2310314<<<<<<<<<<<<<<<4") };
        let dummy = td3.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_td3_with_optional_data() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.are_check_digits_numeric();
        assert!(dummy);
    }

    #[test]
    fn test_are_check_digits_numeric_td3_invalid_checksum() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<1A") };
        let dummy = td3.are_check_digits_numeric();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_icao_sample_with_optional_data() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_check_digits_validity_icao_sample_without_optional_data() {
        let td3 = TD3{raw_mrz: String::from("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122F2310314<<<<<<<<<<<<<<<4") };
        let dummy = td3.are_check_digits_valid();
        assert!(dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_doc_no() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C35UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_doe() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204158ZE184226B<<<<<10") };
        let dummy = td3.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_dob() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408123F1204159ZE184226B<<<<<10") };
        let dummy = td3.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_optional_data() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<20") };
        let dummy = td3.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_check_digits_validity_invalid_checksum_composite() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<11") };
        let dummy = td3.are_check_digits_valid();
        assert!(!dummy);
    }

    #[test]
    fn test_get_document_type_p() {
        let td3 = TD3{raw_mrz: String::from("P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_document_type();
        assert_eq!(dummy, "P");
    }

    #[test]
    fn test_get_document_type_pa() {
        let td3 = TD3{raw_mrz: String::from("PAUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_document_type();
        assert_eq!(dummy, "PA");
    }

    #[test]
    fn test_get_issuing_state_uto() {
        let td3 = TD3{raw_mrz: String::from("PAUTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_issuing_state();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_issuing_state_d() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_issuing_state();
        assert_eq!(dummy, "D");
    }

    #[test]
    fn test_get_surname() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_surname();
        assert_eq!(dummy, "ERIKSSON");
    }

    #[test]
    fn test_get_given_name() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_given_name();
        assert_eq!(dummy, "ANNA MARIA");
    }

    #[test]
    fn test_get_given_name_long() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_given_name();
        assert_eq!(dummy, "ANNA MARIA WILHEMINA CHARLOTT");
    }

    #[test]
    fn test_get_document_number() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_document_number();
        assert_eq!(dummy, "L898902C3");
    }

    #[test]
    fn test_get_nationality() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_nationality();
        assert_eq!(dummy, "UTO");
    }

    #[test]
    fn test_get_dob() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_date_of_birth();
        assert_eq!(dummy, "740812");
    }

    #[test]
    fn test_get_sex() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_sex();
        assert_eq!(dummy, "F");
    }

    #[test]
    fn test_get_sex_male() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122M1204159ZE184226B<<<<<10") };
        let dummy = td3.get_sex();
        assert_eq!(dummy, "M");
    }

    #[test]
    fn test_get_sex_unspecified() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122<1204159ZE184226B<<<<<10") };
        let dummy = td3.get_sex();
        assert_eq!(dummy, "<");
    }

    #[test]
    fn test_get_date_of_expiry() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_date_of_expiry();
        assert_eq!(dummy, "120415");
    }

    #[test]
    fn test_get_optional_data() {
        let td3 = TD3{raw_mrz: String::from("PAD<<ERIKSSON<<ANNA<MARIA<WILHEMINA<CHARLOTTL898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_optional_data();
        assert_eq!(dummy, "ZE184226B");
    }

    #[test]
    fn test_get_surname_single_name_without_separator() {
        // Name field without `<<`: the whole field is the primary identifier
        let td3 = TD3{raw_mrz: String::from("P<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLML898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_surname();
        assert_eq!(dummy, "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLM");
    }

    #[test]
    fn test_get_given_name_single_name_without_separator() {
        let td3 = TD3{raw_mrz: String::from("P<UTOABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLML898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_given_name();
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_get_surname_only_padded() {
        let td3 = TD3{raw_mrz: String::from("P<UTOARKFREITH<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_surname();
        assert_eq!(dummy, "ARKFREITH");
    }

    #[test]
    fn test_get_given_name_surname_only_padded() {
        let td3 = TD3{raw_mrz: String::from("P<UTOARKFREITH<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<L898902C36UTO7408122F1204159ZE184226B<<<<<10") };
        let dummy = td3.get_given_name();
        assert_eq!(dummy, "");
    }

    #[test]
    fn test_get_optional_data_empty() {
        let td3 = TD3{raw_mrz: String::from("P<D<<MUSTERMANN<<ERIKA<<<<<<<<<<<<<<<<<<<<<<C11T002JM4D<<9608122F2310314<<<<<<<<<<<<<<<4") };
        let dummy = td3.get_optional_data();
        assert_eq!(dummy, "");
    }

}


const FILLER: char = '<';
const MRZ_WEIGHTS: [u32; 3] = [7, 3, 1];

fn get_character_value(c: char) -> u32 {
    if c == FILLER {
        return 0;
    }
    if c.is_ascii_digit() {
        let digit = std::char::from_u32(c as u32 - 48).unwrap_or(c);
        return digit as u32;
    }
    if c.is_ascii_uppercase() {
        
        let digit = std::char::from_u32(c as u32 - 55).unwrap_or(c);
        digit as u32
        
    } else {
        0//ToDo: Add correct handling for undefined characters
    }
}

pub fn is_check_digit_numeric(c: char) -> bool {
    if c == FILLER {
        return true;
    }
    c.is_ascii_digit()

}

pub fn is_valid_char(c: char) -> bool {
    (c == FILLER) || c.is_ascii_digit() || c.is_ascii_uppercase()
}

pub fn compute_check_digit(s: &str) -> u32 {
    let mut result = 0;
    for (i, c) in s.chars().enumerate() {
        result += get_character_value(c) * MRZ_WEIGHTS[i % MRZ_WEIGHTS.len()];
    }
    result % 10
}

pub fn trim(s: &str) -> String {
    s.replace('<', " ").trim().to_string()
}

pub fn validate_check_digit(mrz: &str, data: &str, cd_pos: usize) -> bool {
    match mrz.get(cd_pos..cd_pos + 1).and_then(|s| s.parse::<u32>().ok()) {
        Some(expected) => compute_check_digit(data) == expected,
        None => false,
    }
}

pub fn check_digits_numeric(mrz: &str, positions: &[usize]) -> bool {
    positions.iter().all(|&pos| {
        mrz.chars().nth(pos).is_some_and(is_check_digit_numeric)
    })
}

#[test]
fn test_is_check_digit_numeric_0() {
    let dummy = is_check_digit_numeric('0');
    assert!(dummy);
}   

#[test]
fn test_is_check_digit_numeric_9() {
    let dummy = is_check_digit_numeric('9');
    assert!(dummy);
}   

#[test]
fn test_is_check_digit_numeric_lt() {
    let dummy = is_check_digit_numeric('<');
    assert!(dummy);
}   

#[test]
fn test_is_check_digit_numeric_a() {
    let dummy = is_check_digit_numeric('A');
    assert!(!dummy);
}  

#[test]
fn test_char_value_a() {
    let dummy = get_character_value('A');
    assert_eq!(dummy, 10);
}   

#[test]
fn test_char_value_b() {
    let dummy = get_character_value('B');
    assert_eq!(dummy, 11);
}   

#[test]
fn test_char_value_z() {
    let dummy = get_character_value('Z');
    assert_eq!(dummy, 35);
}   

#[test]
fn test_char_value_lt() {
    let dummy = get_character_value('<');
    assert_eq!(dummy, 0);
}   

#[test]
fn test_char_value_1() {
    let dummy = get_character_value('1');
    assert_eq!(dummy, 1);
}   

#[test]
fn test_char_value_0() {
    let dummy = get_character_value('0');
    assert_eq!(dummy, 0);
}   

#[test]
fn test_char_value_9() {
    let dummy = get_character_value('9');
    assert_eq!(dummy, 9);
}   

#[test]
fn test_char_value_bt() {
    let dummy = get_character_value('>');
    assert_eq!(dummy, 0);
}   

#[test]
fn test_checkdigit_520727() {
    let dummy = compute_check_digit("520727");
    assert_eq!(dummy, 3);
}   


#[test]
fn test_checkdigit_ab2134() {
    let dummy = compute_check_digit("AB2134");
    assert_eq!(dummy, 5);
}   

#[test]
fn test_checkdigit_0() {
    let dummy = compute_check_digit("<<<<<<<<0");
    assert_eq!(dummy, 0);
}   

#[test]
fn test_checkdigit_incomplete_date() {
    let dummy = compute_check_digit("9608<<");
    assert_eq!(dummy, 7);
}   

#[test]
fn test_checkdigit_td2_2nd_line() {
    let dummy = compute_check_digit("HA672242<658022549601086<<<<<<<");
    assert_eq!(dummy, 8);
}   



#[test]
fn test_checkdigit_icao_sample() {
    let dummy = compute_check_digit("D231458907<<<<<<<<<<<<<<<74081221204159<<<<<<<<<<<");
    assert_eq!(dummy, 6);
}   


#[test]
fn test_checkdigit_icao_sample_with_optional_data() {
    let dummy = compute_check_digit("L898902C3674081221204159ZE184226B<<<<<1");
    assert_eq!(dummy, 0);
}   


#[test]
fn test_isvalid() {
    let dummy = is_valid_char('A');
    assert!(dummy);
}   

#[test]
fn test_isvalid_lt() {
    let dummy = is_valid_char('<');
    assert!(dummy);
}   

#[test]
fn test_isvalid_lower_a() {
    let dummy = is_valid_char('a');
    assert!(!dummy);
}   

#[test]
fn test_isvalid_bt() {
    let dummy = is_valid_char('>');
    assert!(!dummy);
}   

#[test]
fn test_trim_p_lt() {
    let dummy = trim("P<");
    assert_eq!(dummy, "P");
}   

#[test]
fn test_trim_pa() {
    let dummy = trim("PA");
    assert_eq!(dummy, "PA");
}   

#[test]
fn test_trim_empty() {
    let dummy = trim("");
    assert_eq!(dummy, "");
}   

#[test]
fn test_trim_name() {
    let dummy = trim("ANNA<MARIA<<<<<<<<<<<<<<<<<<<");
    assert_eq!(dummy, "ANNA MARIA");
}

#[test]
fn test_isvalid_digit_0() {
    let dummy = is_valid_char('0');
    assert!(dummy);
}

#[test]
fn test_isvalid_digit_9() {
    let dummy = is_valid_char('9');
    assert!(dummy);
}

#[test]
fn test_char_value_lowercase_a() {
    let dummy = get_character_value('a');
    assert_eq!(dummy, 0);
}

#[test]
fn test_char_value_space() {
    let dummy = get_character_value(' ');
    assert_eq!(dummy, 0);
}

#[test]
fn test_validate_check_digit_valid() {
    // compute_check_digit("740812") == 2, and mrz[6] == '2'
    assert!(validate_check_digit("7408122", "740812", 6));
}

#[test]
fn test_validate_check_digit_mismatch() {
    assert!(!validate_check_digit("7408123", "740812", 6));
}

#[test]
fn test_validate_check_digit_position_out_of_bounds() {
    // cd_pos beyond the string -> get() returns None -> false
    assert!(!validate_check_digit("740812", "740812", 99));
}

#[test]
fn test_validate_check_digit_non_numeric_position() {
    // non-digit at cd_pos -> parse::<u32>() fails -> None -> false
    assert!(!validate_check_digit("740812X", "740812", 6));
}

#[test]
fn test_check_digits_numeric_all_numeric() {
    // '<' counts as numeric-valid for a check-digit slot
    assert!(check_digits_numeric("12<45", &[0, 1, 2, 3, 4]));
}

#[test]
fn test_check_digits_numeric_non_numeric() {
    assert!(!check_digits_numeric("12A45", &[2]));
}

#[test]
fn test_check_digits_numeric_position_out_of_bounds() {
    // position beyond the string -> nth() returns None -> false
    assert!(!check_digits_numeric("123", &[10]));
}
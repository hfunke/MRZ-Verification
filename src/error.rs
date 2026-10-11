use std::fmt;

// The `Invalid` prefix is deliberate: variants read as `Error::InvalidLength`
// at call sites and match the messages returned to library consumers.
#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidStructure,
    InvalidLength,
    InvalidDateOfBirth,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        let message = match self {
            InvalidStructure => "Invalid structure of MRZ",
            InvalidLength => "Invalid length of MRZ",
            InvalidDateOfBirth => "Invalid date of birth",
        };
        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_invalid_structure() {
        assert_eq!(Error::InvalidStructure.to_string(), "Invalid structure of MRZ");
    }

    #[test]
    fn test_display_invalid_length() {
        assert_eq!(Error::InvalidLength.to_string(), "Invalid length of MRZ");
    }

    #[test]
    fn test_display_invalid_date_of_birth() {
        assert_eq!(Error::InvalidDateOfBirth.to_string(), "Invalid date of birth");
    }
}
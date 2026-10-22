[![Rust](https://github.com/hfunke/MRZ/actions/workflows/rust.yml/badge.svg)](https://github.com/hfunke/MRZ-Verification/actions/workflows/rust.yml)

# ICAO MRZ Verification

A Rust library for parsing and validating the **Machine Readable Zone (MRZ)** of
electronic Machine Readable Travel Documents (eMRTDs) as specified by
**ICAO Doc 9303**.

Given the raw MRZ string of a passport, ID card or visa, the library detects the
document format, extracts every field, and reports the outcome of a series of
validity checks in a single `Document` structure.

## Supported formats

| Format | Layout        | Length | Typical document                     |
|--------|---------------|--------|--------------------------------------|
| TD1    | 3 lines × 30  | 90     | ID cards / residence permits         |
| TD2    | 2 lines × 36  | 72     | ID cards, older travel documents     |
| TD3    | 2 lines × 44  | 88     | Passports                            |
| MRV-A  | 2 lines × 44  | 88     | Visas (type A)                       |
| MRV-B  | 2 lines × 36  | 72     | Visas (type B)                       |

The MRZ must be passed as one continuous string with the line-break characters
removed (i.e. the lines concatenated).

## How it works

The single entry point is:

```rust
pub fn parse_mrz(str: String) -> Result<Document, Error>
```

Parsing proceeds in the following stages:

1. **Length check** — the input must have one of the five known lengths (72, 88 or
   90). Otherwise `Err(Error::InvalidLength)` is returned.
2. **Character guard** — the MRZ is required to be ASCII; every field is later read
   at fixed byte offsets, so this keeps those offsets on character boundaries.
3. **Format detection** — a regular expression per format validates the structure.
   Detection is dispatched by length, because lengths overlap (88 = TD3 or MRV-A,
   72 = TD2 or MRV-B). If no structure matches, `Err(Error::InvalidStructure)` is
   returned.
4. **Field extraction** — the matching format reads each field from its fixed
   position in the raw MRZ and assembles a `Document`.

Alongside extraction, the library performs the ICAO checks: **check-digit
verification** (using the standard 7-3-1 weighting), **character validity**,
**document-type validity**, and **country-code lookup** against ISO 3166-1 and the
ICAO Doc 9303 code tables.

### Validation philosophy

The distinction between the two failure modes is deliberate:

- **Structural problems** (wrong length, malformed layout) make the MRZ
  unparseable and return an `Err`.
- **Semantic problems** (a bad check digit, an unknown country code, an invalid
  character, an unrecognised document type) still return `Ok(Document)`, with the
  corresponding boolean flag set to `false`.

This lets a caller inspect a syntactically well-formed but partially invalid MRZ
rather than getting nothing back.

### Special cases handled

- **Incomplete dates of birth.** ICAO permits partially unknown birth dates.
  Unknown parts are rendered with `X`, e.g. `"74-08-XX"`. `date_of_birth` is
  therefore returned as a formatted `String`, while `expiry_date` is a
  `Result<NaiveDate, ParseError>`.
- **Long document numbers (> 9 characters).** For TD1 and TD2, a document number
  that exceeds the 9-character field overflows into the optional-data field: the
  check-digit position holds a filler `<`, and the remaining characters plus the
  document-number check digit follow at the start of the optional data. Both
  extraction and check-digit validation account for this.
- **Missing given names / optional data**, and the unspecified sex code `<`
  (mapped to `Gender::Other`).

## The `Document` result

```rust
pub struct Document {
    pub numeric_check_digits: bool,          // check-digit positions are numeric
    pub valid_check_digits: bool,            // all check digits verify
    pub check_digits: Vec<(&'static str, bool)>, // result per check digit
    pub valid_issuing_country_code: bool,
    pub valid_nationality_country_code: bool,
    pub issuing_country_code_type: String,   // "ISO/IEC 3166-1" or "ICAO Doc9303"
    pub nationality_country_code_type: String,
    pub valid_characters: bool,              // only A-Z, 0-9 and '<'
    pub valid_document_type: bool,
    pub document_type: String,
    pub issuing_country_code: String,
    pub issuing_country_name: String,
    pub surname: String,
    pub given_name: String,
    pub document_number: String,
    pub nationality_code: String,
    pub nationality_name: String,
    pub date_of_birth: String,               // "yy-mm-dd", unknown parts as "XX"
    pub gender: Gender,                       // Female | Male | Other
    pub expiry_date: Result<NaiveDate, ParseError>,
    pub optional_data: String,
}
```

`check_digits` lists each check digit the format carries, in MRZ order, so a
caller can tell which field failed. The names are exported as constants:

| Constant | Name | Present in |
|---|---|---|
| `CHECK_DIGIT_DOCUMENT_NUMBER` | `document_number` | all formats |
| `CHECK_DIGIT_DATE_OF_BIRTH` | `date_of_birth` | all formats |
| `CHECK_DIGIT_EXPIRY_DATE` | `expiry_date` | all formats |
| `CHECK_DIGIT_PERSONAL_NUMBER` | `personal_number` | TD3, only if the field is not empty |
| `CHECK_DIGIT_COMPOSITE` | `composite` | TD1, TD2, TD3 |

`Document` and `Gender` implement `Clone`. With the optional `serde` feature
(`mrz_verifier = { version = "...", features = ["serde"] }`) both also implement
`serde::Serialize`. `check_digits` is serialized as `[["document_number", true], ...]`,
and `expiry_date` as `"YYYY-MM-DD"`, or `null` if it could not be parsed.

On failure, `parse_mrz` returns one of:

```rust
pub enum Error {
    InvalidStructure,
    InvalidLength,
    InvalidDateOfBirth,
}
```

## Example

```rust
let mrz = "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<\
           L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string();

match parse_mrz(mrz) {
    Ok(doc) => {
        assert_eq!(doc.document_type, "P");
        assert_eq!(doc.issuing_country_name, "Utopia");
        assert_eq!(doc.surname, "ERIKSSON");
        assert_eq!(doc.given_name, "ANNA MARIA");
        assert_eq!(doc.document_number, "L898902C3");
        assert_eq!(doc.date_of_birth, "74-08-12");
        assert_eq!(doc.gender, Gender::Female);
        assert!(doc.valid_check_digits);
    }
    Err(e) => eprintln!("could not parse MRZ: {}", e),
}
```

## Build and test

The crate is built as a `cdylib`.

```sh
cargo build              # build the library
cargo test               # run the full test suite
cargo test td1::         # run one module's tests
cargo test test_parse_mrz_td3   # run a single test
```

## Dependencies

- [`regex`](https://crates.io/crates/regex) — structural validation of each format
- [`phf`](https://crates.io/crates/phf) — compile-time country-code tables
- [`chrono`](https://crates.io/crates/chrono) — date parsing for the expiry date

## Standards

- [ICAO Doc 9303 — Machine Readable Travel Documents, edition 8](https://www.icao.int/publications/doc-series/doc-9303)
- [ISO/IEC 3166-1 — country codes](https://www.iso.org/iso-3166-country-codes.html)

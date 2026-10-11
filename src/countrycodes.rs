use phf::phf_map;

pub(crate) const COUNTRY_TYPE_ISO: &str = "ISO/IEC 3166-1";
pub(crate) const COUNTRY_TYPE_ICAO: &str = "ICAO Doc9303";


static COUNTRIES_ICAO: phf::Map<&'static str, &'static str> = phf_map! {
   
	"D" => "Germany",
    "GBD" => "British Overseas Territories Citizen",
    "GBN" => "British National (Overseas)",
    "GBO" => "British Overseas Citizen",
    "GBS" => "British Subject", 
    "GBP" => "British Protected Person",
    "RKS" => "Republic of Kosovo",
    "EUE" => "European Union (EU)",
    "UNO" => "United Nations Organization or one of its officials",
    "UNA" => "United Nations specialized agency or one of its officials",
    "UNK" => "Resident of Kosovo to whom a travel document has been issued by the United Nations Interim Administration Mission in Kosovo(UNMIK)",
    "XBA" => "African Development Bank (ADB)",
    "XIM" => "African Export-Import Bank (AFREXIM bank)",
    "XCC" => "Caribbean Community or one of its emissaries (CARICOM)",
    "XCE" => "Council of Europe", 
    "XCO" => "Common Market for Eastern and Southern Africa (COMESA)",
    "XEC" => "Economic Community of West African States (ECOWAS)",
    "XPO" => "International Criminal Police Organization (INTERPOL)",
    "XES" => "Organization of Eastern Caribbean States (OECS)",
    "XOM" => "Sovereign Military Order of Malta or one of its emissaries ",
    "XDC" => "Southern African Development Community",
    "XXA" => "Stateless person, as defined in article 1 of the 1954 convention relating to the status of stateless persons",
    "XXB" => "Refugee, as defined in article 1 of the 1951 convention relating to the status of refugees as amended by the 1967 protocol", 
    "XXC" => "Refugee, other than as defined under the code XXB",
    "XXX" => "Person of unspecified nationality",
    "ANT" => "Netherlands Antilles", // deprecated!
    "NTZ" => "Neutral Zone", // deprecated!
    "UTO" => "Utopia", // only for specimen!
}; 

static COUNTRIES_ISO: phf::Map<&'static str, &'static str> = phf_map! {
    "ABW" => "Aruba",    
    "AFG" => "Afghanistan",
    "AGO" => "Angola",
	"AIA" => "Anguilla",
	"ALA" => "Aland Islands",
    "ALB" => "Albania",
    "ALG" => "Algeria",
	"AND" => "Andorra",
	"ARE" => "United Arab Emirates",
	"ARG" => "Argentina",
	"ARM" => "Armenia",
	"ASM" => "American Samoa",
	"ATA" => "Antarctica",
	"ATF" => "French Southern Territories",
	"ATG" => "Antigua and Barbuda",
	"AUS" => "Australia",
	"AUT" => "Austria",
	"AZE" => "Azerbaijan",
	"BDI" => "Burundi",
	"BEL" => "Belgium",
	"BEN" => "Benin",
	"BES" => "Bonaire, Sint Eustatius and Saba",
	"BFA" => "Burkina Faso",
	"BGD" => "Bangladesh",
	"BGR" => "Bulgaria",
	"BHR" => "Bahrain",
	"BHS" => "Bahamas",
	"BIH" => "Bosnia and Herzegovina",
	"BLM" => "Saint Bartelemy",
	"BLR" => "Belarus",
	"BLZ" => "Belize",
	"BMU" => "Bermuda",
	"BOL" => "Bolivia",
	"BRA" => "Brazil",
	"BRB" => "Barbados",
	"BRN" => "Brunei Darussalam",
	"BTN" => "Bhutan",
	"BVT" => "Bouvet Island",
	"BWA" => "Botswana",
	"CAF" => "Central African Republic",
	"CAN" => "Canada",
	"CCK" => "Cocos Island",
	"CHE" => "Switzerland",
	"CHL" => "Chile",
	"CHN" => "China",
	"CIV" => "Cote d'Ivoire",
	"CMR" => "Cameroon",
	"COD" => "Congo (Democratic Republic)",
	"COG" => "Congo",
	"COK" => "Cook Islands",
	"COL" => "Colombia",
	"COM" => "Comores",
	"CPV" => "Cabo Verde",
	"CRI" => "Costa Rica",
	"CUB" => "Cuba",
	"CUW" => "Curacao",
	"CXR" => "Christmas Islands",
	"CYM" => "Cayman Islands",
	"CYP" => "Cyprus",
	"CZE" => "Czechia",
	"DEU" => "Germany",
	"DJI" => "Djibouti",
	"DMA" => "Dominica",
	"DNK" => "Denmark",
	"DOM" => "Dominican Republic",
	"DZA" => "Algeria",
	"ECU" => "Ecuador",
	"EGY" => "Egypt",
	"ERI" => "Eritrea",
	"ESH" => "Western Sahara",
	"ESP" => "Spain",
	"EST" => "Estonia",
	"ETH" => "Ethiopia",
	"FIN" => "Finland",
	"FJI" => "Fiji",
	"FLK" => "Falkland Islands",
	"FRA" => "France",
	"FRO" => "Faroe Islands",
	"FSM" => "Macronesia",
	"GAB" => "Gabon",
	"GBR" => "Great Britain",
	"GEO" => "Georgia",
	"GGY" => "Guernsey",
	"GHA" => "Ghana",
	"GIB" => "Gibraltar",
	"GIN" => "Guinea",
	"GLP" => "Guadeloupe",
	"GMB" => "Gambia",
	"GNB" => "Guinea-Bissau",
	"GNQ" => "Equatorial Guinea",
	"GRC" => "Greece",
	"GRD" => "Grenada",
	"GRL" => "Greenland",
	"GTM" => "Guatemala",
	"GUF" => "French Guinea",
	"GUM" => "Guam",
	"GUY" => "Guyana",
	"HKG" => "Hong Kong",
	"HMD" => "Heard Islands and McDonalds Islands",
	"HND" => "Honduras",
	"HRV" => "Croatia",
	"HTI" => "Haiti",
	"HUN" => "Hungary",
	"IDN" => "Indonesia",
	"IMN" => "Isle of Man",
	"IND" => "India",
	"IOT" => "British Indian Ocean Territory",
	"IRL" => "Ireland",
	"IRN" => "Iran",
	"IRQ" => "Iraq",
	"ISL" => "Iceland",
	"ISR" => "Israel",
	"ITA" => "Italy",
	"JAM" => "Jamaice",
	"JEY" => "Jersey",
	"JOR" => "Jordan",
	"JPN" => "Japan",
	"KAZ" => "Kazakhstan",
	"KEN" => "Kenya",
	"KGZ" => "Kyrgyzstan",
	"KHM" => "Cambodia",
	"KIR" => "Kiribati",
	"KNA" => "Saint Kitts and Nevis",
	"KOR" => "Korea",
	"KWT" => "Kuwait",
	"LAO" => "Lao",
	"LBN" => "Lebanon",
	"LBR" => "Liberia",
	"LBY" => "Lybia",
	"LCA" => "Saint Lucia",
	"LIE" => "Liechtenstein",
	"LKA" => "Sri Lanka",
	"LSO" => "Lesotho",
	"LTU" => "Lithuania",
	"LUX" => "Luxemburg",
	"LVA" => "Latvia",
	"MAC" => "Macao",
	"MAF" => "Saint Martin",
	"MAR" => "Marocco",
	"MCO" => "Monaco",
	"MDA" => "Moldova",
	"MDG" => "Madagascar",
	"MDV" => "Maldives",
	"MEX" => "Mexico",
	"MHL" => "Marshall Islands",
	"MKD" => "North Macedonia",
	"MLI" => "Mali",
	"MLT" => "Malta",
	"MMR" => "Myanmar",
	"MNE" => "Montenegro",
	"MNG" => "Mongolia",
	"MNP" => "Northern Mariana Islands",
	"MOZ" => "Mozambique",
	"MRT" => "Mauritania",
	"MSR" => "Montserrat",
	"MTQ" => "Martinique",
	"MUS" => "Mauritius",
	"MWI" => "Malawi",
	"MYS" => "Malaysia",
	"MYT" => "Mayotte",
	"NAM" => "Namibia",
	"NCL" => "New Caledonia",
	"NER" => "Niger",
	"NFK" => "Norfolk Islands",
	"NGA" => "Nigeria",
	"NIC" => "Nicaragua",
	"NIU" => "Niue",
	"NLD" => "Netherlands",
	"NOR" => "Norway",
	"NPL" => "Nepal",
	"NRU" => "Nauru",
	"NZL" => "New Zealand",
	"OMN" => "Oman",
	"PAK" => "Pakistan",
	"PAN" => "Panama",
	"PCN" => "Pitcaim",
	"PER" => "Peru",
	"PHL" => "Philippines",
	"PLW" => "Palau",
	"PNG" => "Papua New Guinea",
	"POL" => "Poland",
	"PRI" => "Puerto Rico",
	"PRK" => "Korea",
	"PRT" => "Portgual",
	"PRY" => "Paraguay",
	"PSE" => "Palestine",
	"PYF" => "French Polynesia",
	"QAT" => "Qatar",
	"REU" => "Reunion",
	"ROU" => "Romania",
	"RUS" => "Russian Federation",
	"RWA" => "Rwanda",
	"SAU" => "Saudi Arabia",
	"SDN" => "Sudan",
	"SEN" => "Senegal",
	"SGP" => "Singapore",
	"SGS" => "Sourth Georgia and the South Sandwich Islands",
	"SHN" => "Saint Helena, Ascension and Tristan da Cunha",
	"SJM" => "Svalbard and Jan Mayen",
	"SLB" => "Soloman Islands",
	"SLE" => "Sierra Leone",
	"SLV" => "El Salvador",
	"SMR" => "San Marino",
	"SOM" => "Somalia",
	"SPM" => "Saint Pierre and Miquelon",
	"SRB" => "Serbia",
	"SSD" => "South Sudan",
	"STP" => "Sao Tome and Principe",
	"SUR" => "Suriname",
	"SVK" => "Slovakia",
	"SVN" => "Slovenia",
	"SWE" => "Sweden",
	"SWZ" => "Eswatini",
	"SXM" => "Sint Maarten",
	"SYC" => "Seychelles",
	"SYR" => "Syria",
	"TCA" => "Turks and Caicos Islands",
	"TCD" => "Chad",
	"TGO" => "Togo",
	"THA" => "Thailand",
	"TJK" => "Tajikistan",
	"TKL" => "Tokelau",
	"TKM" => "Turkmenistan",
	"TLS" => "Timor Leste",
	"TON" => "Tonga",
	"TTO" => "Trinidad and Tobago",
	"TUN" => "Tunisia",
	"TUR" => "Turkiye",
	"TUV" => "Tuvalu",
	"TWN" => "Taiwan",
	"TZA" => "Tanzania",
	"UGA" => "Uganda",
	"UKR" => "Ukraine",
	"UMI" => "United States Minor Outlying Islands",
	"URY" => "Uruguay",
	"USA" => "United States of America",
	"UZB" => "Uzbekistan",
	"VAT" => "Holy See",
	"VCT" => "Saint Vincent and the Grenadines",
	"VEN" => "Venzuela",
	"VGB" => "Virgin Islands (British)",
	"VIR" => "Virgin Islands (U.S.)",
	"VNM" => "Vietnam",
	"VUT" => "Vanuatu",
	"WLF" => "Wallis and Futuna",
	"WSM" => "Samoa",
	"YEM" => "Yemen",
	"ZAF" => "South Africa",
	"ZMB" => "Zambia",
	"ZWE" => "Zimbabwe",
};

pub fn is_country_code_valid(country_code: &str) -> bool {
    if country_code == "D" {
        return true;
    }
    if country_code.len() != 3 {
        return false;
    }
    COUNTRIES_ISO.contains_key(country_code) || COUNTRIES_ICAO.contains_key(country_code)
}

pub fn get_country_name(country_code: &str) -> String {
    if COUNTRIES_ISO.contains_key(country_code) {
        COUNTRIES_ISO.get(country_code).unwrap().to_string()
    } else if COUNTRIES_ICAO.contains_key(country_code) {
        COUNTRIES_ICAO.get(country_code).unwrap().to_string()
    } else {
        "Invalid country code!".to_owned()
    }
}

pub fn get_country_code_type(country_code: &str) -> String {
    if COUNTRIES_ISO.contains_key(country_code) {
        COUNTRY_TYPE_ISO.to_string()
    } else if COUNTRIES_ICAO.contains_key(country_code) {
        COUNTRY_TYPE_ICAO.to_string()
    } else {
        "Invalid country code!".to_owned()
    }
}



#[test]
fn test_iso_3166_country_code_iso_alg() {
    let dummy =  get_country_name("ALG");
    assert_eq!(dummy, "Algeria");
}   

#[test]
fn test_iso_3166_country_code_icao_eue() {
    let dummy =  get_country_name("EUE");
    assert_eq!(dummy, "European Union (EU)");
}   

#[test]
fn test_iso_3166_country_code_invalid() {
    let dummy =  get_country_name("BBB");
    assert_eq!(dummy, "Invalid country code!");
}   


#[test]
fn test_country_code_validity_alb() {
    let dummy =  is_country_code_valid("ALG");
    assert!(dummy);
}   

#[test]
fn test_country_code_validity_aaa() {
    let dummy =  is_country_code_valid("AAA");
    assert!(!dummy);
}   

#[test]
fn test_country_code_validity_d() {
    let dummy =  is_country_code_valid("D");
    assert!(dummy);
}   

#[test]
fn test_country_code_validity_xxx() {
    let dummy =  is_country_code_valid("XXX");
    assert!(dummy);
}   

#[test]
fn test_country_code_validity_lowercase() {
    let dummy =  is_country_code_valid("alg");
    assert!(!dummy);
}   

#[test]
fn test_country_code_validity_a() {
    let dummy =  is_country_code_valid("A");
    assert!(!dummy);
}   
 
#[test]
fn test_country_code_validity_aaaa() {
    let dummy =  is_country_code_valid("AAAA");
    assert!(!dummy);
}   

#[test]
fn test_country_code_type_iso() {
    let dummy =  get_country_code_type("ALG");
    assert_eq!(dummy, COUNTRY_TYPE_ISO);
}   

#[test]
fn test_country_code_type_icao() {
    let dummy =  get_country_code_type("XXX");
    assert_eq!(dummy, COUNTRY_TYPE_ICAO);
}   

#[test]
fn test_country_code_type_invalid() {
    let dummy =  get_country_code_type("CCC");
    assert_eq!(dummy, "Invalid country code!");
}

#[test]
fn test_country_name_d() {
    let dummy = get_country_name("D");
    assert_eq!(dummy, "Germany");
}

#[test]
fn test_country_code_type_d() {
    let dummy = get_country_code_type("D");
    assert_eq!(dummy, COUNTRY_TYPE_ICAO);
}
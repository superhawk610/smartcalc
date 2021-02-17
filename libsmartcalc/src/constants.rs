use lazy_static::*;
use mut_static::MutStatic;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use regex::Regex;
use crate::types::Money;

use crate::worker::{rule::RuleLanguage};

pub static mut SYSTEM_INITED: bool = false;
lazy_static! {
    pub static ref CURRENCIES: MutStatic<BTreeMap<String, Money>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref CURRENCY_ALIAS: MutStatic<BTreeMap<String, String>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };
    
    pub static ref CURRENCY_RATES: MutStatic<BTreeMap<String, f64>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref TOKEN_PARSE_REGEXES: MutStatic<BTreeMap<String, Vec<Regex>>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref WORD_GROUPS: MutStatic<BTreeMap<String, Vec<String>>> = {
        let m = BTreeMap::new();
        MutStatic::from(m)
    };

    pub static ref ALIAS_REGEXES: MutStatic<Vec<(Regex, String)>> = {
        let m = Vec::new();
        MutStatic::from(m)
    };

    pub static ref RULES: MutStatic<RuleLanguage> = {
        let m = RuleLanguage::new();
        MutStatic::from(m)
    };
}

pub const JSON_DATA: &str = r#"{
  "parse":  {
    "percent": [
        "(?P<NUMBER>[-+]?[0-9]+([,\\.][0-9]+){0,})(?P<PERCENT>%)",
        "(?P<PERCENT>%)(?P<NUMBER>[-+]?[0-9]+([,\\.][0-9]+){0,})"
    ],
    "time": [
        "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
        "(?P<hour>1[0-2]|0?[1-9]):(?P<minute>[0-5][0-9]) ?(?P<meridiem>[AaPp][Mm])",
        "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9]):(?P<second>[0-5][0-9])",
        "(?P<hour>[0-1]?[0-9]|2[0-3]):(?P<minute>[0-5][0-9])"
    ],
    "money": [
        "(?P<CURRENCY>\\p{Currency_Symbol})(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})(?P<NOTATION>[kKMGTPZY]{0,1})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>[a-zA-Z]{2,3})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})[ ]*(?P<CURRENCY>\\p{Currency_Symbol})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})(?P<NOTATION>[kKMGTPZY])[ ]{1,}(?P<CURRENCY>[a-zA-Z]{2,3})",
        "(?P<PRICE>[-+]?[0-9]+[0-9.,]{0,})(?P<NOTATION>[kKMGTPZY])[ ]{1,}(?P<CURRENCY>\\p{Currency_Symbol})"
    ],
    "number": [
        "0[xX](?P<HEX>[0-9a-fA-F]+)",
        "0[oO](?P<OCTAL>[0-7]+)",
        "0[bB](?P<BINARY>[01]+)",
        "(?P<DECIMAL>[-+]?[0-9]+[0-9.,]{0,})(?P<NOTATION>[kKMGTPZY]{0,1})"
    ],
    "text": [
        "(?P<TEXT>[\\p{L}]+)"
    ],
    "field": [
        "(\\{(?P<FIELD>[A-Z]+):(?P<NAME>[^}]+)\\})"
    ],
    "atom": [
        "(\\[(?P<ATOM>[A-Z]+):(?P<DATA>[^\\]]+)\\])"
    ],
    "whitespace": [
        "(?P<WHITESPACE>[ ]+)"
    ],
    "operator": [
        "(?P<OPERATOR>[^0-9\\p{L} ])"
    ]
  },

    "rules": {
        "en": {
            "hour_add": ["{TIME:time} add {NUMBER:hour} {GROUP:hour_group}"],
            "time_for_location": ["time in {TEXT:location}", "time at {TEXT:location}", "time for {TEXT:location}"],
            
            "convert_money": ["{MONEY:money} {GROUP:conversion_group} {TEXT:currency}", "{MONEY:money} {TEXT:currency}"],
            "money_on": ["{PERCENT:p} on {MONEY:money}"],
            "money_of": ["{PERCENT:p} of {MONEY:money}"],
            "money_off": ["{PERCENT:p} off {MONEY:money}"],

            "number_on": ["{PERCENT:p} on {NUMBER:number}"],
            "number_of": ["{PERCENT:p} of {NUMBER:number}"],
            "number_off": ["{PERCENT:p} off {NUMBER:number}"],

            "division_cleanup": ["{PERCENT:data}/{TEXT:text}", "{MONEY:data}/{TEXT:text}", "{NUMBER:data}/{TEXT:text}"],

            "find_numbers_percent": ["{NUMBER:part} is what % of {NUMBER:total}"],
            "find_total_from_percent": ["{NUMBER:number_part} is {PERCENT:percent_part} of what"]
        }
    },

    "word_group": {
        "hour_group": ["hour", "hours"],
        "week_group": ["week", "weeks"],
        "conversion_group": ["in", "into", "as", "to"]
    },

  "alias": {
    "_": "",
    ";": "",
    "!": "",
    "\\?": "",
    "'": "",
    "&": "",
    "\\^": "",

    "times": "[OPERATOR:*]",
    "multiply": "[OPERATOR:*]",
    "x": "[OPERATOR:*]",
    "×": "[OPERATOR:*]",

    "add": "[OPERATOR:+]",
    "sum": "[OPERATOR:+]",
    "append": "[OPERATOR:+]",

    "exclude": "[OPERATOR:-]",
    "minus": "[OPERATOR:-]",

    "euro": "eur"
  },
  "currency_alias" : {
    "try": "try",
    "tl": "try",
    "₺": "try",

    "$": "usd",
    "usd": "usd",
    "dollar": "usd",

    "sek": "sek",

    "dkk": "dkk",
    "kr": "dkk",
    "kroner": "dkk",

    "bgn": "bgn",
    "leva": "bgn",
    "lef": "bgn",
    "лв": "bgn",

    "eur": "eur",
    "euro": "eur",
    "avro": "eur",
    "€": "eur"
  },
  "currency_rates": {
    "hkd": 7.7526495869,
    "isk": 129.2664608195,
    "php": 48.1023116081,
    "dkk": 6.2056246349,
    "huf": 297.5715597096,
    "czk": 21.5355086372,
    "gbp": 0.7305182342,
    "ron": 4.0680130184,
    "sek": 8.4476341484,
    "idr": 14036.752065426,
    "inr": 72.9091212551,
    "brl": 5.4450471501,
    "rub": 74.7997162647,
    "hrk": 6.3100225319,
    "jpy": 105.749812234,
    "thb": 30.1001418676,
    "chf": 0.9033630977,
    "eur": 0.8345155637,
    "myr": 4.0705165651,
    "bgn": 1.6321455395,
    "try": 7.0727697572,
    "cny": 6.4704164233,
    "nok": 8.6011850121,
    "nzd": 1.3999833097,
    "zar": 14.9717933739,
    "usd": 1.0,
    "mxn": 20.3196194609,
    "sgd": 1.3379788033,
    "aud": 1.31527998,
    "ils": 3.2926646082,
    "krw": 1122.7989652007,
    "pln": 3.7572394225
  },

  "currencies": {
    "AED": {
      "code": "AED",
      "symbol": "د.إ.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "AFN": {
      "code": "AFN",
      "symbol": "؋",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "ALL": {
      "code": "ALL",
      "symbol": "Lek",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "AMD": {
      "code": "AMD",
      "symbol": "֏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "ANG": {
      "code": "ANG",
      "symbol": "ƒ",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "AOA": {
      "code": "AOA",
      "symbol": "Kz",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "ARS": {
      "code": "ARS",
      "symbol": "$",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "AUD": {
      "code": "AUD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "AWG": {
      "code": "AWG",
      "symbol": "ƒ",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "AZN": {
      "code": "AZN",
      "symbol": "₼",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BAM": {
      "code": "BAM",
      "symbol": "КМ",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BBD": {
      "code": "BBD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "BDT": {
      "code": "BDT",
      "symbol": "৳",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 0
    },
    "BGN": {
      "code": "BGN",
      "symbol": "лв.",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BHD": {
      "code": "BHD",
      "symbol": "د.ب.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 3
    },
    "BIF": {
      "code": "BIF",
      "symbol": "FBu",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "BMD": {
      "code": "BMD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "BND": {
      "code": "BND",
      "symbol": "$",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "BOB": {
      "code": "BOB",
      "symbol": "Bs",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BRL": {
      "code": "BRL",
      "symbol": "R$",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BSD": {
      "code": "BSD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "BTC": {
      "code": "BTC",
      "symbol": "Ƀ",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 8
    },
    "BTN": {
      "code": "BTN",
      "symbol": "Nu.",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 1
    },
    "BWP": {
      "code": "BWP",
      "symbol": "P",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "BYR": {
      "code": "BYR",
      "symbol": "р.",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "BZD": {
      "code": "BZD",
      "symbol": "BZ$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CAD": {
      "code": "CAD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CDF": {
      "code": "CDF",
      "symbol": "FC",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CHF": {
      "code": "CHF",
      "symbol": "CHF",
      "thousandsSeparator": "'",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "CLP": {
      "code": "CLP",
      "symbol": "$",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "CNY": {
      "code": "CNY",
      "symbol": "¥",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "COP": {
      "code": "COP",
      "symbol": "$",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "CRC": {
      "code": "CRC",
      "symbol": "₡",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CUC": {
      "code": "CUC",
      "symbol": "CUC",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CUP": {
      "code": "CUP",
      "symbol": "$MN",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CVE": {
      "code": "CVE",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "CZK": {
      "code": "CZK",
      "symbol": "Kč",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "DJF": {
      "code": "DJF",
      "symbol": "Fdj",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "DKK": {
      "code": "DKK",
      "symbol": "kr.",
      "thousandsSeparator": "",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "DOP": {
      "code": "DOP",
      "symbol": "RD$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "DZD": {
      "code": "DZD",
      "symbol": "د.ج.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "EGP": {
      "code": "EGP",
      "symbol": "ج.م.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "ERN": {
      "code": "ERN",
      "symbol": "Nfk",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "ETB": {
      "code": "ETB",
      "symbol": "ETB",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "EUR": {
      "code": "EUR",
      "symbol": "€",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "FJD": {
      "code": "FJD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "FKP": {
      "code": "FKP",
      "symbol": "£",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GBP": {
      "code": "GBP",
      "symbol": "£",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GEL": {
      "code": "GEL",
      "symbol": "Lari",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "GHS": {
      "code": "GHS",
      "symbol": "₵",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GIP": {
      "code": "GIP",
      "symbol": "£",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GMD": {
      "code": "GMD",
      "symbol": "D",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GNF": {
      "code": "GNF",
      "symbol": "FG",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "GTQ": {
      "code": "GTQ",
      "symbol": "Q",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "GYD": {
      "code": "GYD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "HKD": {
      "code": "HKD",
      "symbol": "HK$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "HNL": {
      "code": "HNL",
      "symbol": "L.",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "HRK": {
      "code": "HRK",
      "symbol": "kn",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "HTG": {
      "code": "HTG",
      "symbol": "G",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "HUF": {
      "code": "HUF",
      "symbol": "Ft",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "IDR": {
      "code": "IDR",
      "symbol": "Rp",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "ILS": {
      "code": "ILS",
      "symbol": "₪",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "INR": {
      "code": "INR",
      "symbol": "₹",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "IQD": {
      "code": "IQD",
      "symbol": "د.ع.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "IRR": {
      "code": "IRR",
      "symbol": "﷼",
      "thousandsSeparator": ",",
      "decimalSeparator": "/",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "ISK": {
      "code": "ISK",
      "symbol": "kr.",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 0
    },
    "JMD": {
      "code": "JMD",
      "symbol": "J$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "JOD": {
      "code": "JOD",
      "symbol": "د.ا.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 3
    },
    "JPY": {
      "code": "JPY",
      "symbol": "¥",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "KES": {
      "code": "KES",
      "symbol": "KSh",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "KGS": {
      "code": "KGS",
      "symbol": "сом",
      "thousandsSeparator": " ",
      "decimalSeparator": "-",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "KHR": {
      "code": "KHR",
      "symbol": "៛",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "KMF": {
      "code": "KMF",
      "symbol": "CF",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "KPW": {
      "code": "KPW",
      "symbol": "₩",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "KRW": {
      "code": "KRW",
      "symbol": "₩",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "KWD": {
      "code": "KWD",
      "symbol": "د.ك.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 3
    },
    "KYD": {
      "code": "KYD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "KZT": {
      "code": "KZT",
      "symbol": "₸",
      "thousandsSeparator": " ",
      "decimalSeparator": "-",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "LAK": {
      "code": "LAK",
      "symbol": "₭",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "LBP": {
      "code": "LBP",
      "symbol": "ل.ل.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "LKR": {
      "code": "LKR",
      "symbol": "₨",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 0
    },
    "LRD": {
      "code": "LRD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "LSL": {
      "code": "LSL",
      "symbol": "M",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "LYD": {
      "code": "LYD",
      "symbol": "د.ل.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 3
    },
    "MAD": {
      "code": "MAD",
      "symbol": "د.م.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "MDL": {
      "code": "MDL",
      "symbol": "lei",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "MGA": {
      "code": "MGA",
      "symbol": "Ar",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "MKD": {
      "code": "MKD",
      "symbol": "ден.",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "MMK": {
      "code": "MMK",
      "symbol": "K",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MNT": {
      "code": "MNT",
      "symbol": "₮",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MOP": {
      "code": "MOP",
      "symbol": "MOP$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MRO": {
      "code": "MRO",
      "symbol": "UM",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MTL": {
      "code": "MTL",
      "symbol": "₤",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MUR": {
      "code": "MUR",
      "symbol": "₨",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MVR": {
      "code": "MVR",
      "symbol": "MVR",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 1
    },
    "MWK": {
      "code": "MWK",
      "symbol": "MK",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MXN": {
      "code": "MXN",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MYR": {
      "code": "MYR",
      "symbol": "RM",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "MZN": {
      "code": "MZN",
      "symbol": "MT",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "NAD": {
      "code": "NAD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "NGN": {
      "code": "NGN",
      "symbol": "₦",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "NIO": {
      "code": "NIO",
      "symbol": "C$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "NOK": {
      "code": "NOK",
      "symbol": "kr",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "NPR": {
      "code": "NPR",
      "symbol": "₨",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "NZD": {
      "code": "NZD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "OMR": {
      "code": "OMR",
      "symbol": "﷼",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 3
    },
    "PAB": {
      "code": "PAB",
      "symbol": "B/.",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "PEN": {
      "code": "PEN",
      "symbol": "S/.",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "PGK": {
      "code": "PGK",
      "symbol": "K",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "PHP": {
      "code": "PHP",
      "symbol": "₱",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "PKR": {
      "code": "PKR",
      "symbol": "₨",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "PLN": {
      "code": "PLN",
      "symbol": "zł",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "PYG": {
      "code": "PYG",
      "symbol": "₲",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "QAR": {
      "code": "QAR",
      "symbol": "﷼",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "RON": {
      "code": "RON",
      "symbol": "L",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "RSD": {
      "code": "RSD",
      "symbol": "Дин.",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "RUB": {
      "code": "RUB",
      "symbol": "₽",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "RWF": {
      "code": "RWF",
      "symbol": "RWF",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "SAR": {
      "code": "SAR",
      "symbol": "﷼",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "SBD": {
      "code": "SBD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SCR": {
      "code": "SCR",
      "symbol": "₨",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SDD": {
      "code": "SDD",
      "symbol": "LSd",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SDG": {
      "code": "SDG",
      "symbol": "£‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SEK": {
      "code": "SEK",
      "symbol": "kr",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "SGD": {
      "code": "SGD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SHP": {
      "code": "SHP",
      "symbol": "£",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SLL": {
      "code": "SLL",
      "symbol": "Le",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SOS": {
      "code": "SOS",
      "symbol": "S",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SRD": {
      "code": "SRD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "STD": {
      "code": "STD",
      "symbol": "Db",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SVC": {
      "code": "SVC",
      "symbol": "₡",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "SYP": {
      "code": "SYP",
      "symbol": "£",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "SZL": {
      "code": "SZL",
      "symbol": "E",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "THB": {
      "code": "THB",
      "symbol": "฿",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TJS": {
      "code": "TJS",
      "symbol": "TJS",
      "thousandsSeparator": " ",
      "decimalSeparator": ";",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "TMT": {
      "code": "TMT",
      "symbol": "m",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "TND": {
      "code": "TND",
      "symbol": "د.ت.‏",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 3
    },
    "TOP": {
      "code": "TOP",
      "symbol": "T$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TRY": {
      "code": "TRY",
      "symbol": "₺",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TTD": {
      "code": "TTD",
      "symbol": "TT$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TVD": {
      "code": "TVD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TWD": {
      "code": "TWD",
      "symbol": "NT$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "TZS": {
      "code": "TZS",
      "symbol": "TSh",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "UAH": {
      "code": "UAH",
      "symbol": "₴",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "UGX": {
      "code": "UGX",
      "symbol": "USh",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "USD": {
      "code": "USD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "UYU": {
      "code": "UYU",
      "symbol": "$U",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "UZS": {
      "code": "UZS",
      "symbol": "сўм",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "VEB": {
      "code": "VEB",
      "symbol": "Bs.",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "VEF": {
      "code": "VEF",
      "symbol": "Bs. F.",
      "thousandsSeparator": ".",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "VND": {
      "code": "VND",
      "symbol": "₫",
      "thousandsSeparator": ".",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 0
    },
    "VUV": {
      "code": "VUV",
      "symbol": "VT",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 0
    },
    "WST": {
      "code": "WST",
      "symbol": "WS$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "XAF": {
      "code": "XAF",
      "symbol": "F",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "XCD": {
      "code": "XCD",
      "symbol": "$",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "XBT": {
      "code": "XBT",
      "symbol": "Ƀ",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "XOF": {
      "code": "XOF",
      "symbol": "F",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "XPF": {
      "code": "XPF",
      "symbol": "F",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": false,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "YER": {
      "code": "YER",
      "symbol": "﷼",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": true,
      "decimalDigits": 2
    },
    "ZAR": {
      "code": "ZAR",
      "symbol": "R",
      "thousandsSeparator": " ",
      "decimalSeparator": ",",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "ZMW": {
      "code": "ZMW",
      "symbol": "ZK",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    },
    "WON": {
      "code": "WON",
      "symbol": "₩",
      "thousandsSeparator": ",",
      "decimalSeparator": ".",
      "symbolOnLeft": true,
      "spaceBetweenAmountAndSymbol": false,
      "decimalDigits": 2
    }
  }
}"#;
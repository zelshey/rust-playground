struct Character {
    arabic: u32,
    roman: &'static str,
}

const CHARACTERS: [Character; 13] = [
    Character {
        arabic: 1000,
        roman: "M",
    },
    Character {
        arabic: 900,
        roman: "CM",
    },
    Character {
        arabic: 500,
        roman: "D",
    },
    Character {
        arabic: 400,
        roman: "CD",
    },
    Character {
        arabic: 100,
        roman: "C",
    },
    Character {
        arabic: 90,
        roman: "XC",
    },
    Character {
        arabic: 50,
        roman: "L",
    },
    Character {
        arabic: 40,
        roman: "XL",
    },
    Character {
        arabic: 10,
        roman: "X",
    },
    Character {
        arabic: 9,
        roman: "IX",
    },
    Character {
        arabic: 5,
        roman: "V",
    },
    Character {
        arabic: 4,
        roman: "IV",
    },
    Character {
        arabic: 1,
        roman: "I",
    },
];

pub fn arabic_to_roman(num: u32) -> String {
    let mut ret: String = String::new();
    let mut num = num;
    while num > 0 {
        for character in CHARACTERS.iter() {
            if num >= character.arabic {
                ret.push_str(character.roman);
                num -= character.arabic;
                break;
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::arabic_to_roman;

    #[test]
    fn arabic() {
        assert_eq!(arabic_to_roman(1), "I", "1 should be I");
        assert_eq!(arabic_to_roman(2), "II", "2 should be II");
        assert_eq!(arabic_to_roman(3), "III", "3 should be III");
        assert_eq!(arabic_to_roman(4), "IV", "4 should be IV");
        assert_eq!(arabic_to_roman(5), "V", "5 should be V");
        assert_eq!(arabic_to_roman(6), "VI", "6 should be VI");
        assert_eq!(arabic_to_roman(7), "VII", "7 should be VII");
        assert_eq!(arabic_to_roman(8), "VIII", "8 should be VIII");
        assert_eq!(arabic_to_roman(9), "IX", "9 should be IX");
        assert_eq!(arabic_to_roman(10), "X", "10 should be X");
        assert_eq!(arabic_to_roman(14), "XIV", "14 should be XIV");
        assert_eq!(arabic_to_roman(18), "XVIII", "18 should be XVIII");
        assert_eq!(arabic_to_roman(20), "XX", "20 should be XX");
        assert_eq!(arabic_to_roman(39), "XXXIX", "39 should be XXXIX");
        assert_eq!(arabic_to_roman(40), "XL", "40 should be XL");
        assert_eq!(arabic_to_roman(47), "XLVII", "47 should be XLVII");
        assert_eq!(arabic_to_roman(49), "XLIX", "49 should be XLIX");
        assert_eq!(arabic_to_roman(50), "L", "50 should be L");
        assert_eq!(arabic_to_roman(100), "C", "100 should be C");
        assert_eq!(arabic_to_roman(90), "XC", "90 should be XC");
        assert_eq!(arabic_to_roman(400), "CD", "400 should be CD");
        assert_eq!(arabic_to_roman(500), "D", "500 should be D");
        assert_eq!(arabic_to_roman(900), "CM", "900 should be CM");
        assert_eq!(arabic_to_roman(1000), "M", "1000 should be M");
        assert_eq!(
            arabic_to_roman(1984),
            "MCMLXXXIV",
            "1984 should be MCMLXXXIV"
        );
        assert_eq!(
            arabic_to_roman(3999),
            "MMMCMXCIX",
            "3999 should be MMMCMXCIX"
        );
        assert_eq!(arabic_to_roman(2014), "MMXIV", "2014 should be MMXIV");
        assert_eq!(arabic_to_roman(1006), "MVI", "1006 should be MVI");
        assert_eq!(arabic_to_roman(798), "DCCXCVIII", "798 should be DCCXCVIII");
    }
}

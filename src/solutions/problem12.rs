pub trait Problem12 {
    // Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
    // Symbol       Value
    // I             1
    // V             5
    // X             10
    // L             50
    // C             100
    // D             500
    // M             1000

    // For example, 2 is written as II in Roman numeral, just two one's added
    // together. 12 is written as XII, which is simply X + II. The number 27 is
    // written as XXVII, which is XX + V + II. Roman numerals are usually written
    // largest to smallest from left to right. However, the numeral for four is not
    // IIII. Instead, the number four is written as IV. Because the one is before
    // the five we subtract it making four. The same principle applies to the number
    // nine, which is written as IX. There are six instances where subtraction is
    // used:
    //     I can be placed before V (5) and X (10) to make 4 and 9.
    //     X can be placed before L (50) and C (100) to make 40 and 90.
    //     C can be placed before D (500) and M (1000) to make 400 and 900.
    // Given an integer, convert it to a roman numeral.
    fn int_to_roman(num: i32) -> String;
}

struct Solution;

impl Problem12 for Solution {
    fn int_to_roman(num: i32) -> String {
        vec!['I'; num as usize]
            .iter()
            .collect::<String>()
            .replace("IIIII", "V")
            .replace("VV", "X")
            .replace("XXXXX", "L")
            .replace("LL", "C")
            .replace("CCCCC", "D")
            .replace("DD", "M")
            .replace("DCCCC", "CM")
            .replace("CCCC", "CD")
            .replace("LXXXX", "XC")
            .replace("XXXX", "XL")
            .replace("VIIII", "IX")
            .replace("IIII", "IV")
    }
}

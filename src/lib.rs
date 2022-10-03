use std::{io, collections::HashMap};


pub struct ArabNum {
    pub num: i32,
}

impl  ArabNum {
    pub fn build() -> Result<ArabNum, &'static str> {
        
        let mut arab_nubmer: String = String::new();
        
        match io::stdin()
        .read_line(&mut arab_nubmer) {
            Ok(string) => string,
            Err(_) => return Err("Error while readin in the nubmer"),
        };
        
        let arab_nubmer: i32 = match arab_nubmer.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err("error parsing the number"),
        };

        if arab_nubmer > 3999 {
            return Err("Too big number")
        }
        
        Ok(ArabNum{
            num: arab_nubmer,
        })
    }    
}

fn arab_digit_maker(arab_num: i32) -> Vec<i32> {
    
    const RADIX: u32 = 10;

    let arab_digits: Vec<i32> = arab_num
    .to_string()
    .chars()
    .map(|char| char
        .to_digit(RADIX)
        .unwrap()
        .try_into()
        .unwrap()
    )
    .collect();

    arab_digits
}

pub fn convert_to_romaniam_num(arab_num: i32) -> String{
    let arab_digits = arab_digit_maker(arab_num);
    
    let mut romanian_number = String::new();

    let ten: i32 = 10;
    let digits_length = arab_digits.len();

    for (count, arab_digit) in arab_digits.iter().enumerate() {
        let exp =  digits_length - 1 - count;
        let dec: i32 = ten.pow(exp.try_into().unwrap());
        romanian_number += &convert_romaniam_digit(*arab_digit, dec);
    }
    
    romanian_number
    
}

fn convert_romaniam_digit(arab_digit: i32, dec: i32) -> String { 
    let romanian_digits = HashMap::from([
            (1, vec!["I".to_string(), "V".to_string(), "X".to_string()]),
            (10, vec!["X".to_string(), "L".to_string(), "C".to_string()]),
            (100, vec!["C".to_string(), "D".to_string(), "M".to_string()]),
            (1000, vec!["M".to_string(), "ERR".to_string(), "ERR".to_string()]),
        ]);
    
    let div_five_rest: i32 = arab_digit % 5;
    let div_five_times: i32 = arab_digit / 5;
    
    let ones = &romanian_digits.get(&dec).unwrap()[0];
    let fives = &romanian_digits.get(&dec).unwrap()[1];
    let tens = &romanian_digits.get(&dec).unwrap()[2];
    
    romanian_digit_translator(div_five_rest, div_five_times, ones, fives, tens)
}

fn romanian_digit_translator(
    div_five_rest: i32,
    div_five_times: i32,
    ones: &String,
    fives: &String,
    tens: &String) -> String {
    match div_five_times {
        0 => {
        match div_five_rest{ 
            0 => return String::new(),
            4 => return format!("{}{}", ones, fives),
            _ => {
            return ones.repeat(div_five_rest.
                try_into().
                unwrap());
        }}},  
        1 => {
        match div_five_rest {
            0 => return format!("{}", fives),
            4 => return format!("{}{}", ones, tens),
            _ => {
            let many_ones = &ones.repeat(div_five_rest.
                try_into().
                unwrap());
            return format!("{}{}", fives, many_ones) 
        }}},
        _ => panic!("Somehow here"),
        };
    }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_three() {
        let arab_num = 3;
        let romaniam_num = "III";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }

    #[test]
    fn test_full_five() {
        let arab_num = 5;
        let romaniam_num = "V";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }    

    #[test]
    fn test_full_four() {
        let arab_num = 4;
        let romaniam_num = "IV";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }    

    #[test]
    fn test_full_eight() {
        let arab_num = 8;
        let romaniam_num = "VIII";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }   

    #[test]
    fn test_digit_three() {
        let arab_num = 3;
        let dec = 1;
        let romaniam_num = "III";

        assert_eq!(
            romaniam_num, 
            convert_romaniam_digit(
                arab_num, dec

            ));
        }
            
        
 
#[test]
    fn test_digit_five() {
        let arab_num = 5;
        let dec = 1;
        let romaniam_num = "V";

        assert_eq!(
            romaniam_num, 
            convert_romaniam_digit(
                arab_num, dec
            ));
    }

#[test]
    fn test_digit_four() {
        let arab_num = 4;
        let dec = 1;
        let romaniam_num = "IV";

        assert_eq!(
            romaniam_num, 
            convert_romaniam_digit(
                arab_num, dec
            ));     
    }

#[test]
    fn test_digit_eight() {
        let arab_num = 8;
        let dec = 1;
        let romaniam_num = "VIII";

        assert_eq!(
            romaniam_num, 
            convert_romaniam_digit(
                arab_num, dec
            ));
        }  

        #[test]
        fn test_digit_eighty() {
            let arab_num = 8;
            let dec = 10;
            let romaniam_num = "LXXX";
    
            assert_eq!(
                romaniam_num, 
                convert_romaniam_digit(
                    arab_num, dec
                ));
            }          

        #[test]
        fn test_digit_ten() {
            let arab_num = 1;
            let dec = 10;
            let romaniam_num = "X";
    
            assert_eq!(
                romaniam_num, 
                convert_romaniam_digit(
                    arab_num, dec
                ));
                }        

    #[test]
    fn test_ten() {
        let arab_num = 10;
        let romaniam_num = "X";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }
    
    #[test]
    fn test_hundred_eighty_nine() {
        let arab_num = 189;
        let romaniam_num = "CLXXXIX";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }

    #[test]
    fn test_three_thousand_five_hundred_sixty_eight() {
        let arab_num = 3568;
        let romaniam_num = "MMMDLXVIII";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }

    #[test]
    fn test_three_thousand_nine_hundred_ninty_nine() {
        let arab_num = 3999;
        let romaniam_num = "MMMCMXCIX";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }

    #[test]
    fn test_arab_digits_three() {
        let arab_num = 3;
        let digits = vec![3];

        assert_eq!(digits, arab_digit_maker(arab_num))
    }

    #[test]
    fn test_arab_digits_thirty_one() {
        let arab_num = 31;
        let digits = vec![3, 1];

        assert_eq!(digits, arab_digit_maker(arab_num))
    }

    #[test]
    fn test_arab_digits_three_thousand_nine_hundred_ninty_nine() {
        let arab_num = 3999;
        let digits = vec![3, 9, 9, 9];

        assert_eq!(digits, arab_digit_maker(arab_num))
    }
}
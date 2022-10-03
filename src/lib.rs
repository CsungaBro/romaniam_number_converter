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
    
        Ok(ArabNum{
            num: arab_nubmer,
        })
    }    
}

pub fn arab_digir_maker(arab_num: i32) -> Vec<i32> {
    let mut arab_digits = Vec::new();
    
    let ten: i32 = 10;
    for exp in 0..5 {
        let dec = ten.pow(exp);
        let digit_remainder = (arab_num / dec) % 10;
        
        println!("digit_remainder: {digit_remainder}, dec: {dec}");
        arab_digits.push(digit_remainder);
        let times: i32 = arab_num / ten.pow(exp+1);
        if times == 0 {
            break
        }
    }

    arab_digits
}

pub fn convert_to_romaniam_num(arab_num: i32) -> String{
    let arab_digits = arab_digir_maker(arab_num);
    
    println!("{:?}", arab_digits);
    
    let mut romanian_number = String::new();
    
    for (exp, arab_digit) in arab_digits.iter().enumerate().rev() {
        let ten: i32 = 10;
        romanian_number += &convert_romaniam_digit(*arab_digit, ten.pow(exp.try_into().unwrap()));
        println!("romanian_number: {:?}", romanian_number);

    }
    
    romanian_number
    
}

pub fn convert_romaniam_digit(arab_digit: i32, dec: i32) -> String { 
    let romanian_digits = HashMap::from([
            (1, vec!["I".to_string(), "V".to_string(), "X".to_string()]),
            (10, vec!["X".to_string(), "L".to_string(), "C".to_string()]),
            (100, vec!["C".to_string(), "D".to_string(), "M".to_string()]),
        ]);

    let div_five_rest: i32 = arab_digit % 5;
    let div_five_times: i32 = arab_digit / 5;
    
    println!("dec: {dec}");
    println!("{:?}", romanian_digits);
    let ones = &romanian_digits.get(&dec).unwrap()[0];
    let fives = &romanian_digits.get(&dec).unwrap()[1];
    let tens = &romanian_digits.get(&dec).unwrap()[2];

    if div_five_times == 0 {
        if div_five_rest == 0 {
            return String::new()
        } 
        else if div_five_rest == 4
        {
            return format!("{}{}", ones, fives);
        }else
        {
            return ones.repeat(div_five_rest.
                try_into().
                unwrap());
        }

    } else if div_five_times == 1 {
        if div_five_rest == 0 {
            return format!("{}", fives);
        } else if div_five_rest == 4
        {
            return format!("{}{}", ones, tens);
        } else {
            let many_ones = &ones.repeat(div_five_rest.
                try_into().
                unwrap());
            return format!("{}{}", fives, many_ones) 
        }
    } else{
        panic!("Somehow here")
    }   
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

}
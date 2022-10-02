use std::{io};


pub struct Numbers {
    pub arabian_num: i32,
    pub romanian_num: String,
}

impl  Numbers {
    pub fn build() -> Result<Numbers, &'static str> {
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
    
        Ok(Numbers{
            arabian_num: arab_nubmer,
            romanian_num: String::new(),
        })
    }    
}

pub fn convert_to_romaniam_num(arab_num: i32) -> String{
    let mut romanian_num = String::new();
    
    let div_five_rest: i32 = arab_num % 5;
    let div_five_times: i32 = arab_num / 5;

    println!("div_five_rest :{div_five_rest}");
    println!("div_five_times :{div_five_times}");

    if div_five_times == 0 {
        if div_five_rest == 0 {} 
        else if div_five_rest == 4
        {
            romanian_num.push_str("IV");
        }else
        {
            for _ in 0..div_five_rest {
                romanian_num.push_str("I");
            }
        }
    } else if div_five_times == 1 {
        if div_five_rest == 0 {
            romanian_num.push_str("V")
        } 
        else if div_five_rest == 4
        {
            romanian_num.push_str("IX");
        }else
        {
            romanian_num.push_str("V");
            for _ in 0..div_five_rest {
                romanian_num.push_str("I");
            }
        }
    }    

    romanian_num
}

pub fn convert_romaniam_digit(
    div_five_rest: i32, 
    div_five_times: i32, 
    ones: String,
    fives: String,
    tens: String,
) -> String { 
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
            return fives + &ones.repeat(div_five_rest.
                try_into().
                unwrap());
        }
    } else{
        panic!("Somehow here")
    }   
}

fn main() {
    let numbers = Numbers::build().unwrap_or_else(|err| {
        panic!("Problem parsing arguments: {err}");
    });
    
    println!("arab_nubmer: {}", numbers.arabian_num);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three() {
        let arab_num = 3;
        let romaniam_num = "III";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }

    #[test]
    fn test_five() {
        let arab_num = 5;
        let romaniam_num = "V";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }    

    #[test]
    fn test_four() {
        let arab_num = 4;
        let romaniam_num = "IV";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
    }    

    #[test]
    fn test_eight() {
        let arab_num = 8;
        let romaniam_num = "VIII";

        assert_eq!(romaniam_num, convert_to_romaniam_num(arab_num))
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
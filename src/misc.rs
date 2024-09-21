use num_complex::Complex;
use regex::Regex;
const L_BRAC: [&str; 3] = ["(", "{", "["];
const R_BRAC: [&str; 3] = [")", "}", "]"];

pub fn string_to_num(inp: String) -> f64 {
    inp.trim()
        .parse::<f64>()
        .expect("Provided String cannot be a number")
}

pub fn string_to_cmplx(mut inp: String) -> Complex<f64> {
    let first_part: String;
    let second_part: String;
    let negative = inp.starts_with('-');
    if negative {
        inp = inp[1..].to_string();
    }
    let sign = if inp.contains("-") { "-" } else { "+" };
    let groups = inp.split(sign).collect::<Vec<&str>>();
    if groups.len() == 1 {
        if groups[0].contains("i") {
            first_part = "0".to_string();
            if negative {
                second_part = "-".to_string() + groups[0];
            } else {
                second_part = groups[0].to_string();
            }
        } else {
            if negative {
                first_part = "-".to_string() + groups[0];
            } else {
                first_part = groups[0].to_string();
            }
            second_part = "0".to_string();
        }
    } else {
        first_part = groups[0].to_string();
        second_part = groups[1].to_string();
    }
    let return_num = Complex::new(
        string_to_num(first_part.to_string()),
        if sign == "-" {
            string_to_num((sign.to_string() + &second_part).replace("i", ""))
        } else {
            string_to_num(second_part.replace("i", ""))
        },
    );
    return return_num;
}

pub fn create_2dvec_from_1dvecs<'a>(inp: Vec<String>, sep: &'a str) -> Vec<Vec<String>> {
    // :TODO: understand the following line better, why Vec<&[&str]> does not work
    let slices: Vec<Vec<String>> = inp.split(|c| c == sep).map(|s| s.to_vec()).collect();
    let mut slices_vec: Vec<Vec<String>> = vec![];
    for sl in slices {
        slices_vec.push(sl);
    }
    return slices_vec;
}

pub fn create_vecs_from_str<'a>(inp: String, sep: &'a str) -> Vec<String> {
    // :TODO: understand the lifetime params
    let slices = inp.split(sep).filter(|c| !c.is_empty());
    let mut slices_vec: Vec<String> = vec![];
    for sl in slices {
        slices_vec.push(sl.to_string());
    }
    return slices_vec;
}

pub fn is_string_lbrac(str: String) -> bool {
    for i in 0..L_BRAC.len() {
        if str == L_BRAC[i] {
            return true;
        }
    }
    return false;
}
pub fn is_string_rbrac(str: String) -> bool {
    for i in 0..L_BRAC.len() {
        if str == R_BRAC[i] {
            return true;
        }
    }
    return false;
}

pub fn is_string_numeric(str: String) -> bool {
    let re_complex = Regex::new(r"([0-9])*(\+|\-)?([0-9])+i").unwrap();
    return str.parse::<f64>().is_ok() || re_complex.is_match(&str);
}

pub fn give_rbrac_from_lbrac(l_brac: String) -> String {
    for ind in 0..L_BRAC.len() {
        if l_brac == L_BRAC[ind] {
            return R_BRAC[ind].to_string();
        }
    }
    return l_brac;
}
pub fn give_lbrac_from_rbrac(r_brac: String) -> String {
    for ind in 0..L_BRAC.len() {
        if r_brac == R_BRAC[ind] {
            return L_BRAC[ind].to_string();
        }
    }
    return r_brac;
}

pub fn compare_brac(brac1: String, brac2: String) -> i32 {
    let mut brac1_ind: i32 = -1;
    let mut brac2_ind: i32 = -1;
    for i in 0..L_BRAC.len() {
        brac1_ind = if brac1 == L_BRAC[i] {
            i.try_into().unwrap()
        } else {
            brac1_ind
        };
        brac2_ind = if brac2 == L_BRAC[i] {
            i.try_into().unwrap()
        } else {
            brac2_ind
        };
        brac1_ind = if brac1 == R_BRAC[i] {
            i.try_into().unwrap()
        } else {
            brac1_ind
        };
        brac2_ind = if brac2 == R_BRAC[i] {
            i.try_into().unwrap()
        } else {
            brac2_ind
        };
    }
    return if brac1_ind > brac2_ind {
        1
    } else if brac1_ind < brac2_ind {
        -1
    } else {
        0
    };
}

pub fn factorial(n: i32) -> i32 {
    let mut fact: i32 = 1;
    for i in 1..=n {
        fact = fact * i;
    }
    return fact;
}

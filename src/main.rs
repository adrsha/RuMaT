// TODO: config, eqn, integration, derivative, plot, etc
use core::f64;
use num_complex::Complex;
use regex::Regex;

use colored::*;
use homedir::my_home;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod misc;

const L_BRAC: [&str; 3] = ["(", "{", "["];
const R_BRAC: [&str; 3] = [")", "}", "]"];
const CONSTS: [(&str, &str); 2] = [("e", "2.718281828459045"), ("pi", "3.141592653589793")];
const FUNCS: [&str; 13] = [
    "cos", "sin", "tan", "sec", "cosec", "cot", "cosh", "sinh", "tanh", "sech", "cosech", "coth",
    "log",
];
// const OPERATORS: [&str; 9] = ["+", "-", "*", "/", "^", "!", "%", "C", "P"];
const OPERATORS: [&str; 9] = ["P", "C", "%", "!", "^", "/", "*", "-", "+"];

fn round_nums(n: f64) -> f64 {
    let mut m: f64 = n * 10_f64.powi(15);
    m = m.round();
    m = m / 10_f64.powi(15);
    return m;
}

fn format_out(op_vec: &mut Vec<Vec<String>>) {
    let mut out_string_vec: Vec<String> = vec![];
    // remove the duplicates to reduce iterations
    op_vec.dedup();
    // Conversion to string
    for (ind, vec) in op_vec.iter().enumerate() {
        let mut out_buf = String::from("");
        for el in vec.iter() {
            out_buf = out_buf + &format!(" {} ", el);
        }
        // remove double spaces
        while out_buf.contains("  ") {
            out_buf = out_buf.replace("  ", " ");
        }
        // replacements
        out_buf = out_buf.replace("+ -", "-");

        // trim end spaces
        out_buf = out_buf.trim().to_string()
            .replace("x2", "x²")
            .replace("x3", "x³")
            .replace("x4", "x⁴")
            .replace("x5", "x⁵")
            .replace("x6", "x⁶")
            .replace("x7", "x⁷")
            .replace("x8", "x⁸")
            .replace("x9", "x⁹");
       
        // remove duplicates
        let temp_out_buf = out_buf
            .clone()
            .replace(" ", "")
            .replace("(", "")
            .replace(")", "")
            .replace("{", "")
            .replace("}", "")
            .replace("[", "")
            .replace("]", "");
        if ind > 0 {
            if temp_out_buf
                == out_string_vec[out_string_vec.len() - 1]
                    .clone()
                    .replace(" ", "")
                    .replace("(", "")
                    .replace(")", "")
                    .replace("{", "")
                    .replace("}", "")
                    .replace("[", "")
                    .replace("]", "")
            {
                continue;
            }
        }

        out_string_vec.push(out_buf);
    }

    // To Get highlights for all operations and operators
    for ind in 0..out_string_vec.len() {
        for op in OPERATORS {
            out_string_vec[ind] = out_string_vec[ind].replace(op, &format!(" {} ", op));
        }
    }

    out_string_vec.dedup();
    for step in out_string_vec {
        let mut out_buf = String::from(format!("{}", "= ".black()));
        for el in step.split(" ") {
            if misc::is_string_numeric(el.to_string()) {
                out_buf = out_buf + &format!("{}", el.yellow());
            } else if misc::is_string_lbrac(el.to_string()) || misc::is_string_rbrac(el.to_string())
            {
                out_buf = out_buf + &format!("{}", el.red());
            } else if misc::is_string_operator(el.to_string()) {
                out_buf = out_buf + &format!(" {} ", el.green());
            } else {
                out_buf = out_buf + &format!("{}", el.blue());
            }
        }

        println!("{}", out_buf);
    }
}

fn neg_fixer(new_inp_vec: Vec<String>) -> Vec<String> {
    // Make sure the -ves work as need be

    let mut inp_vec_temp = new_inp_vec.clone();

    if new_inp_vec.len() >= 2 {
        for ind in 0..new_inp_vec.len() {
            if new_inp_vec[ind] == "-" {
                if ind > 0 {
                    if new_inp_vec[ind - 1] == "/" || new_inp_vec[ind - 1] == "*" {
                        inp_vec_temp[ind + 1] = "-".to_string() + &new_inp_vec[ind + 1];
                        inp_vec_temp.remove(ind);
                        continue;
                    }
                }
                inp_vec_temp[ind] = String::from("+");
                inp_vec_temp[ind + 1] = "-".to_string() + &new_inp_vec[ind + 1];
            }
        }
    }
    return inp_vec_temp;
    // -- --
}

fn simplify(
    inp_vec: Vec<String>,
    l_ind: usize,
    mut r_ind: usize,
    op_vec: &mut Vec<Vec<String>>,
    cur_modes: &Modes,
) -> Vec<String> {
    let mut new_inp_vec = inp_vec;

    let mut inp_vec_mod = (&new_inp_vec[l_ind..=r_ind]).to_vec();
    inp_vec_mod = constants(inp_vec_mod.clone());
    inp_vec_mod = neg_fixer(inp_vec_mod);
    inp_vec_mod = operation_one_operand(inp_vec_mod.clone());
    inp_vec_mod = functions(inp_vec_mod.clone(), cur_modes);
    op_vec.push(new_inp_vec.clone());
    {
        let mut inp_vec_temp = new_inp_vec[..l_ind].to_vec();
        inp_vec_temp.extend(inp_vec_mod.clone());
        inp_vec_temp.extend(new_inp_vec[r_ind + 1..].to_vec());
        new_inp_vec = inp_vec_temp;
    }
    if inp_vec_mod.len() - 1 < r_ind - l_ind {
        r_ind = l_ind + inp_vec_mod.len() - 1;
    }

    op_vec.push(new_inp_vec.clone());

    for operator in OPERATORS {
        op_vec.push(new_inp_vec.clone());

        inp_vec_mod = (&new_inp_vec[l_ind..=r_ind]).to_vec();
        let inp_vec_mod = misc::create_2dvec_from_1dvecs(inp_vec_mod, operator);
        let inp_vec_mod = operation_two_operands(inp_vec_mod, operator.chars().next().unwrap());
        {
            let mut inp_vec_temp = new_inp_vec[..l_ind].to_vec();
            inp_vec_temp.extend(inp_vec_mod.clone());
            inp_vec_temp.extend(new_inp_vec[r_ind + 1..].to_vec());
            new_inp_vec = inp_vec_temp;
        }

        if inp_vec_mod.len() - 1 < r_ind - l_ind {
            r_ind = l_ind + inp_vec_mod.len() - 1;
        }

    }

    // println!("bot new_inp_vec: {:?}", new_inp_vec);
    return new_inp_vec;
}

fn format_inp(mut inp: String) -> String {
    inp = inp.replace(" ", "").replace("\n", "");
    // add multiplication in between number <x> combo
    // -- --
    for num in 0..=9 {
        for c in FUNCS {
            inp = inp.replace(
                &format!("{}{}", num, c).to_string(),
                &format!("{}*{}", num, c).to_string(),
            );
        }
        for c in CONSTS {
            inp = inp.replace(
                &format!("{}{}", num, c.0).to_string(),
                &format!("{}*{}", num, c.0).to_string(),
            );
        }
        for c in L_BRAC {
            inp = inp.replace(
                &format!("{}{}", num, c).to_string(),
                &format!("{}*{}", num, c).to_string(),
            );
        }
        for c in R_BRAC {
            inp = inp.replace(
                &format!("{}{}", c, num).to_string(),
                &format!("{}*{}", c, num).to_string(),
            );
        }
    }
    for (t, _) in CONSTS {
        for c in R_BRAC {
            inp = inp.replace(
                &format!("{}{}", c, t).to_string(),
                &format!("{}*{}", c, t).to_string(),
            );
        }
        for c in L_BRAC {
            inp = inp.replace(
                &format!("{}{}", t, c).to_string(),
                &format!("{}*{}", t, c).to_string(),
            );
        }
    }

    // Add padding around operations
    // -- --
    for op in OPERATORS {
        inp = inp.replace(op, &format!(" {} ", op));
    }

    for c in FUNCS {
        inp = inp.replace(
            &format!("{}", c).to_string(),
            &format!(" {} ", c).to_string(),
        );
    }
    // Add padding around Bracs
    // -- --
    inp = inp.replace("[", " [ ");
    inp = inp.replace("]", " ] ");
    inp = inp.replace("{", " { ");
    inp = inp.replace("}", " } ");
    inp = inp.replace("(", " ( ");
    inp = inp.replace(")", " ) ");
    // -- --

    // remove double spaces
    while inp.contains("  ") {
        inp = inp.replace("  ", " ");
    }
    return inp;
}

fn unequal_brac(inp_vec: Vec<String>, mut missed_bracs: i32) -> (Vec<String>, i32) {
    let mut brac_stack: Vec<String> = vec![];
    let mut new_inp_vec = inp_vec.clone();
    let mut brac_pos: Vec<usize> = vec![];
    for (ind, ch) in inp_vec.iter().enumerate() {
        if misc::is_string_lbrac(ch.clone()) {
            brac_stack.push(ch.clone());
            brac_pos.append(&mut vec![ind]);
        } else if misc::is_string_rbrac(ch.clone()) {
            if brac_stack.len() > 0 {
                if misc::compare_brac(brac_stack[brac_stack.len() - 1].clone(), ch.clone()) == 1 {
                    new_inp_vec.insert(
                        brac_pos[brac_pos.len() - 1] + 1,
                        misc::give_lbrac_from_rbrac(ch.clone()),
                    );
                    missed_bracs = missed_bracs + 1;
                } else if misc::compare_brac(brac_stack[brac_stack.len() - 1].clone(), ch.clone())
                    == -1
                {
                    new_inp_vec.insert(
                        ind,
                        misc::give_rbrac_from_lbrac(brac_stack[brac_stack.len() - 1].clone()),
                    );
                    missed_bracs = missed_bracs + 1;
                    brac_stack.pop();
                    brac_pos.pop();
                } else {
                    brac_stack.pop();
                    brac_pos.pop();
                }
            } else {
                new_inp_vec.insert(0, misc::give_lbrac_from_rbrac(ch.clone()));
            }
        }
    }
    while brac_stack.len() > 0 {
        new_inp_vec.extend(vec![misc::give_rbrac_from_lbrac(
            brac_stack[brac_stack.len() - 1].clone(),
        )]);
        missed_bracs = missed_bracs + 1;
        brac_stack.pop();
    }
    return (new_inp_vec, missed_bracs);
}

fn operation_two_operands(oper_vec: Vec<Vec<String>>, op: char) -> Vec<String> {
    let mut temp_vec: Vec<String> = vec![];

    for exp_ind in 0..oper_vec.len() {
        for j in 0..oper_vec[exp_ind].len() {
            temp_vec.push(oper_vec[exp_ind][j].clone());
        }
    }

    let mut i = 0;
    if temp_vec.len() > 1 {
        loop {
            let res: String;
            if misc::is_string_numeric(temp_vec[i].clone())
                && misc::is_string_numeric(temp_vec[i + 1].clone())
            {
                let re_complex =
                    Regex::new(r"([0-9]*\.?[0-9]*)(\+|\-)?([0-9]+\.?[0-9]*i)").unwrap();

                if re_complex.is_match(&temp_vec[i]) || re_complex.is_match(&temp_vec[i + 1]) {
                    let first_el: Complex<f64> = misc::string_to_cmplx(temp_vec[i].clone());
                    let sec_sign = if op == '-' { "-" } else { "" };
                    let sec_el: Complex<f64> =
                        misc::string_to_cmplx(sec_sign.to_string() + &temp_vec[i + 1]);
                    res = match op {
                        'C' => {
                            println!("{}", "Complex Numbers cannot have combination".red());
                            std::process::exit(0);
                        }
                        'P' => {
                            println!("{}", "Complex Numbers cannot have permutation".red());
                            std::process::exit(0);
                        }
                        '^' => (first_el.powc(sec_el)).to_string(),
                        '/' => (first_el / sec_el).to_string(),
                        '*' => (first_el * sec_el).to_string(),
                        '-' => (first_el - sec_el).to_string(),
                        '+' => (first_el + sec_el).to_string(),
                        _ => String::from(""),
                    };
                } else {
                    let first_el: f64 = misc::string_to_num(temp_vec[i].clone());
                    let sec_el: f64 = misc::string_to_num(temp_vec[i + 1].clone());
                    res = match op {
                        'C' => {
                            if first_el == first_el.round() {
                                (misc::factorial(first_el as i32) as f64
                                    / ((misc::factorial(sec_el as i32) as f64)
                                        * (misc::factorial((first_el - sec_el) as i32) as f64)))
                                    .to_string()
                            } else {
                                println!("{}", "Non Integers cannot have combination".red());
                                std::process::exit(0);
                            }
                        }
                        'P' => {
                            if first_el == first_el.round() {
                                (misc::factorial(first_el as i32) as f64
                                    / (misc::factorial((first_el - sec_el) as i32) as f64))
                                    .to_string()
                            } else {
                                println!("{}", "Non Integers cannot have permutation".red());
                                std::process::exit(0);
                            }
                        }
                        '^' => round_nums(first_el.powf(sec_el)).to_string(),
                        '/' => round_nums(first_el / sec_el).to_string(),
                        '*' => round_nums(first_el * sec_el).to_string(),
                        '-' => round_nums(first_el - sec_el).to_string(),
                        '+' => round_nums(first_el + sec_el).to_string(),
                        _ => String::from(""),
                    };
                }
                temp_vec.remove(i + 1);
            } else if temp_vec.len()>1 && (temp_vec[i].contains("x") || temp_vec[i+1].contains("x")) && !(misc::is_string_operator(temp_vec[i+1].clone())) && !(misc::is_string_operator(temp_vec[i].clone())) && !(misc::is_string_lbrac(temp_vec[i+1].clone())) && !(misc::is_string_rbrac(temp_vec[i+1].clone())) && !(misc::is_string_lbrac(temp_vec[i].clone())) && !(misc::is_string_rbrac(temp_vec[i].clone())) {
                // x simplification
                    let first_el = temp_vec[i].clone();
                    let sec_el = temp_vec[i+1].clone();
                    let re_poly = Regex::new(r"((0-9)*)?x((0-9)*)?").unwrap();
                    let mut max_power:i32 = 0;

                    res = match op {
                        '*' => {
                            let elements = vec![first_el.split("+").into_iter(), sec_el.split("+").into_iter()];
                            let mut coef:Vec<Vec<f64>> = vec![vec![], vec![]];

                            for i in 0..coef.len(){
                                max_power = 0;
                                for el in elements[i].clone(){
                                    let coef_vals:Vec<&str>;
                                    if re_poly.is_match(el){
                                        let temp_el = &el.replace("x", " x ");
                                        coef_vals = temp_el.split(" ").collect();
                                        let diff = coef_vals[2].parse::<i32>().unwrap_or(1) - max_power;
                                        if diff > 0 {
                                            max_power = coef_vals[2].parse::<i32>().unwrap_or(1);
                                            for _ in 0..=diff {
                                                coef[i].push(0.0);
                                            }
                                        }
                                        for j in 0..coef[i].len() {
                                            if j == coef_vals[2].parse::<usize>().unwrap_or(1) {
                                                coef[i][j] = coef_vals[0].parse::<f64>().unwrap_or(1.0);
                                            }
                                        }
                                    } else{
                                        if coef[i].len() == 0 {
                                            coef[i].push(el.parse::<f64>().unwrap_or(0.0));
                                        } else {
                                            coef[i][0] = coef[i][0] + el.parse::<f64>().unwrap_or(0.0);
                                        }
                                    }
                                }
                            }
                            let mut out_coef = vec![];
                            for _ in 0..(coef[0].len()*coef[1].len()){
                                out_coef.push(0.0);
                            }
                            for (f_i, f) in coef[0].clone().into_iter().enumerate(){
                                for (s_i, s) in coef[1].clone().into_iter().enumerate(){
                                        out_coef[f_i+s_i] = out_coef[f_i+s_i] + f*s;
                                }
                            }

                            let mut output = "".to_string();
                            for (o_i, o) in out_coef.iter().enumerate(){
                                if *o != 0.0 {
                                    if o_i == 0 {
                                        output = output + &format!("{}+", o);
                                        continue;
                                    }
                                    output = output + &format!("{}x{}+", o, if o_i == 1 {"".to_string()} else {o_i.to_string()});
                                }
                            }
                            temp_vec.remove(i + 1);
                            output[0..output.len()-1].to_string()
                        },
                        '+' => {
                            let mut coef:Vec<f64> = vec![];
                            let sum = first_el.clone() + "+" + &sec_el;

                            let sum_el = sum.split("+").into_iter();
                            for el in sum_el{
                                let coef_vals:Vec<&str>;
                                if re_poly.is_match(el){
                                    let temp_el = &el.replace("x", " x ");
                                    coef_vals = temp_el.split(" ").collect();
                                    let diff = coef_vals[2].parse::<i32>().unwrap_or(1) - max_power;
                                    if diff > 0 {
                                        max_power = coef_vals[2].parse::<i32>().unwrap_or(1);
                                        for _ in 0..=diff {
                                            coef.push(0.0);
                                        }
                                    }
                                    coef[coef_vals[2].parse::<usize>().unwrap_or(1)] = coef[coef_vals[2].parse::<usize>().unwrap_or(1)] + coef_vals[0].parse::<f64>().unwrap();
                                } else{
                                    if coef.len() == 0 {
                                        coef.push(el.parse::<f64>().unwrap());
                                    } else {
                                        coef[0] = coef[0] + el.parse::<f64>().unwrap();
                                    }
                                }
                            }
                            let mut output_val = "".to_string();
                            for (power, coefficient) in coef.iter().enumerate() {
                                if coefficient != &0.0 {
                                    if power == 0 {
                                        output_val = output_val + &format!("{}+", coefficient);
                                    }else{
                                        output_val = output_val + &format!("{}x{}+", coefficient, if power == 1 {"".to_string()} else {power.to_string()});
                                    }
                                }
                            }
                            temp_vec.remove(i + 1);
                            output_val[..output_val.len()-1].to_string()
                        },
                        _ => {
                            i = i + 1;
                            temp_vec[i].clone()
                        },
                    };
                    // temp_vec.remove(i);

            } else {
                res = temp_vec[i+1].clone();
                i = i + 1;
            }
            temp_vec[i] = res;
            // println!("len:{},i:{}", temp_vec.len(), i);
            // println!("Temp_Vec: {:?}", temp_vec);
            // println!("Operator: {:?}", op);
            if i >= temp_vec.len() - 1 {
                break;
            }
        }
    }
    return temp_vec;
}
fn trigonometry_cmplx(trig: &String, mut c: Complex<f64>, cur_modes: &Modes) -> Complex<f64> {
    if !cur_modes.rad {
        c = c * f64::consts::PI / 180.0;
    }
    // INFO: cos(a+bi)=cosacoshb−isinasinhb.
    // sin(a+bi)=sinacoshb+icosasinhb.
    // cosh(a+bi)=coshacosb+isinhasinb.
    // sinh(a+bi)=sinhacosb+icoshasinb.

    let cos = Complex::new(
        round_nums(c.re.cos() * c.im.cosh()),
        round_nums(c.re.sin() * c.im.sinh()),
    );
    let sin = Complex::new(
        round_nums(c.re.sin() * c.im.cosh()),
        round_nums(c.re.cos() * c.im.sinh()),
    );
    let cosh = Complex::new(
        round_nums(c.re.cosh() * c.im.cos()),
        round_nums(c.re.sinh() * c.im.sin()),
    );
    let sinh = Complex::new(
        round_nums(c.re.sinh() * c.im.cos()),
        round_nums(c.re.cosh() * c.im.sin()),
    );
    if trig == "cos" {
        return cos;
    } else if trig == "sin" {
        return sin;
    } else if trig == "tan" {
        return sin / cos;
    } else if trig == "sinh" {
        return sinh;
    } else if trig == "cosh" {
        return cosh;
    } else if trig == "tanh" {
        return sinh / cosh;
    } else if trig == "cosec" {
        return 1.0 / sin;
    } else if trig == "sec" {
        return 1.0 / cos;
    } else if trig == "cot" {
        return cos / sin;
    } else if trig == "cosech" {
        return 1.0 / sinh;
    } else if trig == "sech" {
        return 1.0 / cosh;
    } else if trig == "coth" {
        return cosh / sinh;
    } else {
        return Complex::new(0_f64, 0_f64);
    }
}

fn trigonometry(trig: &String, mut n: f64, cur_modes: &Modes) -> f64 {
    if !cur_modes.rad {
        n = n * f64::consts::PI / 180.0;
    }
    if trig == "cos" {
        return n.cos();
    } else if trig == "sin" {
        return n.sin();
    } else if trig == "tan" {
        return n.tan();
    } else if trig == "sinh" {
        return n.sinh();
    } else if trig == "cosh" {
        return n.cosh();
    } else if trig == "tanh" {
        return n.tanh();
    } else if trig == "cosec" {
        return 1.0 / n.sin();
    } else if trig == "sec" {
        return 1.0 / n.cos();
    } else if trig == "cot" {
        return 1.0 / n.tan();
    } else if trig == "cosech" {
        return 1.0 / n.sinh();
    } else if trig == "sech" {
        return 1.0 / n.cosh();
    } else if trig == "coth" {
        return 1.0 / n.tanh();
    } else {
        return 0.0;
    }
}

fn constants(inp_vec: Vec<String>) -> Vec<String> {
    let mut new_inp_vec = inp_vec.clone();
    for ind in 0..new_inp_vec.len() {
        for c in CONSTS {
            new_inp_vec[ind] = if new_inp_vec[ind] == c.0 {
                c.1.to_string()
            } else {
                new_inp_vec[ind].clone()
            }
        }
    }
    return new_inp_vec;
}

fn functions(oper_vec: Vec<String>, cur_modes: &Modes) -> Vec<String> {
    let re_log = Regex::new(r"^-?log$").unwrap();
    let re_trig = Regex::new(r"^-?((cos|sin|tan|sec|cosec|cot)h?)$").unwrap();
    let re_func_sep = Regex::new(r"^-?(cos|sin|tan|sec|cosec|cot|log(_[0-9]+_)?)h?$").unwrap();
    let mut temp_vec = oper_vec.clone();
    let mut ind = 0;
    let mut neg_val: bool = false;
    while ind < temp_vec.len() {
        let mut operand_vec: Vec<String> = vec![];
        let mut out: String;
        // Minus For any function
        // -- --
        if re_trig.is_match(&temp_vec[ind].to_lowercase())
            || re_log.is_match(&temp_vec[ind].to_lowercase())
        {
            neg_val = temp_vec[ind].starts_with("-");
            if neg_val {
                temp_vec[ind] = temp_vec[ind][1..].to_string();
            }
        }
        // -- --

        // Create a vector of operands
        // -- --
        if re_func_sep.is_match(&temp_vec[ind].to_lowercase()) {
            let mut cur_ind = ind + 1;
            while misc::is_string_numeric(temp_vec[cur_ind].clone()) {
                operand_vec.push(temp_vec[cur_ind].clone());
                if cur_ind < temp_vec.len() - 1 {
                    cur_ind = cur_ind + 1;
                } else {
                    break;
                }
            }
        }
        // -- --

        // Get output of trigs
        // -- --
        if re_trig.is_match(&temp_vec[ind].to_lowercase()) {
            if operand_vec.len() == 1 {
                if operand_vec[0].contains("i") {
                    let val = misc::string_to_cmplx(operand_vec[0].clone());
                    let mut out_num =
                        trigonometry_cmplx(&temp_vec[ind].to_lowercase(), val, &cur_modes);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                } else {
                    let mut out_num = trigonometry(
                        &temp_vec[ind].to_lowercase(),
                        misc::string_to_num(operand_vec[0].clone()),
                        &cur_modes,
                    );
                    out_num = round_nums(out_num);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                }
            } else {
                println!("{}", "Trigonometry is supported for one operand only".red());
                std::process::exit(0);
            }
            temp_vec.remove(ind);
            temp_vec.remove(ind);
            temp_vec.insert(ind, out);
        }
        // -- --

        // Logarithm
        // -- --
        if re_log.is_match(&temp_vec[ind].to_lowercase()) {
            if operand_vec.len() == 1 {
                if operand_vec[0].contains("i") {
                    let val = misc::string_to_cmplx(operand_vec[0].clone());
                    let mag = (val.re.powi(2) + val.im.powi(2)).sqrt();
                    let arg = (val.im / val.re).atan();
                    let mut out_num = Complex::new(mag.ln(), arg);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                } else {
                    let mut out_num = misc::string_to_num(operand_vec[0].clone()).ln();
                    out_num = round_nums(out_num);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                }
                temp_vec.remove(ind);
                temp_vec.remove(ind);
                temp_vec.insert(ind, out);
            } else if operand_vec.len() == 2 {
                let base = misc::string_to_num(operand_vec[0].clone());
                if base < 0.0 {
                    println!("{}", "Logarithm is not defined for negative bases".red());
                    std::process::exit(0);
                }
                if operand_vec[1].contains("i") {
                    let val = misc::string_to_cmplx(operand_vec[1].clone());
                    let mag = (val.re.powi(2) + val.im.powi(2)).sqrt();
                    let arg = (val.im / val.re).atan();
                    let mut out_num = Complex::new(mag.log(base), arg);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                } else {
                    let mut out_num = misc::string_to_num(operand_vec[1].clone()).log(base);
                    out_num = round_nums(out_num);
                    if neg_val {
                        out_num = -out_num;
                    }
                    out = out_num.to_string();
                }
                temp_vec.remove(ind);
                temp_vec.remove(ind);
                temp_vec.remove(ind);
                temp_vec.insert(ind, out);
            } else {
                println!(
                    "{}",
                    "Logarithm is supported for one or two operands only".red()
                );
                std::process::exit(0);
            }
        }
        ind = ind + 1;
    }
    // -- --
    return temp_vec;
}

fn operation_one_operand(oper_vec: Vec<String>) -> Vec<String> {
    let mut temp_vec = oper_vec.clone();
    let mut ind = 0;
    while ind < temp_vec.len() {
        if temp_vec[ind] == "!" {
            // Assume no bracs left in operands

            match oper_vec[ind - 1].parse::<i32>() {
                Ok(value) => {
                    // Call the factorial function with the parsed integer
                    let out = misc::factorial(value);
                    temp_vec.remove(ind - 1);
                    temp_vec.insert(ind - 1, out.to_string());
                    temp_vec.remove(ind);
                }
                Err(_) => {
                    // Print a user-friendly error message
                    eprintln!("{}", "Factorials can only be taken for integers!".red());
                    temp_vec.remove(ind);
                }
            }
        }
        ind = ind + 1;
    }
    return temp_vec;
}

struct Modes {
    rad: bool,
    alias: bool,
    eqn: bool,
}

impl Modes {
    fn mode_status(&self, mode_name: &str, active: bool) -> String {
        let status = if active { "Enabled" } else { "Disabled" };
        let color = if active { Color::Green } else { Color::Red };
        format!("{}: {}", mode_name.blue(), status.color(color))
    }
}

struct Alias {
    value: String,
    alias: String,
    nodes: Vec<Alias>,
}
impl Alias {
    fn unwrap(
        &self,
        temp_self: &Alias,
        mut storage: Vec<(String, String)>,
    ) -> Vec<(String, String)> {
        if temp_self.nodes.len() == 0 {
            return vec![(temp_self.alias.clone(), temp_self.value.clone())];
        } else {
            for i in 0..temp_self.nodes.len() {
                let child = temp_self.unwrap(&temp_self.nodes[i], storage.clone());
                storage.extend(
                    child
                    .iter()
                    .map(|c| (temp_self.alias.clone() + &c.0, c.1.clone())),
                );
                if temp_self.alias != "" {
                    storage.extend(vec![(temp_self.alias.clone(), temp_self.value.clone())]);
                }
            }
            return storage;
        }
    }
}

fn main() {
    let mut cur_modes = Modes {
        rad: false,
        alias: false,
        eqn: false,
    };
    let cur_aliases = Alias {
        value: "".to_string(),
        alias: "".to_string(),
        nodes: vec![
            Alias {
                value: "6".to_string(),
                alias: "s".to_string(),
                nodes: vec![Alias {
                    value: "7".to_string(),
                    alias: "e".to_string(),
                    nodes: vec![],
                }],
            },
            Alias {
                value: "2".to_string(),
                alias: "t".to_string(),
                nodes: vec![Alias {
                    value: "3".to_string(),
                    alias: "h".to_string(),
                    nodes: vec![],
                }],
            },
            Alias {
                value: "0".to_string(),
                alias: "z".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "1".to_string(),
                alias: "o".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "4".to_string(),
                alias: "f".to_string(),
                nodes: vec![Alias {
                    value: "5".to_string(),
                    alias: "i".to_string(),
                    nodes: vec![],
                }],
            },
            Alias {
                value: "8".to_string(),
                alias: "e".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "9".to_string(),
                alias: "n".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "+".to_string(),
                alias: "p".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "*".to_string(),
                alias: "x".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "/".to_string(),
                alias: "b".to_string(),
                nodes: vec![],
            },
            Alias {
                value: "-".to_string(),
                alias: "m".to_string(),
                nodes: vec![],
            },
            ],
    };
    let mut vec_alias = cur_aliases.unwrap(&cur_aliases, vec![]);
    vec_alias.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());
    vec_alias.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    vec_alias.dedup();

    println!("{}", "SHARK Calculator ".magenta());
    let mut history_path = my_home().unwrap().unwrap();
    history_path.push(".history.txt");
    let history_path = history_path.into_os_string().into_string().unwrap();
    'fullblock: loop {
        let mut op_vec: Vec<Vec<String>> = vec![];
        let mut inp;
        let mut rl = DefaultEditor::new().expect("Readline Issues");
        if let Err(_) = rl.load_history(&history_path) {
            println!("No past history.");
        }

        let readline = rl.readline("\n=> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("No history found");
                inp = line;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting");
                break 'fullblock;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break 'fullblock;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break 'fullblock;
            }
        }
        rl.save_history(&history_path)
            .expect("Could not store in history.txt");

        if inp == "h".to_string() || inp == "help".to_string() {
            let title = "SHARK:".blue();

            let help_page = "HELP PAGE (h)".green();
            let features = vec![
                (
                    "1. Brackets Auto Completion",
                    "(Try using incomplete bracket combinations)",
                ),
                ("2. Trigonometry Integration", "(Try `sin` or `sinh`)"),
                ("3. Built in Constants", "(Try `e` or `pi`)"),
                ("4. Different Modes:", "(Try `modes`)"),
                ("5. Aliases for faster calculations:", "(Try `aliases`)"),
            ];
            let formatted_features: String = features
                .iter()
                .map(|&feature| format!("{} {}\n", feature.0.blue(), feature.1.green()))
                .collect();

            println!("{}\n\n{}\n\n{}", title, help_page, formatted_features);
            continue;
        } else if inp == "".to_string() {
            continue;
        } else if inp == "q".to_string() {
            std::process::exit(0);
        } else if inp == "aliases".to_string() {
            let title = "ALIASES".blue();

            let mut aliases = vec_alias.clone();
            aliases.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let formatted_features: String = aliases
                .iter()
                .map(|feature| format!("{} {}\n", feature.0.blue(), feature.1.green()))
                .collect();
            println!("{}\n\n{}", title, formatted_features);
            continue;
        } else if inp == "modes" {
            let mut close_modes = false;
            while !close_modes {
                let mode_inp: String;
                let title = "MODES MENU".yellow();

                let mut rad_mode = cur_modes.mode_status("Radian Mode", cur_modes.rad);
                let mut alias_mode = cur_modes.mode_status("Alias Input", cur_modes.alias);
                let mut eqn_mode = cur_modes.mode_status("Equation Solver", cur_modes.eqn);

                println!(
                    "{}\n\n1. {} \n2. {} \n3. {} \n\n{}",
                    title,
                    rad_mode,
                    alias_mode,
                    eqn_mode,
                    "Choose respective number to toggle:".green()
                );

                let readline_modes = rl.readline("\nModes Number => ");
                match readline_modes {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str())
                            .expect("No history found");
                        mode_inp = line;
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("Exiting");
                        break 'fullblock;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break 'fullblock;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break 'fullblock;
                    }
                }
                if mode_inp == "1" {
                    cur_modes.rad = !cur_modes.rad;
                    rad_mode = cur_modes.mode_status("Radian Mode", cur_modes.rad);
                    println!("{}\n ", rad_mode);
                } else if mode_inp == "2" {
                    cur_modes.alias = !cur_modes.alias;
                    alias_mode = cur_modes.mode_status("Alias Input", cur_modes.alias);
                    println!("{}\n ", alias_mode);
                } else if mode_inp == "3" {
                    cur_modes.eqn = !cur_modes.eqn;
                    eqn_mode = cur_modes.mode_status("Equation Solver", cur_modes.eqn);
                    println!("{}\n ", eqn_mode);
                } else if mode_inp == "q" {
                    close_modes = true;
                    continue;
                }
            }
            continue;
        }

        // Aliasing
        //-- --
        if cur_modes.alias {
            for i in vec_alias.iter().rev() {
                inp = inp.replace(&i.0, &i.1.to_string());
            }
            println!("{}", "Aliasing is on!".green());
        }
        //-- --

        // Separate each exps in input in a vector
        // -- --
        inp = format_inp(inp);
        //-- --

        let mut inp_vec = misc::create_vecs_from_str(inp, " ");
        // println!("@main inp_vec{:?}", inp_vec);
        // -- --
        let mut output_obtained = false;

        while !output_obtained {
            // Brackets Autocomplete
            // -- --
            let mut brac_found: bool = false;
            let mut missed_bracs = 0;
            let mut inp_vec_prev: Vec<String> = vec![];
            while inp_vec_prev != inp_vec {
                inp_vec_prev = inp_vec.clone();
                (inp_vec, missed_bracs) = unequal_brac(inp_vec, missed_bracs);
            }

            if missed_bracs > 0 {
                println!(
                    "{} {}",
                    "⚠︎ Warning!".yellow(),
                    "Brackets were not all matched.",
                )
            }
            // -- --
            let mut l_ind = 0;
            let mut r_ind = inp_vec.len() - 1;

            // In Brackets Crop
            // -- --
            for (ind, c) in inp_vec.iter().enumerate() {
                if misc::is_string_lbrac((*c).clone()) {
                    l_ind = ind;
                } else if misc::is_string_rbrac((*c).clone()) {
                    r_ind = ind;
                    brac_found = true;
                    break;
                }
            }
            // -- --
            // Operation
            inp_vec = simplify(inp_vec, l_ind, r_ind, &mut op_vec, &cur_modes);

            op_vec.push(inp_vec.clone());
            // Get rid of the extra brackets around
            // NOTE: dont rely on r_ind as it will be different from the true one at this point
            if brac_found == true {
                let mut inp_vec_temp = inp_vec[..l_ind].to_vec();
                inp_vec_temp.extend(vec![inp_vec[l_ind + 1].clone()]);
                inp_vec_temp.extend(inp_vec[l_ind + 3..].to_vec());
                inp_vec = inp_vec_temp;
                op_vec.push(inp_vec.clone());
            }
            // break;
            // println!("@main while inp_vec{:?}", inp_vec);

            if inp_vec.len() <= 1 {
                output_obtained = true;
            }
        }
        format_out(&mut op_vec);
        // -- --
    }
    // -- --
}

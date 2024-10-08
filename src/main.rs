// TODO: config, eqn, integration, derivative, plot, etc

use core::f64;
use misc::{is_string_lbrac, is_string_rbrac};
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
        for (el_i, el) in vec.iter().enumerate() {
            let el_contains_op = el.contains("+")
                || el.contains("-")
                || el.contains("*")
                || el.contains("/")
                || el.contains("^")
                || el.contains("!")
                || el.contains("%")
                || el.contains("C")
                || el.contains("P");
            if el_contains_op && el.contains("x") {
                if el_i > 0 {
                    if is_string_lbrac(vec[el_i - 1].clone())
                        && is_string_rbrac(vec[el_i + 1].clone())
                    {
                        out_buf = out_buf + &format!("{}", el);
                        continue;
                    }
                }
                out_buf = out_buf + &format!("( {} )", el);
                continue;
            }
            out_buf = out_buf + &format!(" {} ", el);
        }
        // remove double spaces
        while out_buf.contains("  ") {
            out_buf = out_buf.replace("  ", " ");
        }
        // replacements
        out_buf = out_buf.replace("+ -", "-");
        out_buf = out_buf.replace("+-", "-");

        // trim end spaces
        out_buf = out_buf
            .trim()
            .to_string()
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
        for op in L_BRAC {
            out_string_vec[ind] = out_string_vec[ind].replace(op, &format!(" {} ", op));
        }
        for op in R_BRAC {
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
            } else if el == "=" {
                out_buf = out_buf + &format!(" {} ", el.magenta());
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
    inp = inp.replace("+-", "-");
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

    inp = inp.replace("=", " = ");

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
                missed_bracs = missed_bracs + 1;
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
                        '%' => round_nums(first_el % sec_el).to_string(),
                        '*' => round_nums(first_el * sec_el).to_string(),
                        '-' => round_nums(first_el - sec_el).to_string(),
                        '+' => round_nums(first_el + sec_el).to_string(),
                        _ => String::from(""),
                    };
                }
                temp_vec.remove(i + 1);
            } else if temp_vec.len() > 1
                && (temp_vec[i].contains("x") || temp_vec[i + 1].contains("x"))
                && !(misc::is_string_operator(temp_vec[i + 1].clone()))
                && !(misc::is_string_operator(temp_vec[i].clone()))
                && !(misc::is_string_lbrac(temp_vec[i + 1].clone()))
                && !(misc::is_string_rbrac(temp_vec[i + 1].clone()))
                && !(misc::is_string_lbrac(temp_vec[i].clone()))
                && !(misc::is_string_rbrac(temp_vec[i].clone()))
            {
                // x simplification
                let first_el = temp_vec[i].clone();
                let sec_el = temp_vec[i + 1].clone();

                res = match op {
                    '^' => {
                        temp_vec.remove(i + 1);
                        poly_power(first_el, sec_el)
                    }
                    '/' => {
                        temp_vec.remove(i + 1);
                        poly_division(first_el, sec_el)
                    }
                    '*' => {
                        temp_vec.remove(i + 1);
                        poly_multiplication(first_el, sec_el)
                    }
                    '+' => {
                        temp_vec.remove(i + 1);
                        poly_addition(first_el, sec_el)
                    }
                    _ => {
                        i = i + 1;
                        temp_vec[i].clone()
                    }
                };

                // temp_vec.remove(i);
            } else {
                res = temp_vec[i + 1].clone();
                i = i + 1;
            }
            temp_vec[i] = res;
            if i >= temp_vec.len() - 1 {
                break;
            }
        }
    }
    return temp_vec;
}

fn poly_power(first_el: String, sec_el: String) -> String {
    if !misc::is_string_numeric(sec_el.to_string()) {
        println!("{}", "Cannot raise to the power of a non-integer".red());
        std::process::exit(0);
    }
    let mut temp_el = first_el.clone();
    for _ in 0..sec_el.trim().parse::<i32>().unwrap_or(1) - 1 {
        temp_el = poly_multiplication(temp_el.clone(), first_el.clone());
    }
    temp_el
}

fn poly_division(first_el: String, sec_el: String) -> String {
    let elements = vec![
        first_el.split("+").into_iter(),
        sec_el.split("+").into_iter(),
    ];
    let mut coef: Vec<Vec<f64>> = create_coefs(elements);
    let mut log_coef: Vec<f64> = vec![];
    let mut log_deg: Vec<f64> = vec![];

    let mut p_coef;
    let mut p_deg = coef[0].len() as i32 - coef[1].len() as i32;

    while p_deg >= 0 {
        p_coef = coef[0][coef[0].len() - 1] / coef[1][coef[1].len() - 1];

        log_coef.push(p_coef);
        log_deg.push(p_deg as f64);

        // Multiply by reqd coef
        for ind in 0..coef[1].len() {
            coef[1][ind] = coef[1][ind] * p_coef;
        }

        // Multiply by x that many times
        for _ in 0..p_deg {
            coef[1].insert(0, 0.0);
        }

        // Subtract
        for ind in 0..coef[1].len() {
            coef[0][ind] = coef[0][ind] - coef[1][ind];
        }
        let mut new_coef = coef[0].clone();

        while new_coef[new_coef.len() - 1] == 0.0 {
            let v = new_coef.len() - 1;
            new_coef.remove(v);
            if new_coef.len() == 0 {
                break;
            }
        }
        coef[0] = new_coef;

        // Revert the coef[1] to the original
        for _ in 0..p_deg {
            coef[1].remove(0);
        }
        for ind in 0..coef[1].len() {
            coef[1][ind] = round_nums(coef[1][ind] / p_coef);
        }

        p_deg = coef[0].len() as i32 - coef[1].len() as i32;
    }

    let mut output = String::from("");
    for ind in 0..log_deg.len() {
        if log_deg[ind] == 0.0 {
            output = output + &format!("{}+", log_coef[ind]);
        } else {
            output = output
                + &format!(
                    "{}x{}+",
                    if log_coef[ind] == 1.0 {
                        "".to_string()
                    } else {
                        log_coef[ind].to_string()
                    },
                    if log_deg[ind] == 1.0 {
                        "".to_string()
                    } else {
                        log_deg[ind].to_string()
                    }
                );
        }
    }
    if coef[0].len() != 0 {
        output = output + "(";
        for ind in 0..coef[0].len() {
            if coef[0][ind] == 0.0 {
                continue;
            }
            if ind == 0 {
                output = output + &format!("{}+", coef[0][ind]);
            } else {
                output = output
                    + &format!(
                        "{}x{}+",
                        if coef[0][ind] == 1.0 {
                            "".to_string()
                        } else {
                            coef[0][ind].to_string()
                        },
                        if ind == 1 {
                            "".to_string()
                        } else {
                            ind.to_string()
                        }
                    );
            }
        }
        output = output[0..output.len() - 1].to_string();
        (output + ")/(" + &sec_el + ")").to_string()
    } else {
        output[0..output.len() - 1].to_string()
    }
}

fn poly_addition(first_el: String, sec_el: String) -> String {
    let mut coef: Vec<f64> = vec![];
    let sum = first_el.clone() + "+" + &sec_el;
    let re_poly = Regex::new(r"((0-9)*)?x((0-9)*)?").unwrap();
    let mut max_power: i32 = 0;

    let sum_el = sum
        .split("+")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    for el in sum_el {
        let f_el = format_inp(el.clone());
        let f_el_vec = misc::create_vecs_from_str(f_el, " ");

        if unequal_brac(f_el_vec, 0).1 == 0 {
            let coef_vals: Vec<&str>;
            if re_poly.is_match(&el) {
                let temp_el = &el.replace("x", " x ");
                coef_vals = temp_el.split(" ").collect();
                let diff = coef_vals[2].parse::<i32>().unwrap_or(1) - max_power;
                if diff > 0 {
                    max_power = coef_vals[2].parse::<i32>().unwrap_or(1);
                    for _ in 0..=diff {
                        coef.push(0.0);
                    }
                }
                coef[coef_vals[2].parse::<usize>().unwrap_or(1)] = coef
                    [coef_vals[2].parse::<usize>().unwrap_or(1)]
                    + coef_vals[0].parse::<f64>().unwrap_or(1.0);
            } else {
                if coef.len() == 0 {
                    coef.push(el.parse::<f64>().unwrap());
                } else {
                    coef[0] = round_nums(coef[0] + el.parse::<f64>().unwrap());
                }
            }
        }
    }
    let mut output_val = "".to_string();
    for (power, coefficient) in coef.iter().enumerate().rev() {
        if coefficient != &0.0 {
            if power == 0 {
                output_val = output_val + &format!("{}+", coefficient);
            } else {
                output_val = output_val
                    + &format!(
                        "{}x{}+",
                        if *coefficient == 1.0 {
                            "".to_string()
                        } else {
                            coefficient.to_string()
                        },
                        if power == 1 {
                            "".to_string()
                        } else {
                            power.to_string()
                        }
                    );
            }
        }
    }
    if output_val == "" {
        return "0".to_string();
    } else {
        return output_val[..output_val.len() - 1].to_string();
    }
}

fn poly_multiplication(first_el: String, sec_el: String) -> String {
    let elements = vec![
        first_el.split("+").into_iter(),
        sec_el.split("+").into_iter(),
    ];
    let coef: Vec<Vec<f64>> = create_coefs(elements);
    let mut out_coef = vec![];
    for _ in 0..(coef[0].len() * coef[1].len()) {
        out_coef.push(0.0);
    }
    for (f_i, f) in coef[0].clone().into_iter().enumerate() {
        for (s_i, s) in coef[1].clone().into_iter().enumerate() {
            out_coef[f_i + s_i] = out_coef[f_i + s_i] + round_nums(f * s);
        }
    }

    let mut output = "".to_string();
    for (o_i, o) in out_coef.iter().enumerate().rev() {
        if *o != 0.0 {
            if o_i == 0 {
                output = output + &format!("{}+", o);
            } else {
                output = output
                    + &format!(
                        "{}x{}+",
                        if *o == 1.0 {
                            "".to_string()
                        } else {
                            o.to_string()
                        },
                        if o_i == 1 {
                            "".to_string()
                        } else {
                            o_i.to_string()
                        }
                    );
            }
        }
    }
    output[0..(output.len() - 1)].to_string()
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

fn create_coefs(elements: Vec<std::str::Split<'_, &str>>) -> Vec<Vec<f64>> {
    let re_poly = Regex::new(r"((0-9)*)?x((0-9)*)?").unwrap();
    let mut coef: Vec<Vec<f64>> = vec![vec![0.0], vec![0.0]];

    for i in 0..coef.len() {
        let mut max_power = 0;
        for el in elements[i].clone() {
            let coef_vals: Vec<&str>;
            if re_poly.is_match(el) {
                let temp_el = &el.replace("x", " x ");
                coef_vals = temp_el.split(" ").collect();
                let diff = coef_vals[2].parse::<i32>().unwrap_or(1) - max_power;
                if diff > 0 {
                    max_power = coef_vals[2].parse::<i32>().unwrap_or(1);
                    for _ in 0..=diff - 1 {
                        coef[i].push(0.0);
                    }
                }
                for j in 0..coef[i].len() {
                    if j == coef_vals[2].parse::<usize>().unwrap_or(1) {
                        coef[i][j] = coef_vals[0].parse::<f64>().unwrap_or(1.0);
                    }
                }
            } else {
                if coef[i].len() == 0 {
                    coef[i].push(el.parse::<f64>().unwrap_or(0.0));
                } else {
                    coef[i][0] = coef[i][0] + el.parse::<f64>().unwrap_or(0.0);
                }
            }
        }
    }
    return coef;
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

fn brac_handler(mut inp: String, cur_modes: &Modes) -> Vec<Vec<String>> {
    inp = format_inp(inp);

    let mut inp_vec = misc::create_vecs_from_str(inp, " ");
    // println!("@main inp_vec{:?}", inp_vec);
    // -- --
    let mut op_vec: Vec<Vec<String>> = vec![vec![]];
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

        let mut l_ind = 0;
        let mut r_ind = inp_vec.len() - 1;

        // In Brackets Crop

        for (ind, c) in inp_vec.iter().enumerate() {
            if misc::is_string_lbrac((*c).clone()) {
                l_ind = ind;
            } else if misc::is_string_rbrac((*c).clone()) {
                r_ind = ind;
                brac_found = true;
                break;
            }
        }

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
    return op_vec;
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

fn remove_surr_bracs(inp: String) -> String {
    let mut new_inp = inp.replace("[", "[ ");
    new_inp = new_inp.replace("]", " ] ");
    new_inp = new_inp.replace("{", " { ");
    new_inp = new_inp.replace("}", " } ");
    new_inp = new_inp.replace("(", " ( ");
    new_inp = new_inp.replace(")", " ) ");

    let mut inp_vec: Vec<String> = new_inp.split(' ').map(|s| s.to_string()).collect();

    let mut ind = 0;
    while ind < inp_vec.len() {
        if inp_vec[ind] == "" {
            inp_vec.remove(ind);
            continue;
        }
        ind = ind + 1;
    }

    if is_string_lbrac(inp_vec[0].clone()) && is_string_rbrac(inp_vec[inp_vec.len() - 1].clone()) {
        let test = inp_vec[1..inp_vec.len() - 1].join("");
        if unequal_brac(inp_vec.clone(), 0).1 == 0 {
            return test;
        } else {
            return inp_vec.join("");
        }
    } else {
        inp_vec.join("")
    }
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
        let mut input;
        let mut rl = DefaultEditor::new().expect("Readline Issues");
        if let Err(_) = rl.load_history(&history_path) {
            println!("No past history.");
        }

        let readline = rl.readline("\n=> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .expect("No history found");
                input = line;
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

        if input == "h".to_string() || input == "help".to_string() {
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
        } else if input == "".to_string() {
            continue;
        } else if input == "q".to_string() {
            std::process::exit(0);
        } else if input == "aliases".to_string() {
            let title = "ALIASES".blue();

            let mut aliases = vec_alias.clone();
            aliases.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let formatted_features: String = aliases
                .iter()
                .map(|feature| format!("{} {}\n", feature.0.blue(), feature.1.green()))
                .collect();
            println!("{}\n\n{}", title, formatted_features);
            continue;
        } else if input == "modes" {
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
                input = input.replace(&i.0, &i.1.to_string());
            }
            println!("{}", "Aliasing is on!".green());
        }
        //-- --

        // Separate each exps in input in a vector
        let mut handside: Vec<Vec<Vec<String>>> = vec![];
        for (ind, inp) in input.split('=').map(|s| s.to_string()).enumerate() {
            handside.push(brac_handler(inp, &cur_modes));
            handside[ind].remove(0);
        }

        let mut max_len = 0;
        let mut max_len_ind = 0;
        let no_of_hands = handside.len();

        for (op_ind, _) in handside.clone().into_iter().enumerate() {
            if max_len < handside[op_ind].len() {
                max_len = handside[op_ind].len();
                max_len_ind = op_ind;
            }
        }
        for l in 0..no_of_hands {
            for _ in handside[l].len()..max_len {
                let last_el = handside[l][handside[l].len() - 1].clone();
                handside[l].push(last_el);
            }
        }

        if no_of_hands == 2 {
            let mut rhs = handside[1][handside[1].len() - 1][0].clone();
            let mut lhs = handside[0][handside[0].len() - 1][0].clone();
            let mut rhs_op_hei = misc::operations_heirarchy(rhs.clone());
            let mut lhs_op_hei = misc::operations_heirarchy(lhs.clone());
            // get the poly part
            let x_side: String;
            let val_side: String;

            if rhs.contains('x') && !lhs.contains('x') {
                x_side = rhs.clone();
                val_side = "lhs".to_string();
            } else if lhs.contains('x') && !rhs.contains('x') {
                x_side = lhs.clone();
                val_side = "rhs".to_string();
            } else if rhs.contains('x') && lhs.contains('x') {
                if lhs.contains("/") {
                    x_side = rhs.clone();
                    val_side = "lhs".to_string();
                } else if rhs.contains("/") {
                    x_side = lhs.clone();
                    val_side = "rhs".to_string();
                } else {
                    x_side = lhs.clone();
                    val_side = "rhs".to_string();
                }
            } else {
                x_side = lhs.clone();
                val_side = "rhs".to_string();
            }
            let mut value;

            let re_higher_order = Regex::new(r"x[2-9]+").unwrap();

            if !re_higher_order.is_match(&x_side) && !x_side.contains("/") {
                if rhs == x_side {
                    let rhs_sum_const = rhs
                        .split('+')
                        .filter(|x| !x.contains('x'))
                        .collect::<Vec<&str>>()[0]
                        .to_string();
                    lhs = poly_addition(
                        lhs.clone(),
                        poly_multiplication(rhs_sum_const.clone(), "-1".to_string()),
                    );
                    rhs = poly_addition(
                        rhs,
                        poly_multiplication(rhs_sum_const.clone(), "-1".to_string()),
                    );
                    let rhs_coef = rhs.replace("x", "");
                    rhs = poly_division(rhs, rhs_coef.clone());
                    lhs = poly_division(lhs, rhs_coef.clone());
                }
                if lhs == x_side {
                    let lhs_sum_const = lhs
                        .split('+')
                        .filter(|x| !x.contains('x'))
                        .collect::<Vec<&str>>()[0]
                        .to_string();
                    lhs = poly_addition(
                        lhs,
                        poly_multiplication(lhs_sum_const.clone(), "-1".to_string()),
                    );
                    rhs = poly_addition(
                        rhs.clone(),
                        poly_multiplication(lhs_sum_const.clone(), "-1".to_string()),
                    );
                    let lhs_coef = lhs.replace("x", "");
                    rhs = poly_division(rhs, lhs_coef.clone());
                    lhs = poly_division(lhs, lhs_coef.clone());
                }

                handside[1].push(vec![rhs]);
                handside[0].push(vec![lhs]);
            } else {
                if val_side == "lhs" {
                    value = lhs.clone();
                } else {
                    value = rhs.clone();
                }

                while !misc::is_string_numeric(value) {
                    // println!("t lhs_op_hei: {:?}, lhs: {}", lhs_op_hei, lhs);
                    // println!("t rhs_op_hei: {:?}, rhs: {}", rhs_op_hei, rhs);
                    if lhs_op_hei.len() > 0 {
                        if lhs_op_hei[0].1 == "/" {
                            let lhs_prods = lhs.split('/').collect::<Vec<&str>>();
                            let lhs_denom = remove_surr_bracs(lhs_prods[1].to_string());

                            rhs = poly_multiplication(rhs, lhs_denom);
                            lhs = lhs_prods[0].to_string();
                            lhs_op_hei.remove(0);
                        } else if lhs_op_hei[0].1 == "+" && rhs == x_side {
                            let lhs_summer = lhs.split('+').collect::<Vec<&str>>();
                            rhs = poly_addition(
                                rhs,
                                poly_multiplication(lhs_summer[0].to_string(), "-1".to_string()),
                            );
                            lhs = lhs_summer[1..].join("+").to_string();
                            lhs_op_hei.remove(0);
                        }
                    }
                    if rhs_op_hei.len() > 0 {
                        if rhs_op_hei[0].1 == "/" {
                            let rhs_prods = rhs.split('/').collect::<Vec<&str>>();
                            let rhs_denom = remove_surr_bracs(rhs_prods[1].to_string());

                            lhs = poly_multiplication(lhs, rhs_denom);
                            rhs = rhs_prods[0].to_string();
                            rhs_op_hei.remove(0);
                        } else if rhs_op_hei[0].1 == "+" && lhs == x_side {
                            let rhs_summer = rhs.split('+').collect::<Vec<&str>>();
                            lhs = poly_addition(
                                lhs,
                                poly_multiplication(rhs_summer[0].to_string(), "-1".to_string()),
                            );
                            rhs = rhs_summer[1..].join("+").to_string();
                            rhs_op_hei.remove(0);
                        }
                    }
                    // println!("m lhs_op_hei: {:?}, lhs: {}", lhs_op_hei, lhs);
                    // println!("m rhs_op_hei: {:?}, rhs: {}", rhs_op_hei, rhs);

                    if val_side == "lhs" {
                        value = remove_surr_bracs(lhs.clone());
                    } else {
                        value = remove_surr_bracs(rhs.clone());
                    }
                    // println!("b lhs_op_hei: {:?}, lhs: {}", lhs_op_hei, lhs);
                    // println!("b rhs_op_hei: {:?}, rhs: {}", rhs_op_hei, rhs);
                }
                handside[1].push(vec![rhs.clone()]);
                handside[0].push(vec![lhs.clone()]);

                if val_side == "lhs" {
                    value = remove_surr_bracs(lhs.clone());
                    lhs = poly_addition(
                        remove_surr_bracs(lhs.clone()),
                        poly_multiplication(value.clone(), "-1".to_string()),
                    );
                    rhs = poly_addition(
                        remove_surr_bracs(rhs.clone()),
                        poly_multiplication(value.clone(), "-1".to_string()),
                    );
                } else {
                    value = remove_surr_bracs(rhs.clone());
                    lhs = poly_addition(
                        remove_surr_bracs(lhs.clone()),
                        poly_multiplication(value.clone(), "-1".to_string()),
                    );
                    rhs = poly_addition(
                        remove_surr_bracs(rhs.clone()),
                        poly_multiplication(value.clone(), "-1".to_string()),
                    );
                }
                handside[1].push(vec![rhs.clone()]);
                handside[0].push(vec![lhs.clone()]);
            }
        }

        // To combine each hands
        let mut temp_op_vec = handside[max_len_ind].clone();
        for hand in 0..no_of_hands {
            if hand != max_len_ind {
                for k in 0..handside[hand].len() {
                    let mut temp_op_val = handside[hand][k].clone();
                    temp_op_val.insert(0, " = ".to_string());
                    temp_op_vec[k].extend(temp_op_val);
                }
            }
        }

        // To print the op
        format_out(&mut temp_op_vec);
        // -- --
    }
    // -- --
}

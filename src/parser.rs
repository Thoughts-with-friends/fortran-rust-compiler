use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pest/fortran.pest"]
struct FortranParser;

pub fn parse(src: &str) {
    let pairs = FortranParser::parse(Rule::source, src).unwrap_or_else(|e| panic!("{}", e));

    let mut nest_times: usize = 0;

    for pair in pairs {
        match pair.as_rule() {
            Rule::program_keyword => print!("fn "),
            Rule::program_name => {
                println!("{}() {{", pair.as_str());
                nest_times += 1;
            }
            Rule::declare_variable => parse_declare_variable(pair, nest_times),
            Rule::assign_to_variable => parse_assign_to_variable(pair, nest_times),
            Rule::call_function => parse_call_func(pair, nest_times),
            Rule::do_statement => parse_do_statement(pair, nest_times),
            Rule::end_program_keyword => {
                println!("\n}}");
            }
            Rule::non_nest_new_line => (),
            Rule::num => print!("{}", pair.as_str()),
            Rule::add => print!(" + "),
            Rule::EOI => break,
            _ => error_message(pair),
        }
    }
}

fn print_offset(nest_times: usize) {
    let offset_str = String::from_utf8(vec![b' '; 4 * nest_times]).unwrap();
    print!("{offset_str}");
}

fn parse_declare_variable(pair: pest::iterators::Pair<Rule>, nest_times: usize) {
    let mut variable_name = "";
    let mut variable_type = "";

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                variable_name = pair.as_str();
            }
            Rule::variable_type => match pair.as_str() {
                "integer" => variable_type = "usize",
                "real" => variable_type = "f64",
                _ => error_message(pair),
            },
            _ => error_message(pair),
        }

        if !variable_name.is_empty() && !variable_type.is_empty() {
            print_offset(nest_times);
            println!("let mut {}: {};", variable_name, variable_type);
        }
    }
}

fn parse_assign_to_variable(pair: pest::iterators::Pair<Rule>, nest_times: usize) {
    let mut variable_name = "";
    let mut variable_value = "";

    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::identifier => {
                variable_name = pair.as_str().trim();
            }
            Rule::num => {
                variable_value = pair.as_str();
            }
            _ => error_message(pair),
        }
    }
    if !variable_name.is_empty() || !variable_value.is_empty() {
        print_offset(nest_times);
        println!("{} = {};", variable_name, variable_value);
    }
}

fn parse_call_func(pair: pest::iterators::Pair<Rule>, nest_times: usize) {
    // println!("call function!");

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::func_name => {
                if inner_pair.as_str() == "print" {
                    print_offset(nest_times);
                    print!("println!(\"{{}}\", ");
                } else {
                    print!("{}(", inner_pair.as_str())
                }
            }
            Rule::func_args => {
                print!("{}", inner_pair.as_str());
            }
            _ => error_message(inner_pair),
        }
    }
    print!(");");
}

fn parse_do_statement(pair: pest::iterators::Pair<Rule>, nest_times: usize) {
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::do_keyword => {
                print_offset(nest_times);
                print!("for ")
            }
            Rule::do_variable => print!("{} in ", inner_pair.as_str().trim()),
            Rule::range_expr => {
                let re = regex::Regex::new(r"(\d+),\s*(\d+)").unwrap();
                let caps = re.captures(inner_pair.as_str()).unwrap();
                println!(
                    "{}..{} {{",
                    caps.get(1).unwrap().as_str(),
                    caps.get(2).unwrap().as_str()
                );
            }
            Rule::do_loop_body => {
                print_offset(nest_times);
                print!("    ");
                print!("{}", inner_pair.as_str());
            }
            Rule::end_do_keyword => {
                print_offset(nest_times);
                println!("}};")
            }
            _ => error_message(inner_pair),
        }
    }
}

fn error_message(pair: pest::iterators::Pair<Rule>) {
    println!("--------------------------------------------------------");
    println!("Unreachable!!!!");
    println!("Rule:    {:?}", pair.as_rule());
    println!("Span:    {:?}", pair.as_span());
    println!("Text:    {}", pair.as_str());
    println!("--------------------------------------------------------");
}

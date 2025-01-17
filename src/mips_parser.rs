use std::collections::HashMap;

use super::utils::smart_split::SmartSplit;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug, PartialEq, Clone)]
pub struct MIPSLocation {
    file: String,
    line_num: usize,
    line_text: String
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MIPSDirective {
    directive_type: String,
    directive_value: Vec<String>,
    directive_location: MIPSLocation
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MIPSLabel {
    label: String,
} 

#[derive(Debug)]
#[derive(PartialEq)]
pub struct MIPSInstruction {
    instr_type: String,
    instr_args: Vec<String>,
    instr_location: MIPSLocation
} 


#[derive(Debug)]
#[derive(PartialEq)]
pub enum MIPSComponent {
    Directive(MIPSDirective),
    Label(MIPSLabel),
    Instruction(MIPSInstruction),
}

fn parse_line_to_parts<'a>(line: &'a String) -> impl Iterator<Item = &'a str> {
    SmartSplit::new(line)
}

fn parse_parts_to_component<'a>(parts: &mut impl Iterator<Item =&'a str>, location: MIPSLocation) -> Option<Vec<MIPSComponent>> {
    let mut return_parts:Vec<MIPSComponent> = Vec::new();
    let possible_label = parts.next()?;
    if possible_label.chars().last().unwrap() == ':' {
        let new_label =  MIPSLabel {
            label: possible_label.to_owned()
        };
        return_parts.push(MIPSComponent::Label(new_label));
        let recursed_parts = parse_parts_to_component(parts, location.clone());
        if let Some(recursed_parts) = recursed_parts {
            return_parts.extend(recursed_parts);
        }
    } else {
        let mut possible_directive = possible_label.chars();
        let possible_instruction = possible_label.chars();
        if possible_directive.next().unwrap() == '.' {
            let directive_type_str: String = possible_directive.collect();
            let directive =  MIPSDirective {
                directive_type: directive_type_str.to_owned(),
                directive_value: parts.map(|x| x.to_owned()).collect(),
                directive_location: location
            };
            return_parts.push(MIPSComponent::Directive(directive));
        } else {
            let instruction = MIPSInstruction {
                instr_type: possible_instruction.collect(),
                instr_args: parts.map(|x| x.to_owned()).collect(),
                instr_location: location
            };
            return_parts.push(MIPSComponent::Instruction(instruction));
        }
    }

    Some(return_parts)
}

fn parse_line_to_component(line: &String, location: MIPSLocation) -> Option<Vec<MIPSComponent>> {
    let mut parts = parse_line_to_parts(line);
    parse_parts_to_component(&mut parts, location)

}

pub fn read_file_to_state<'a>(file_name: &String) -> Result<(Vec<MIPSComponent> , HashMap<String, &'a MIPSComponent>)> {
    let component_list = Vec::new();
    let label_map = HashMap::new();


    let file = File::open(file_name)?;
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let mut line = line?.clone();
        let location = MIPSLocation {
            file: file_name.to_owned(),
            line_num: i,
            line_text: line.clone()
        };
        parse_line_to_component(&mut line, location);
    }

    Ok((component_list, label_map))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn compare_results(test: Vec<MIPSComponent>, expected: Vec<MIPSComponent>) {
        let test_len = test.len();
        let exp_len = expected.len();
        assert_eq!(
            test_len, exp_len, 
            "Test vector (length: {}) and expected vector (length: {}) different lengths", 
            test_len, exp_len
        );
        for (test, exp) in expected.iter().zip(test.iter()) {
            match (test, exp) {
                (MIPSComponent::Label(x), MIPSComponent::Label(y)) => assert_eq!(x, y),
                (MIPSComponent::Instruction(x), MIPSComponent::Instruction(y)) => assert_eq!(x, y),
                (MIPSComponent::Directive(x), MIPSComponent::Directive(y)) => assert_eq!(x, y),
                (x, y) => panic!("MIPS Components do not match: {:?}, {:?}", x, y)

            }
        }
    }

    #[test]
    fn test_parse_line_to_parts_label() {
        let loc = MIPSLocation {file: "".to_string(), line_num: 0, line_text: "".to_string()};
        compare_results(
            parse_line_to_component(&"label: ".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label:".to_string()
                })
            ]
        );
        compare_results(
            parse_line_to_component(&"label1: label2: ".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label1:".to_string()
                }),
                MIPSComponent::Label(MIPSLabel {
                    label: "label2:".to_string()
                }),
            ]
        );
    }
    
    #[test]
    fn test_parse_line_to_parts_directive() {
        let loc = MIPSLocation {file: "".to_string(), line_num: 0, line_text: "".to_string()};
        compare_results(
            parse_line_to_component(&".directive".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Directive(MIPSDirective {
                    directive_type: "directive".to_string(),
                    directive_value: vec![],
                    directive_location: loc.clone()
                }),
            ]
        );
        compare_results(
            parse_line_to_component(&".directive arg1 arg2".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Directive(MIPSDirective {
                    directive_type: "directive".to_string(),
                    directive_value: vec!["arg1".to_string(), "arg2".to_string()],
                    directive_location: loc.clone(),
                }),
            ]
        );
    }
    
    #[test]
    fn test_parse_line_to_parts_label_directive() {
        let loc = MIPSLocation {file: "".to_string(), line_num: 0, line_text: "".to_string()};
        compare_results(
            parse_line_to_component(&"label:    \t .directive".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label:".to_string()
                }),
                MIPSComponent::Directive(MIPSDirective {
                    directive_type: "directive".to_string(),
                    directive_value: vec![],
                    directive_location: loc.clone()
                }),
            ]
        );
        compare_results(
            parse_line_to_component(&"label: .directive \targ1 \targ2".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label:".to_string()
                }),
                MIPSComponent::Directive(MIPSDirective {
                    directive_type: "directive".to_string(),
                    directive_value: vec!["arg1".to_string(), "arg2".to_string()],
                    directive_location: loc.clone()
                }),
            ]
        );
        compare_results(
            parse_line_to_component(&"label: .directive 'text text text'".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label:".to_string()
                }),
                MIPSComponent::Directive(MIPSDirective {
                    directive_type: "directive".to_string(),
                    directive_value: vec!["'text text text'".to_string()],
                    directive_location: loc.clone()
                }),
            ]
        );
    }
    
    #[test]
    fn test_parse_line_to_parts_assembly() {
        let loc = MIPSLocation {file: "".to_string(), line_num: 0, line_text: "".to_string()};
        compare_results(
            parse_line_to_component(&"asm".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Instruction(MIPSInstruction {
                    instr_type: "asm".to_string(),
                    instr_args: vec![],
                    instr_location: loc.clone()
                }),
            ]
        );
        compare_results(
            parse_line_to_component(&"asm arg1 arg2".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Instruction(MIPSInstruction {
                    instr_type: "asm".to_string(),
                    instr_args: vec!["arg1".to_string(), "arg2".to_string()],
                    instr_location: loc.clone()
                }),
            ]
        );
    }
    
    #[test]
    fn test_parse_line_to_parts_label_assembly() {
        let loc = MIPSLocation {file: "".to_string(), line_num: 0, line_text: "".to_string()};
        compare_results(
            parse_line_to_component(&"label:    \t asm".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label:".to_string()
                }),
                MIPSComponent::Instruction(MIPSInstruction {
                    instr_type: "asm".to_string(),
                    instr_args: vec![],
                    instr_location: loc.clone()
                        
                }),
            ]
        );
        compare_results(
            parse_line_to_component(&"label1: label2: asm \targ1 \targ2".to_string(), loc.clone()).unwrap(),
            vec![
                MIPSComponent::Label(MIPSLabel {
                    label: "label1:".to_string()
                }),
                MIPSComponent::Label(MIPSLabel {
                    label: "label2:".to_string()
                }),
                MIPSComponent::Instruction(MIPSInstruction {
                    instr_type: "asm".to_string(),
                    instr_args: vec!["arg1".to_string(), "arg2".to_string()],
                    instr_location: loc.clone()
                }),
            ]
        );
    }
}

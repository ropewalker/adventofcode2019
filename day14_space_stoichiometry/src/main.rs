use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const INPUT_PATH: &str = "input.txt";

fn read_input(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why.description())
    }

    s
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Ingredient {
    element: String,
    quantity: isize,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct Formula {
    input: Vec<Ingredient>,
    output: Ingredient,
}

fn parse_input(input: String) -> HashMap<String, Formula> {
    let mut formulas: HashMap<String, Formula> = HashMap::new();

    for formula in input.lines() {
        let sides: Vec<&str> = formula.split(" => ").collect();

        let mut input: Vec<Ingredient> = Vec::new();

        for inputs in sides[0].trim().split(", ") {
            let parameters: Vec<&str> = inputs.split(' ').collect();
            let quantity = parameters[0].trim().parse::<isize>().unwrap();
            let element = String::from(parameters[1].trim());

            input.push(Ingredient { element, quantity });
        }

        let parameters: Vec<&str> = sides[1].trim().split(' ').collect();
        let quantity = parameters[0].trim().parse::<isize>().unwrap();
        let element = String::from(parameters[1].trim());

        let output = Ingredient {
            element: element.clone(),
            quantity,
        };

        formulas.insert(
            element.clone(),
            Formula {
                input: input.clone(),
                output: output.clone(),
            },
        );
    }

    formulas
}

fn main() {
    let formulas = parse_input(read_input(INPUT_PATH));

    let mut ore_total: isize = 1_000_000_000_000;
    let mut fuel_total: isize = 0;
    let mut fuel_demand: isize = 0;

    let mut test_elements_balance: HashMap<String, isize> = HashMap::new();
    test_elements_balance.insert("FUEL".to_string(), -1);

    loop {
        let mut ratio = 1;
        let mut demand: Vec<Ingredient> = Vec::new();

        for (element, balance) in &mut test_elements_balance {
            if *balance >= 0 || element == "ORE" {
                continue;
            }

            let formula = formulas.get(element).unwrap();

            if balance.abs() % formula.output.quantity == 0 {
                ratio = balance.abs() / formula.output.quantity;
            } else {
                ratio = balance.abs() / formula.output.quantity + 1;
            }

            *balance += formula.output.quantity * ratio;
            demand = formula.input.clone();

            break;
        }

        if demand.is_empty() {
            break;
        }

        for ingredient in demand.iter() {
            let supply = test_elements_balance
                .entry(ingredient.element.clone())
                .or_insert(0);
            *supply -= ingredient.quantity * ratio;
        }
    }

    let test_ore = test_elements_balance
        .entry("ORE".to_string())
        .or_insert(0)
        .abs();

    let mut elements_balance: HashMap<String, isize> = HashMap::new();

    loop {
        elements_balance.insert("ORE".to_string(), 0);

        let fuel_demand = if test_ore > ore_total {
            1
        } else {
            ore_total / test_ore
        };

        elements_balance.insert("FUEL".to_string(), -fuel_demand);

        loop {
            let mut ratio = 1;
            let mut demand: Vec<Ingredient> = Vec::new();

            for (element, balance) in &mut elements_balance {
                if *balance >= 0 || element == "ORE" {
                    continue;
                }

                let formula = formulas.get(element).unwrap();

                if balance.abs() % formula.output.quantity == 0 {
                    ratio = balance.abs() / formula.output.quantity;
                } else {
                    ratio = balance.abs() / formula.output.quantity + 1;
                }

                *balance += formula.output.quantity * ratio;
                demand = formula.input.clone();

                break;
            }

            if demand.is_empty() {
                break;
            }

            for ingredient in demand.iter() {
                let supply = elements_balance
                    .entry(ingredient.element.clone())
                    .or_insert(0);
                *supply -= ingredient.quantity * ratio;
            }
        }

        let ore = elements_balance.entry("ORE".to_string()).or_insert(0).abs();

        if ore > ore_total {
            break;
        }

        fuel_total += fuel_demand
            + test_elements_balance
                .entry("FUEL".to_string())
                .or_insert(0)
                .abs();

        ore_total -= ore;
    }

    println!("{}", fuel_total);
}

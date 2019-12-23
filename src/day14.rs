use aoc_runner_derive::{aoc, aoc_generator};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, line_ending};
use nom::combinator::map_res;
use nom::multi::separated_list;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashMap;

type ChemQuantity = (String, u64);
type Equation = (Vec<ChemQuantity>, ChemQuantity);
type ChemMap = HashMap<String, (u64, Vec<ChemQuantity>)>;

fn chemical(input: &str) -> IResult<&str, ChemQuantity> {
    let (input, num) = map_res(digit1, str::parse)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, name) = alpha1(input)?;

    Ok((input, (name.to_string(), num)))
}

fn equation(input: &str) -> IResult<&str, Equation> {
    separated_pair(separated_list(tag(", "), chemical), tag(" => "), chemical)(input)
}

#[aoc_generator(day14)]
fn parse_chem_map(input: &str) -> ChemMap {
    let (_, equations) = separated_list(line_ending, equation)(input).unwrap();

    equations
        .into_iter()
        .map(|(inputs, output)| (output.0, (output.1, inputs)))
        .collect()
}

fn cost_of<'a>(
    chem: &'a str,
    quantity: u64,
    store: &mut HashMap<&'a str, u64>,
    chem_map: &'a ChemMap,
) -> u64 {
    if chem == "ORE" {
        quantity
    } else {
        let stored = store.remove(chem).unwrap_or(0);

        if stored >= quantity {
            store.insert(chem, stored - quantity);
            0
        } else {
            let &(output_quantity, ref inputs) = &chem_map[chem];
            let times = (quantity - stored + output_quantity - 1) / output_quantity;

            let extra = times * output_quantity - quantity + stored;
            store.insert(chem, extra);

            inputs.iter()
                .map(|(chem, quantity)| cost_of(chem, *quantity * times, store, chem_map))
                .sum()
        }
    }
}

#[aoc(day14, part1)]
fn ore_cost(chem_map: &ChemMap) -> u64 {
    cost_of("FUEL", 1, &mut HashMap::new(), chem_map)
}

#[aoc(day14, part2)]
fn max_fuel(chem_map: &ChemMap) -> u64 {
    let mut fuel = 1_000_000_000_000;
    let mut count = 0;
    let mut quantity = 1;

    let mut store = HashMap::new();
    loop {
        let last_store = store.clone();

        let cost = cost_of("FUEL", quantity, &mut store, chem_map);

        if cost <= fuel {
            fuel -= cost;
            count += quantity;
            quantity *= 2;
        } else if quantity == 1 {
            return count;
        } else {
            store = last_store;
            quantity = 1;
        }
    }
}


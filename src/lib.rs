pub const SOURCES_TO_DOWNLOAD: [&str;22] = [
    "europe/france/alsace",
    "europe/france/aquitaine",
    "europe/france/auvergne",
    "europe/france/basse-normandie",
    "europe/france/bourgogne",
    "europe/france/bretagne",
    "europe/france/centre",
    "europe/france/champagne-ardenne",
    "europe/france/corse",
    "europe/france/franche-comte",
    "europe/france/haute-normandie",
    "europe/france/ile-de-france",
    "europe/france/languedoc-roussillon",
    "europe/france/limousin",
    "europe/france/lorraine",
    "europe/france/midi-pyrenees",
    "europe/france/nord-pas-de-calais",
    "europe/france/pays-de-la-loire",
    "europe/france/picardie",
    "europe/france/poitou-charentes",
    "europe/france/provence-alpes-cote-d-azur",
    "europe/france/rhone-alpes",
];

use geo::{geometry::{MultiPolygon, Polygon}, LineString};
use std::fs;
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "poly_format.pest"]
pub struct PolyParser;

fn poly_parser(path: &str) -> Result<MultiPolygon, Box<dyn std::error::Error>> {
    
    let input = fs::read_to_string(path)?;

    let file = PolyParser::parse(Rule::file, &input).expect("failed parse").next().unwrap();

    let mut result_polygon: Vec<Polygon> = vec![];

    for file_pair in file.into_inner() {
        let mut nodes = Vec::new();
        match file_pair.as_rule() {
            Rule::shape => {
                for shape_pair in file_pair.into_inner() {
                    match shape_pair.as_rule() {
                        Rule::point => {
                            let mut x: f64 = 0.0;
                            let mut y: f64 = 0.0;
                            for ordered_pair in shape_pair.into_inner() {
                                match ordered_pair.as_rule() {
                                    Rule::x => {
                                        x = ordered_pair.as_str().parse().unwrap()
                                    },
                                    Rule::y => {
                                        y = ordered_pair.as_str().parse().unwrap()
                                    },
                                    _ => unreachable!(),
                                }
                            }
                            nodes.push((x,y));
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        let polygon = Polygon::new(LineString::from(nodes), vec![]);
        result_polygon.push(polygon);
    }

    Ok(MultiPolygon::new(result_polygon))


}

#[cfg(test)]
mod tests {
    use crate::poly_parser;

    #[test]
    fn and_find_out() {
        let x = poly_parser("C:\\Users\\bookw\\Downloads\\aus.poly");
        println!("aaa{:?}", x)
    }
}

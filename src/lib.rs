use geo::{
    geometry::{MultiPolygon, Polygon},
    LineString,
};
use osmpbf::Element;
use osmpbfreader::{OsmObj, Tags};
use pest::Parser;
use std::fs;
use std::io::Write;
use std::iter::FlatMap;

pub const SOURCES_TO_DOWNLOAD: [&str; 22] = [
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

pub async fn download_and_filter_all(
    temp_pbf_path: &str,
    poly_folder_path: &str,
    filtered_folder_path: &str,
) {
    fs::create_dir_all(temp_pbf_path).unwrap();
    fs::create_dir_all(poly_folder_path).unwrap();
    fs::create_dir_all(filtered_folder_path).unwrap();

    let client = reqwest::Client::new();

    for source in SOURCES_TO_DOWNLOAD {
        let request_url =
            format!("https://download.geofabrik.de/europe/france/{source}-latest.osm.pbf");
        let poly_url = format!("https://download.geofabrik.de/europe/{source}.poly");

        let request_pbf = client.get(&request_url).build().unwrap();
        println!("Downloading {} pbf", source);

        let response_pbf = client.execute(request_pbf).await.unwrap();

        let bytes_from_response_pbf = response_pbf.bytes().await.unwrap();

        println!(
            "Downloaded {} pbf, {:.2} MB",
            source,
            bytes_from_response_pbf.as_ref().len() / 1_000_000
        );

        let mut read_elements = osmpbfreader::OsmPbfReader::new(bytes_from_response_pbf.as_ref());

        let new_elements = read_elements
            .iter()
            .map(|element| match element {
                Ok(element) => {
                    let flat_map = element.tags().clone().into_inner();

                    match include_tag_rail_and_metro(&flat_map) {
                        true => Some(element),
                        false => None,
                    }
                }
                Err(_) => None,
            })
            .flatten()
            .collect::<Vec<OsmObj>>();

        drop(read_elements);
        drop(bytes_from_response_pbf);
    }
}

//Filters tags based on the criteria in Patrick Brosi, PhD's Pfaedle config system
pub fn include_tag_rail_and_metro(
    tags: &flat_map::FlatMap<smartstring::alias::String, smartstring::alias::String>,
) -> bool {
    let mut include = false;

    if let Some(railway) = tags.get("railway") {
        if matches!(
            railway.as_str(),
            "rail"
                | "light_rail"
                | "tram"
                | "narrow_gauge"
                | "train"
                | "subway"
                | "funicular"
                | "station"
                | "halt"
                | "tram_stop"
                | "railway_crossing"
                | "stop"
                | "subway_stop"
        ) {
            include = true;
        }
    }

    if let Some(route) = tags.get("route") {
        if matches!(
            route.as_str(),
            "rail" | "light_rail" | "tram" | "narrow_gauge" | "train" | "funicular" | "subway"
        ) {
            include = true;
        }
    }

    if let Some(route) = tags.get("public_transport") {
        if matches!(
            route.as_str(),
            "rail"
                | "light_rail"
                | "tram"
                | "narrow_gauge"
                | "train"
                | "funicular"
                | "subway"
                | "stop_area"
                | "platform"
                | "station"
        ) {
            include = true;
        }
    }

    if let Some(route) = tags.get("subway") {
        if matches!(route.as_str(), "yes") {
            include = true;
        }
    }

    if let Some(route) = tags.get("tram") {
        if matches!(route.as_str(), "yes") {
            include = true;
        }
    }

    if let Some(route) = tags.get("metro") {
        if matches!(route.as_str(), "yes") {
            include = true;
        }
    }

    include
}

//custom deserialisation library
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "poly_format.pest"]
pub struct PolyParser;

//converts .poly files found on https://download.geofabrik.de/ used for Osmosis filtering into MultiPolygons

pub fn poly_parser(path: &str) -> Result<MultiPolygon, Box<dyn std::error::Error>> {
    let input = fs::read_to_string(path)?;

    let file = PolyParser::parse(Rule::file, &input)
        .expect("failed parse")
        .next()
        .unwrap();

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
                                    Rule::x => x = ordered_pair.as_str().parse().unwrap(),
                                    Rule::y => y = ordered_pair.as_str().parse().unwrap(),
                                    _ => unreachable!(),
                                }
                            }
                            nodes.push((x, y));
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
        let x = poly_parser("./france.poly");
        println!("aaa{:?}", x)
    }
}

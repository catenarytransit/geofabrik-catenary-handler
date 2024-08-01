use std::io::Write;
use geo::{geometry::{MultiPolygon, Polygon}, LineString};
use std::fs;
use pest::Parser;

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


pub async fn download_and_filter_all(temp_pbf_path: &str, poly_folder_path: &str, filtered_folder_path: &str) {

    fs::create_dir_all(temp_pbf_path).unwrap();
    fs::create_dir_all(poly_folder_path).unwrap();
    fs::create_dir_all(filtered_folder_path).unwrap();

    let client = reqwest::Client::new();

    for source in SOURCES_TO_DOWNLOAD {
        let request_url = format!("https://download.geofabrik.de/europe/france/{source}-latest.osm.pbf");
        let poly_url = format!("https://download.geofabrik.de/europe/{source}.poly");

        let request_pbf = client.get(&request_url).build().unwrap();
        println!("Downloading {} pbf", source);

        let response_pbf = client.execute(request_pbf).await.unwrap();

        let bytes_from_response_pbf = response_pbf.bytes().await.unwrap();

        println!("Downloaded {} pbf, {:.2} MB", source, bytes_from_response_pbf.as_ref().len() / 1_000_000);

        let read_elements = osmpbf::reader::ElementReader::new(bytes_from_response_pbf.as_ref());

        let mut new_elements: Vec<osmpbf::Element> = vec![];

        let filtered_elements = read_elements.for_each(|element| {
            let drop = false;

        });

    }
}


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

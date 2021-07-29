extern crate csv;
use models::*;
use std;

pub fn csv(nodes: Vec<Node>, edges: Vec<Edge>) {
    let edges_path = std::path::Path::new("edges.csv");
    //let mut edges_csv = csv::Writer::from_path(edges_path).unwrap();
    let mut edges_csv = csv::WriterBuilder::new().delimiter(b'|').from_path(edges_path).unwrap();
    edges_csv
        .serialize(vec![
            "id",
            "source",
            "target",
            "length",
            "foot",
            "car_forward",
            "car_backward",
            "bike_forward",
            "bike_backward",
            "wkt",
            "speed_limit",
        ])
        .expect("CSV: unable to write edge header");
    for edge in edges {
        edges_csv
            .serialize((
                edge.id.0,
                edge.source.0,
                edge.target.0,
                edge.length(),
                edge.properties.foot,
                edge.properties.car_forward,
                edge.properties.car_backward,
                edge.properties.bike_forward,
                edge.properties.bike_backward,
                edge.as_wkt(),
                edge.properties.speed_limit,
            ))
            .expect("CSV: unable to write edge");
    }

    let nodes_path = std::path::Path::new("nodes.csv");
    //let mut nodes_csv = csv::Writer::from_path(nodes_path).unwrap();
    let mut nodes_csv = csv::WriterBuilder::new().delimiter(b'|').from_path(nodes_path).unwrap();
    nodes_csv
        .serialize(vec!["id", "lon", "lat"])
        .expect("CSV: unable to write node header");
    for node in nodes {
        nodes_csv
            .serialize((node.id.0, node.coord.lon, node.coord.lat))
            .expect("CSV: unable to write node");
    }
}

// pub fn pg(nodes: Vec<Node>, edges: Vec<Edge>) {}

mod hvs;
use hvs::{Location, Units};
use petgraph::graph::UnGraph;
use petgraph::prelude::*;
use petgraph::data::FromElements;
use calamine::{open_workbook, Reader, Xlsx};
use std::collections::{HashMap, HashSet};
use petgraph::algo::min_spanning_tree;

// Original excel sheet has 23312 nodes, that is too many!
// I will only be doing a select amount of states that amount to over 1000 vertices
// This includes NE (CT, RI, VT, NH, MA and ME) with 711 towers total, NY with 636 and NJ with 170.
// Total number of nodes is 1577 which is over the 1000 vertex requirement
// All functions are public so the test can access them

fn main() {
    // Grabs Excel sheet labelled Cellular_Towers_Contiguous to work with
    let path = r"C:\Users\mason\Downloads\Cellular_Towers_Contiguous.xlsx";
    // Defines the set of states included in New England
    let ne_states: HashSet<&str> = ["CT", "RI", "VT", "NH", "MA", "ME"].iter().cloned().collect();
    // Defines the set of states included in the Tri-State area
    let tri_state_states: HashSet<&str> = ["NJ", "CT", "NY"].iter().cloned().collect();
    // Combine the sets to get all selected states
    // Even though CT is in both, it is not counted twice!
    let selected_states: HashSet<&str> = ne_states.iter().chain(tri_state_states.iter()).cloned().collect();

    match load_and_process_data(path, &selected_states) {
        Ok((mst_graphs_by_state, total_cell_towers, total_cell_towers_ne, total_cell_towers_tri_state)) => {
            for (state, (mst_graph, num_cell_towers)) in &mst_graphs_by_state {
                // Prints the state, then the number of cell towers and connections in the state
                println!("State: {}", state);
                println!("Number of Cell Towers: {}", num_cell_towers);
                println!("Number of Connections: {}", mst_graph.edge_count());
            }
            // Prints total number of cell towers overall and in each area defined
            println!("Total Cell Towers: {}", total_cell_towers);
            println!("Total Cell Towers in NE: {}", total_cell_towers_ne);
            println!("Total Cell Towers in Tri-State: {}", total_cell_towers_tri_state);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    // Define any two states to combine that border each other. I chose CT and NY.
    let combined_states: HashSet<&str> = ["CT", "NY"].iter().cloned().collect();
    match load_and_process_data_for_combined_states(path, &combined_states) {
        Ok((mst_graph_combined, total_cell_towers_combined)) => {
            println!("Combined States");
            println!("Total Cell Towers: {}", total_cell_towers_combined);
            println!("Number of Connections: {}", mst_graph_combined.edge_count());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
// Function to read the file and extract coordinates by state
pub fn read_file(path: &str, states: &HashSet<&str>) -> Result<Vec<(usize, f64, f64, String)>, String> {
    let mut excel: Xlsx<_> = open_workbook(path)
        // Returns an error is the file can't be opened
        .map_err(|e| format!("Failed to Open File: {}", e))?;

    let range = excel
        // Name the worksheet we are using
        .worksheet_range("Cellular_Towers")
        // Returns an error if the sheet name is not in the Excel sheet
        .ok_or("Worksheet 'Cellular_Towers' Not Found")?
        .map_err(|_| "Failed to Read Worksheet Range")?;

    let mut coordinates = Vec::new();

    for (index, row) in range.rows().enumerate() {
        if let Some(state) = row.get(18).and_then(|v| v.get_string()) {
            if states.contains(&state) {
                if let (Some(lat_val), Some(lon_val)) = (row.get(1).and_then(|v| v.get_float()), row.get(0).and_then(|v| v.get_float())) {
                    let state_str = state.to_string();
                    //Adds each index, latitude, longitude and state value into the list of coordinates
                    //Only pushes values if latitude and longitude are present
                    //This avoids all rows with missing lat and lon values
                    coordinates.push((index, lat_val, lon_val, state_str));
                }
            }
        }
    }
    Ok(coordinates)
}
// Defines both NE states and Tri-State states globally to avoid errors
const NE_STATES: [&str; 6] = ["CT", "RI", "VT", "NH", "MA", "ME"];
const TRI_STATE_STATES: [&str; 3] = ["NJ", "CT", "NY"];
// Function to build the MST
// Specifically an undirected graph as cell service is not one way
pub fn build_mst(coordinates: &[Location]) -> UnGraph<Location, f64> {
    let mut graph = UnGraph::<Location, f64>::new_undirected();
    let nodes: Vec<NodeIndex> = coordinates
        .iter()
        .map(|&coord| graph.add_node(coord))
        .collect();
    for &node_a in &nodes {
        for &node_b in &nodes {
            if node_a != node_b && !graph.contains_edge(node_a, node_b) {
                // Uses haversine module to weight edge by distance
                let weight = hvs::haversine(graph[node_a], graph[node_b], Units::Km);
                graph.add_edge(node_a, node_b, weight);
            }
        }
    }
    // Creates a minimum spanning tree to find the fewest amount of edges to connect every node
    let mst: Vec<_> = min_spanning_tree(&graph).collect();
    UnGraph::<Location, f64>::from_elements(mst.iter().cloned())
}
// Function to load and process data
pub fn load_and_process_data<'a>(
    path: &str,
    selected_states: &'a HashSet<&'a str>,
) -> Result<(HashMap<&'a str, (UnGraph<Location, f64>, usize)>, usize, usize, usize), String> {
    let coordinates_by_state = read_file(path, selected_states)?;
    let mut mst_graphs_by_state: HashMap<&'a str, (UnGraph<Location, f64>, usize)> = HashMap::new();
    let mut total_cell_towers = 0;
    let mut total_cell_towers_ne = 0;
    let mut total_cell_towers_tri_state = 0;
    for state in selected_states {
        let state_coordinates: Vec<Location> = coordinates_by_state
            .iter()
            .filter(|&(_, _lat, _lon, state_code)| state_code == state)
            .map(|&(_, lat, lon, _)| Location { latitude: lat, longitude: lon })
            .collect();

        let mst_graph = build_mst(&state_coordinates);
        let num_cell_towers = state_coordinates.len();
        mst_graphs_by_state.insert(state, (mst_graph, num_cell_towers));

        total_cell_towers += num_cell_towers;
        if NE_STATES.contains(&state) {
            total_cell_towers_ne += num_cell_towers;
        }
        if TRI_STATE_STATES.contains(&state) {
            total_cell_towers_tri_state += num_cell_towers;
        }
    }
    Ok((mst_graphs_by_state, total_cell_towers, total_cell_towers_ne, total_cell_towers_tri_state))
}
// A function to load and process data but for combined states
pub fn load_and_process_data_for_combined_states<'a>(
    path: &str,
    combined_states: &'a HashSet<&'a str>,
) -> Result<(UnGraph<Location, f64>, usize), String> {
    let coordinates_by_state = read_file(path, combined_states)?;
    let mut combined_state_coordinates: Vec<Location> = Vec::new();
    // Sets original size to 0 as each will be added to
    let mut total_cell_towers = 0;

    for state in combined_states {
        let state_coordinates: Vec<Location> = coordinates_by_state
            .iter()
            .filter(|&(_, _lat, _lon, state_code)| state_code == state)
            .map(|&(_, lat, lon, _)| Location { latitude: lat, longitude: lon })
            .collect();

        combined_state_coordinates.extend(state_coordinates.clone());
        total_cell_towers += state_coordinates.len();
    }
    let mst_graph = build_mst(&combined_state_coordinates);
    Ok((mst_graph, total_cell_towers))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test uses all my Excel data and states I input to check and see if MST works correctly
    // Returns true when # of Cell Towers is 1 greater than # of Connections
    // Test passes currently
    fn test_mst_edges_vs_nodes() {
        let path = r"C:\Users\mason\Downloads\Cellular_Towers_Contiguous.xlsx";
        let ne_states: HashSet<&str> = ["CT", "RI", "VT", "NH", "MA", "ME"].iter().cloned().collect();
        let tri_state_states: HashSet<&str> = ["NJ", "CT", "NY"].iter().cloned().collect();
        let selected_states: HashSet<&str> = ne_states.iter().chain(tri_state_states.iter()).cloned().collect();

        match load_and_process_data(path, &selected_states) {
            Ok((mst_graphs, _, _, _)) => {  // Extract only the HashMap from the tuple
                for (state, (graph, _)) in mst_graphs {  // Correctly uses the HashMap
                    let num_nodes = graph.node_count();
                    let num_edges = graph.edge_count();
                    // Assertation follows earlier definition of MST
                    assert_eq!(num_edges, num_nodes - 1, "State {}: {} edges, {} nodes", state, num_edges, num_nodes);
                }
            }
            Err(e) => {
                panic!("Failed to load or process data: {}", e);
            }
        }
    }
}
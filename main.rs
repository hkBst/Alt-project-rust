use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Define the Cell struct
#[derive(Debug)]
struct Cell {
    oem: Option<String>,
    model: Option<String>,
    launch_announced: Option<String>,
    launch_status: Option<String>,
    body_dimensions: Option<String>,
    body_weight: Option<f32>,
    body_sim: Option<String>,
    display_type: Option<String>,
    display_size: Option<String>,
    display_resolution: Option<String>,
    features_sensors: Option<String>,
    platform_os: Option<String>,
}

impl Cell {
    // Constructor
    fn new() -> Self {
        Cell {
            oem: None,
            model: None,
            launch_announced: None,
            launch_status: None,
            body_dimensions: None,
            body_weight: None,
            body_sim: None,
            display_type: None,
            display_size: None,
            display_resolution: None,
            features_sensors: None,
            platform_os: None,
        }
    }

    // Getter methods
    fn get_oem(&self) -> Option<&str> {
        self.oem.as_deref()
    }

    fn get_model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    fn get_launch_announced(&self) -> Option<&str> {
        self.launch_announced.as_deref()
    }

    fn get_launch_status(&self) -> Option<&str> {
        self.launch_status.as_deref()
    }

    fn get_body_dimensions(&self) -> Option<&str> {
        self.body_dimensions.as_deref()
    }

    fn get_body_weight(&self) -> Option<f32> {
        self.body_weight
    }

    fn get_body_sim(&self) -> Option<&str> {
        self.body_sim.as_deref()
    }

    fn get_display_type(&self) -> Option<&str> {
        self.display_type.as_deref()
    }

    fn get_display_size(&self) -> Option<&str> {
        self.display_size.as_deref()
    }

    fn get_display_resolution(&self) -> Option<&str> {
        self.display_resolution.as_deref()
    }

    fn get_features_sensors(&self) -> Option<&str> {
        self.features_sensors.as_deref()
    }

    fn get_platform_os(&self) -> Option<&str> {
        self.platform_os.as_deref()
    }

    // Setter methods
    fn set_oem(&mut self, value: String) {
        self.oem = Some(value);
    }

    fn set_model(&mut self, value: String) {
        self.model = Some(value);
    }

    fn set_launch_announced(&mut self, value: String) {
        self.launch_announced = Some(value);
    }

    fn set_launch_status(&mut self, value: String) {
        self.launch_status = Some(value);
    }

    fn set_body_dimensions(&mut self, value: String) {
        self.body_dimensions = Some(value);
    }

    fn set_body_weight(&mut self, value: f32) {
        self.body_weight = Some(value);
    }

    fn set_body_sim(&mut self, value: String) {
        self.body_sim = Some(value);
    }

    fn set_display_type(&mut self, value: String) {
        self.display_type = Some(value);
    }

    fn set_display_size(&mut self, value: String) {
        self.display_size = Some(value);
    }

    fn set_display_resolution(&mut self, value: String) {
        self.display_resolution = Some(value);
    }

    fn set_features_sensors(&mut self, value: String) {
        self.features_sensors = Some(value);
    }

    fn set_platform_os(&mut self, value: String) {
        self.platform_os = Some(value);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let path = Path::new("cells.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // HashMap to store Cell objects
    let mut cell_map: HashMap<usize, Cell> = HashMap::new();
    let mut row_index = 0;

    // Read each line of the CSV file and create Cell objects
    for line in reader.lines().skip(1) {
        let line = line?;
        let columns: Vec<&str> = line.split(',').collect();

        // Check if the number of columns is as expected
        if columns.len() == 12 {
            // Create a new Cell object
            let mut cell = Cell::new();

            // Set attributes for the Cell object
            cell.set_oem(columns[0].to_string());
            cell.set_model(columns[1].to_string());
            cell.set_launch_announced(columns[2].to_string());
            cell.set_launch_status(columns[3].to_string());
            cell.set_body_dimensions(columns[4].to_string());

            // Process and set body weight
            let body_weight: Vec<_> = columns[5].split("g").collect();

            cell.set_body_sim(columns[6].to_string());
            cell.set_display_type(columns[7].to_string());
            cell.set_display_size(columns[8].to_string());
            cell.set_display_resolution(columns[9].to_string());
            cell.set_features_sensors(columns[10].to_string());
            cell.set_platform_os(columns[11].to_string());

            // Store the Cell object in the HashMap
            cell_map.insert(row_index, cell);
            row_index += 1;
        } else {
            println!("Skipping invalid line: {}", line);
        }
    }

    // loop through all the cells
    for i in 0..row_index {
    if let Some(i) = cell_map.get(&i) {
        println!("OEM: {:?}", i.get_oem().unwrap_or("Doesnt exist"));
        println!("Model: {:?}", i.get_model().unwrap_or("Doesnt exist"));
        println!("Launch Announced: {:?}", i.get_launch_announced().unwrap_or("Doesnt exist"));
        println!("Launch Status: {:?}", i.get_launch_status().unwrap_or("Doesnt exist"));
        println!("Body Dimensions: {:?}", i.get_body_dimensions().unwrap_or("Doesnt exist"));
        println!("Body Weight: {:?}", i.get_body_weight());
        println!("Body SIM: {:?}", i.get_body_sim().unwrap_or("Doesnt exist"));
        println!("Display Type: {:?}", i.get_display_type().unwrap_or("Doesnt exist"));
        println!("Display Size: {:?}", i.get_display_size().unwrap_or("Doesnt exist"));
        println!("Display Resolution: {:?}", i.get_display_resolution().unwrap_or("Doesnt exist"));
        println!("Features Sensors: {:?}", i.get_features_sensors().unwrap_or("Doesnt exist"));
        println!("Platform OS: {:?}", i.get_platform_os().unwrap_or("Doesnt exist"));
    } else {
        println!("No cell phones found in the CSV file.");
    }
}
//highest 
let mut a: f32 = 1.0;
   for i in 0 .. 1000{
    if let Some(i) = cell_map.get(&i) {
        if let Some(x) = i.get_body_weight() {
            if x > a {
              a = x  
            }
          }
    }
    
    println!("{:?}",a);
    }
    let mut y = 1.0;
    for i in 0 .. row_index{
        if let Some(i) =cell_map.get(&i){
            if let Some(x) = i.get_features_sensors(){
                
            } 
        }

    }

    let mut a = 1.0;
    for i in 0 .. row_index{
        if let Some(i) =cell_map.get(&i){
            if let Some(x) = i.get_features_sensors(){
                
                
            } 
        }

    }
    Ok(())
   }
    

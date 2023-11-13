use std::time::Instant;
use clap::{arg, Command};
use serde_json::json;
use polars::prelude::*; 
use faer::Mat;  
use faer::polars::polars_to_faer_f64;  
  
fn load_file_faer(file_path: &str) -> Result<(Vec<Vec<f64>>, Vec<f32>), Box<dyn std::error::Error>> {       
    let df = CsvReader::from_path(file_path)?    
        .has_header(true)    
        .finish()?;    
      
    let mat: Mat<f64> = polars_to_faer_f64(df.lazy()).unwrap();    
    
    let mut features: Vec<Vec<f64>> = vec![vec![0.0; mat.ncols()-1]; mat.nrows()];  
    for i in 0..mat.ncols()-1 {  
        for j in 0..mat.nrows() {  
            features[j][i] = mat[(j, i)];  
        }  
    }  
   
    let mut labels: Vec<f32> = vec![0.0; mat.nrows()];
    let y_ind = mat.ncols()-1;
    for i in 0..mat.nrows() {  
        labels[i] = mat[(i, y_ind)] as f32; 
    }   
  
    Ok((features, labels))    
}    

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("polars_faer_examples")  
        .version("0.1.0")  
        .author("Daniel Herman daniel.herman@protonmail.com>")  
        .about("Testing out polars + faer")  
        .arg(arg!(--train_input <FILE> "Train table (CSV format)")  
            .required(true))
        .arg(arg!(--test_input <FILE> "Test table (CSV format)")  
            .required(true))
        .get_matches();

    let train_input = matches.get_one::<String>("train_input").expect("required");
    let test_input = matches.get_one::<String>("test_input").expect("required");

    let start = Instant::now();
    let (train_features, train_labels) = match load_file_faer(train_input) {
        Ok((train_features, train_labels)) => (train_features, train_labels),
        Err(error) => panic!("Problem with loading training data: {:?}", error),
    };
    let (test_features, test_labels) = match load_file_faer(test_input) {
        Ok((test_features, test_labels)) => (test_features, test_labels),
        Err(error) => panic!("Problem with loading test data: {:?}", error),
    };
    let duration_load = start.elapsed();

    println!("Loading data took: {:?}", duration_load);
    let elapsed_time = json! {
        {
            "load": duration_load.as_millis(),
        }
    };
    println!("{}", elapsed_time);
    Ok(())
}
use anyhow::{Context, Result};
use csv::ReaderBuilder;

pub fn describe(csv_path: &str, column_name: &str) -> Result<()> {
    // Read the CSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)
        .context("Failed to read CSV file")?;

    // Get headers and find the index of the specified column
    let headers = rdr.headers().context("Failed to read headers")?;
    let column_index = headers
        .iter()
        .position(|h| h == column_name)
        .context(format!("Column '{}' not found", column_name))?;

    let mut values: Vec<f64> = Vec::new();

    // Collect data from the specified column
    for result in rdr.records() {
        let record = result.context("Failed to read record")?;
        if let Some(value) = record.get(column_index) {
            if let Ok(num) = value.parse::<f64>() {
                values.push(num);
            }
        }
    }

    if values.is_empty() {
        println!("No data found in column '{}'.", column_name);
        return Ok(());
    }

    // Calculate statistics
    let count = values.len() as f64;
    let mean = values.iter().sum::<f64>() / count;
    let stdev = (values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count).sqrt();
    let min = *values
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max = *values
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Calculate median
    let mut sorted_values = values.clone();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if count % 2.0 == 0.0 {
        (sorted_values[(count as usize / 2) - 1] + sorted_values[count as usize / 2]) / 2.0
    } else {
        sorted_values[count as usize / 2]
    };

    // Calculate percentiles
    let percentile_25 = sorted_values[(count as usize * 25 / 100) - 1];
    let percentile_75 = sorted_values[(count as usize * 75 / 100) - 1];

    // Print results in a table format
    println!("{:<20} | {}", "Statistic", "Value");
    println!("{:-<20} | {}", "", "");
    println!("{:<20} | {}", "Count", count);
    println!("{:<20} | {:.2}", "Mean", mean);
    println!("{:<20} | {:.2}", "Standard Deviation", stdev);
    println!("{:<20} | {:.2}", "Min", min);
    println!("{:<20} | {:.2}", "Max", max);
    println!("{:<20} | {:.2}", "Median", median);
    println!("{:<20} | {:.2}", "25th Percentile", percentile_25);
    println!("{:<20} | {:.2}", "75th Percentile", percentile_75);

    Ok(())
}

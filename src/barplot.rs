use charts::{BarLabelPosition, Chart, ScaleBand, ScaleLinear, VerticalBarView};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

pub fn generate_hb(path: &str, x_column: &str, y_column: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(path)?;
    let headers = reader.headers()?.clone();

    // Index Find
    let x_index = headers
        .iter()
        .position(|h| h == x_column)
        .ok_or_else(|| format!("Column '{}' not found", x_column))?;

    let y_index = headers
        .iter()
        .position(|h| h == y_column)
        .ok_or_else(|| format!("Column '{}' not found", y_column))?;

    // Count occurrences of each (x, y) pair
    let mut counts: HashMap<(String, String), usize> = HashMap::new();
    let mut x_frequencies: HashMap<String, usize> = HashMap::new(); // To track frequencies of x only

    for result in reader.records() {
        if let Ok(record) = result {
            let x_value = record.get(x_index).map(|v| v.to_string());
            let y_value = record.get(y_index).map(|v| v.to_string());

            if let (Some(x), Some(y)) = (x_value, y_value) {
                *counts.entry((x.clone(), y)).or_insert(0) += 1; // Count (x, y) pairs
                *x_frequencies.entry(x).or_insert(0) += 1; // Count x occurrences
            }
        }
    }

    // Prepare data for the stacked bar plot
    let mut data: Vec<(String, f32, String)> = Vec::new();
    let mut max_y = 0; // Variable to track maximum frequency

    // Populate data for the bar chart and find max_y
    for ((x, y), count) in counts {
        data.push((x.clone(), count as f32, y)); // Include group for stacking
        max_y = max_y.max(count); // Update max_y with current count
    }

    // Define chart related sizes.
    let width = 1280;
    let height = 720;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps x categories to values in [0, availableWidth]
    let x = ScaleBand::new()
        .set_domain(data.iter().map(|(x, _, _)| x.clone()).collect())
        .set_range(vec![0, width - left - right]);

    // Create the y scale using the maximum frequency of x occurrences
    let max_x_frequency = x_frequencies.values().cloned().max().unwrap_or(0); // Max frequency of x only
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, max_x_frequency as f32]) // Set the y-axis maximum to max_x_frequency
        .set_range(vec![height - top - bottom, 0]);

    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_position(BarLabelPosition::Center)
        .load_data(&data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Stacked Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Frequency")
        .add_bottom_axis_label("Categories")
        .save("stacked-vertical-bar-chart.svg")
        .unwrap();

    Ok(())
}

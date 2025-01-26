use charts::{Chart, Color, MarkerType, PointLabelPosition, ScaleLinear, ScatterView};
use csv::ReaderBuilder;
use std::error::Error;

pub fn generate_scatter(path: &str, x_column: &str, y_column: &str) -> Result<(), Box<dyn Error>> {
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

    // Read records and collect scatter data
    let scatter_data: Vec<(f32, f32)> = reader
        .records()
        .filter_map(|record| {
            if let Ok(record) = record {
                let x_value = record.get(x_index).and_then(|v| v.parse::<f32>().ok());
                let y_value = record.get(y_index).and_then(|v| v.parse::<f32>().ok());
                if let (Some(x), Some(y)) = (x_value, y_value) {
                    Some((x, y)) // No label needed
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Define chart related sizes
    let width = 1280;
    let height = 720;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create x and y scales
    let x_min = scatter_data
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let x_max = scatter_data
        .iter()
        .map(|(x, _)| *x)
        .fold(f32::NEG_INFINITY, f32::max);
    let y_min = scatter_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::INFINITY, f32::min);
    let y_max = scatter_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f32::NEG_INFINITY, f32::max);

    let x = ScaleLinear::new()
        .set_domain(vec![x_min, x_max])
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![y_min, y_max])
        .set_range(vec![height - top - bottom, 0]);

    // Create Scatter view
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::color_scheme_dark())
        .set_label_position(PointLabelPosition::E) // Disable data labels
        .set_label_visibility(false)
        .load_data(&scatter_data)?;

    // Generate and save the chart
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(format!("{} vs {}", x_column, y_column)) // Title based on column names
        .add_view(&scatter_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label(x_column) // X-axis label
        .add_bottom_axis_label(y_column) // Y-axis label
        .save("scatter-chart.svg")?;

    Ok(())
}

use crate::config::Config;
use plotters::coord::Shift;
use plotters::prelude::*;

pub fn plot(config: Config) {
    let isSvg = config.outputFilePath.as_str().ends_with(".svg");
    let output_path = config.outputFilePath.clone();
    if isSvg {
        let backend = construct_svg_backend(&output_path, config.width, config.height);
        plot_with(backend, config);
    } else {
        let backend = construct_bitmap_backend(&output_path, config.width, config.height);
        plot_with(backend, config);
    }
}

fn construct_area<Backend: DrawingBackend>(
    backend: Backend,
    title: &str,
) -> Result<DrawingArea<Backend, Shift>, DrawingAreaErrorKind<Backend::ErrorType>> {
    let area = backend.into_drawing_area();
    area.fill(&WHITE)?;
    let area = area.titled(title, ("sans-serif", 60))?;
    Ok(area)
}

fn construct_bitmap_backend(output_path: &str, width: u32, height: u32) -> BitMapBackend {
    BitMapBackend::new(output_path, (width, height))
}

fn construct_svg_backend(output_path: &str, width: u32, height: u32) -> SVGBackend {
    SVGBackend::new(output_path, (width, height))
}

fn plot_with<Backend: DrawingBackend>(
    backend: Backend,
    config: Config,
) -> Result<(), DrawingAreaErrorKind<Backend::ErrorType>> {
    // I don't like how I am splitting this up, but it gets rid of the ownership problems.
    let (title, points1, points2, outputFilePath, subtitle) = match dbg!(config) {
        Config {
            title,
            points1,
            points2,
            outputFilePath,
            subtitle,
            ..
        } => (title, points1, points2, outputFilePath, subtitle),
    };
    let area = construct_area(backend, &title)?;

    let mut cc = ChartBuilder::on(&area)
        .margin(5)
        .set_all_label_area_size(50)
        .caption(subtitle.as_str(), ("sans-serif", 40))
        .build_cartesian_2d(-3i32..3i32, -3i32..3i32)?;
    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;
    // The root issue is that LineSeries wants to own the data, but you are only give it a reference to the data.
    cc.draw_series(LineSeries::new(points1, &RED))?
        .label("Line 1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    cc.draw_series(LineSeries::new(points2, &BLUE))?
        .label("Line 2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    cc.configure_series_labels().border_style(&BLACK).draw()?;
    // I messed with some of the roc_std to get rid of all of the `.as_str`.
    area.present()
        .unwrap_or_else(|_| panic!("I failed to draw your plot to {} !", outputFilePath,));
    println!("I drew your plot to {}", outputFilePath);
    Ok(())
}

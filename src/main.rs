use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Circle;

#[allow(unused_variables)]

fn main(){
  let _ = draw_circle();
  let _ = draw_path();
}

fn draw_circle() -> anyhow::Result<()> {
    let circle = Circle::new()
    .set("cx","150")
    .set("cy","100")
    .set("r","80")
    .set("fill","green");

    let svg_document = Document::new()
    .set("width","300")
    .set("height","200")
    .add(circle);

    svg::save("circle.svg",&svg_document)?;
    Ok(())
}

fn draw_path() -> anyhow::Result<()> {
    let data = Data::new()
        .move_to((10, 80))
        .quadratic_curve_to(((52.5, 10), (95, 80))) // or (52.5, 10, 95, 80))
        .smooth_quadratic_curve_to((180, 80)); // same a T command


    let path = Path::new()
        .set("fill", "transparent")
        .set("stroke", "red")
        .set("d", data);

    let svg_document = Document::new()
        .set("width", "190")
        .set("height", "160")
        .add(path);

    svg::save("quad.svg", &svg_document)?;
    Ok(())
}

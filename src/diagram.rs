mod boolean;

pub use boolean::BooleanDiagram;

pub trait IntoSVG {
    fn into_svg(&self) -> svg::Document;
}

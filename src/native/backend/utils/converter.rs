// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT
use iced_graphics::canvas;
use iced_native::{Color, Point};
use plotters_backend::{BackendColor, BackendCoord, BackendStyle};

pub(crate) fn cvt_color(color: &BackendColor) -> Color {
    let ((r, g, b), a) = (color.rgb, color.alpha);
    Color::from_rgba8(r, g, b, a as f32)
}

pub(crate) fn cvt_stroke<S: BackendStyle>(style: &S) -> canvas::Stroke {
    canvas::Stroke::default()
        .with_color(cvt_color(&style.color()))
        .with_width(style.stroke_width() as f32)
}

pub(crate) trait CvtPoint {
    fn cvt_point(self) -> Point;
}

impl CvtPoint for BackendCoord {
    fn cvt_point(self) -> Point {
        Point::new(self.0 as f32, self.1 as f32)
    }
}

impl CvtPoint for [f64; 2] {
    fn cvt_point(self) -> Point {
        Point::new(self[0] as f32, self[1] as f32)
    }
}
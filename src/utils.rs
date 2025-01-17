// plotters-iced
//
// Iced backend for Plotters
// Copyright: 2022, Joylei <leingliu@gmail.com>
// License: MIT

use iced_graphics::widget::canvas;
use iced_graphics::widget::canvas::Cursor;
use iced_native::{Color, Point};
use plotters_backend::{BackendColor, BackendCoord, BackendStyle};

pub(crate) trait AndExt {
    fn and<F: Fn(Self) -> Self>(self, f: F) -> Self
    where
        Self: Sized;
}

impl<T> AndExt for T {
    #[inline(always)]
    fn and<F: Fn(Self) -> Self>(self, f: F) -> Self
    where
        Self: Sized,
    {
        f(self)
    }
}

/// same as Cursor::from_window_position
pub(crate) fn cursor_from_window_position(cursor_position: Point) -> Cursor {
    if cursor_position.x <= 0_f32 || cursor_position.y <= 0_f32 {
        Cursor::Unavailable
    } else {
        Cursor::Available(cursor_position)
    }
}

#[inline]
pub(crate) fn cvt_color(color: &BackendColor) -> Color {
    let ((r, g, b), a) = (color.rgb, color.alpha);
    Color::from_rgba8(r, g, b, a as f32)
}

#[inline]
pub(crate) fn cvt_stroke<S: BackendStyle>(style: &S) -> canvas::Stroke {
    canvas::Stroke::default()
        .with_color(cvt_color(&style.color()))
        .with_width(style.stroke_width() as f32)
}

pub(crate) trait CvtPoint {
    fn cvt_point(self) -> Point;
}

impl CvtPoint for BackendCoord {
    #[inline]
    fn cvt_point(self) -> Point {
        Point::new(self.0 as f32, self.1 as f32)
    }
}

impl CvtPoint for [f64; 2] {
    #[inline]
    fn cvt_point(self) -> Point {
        Point::new(self[0] as f32, self[1] as f32)
    }
}

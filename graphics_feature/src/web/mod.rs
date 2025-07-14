use affogato_core::{num::Number, sets::Real};
use affogato_math::{geometry::{CalculateCentroid, Circle, CubicSegment2D, HyperSphere, LinearSegment2D, QuadraticSegment2D, Rect, Segment2D, Triangle2D}, vector::{DVec2, Vector2, VectorSpace}};
use web_sys::CanvasRenderingContext2d;

pub trait CanvasRenderable<V: VectorSpace> {
    type Context;
    fn fill_canvas(&self, context: &Self::Context);
}

impl<T: Number> CanvasRenderable<Vector2<T>> for Rect<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &CanvasRenderingContext2d) {
        context.fill_rect(self.min.x().to_f64(), self.min.y().to_f64(), self.width().to_f64(), self.height().to_f64());
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for Circle<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &CanvasRenderingContext2d) {
        context.begin_path();
        context.arc(self.center.x().to_f64(), self.center.y().to_f64(), self.radius().to_f64(), 0.0f64, core::f64::consts::TAU);
        context.fill();
        context.close_path();
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for LinearSegment2D<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        context.begin_path();
        context.move_to(self.start.x().to_f64(), self.start.y().to_f64());
        context.line_to(self.end.x().to_f64(), self.end.y().to_f64());
        context.fill();
        context.close_path();
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for QuadraticSegment2D<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        context.begin_path();
        context.move_to(self.start.x().to_f64(), self.start.y().to_f64());
        context.quadratic_curve_to(self.control.x().to_f64(), self.control.y().to_f64(), self.end.x().to_f64(), self.end.y().to_f64());
        context.fill();
        context.close_path();
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for CubicSegment2D<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        context.begin_path();
        context.move_to(self.start.x().to_f64(), self.start.y().to_f64());
        context.bezier_curve_to(
            self.control1.x().to_f64(), self.control1.y().to_f64(), 
            self.control2.x().to_f64(), self.control2.y().to_f64(), 
            self.end.x().to_f64(), self.end.y().to_f64()
        );
        context.fill();
        context.close_path();
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for Segment2D<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        match self {
            Segment2D::Linear(linear) => linear.fill_canvas(context),
            Segment2D::Quadratic(quadratic) => quadratic.fill_canvas(context),
            Segment2D::Cubic(cubic) => cubic.fill_canvas(context),
        }
    }
}

impl<T: Number> CanvasRenderable<Vector2<T>> for Triangle2D<T> {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        context.begin_path();
        context.move_to(self[0].x().to_f64(), self[0].y().to_f64());
        context.line_to(self[1].x().to_f64(), self[1].y().to_f64());
        context.line_to(self[2].x().to_f64(), self[2].y().to_f64());
        context.fill();
        context.close_path();
    }
}
impl<T: Number, R: CanvasRenderable<Vector2<T>, Context = CanvasRenderingContext2d>> CanvasRenderable<Vector2<T>> for &[R] {
    type Context = CanvasRenderingContext2d;
    fn fill_canvas(&self, context: &Self::Context) {
        for i in self.iter() {
            i.fill_canvas(context);
        }
    }
}

pub fn fill_transformed<T: Real, R: CanvasRenderable<Vector2<T>, Context = CanvasRenderingContext2d> + CalculateCentroid<Vector = Vector2<T>>>(value: &R, context: &CanvasRenderingContext2d, translation: Vector2<T>, rotaton: T) {
    // Rotate in the centroid
    context.translate(value.centroid().x().to_f64(), value.centroid().y().to_f64()).expect("Failed to translate renderable in canvas");
    context.rotate(rotaton.to_f64());
    context.translate(-value.centroid().x().to_f64(), -value.centroid().y().to_f64()).expect("Failed to translate renderable in canvas");
    // Draw to the canvas
    value.fill_canvas(context);
    // Reset rotation
    context.translate(value.centroid().x().to_f64(), value.centroid().y().to_f64()).expect("Failed to translate renderable in canvas");
    context.rotate(-rotaton.to_f64());
    context.translate(-value.centroid().x().to_f64(), -value.centroid().y().to_f64()).expect("Failed to translate renderable in canvas");
}
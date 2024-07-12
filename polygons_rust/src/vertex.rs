use nalgebra::Point2;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub point: Point2<f32>,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            point: Point2::new(x, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex::new(1.0, 2.0);
        assert_eq!(vertex.point, Point2::new(1.0, 2.0));
    }
}

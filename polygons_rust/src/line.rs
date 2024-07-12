use crate::framebuffer::Framebuffer;
use crate::vertex::Vertex;
use crate::colors::Color;

pub trait Line {
    fn line(&mut self, start: Vertex, end: Vertex);
    fn draw_polygon(&mut self, points: &[Vertex]);
    fn fill_polygon(&mut self, points: &[Vertex], color: Color);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vertex, end: Vertex) {
        let mut x1 = start.point.x.round() as isize;
        let mut y1 = start.point.y.round() as isize;
        let x2 = end.point.x.round() as isize;
        let y2 = end.point.y.round() as isize;

        let dx = isize::abs(x2 - x1);
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -isize::abs(y2 - y1);
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy; // error value e_xy

        loop {
            if x1 >= 0 && y1 >= 0 && (x1 as usize) < self.width && (y1 as usize) < self.height {
                self.point(x1 as usize, y1 as usize);
            }
            if x1 == x2 && y1 == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy; // e_xy+e_x > 0
                x1 += sx;
            }
            if e2 <= dx { // e_xy+e_y < 0
                y1 += sy;
                err += dx;
            }
        }
    }

    fn draw_polygon(&mut self, points: &[Vertex]) {
        if points.len() < 3 {
            return; 
        }

        let mut last_point = &points[0]; 
        for point in &points[1..] {
            self.line(*last_point, *point);
            last_point = point;
        }
        // Cerrar el polígono conectando el último punto con el primero
        self.line(*last_point, points[0]);
    }

    fn fill_polygon(&mut self, points: &[Vertex], color: Color) {
        if points.len() < 3 { return; } 

        self.set_current_color(color.to_hex());

        // Determina los límites verticales del polígono
        let min_y = points.iter().min_by(|a, b| a.point.y.partial_cmp(&b.point.y).unwrap()).unwrap().point.y as usize;
        let max_y = points.iter().max_by(|a, b| a.point.y.partial_cmp(&b.point.y).unwrap()).unwrap().point.y as usize;

        // Procesar cada scanline desde min_y hasta max_y
        for y in min_y..=max_y {
            let mut node_x: Vec<usize> = Vec::new(); // Puntos de intersección x de la scanline

            // Revisa cada arista del polígono
            let mut j = points.len() - 1;
            for i in 0..points.len() {
                let point1 = points[i];
                let point2 = points[j];

                if (point1.point.y < y as f32 && point2.point.y >= y as f32) || (point2.point.y < y as f32 && point1.point.y >= y as f32) {
                    // Calcular la posición x de la intersección de la arista con la scanline
                    let x = (point1.point.x + (y as f32 - point1.point.y) / (point2.point.y - point1.point.y) * (point2.point.x - point1.point.x)) as usize;
                    node_x.push(x);
                }
                j = i;
            }

            node_x.sort(); // Ordenar los puntos de intersección

            // Rellena los píxeles entre pares de intersecciones
            for i in (0..node_x.len()).step_by(2) {
                if i + 1 < node_x.len() {
                    for x in node_x[i]..=node_x[i + 1] {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}


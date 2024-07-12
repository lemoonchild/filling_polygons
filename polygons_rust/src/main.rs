mod colors;
mod framebuffer;
mod line;
mod bmp;
mod vertex;

use colors::Color;
use framebuffer::Framebuffer;
use line::Line; 
use vertex::Vertex;

fn main() {
    let mut fb = Framebuffer::new(800, 600, true); // Flip Y-axis

    fb.set_background_color(0x000000);
    fb.clear();

    // Coordenadas del polígono 1
    let vertices_polygon1 = vec![
        Vertex::new(165.0, 380.0),
        Vertex::new(185.0, 360.0),
        Vertex::new(180.0, 330.0),
        Vertex::new(207.0, 345.0),
        Vertex::new(233.0, 330.0),
        Vertex::new(230.0, 360.0),
        Vertex::new(250.0, 380.0),
        Vertex::new(220.0, 385.0),
        Vertex::new(205.0, 410.0),
        Vertex::new(193.0, 383.0),
    ];

    // Rellena y dibuja el polígono 1
    fb.fill_polygon(&vertices_polygon1, Color::from_hex(0xFFFF00)); // Amarillo para el relleno
    fb.set_current_color(0xFFFFFF); // Blanco para el contorno
    fb.draw_polygon(&vertices_polygon1);

    // Coordenadas del polígono 2
    let vertices_polygon2 = vec![
        Vertex::new(321.0, 335.0),
        Vertex::new(288.0, 286.0),
        Vertex::new(339.0, 251.0),
        Vertex::new(374.0, 302.0),
    ];

    // Rellena y dibuja el polígono 2
    fb.fill_polygon(&vertices_polygon2, Color::from_hex(0x0000FF)); // Azul para el relleno
    fb.set_current_color(0xFFFFFF); // Blanco para el contorno
    fb.draw_polygon(&vertices_polygon2);

    // Guarda el resultado en un archivo BMP
    fb.render_buffer("output.bmp").unwrap();
}

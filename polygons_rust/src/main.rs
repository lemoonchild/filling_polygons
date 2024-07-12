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

    // Coordenadas del polígono 3
    let vertices_polygon3 = vec![
        Vertex::new(377.0, 249.0),
        Vertex::new(411.0, 197.0),
        Vertex::new(436.0, 249.0),
    ];

    // Rellena y dibuja el polígono 3
    fb.fill_polygon(&vertices_polygon3, Color::from_hex(0xFF0000)); // Rojo para el relleno
    fb.set_current_color(0xFFFFFF); // Blanco para el contorno
    fb.draw_polygon(&vertices_polygon3);

    // Coordenadas del polígono 4
    let vertices_polygon4 = vec![
        Vertex::new(413.0, 177.0),
        Vertex::new(448.0, 159.0),
        Vertex::new(502.0, 88.0),
        Vertex::new(553.0, 53.0),
        Vertex::new(535.0, 36.0),
        Vertex::new(676.0, 37.0),
        Vertex::new(660.0, 52.0),
        Vertex::new(750.0, 145.0),
        Vertex::new(761.0, 179.0),
        Vertex::new(672.0, 192.0),
        Vertex::new(659.0, 214.0),
        Vertex::new(615.0, 214.0),
        Vertex::new(632.0, 230.0),
        Vertex::new(580.0, 230.0),
        Vertex::new(597.0, 215.0),
        Vertex::new(552.0, 214.0),
        Vertex::new(517.0, 144.0),
        Vertex::new(466.0, 180.0),
    ];

    // Coordenadas del polígono 5 (agujero)
    let vertices_polygon5 = vec![
        Vertex::new(682.0, 175.0),
        Vertex::new(708.0, 120.0),
        Vertex::new(735.0, 148.0),
        Vertex::new(739.0, 170.0),
    ];

    // Rellena y dibuja el polígono 4 con el polígono 5 como agujero
    fb.fill_polygon_with_hole(&vertices_polygon4, &vertices_polygon5, Color::from_hex(0x00FF00)); // Verde para el relleno
    fb.set_current_color(0xFFFFFF); // Blanco para el contorno
    fb.draw_polygon(&vertices_polygon4);
    fb.draw_polygon(&vertices_polygon5);

    // Guarda el resultado en un archivo BMP
    fb.render_buffer("output.bmp").unwrap();
}

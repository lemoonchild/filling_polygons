#[derive(Debug, Clone, Copy, PartialEq)] 
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

// Implementa un constructor para la estructura Color
impl Color {
    pub fn new (r: u8, g: u8, b: u8) -> Color { // u8 es un entero sin signo de 8 bits
        Color { 
            r: r.clamp(0, 255), // se usa clamp para limitar el valor entre 0 y 255
            g: g.clamp(0, 255), 
            b: b.clamp(0, 255)
        }
    }

    // Crea un color desde un valor hexadecimal
    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8; // >> es el operador de desplazamiento a la derecha y & es el operador AND 
        let g = ((hex >> 8) & 0xFF) as u8; // el 0xFF es una máscara que se usa para obtener los 8 bits menos significativos
        let b = (hex & 0xFF) as u8; // no se desplaza el valor hexadecimal porque ya está en los 8 bits menos significativos
        Color { r, g, b }
    }

    // Convierte un color a su valor hexadecimal
    pub fn to_hex(&self) -> u32 { // &self es una referencia al objeto que llama al método
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

// Implementa el trait std::fmt::Display para la estructura Color
impl std::fmt::Display for Color {
    // Se encarga de definir cómo se formatea una instancia de Color para mostrarla como una cadena de texto
     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write! es un macro similar a println! pero utilizado para escribir formateado a un buffer.
        // Aquí, se está escribiendo la representación de Color en el formato específico de "Color(r: {}, g: {}, b: {})"
        // donde {} serán reemplazados por los valores de self.r, self.g, y self.b respectivamente.
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

// Implementa la suma de dos colores
impl std::ops::Add for Color {
    type Output = Color; // Output es el tipo de dato que se devuelve al sumar dos colores

    // Suma dos colores y devuelve un nuevo color
     fn add(self, other: Color) -> Color {
        Color {

            // saturating_add es un método que suma dos números y si el resultado es mayor a 255, devuelve 255
            r: self.r.saturating_add(other.r), // other es el color que se pasa como argumento
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b)
        }
    }
}

// Implementa la multiplicación de dos colores
impl std::ops::Mul<f32> for Color { // se usa f32 ya que se multiplicará un color por un escalar, f32 es un número de punto flotante de 32 bits
    type Output = Color;

    // Multiplica un color por un escalar y devuelve un nuevo color
     fn mul(self, factor: f32) -> Color { // self representa el Color que está siendo multiplicado y factor es el escalar por el cuál se está multiplicando
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8; // clamp limita el valor entre 0 y 255
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8; // se usa as f32 para convertir los valores de los colores a f32
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;

        Color { r, g, b }
    }
}

// Implementa la resta de dos colores
impl std::ops::Sub for Color {
    type Output = Color; // Output es el tipo de dato que se devuelve al restar dos colores

    // Resta dos colores y devuelve un nuevo color
    fn sub(self, other: Color) -> Color {
        Color {
            // saturating_sub es un método que resta dos números y si el resultado es menor que 0, devuelve 0
            r: self.r.saturating_sub(other.r), // other es el color que se pasa como argumento
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Importa todo del módulo padre

    #[test]
    fn test_color_creation() {
        let color = Color::new(255, 255, 255);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 255);
        assert_eq!(color.b, 255);
    }

    #[test]
    fn test_color_addition() {
        let color1 = Color::new(200, 100, 50);
        let color2 = Color::new(55, 155, 205);
        let result = color1 + color2;
        assert_eq!(result, Color::new(255, 255, 255)); 
    }

    #[test]
    fn test_color_multiplication() {
        let color = Color::new(10, 20, 30);
        let result = color * 10.0;
        assert_eq!(result, Color::new(100, 200, 255)); 
    }

    #[test]
    fn test_hex_conversion() {
        let color = Color::new(16, 32, 48);
        assert_eq!(color.to_hex(), 0x102030);
    }

    #[test]
    fn test_hex_parsing() {
        let color = Color::from_hex(0x102030);
        assert_eq!(color, Color::new(16, 32, 48));
    }
}

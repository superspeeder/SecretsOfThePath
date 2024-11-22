use bevy::color::Color;

#[inline]
pub fn brighten(col: Color, fact: f32) -> Color {
    let c = col.to_srgba();
    Color::srgba(c.red * fact, c.green * fact, c.blue * fact, c.alpha)
}

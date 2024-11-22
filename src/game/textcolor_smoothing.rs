use bevy::prelude::*;

#[derive(Default)]
pub struct TextColorSmoothingPlugin;

#[derive(Component)]
pub struct TargetTextColor(pub Color);

#[derive(Component)]
pub struct TextColorSmoothing {
    pub flat: f32,
    pub proportional: f32,
}

impl Default for TextColorSmoothing {
    fn default() -> Self {
        Self {
            flat: 1.0,
            proportional: 8.0,
        }
    }
}

impl Plugin for TextColorSmoothingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update_text_colors);
    }
}

#[inline]
pub fn smoothto(a: f32, b: f32, stepmax: f32) -> f32 {
    if a > b {
        (a - stepmax).max(b)
    } else {
        (a + stepmax).min(b)
    }
}

fn update_text_colors(
    mut query: Query<(&TextColorSmoothing, &TargetTextColor, &mut Text)>,
    time: Res<Time>,
) {
    for (smoothing, target_color, mut text) in query.iter_mut() {
        for section in text.sections.iter_mut() {
            let orig = section.style.color.to_srgba();
            let targ = target_color.0.to_srgba();
            section.style.color = Color::srgba(
                smoothto(orig.red, targ.red, (smoothing.flat + (targ.red - orig.red).abs() * smoothing.proportional) * time.delta_seconds()),
                smoothto(orig.green, targ.green, (smoothing.flat + (targ.green - orig.green).abs() * smoothing.proportional) * time.delta_seconds()),
                smoothto(orig.blue, targ.blue, (smoothing.flat + (targ.blue - orig.blue).abs() * smoothing.proportional) * time.delta_seconds()),
                smoothto(orig.alpha, targ.alpha, (smoothing.flat + (targ.alpha - orig.alpha).abs() * smoothing.proportional) * time.delta_seconds()),
            );
        }
    }
}

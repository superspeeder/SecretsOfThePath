use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use crate::utils::position_smoothing::{PositionSmoothing, TargetPosition};

///
/// Plugin which provides an easy way to do 2d responsive elements, based on using the
///
///
pub struct ResponsivePlugin;

#[derive(Copy, Clone, Debug)]
pub enum ResponsiveValue {
    Absolute(f32),
    Percentage(f32),
}

#[derive(Copy, Clone, Debug)]
pub struct ResponsiveVec2 {
    pub x: ResponsiveValue,
    pub y: ResponsiveValue,
}

#[derive(Copy, Clone, Debug)]
pub struct ResponsiveVec3 {
    pub x: ResponsiveValue,
    pub y: ResponsiveValue,
    pub z: ResponsiveValue,
}

impl ResponsiveVec2 {
    pub fn resolve(&self, area: Rect) -> Vec2 {
        Vec2::new(self.x.resolve(area.min.x, area.max.y), self.y.resolve(area.min.y, area.max.y))
    }
}

impl ResponsiveVec3 {
    pub fn resolve(&self, area: Rect) -> Vec3 {
        Vec3::new(self.x.resolve(area.min.x, area.max.x), self.y.resolve(area.min.y, area.max.y), self.z.resolve(0.0, 1.0))
    }
}

impl ResponsiveValue {
    pub fn resolve(self, min: f32, max: f32) -> f32 {
        match self {
            ResponsiveValue::Absolute(v) => v,
            ResponsiveValue::Percentage(p) => p.remap(0.0, 100.0, min, max),
        }
    }
}

#[derive(Component)]
pub struct ResponsivePosition {
    pub position: ResponsiveVec3
}

impl ResponsivePosition {
    pub fn new(x: ResponsiveValue, y: ResponsiveValue, z: ResponsiveValue) -> Self {
        Self {
            position: ResponsiveVec3 { x, y, z }
        }
    }
}

impl Plugin for ResponsivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_positions, update_smoothed_positions));
    }
}

fn update_positions(mut query: Query<(&mut Transform, &ResponsivePosition), (Without<TargetPosition>)>, cameraq: Query<&OrthographicProjection>) {
    if cameraq.is_empty() { return; }
    let camera = cameraq.single();

    for (mut target, pos) in query.iter_mut() {
        target.translation = pos.position.resolve(camera.area);
    }
}

fn update_smoothed_positions(mut query: Query<(&mut TargetPosition, &ResponsivePosition)>, cameraq: Query<&OrthographicProjection>) {
    if cameraq.is_empty() { return; }
    let camera = cameraq.single();

    for (mut target, pos) in query.iter_mut() {
        target.0 = pos.position.resolve(camera.area);
    }
}


use bevy::math::NormedVectorSpace;
use bevy::prelude::*;

pub struct PositionSmoothingPlugin;

pub const DEFAULT_NEAR_MODIFIER: f32 = 1.0;
pub const DEFAULT_FAR_MODIFIER: f32 = 1.0;
pub const DEFAULT_SMOOTHING_SPEED: f32 = 128.0;
pub const DEFAULT_END_THRESHOLD: f32 = 2.5;

pub struct PSmoothing {
    pub modifier: f32,
    pub end_threshold: f32,
}


// PFP = Proportional, Fixed, Proportional
pub struct PFPSmoothing {
    pub smoothing_speed: f32, // fixed movement speed

    pub far_speedup: bool,
    pub far_threshold: Option<f32>, // this will default to 10.0 * smoothing_speed
    pub far_modifier: f32,          // this will default to DEFAULT_FAR_MODIFIER. This is used as delta_time * (smoothing_speed + (far_modifier * (target_distance - far_threshold))). This effectively just adds extra speed based on distance to the smoothing range. Recommended to tune these numbers to make it seem natural or else you might get some jarring slowdowns. This can be thought of as 1/number of seconds to reach threshold.

    pub near_slowdown: bool,
    pub near_threshold: Option<f32>, // this will default to 0.5 * smoothing_speed.

    /// This will default to DEFAULT_NEAR_MODIFIER.
    /// It is used as delta_time * near_modifier * error
    pub near_modifier: f32,

    /// This determines how close to the destination should be considered at the destination (generally, you can set this to be something like 0.5 and be guaranteed that you are at your destination. Setting this to a larger value will likely result in a bit of teleportation, and a smaller value will likely result in a longer chunk of time when this keeps trying to move by tiny amounts).
    pub end_threshold: f32,
}

#[derive(Component, Default)]
pub struct TargetPosition(pub Vec3);

impl Default for PFPSmoothing {
    fn default() -> Self {
        Self {
            smoothing_speed: DEFAULT_SMOOTHING_SPEED,
            far_speedup: false,
            far_threshold: None,
            far_modifier: DEFAULT_FAR_MODIFIER,
            near_slowdown: true,
            near_threshold: None,
            near_modifier: DEFAULT_FAR_MODIFIER,
            end_threshold: DEFAULT_END_THRESHOLD,
        }
    }
}

impl PFPSmoothing {

    /// Smooth a position based on the settings present in this component.
    ///
    /// # Arguments
    ///
    /// * `position`: The current position.
    /// * `target`: The target position
    ///
    /// returns: The new position
    ///
    /// # Examples
    ///
    /// ```
    /// // Normally you would be placing this on an entity and allowing the system to use it, but for the purpose of this example we will fabricate some values.
    /// let smooth_component = SmoothPosition::default();
    /// let current = Vec3::new(0.0, 0.0, 0.0);
    /// let target = Vec3::new(512.0, 0.0, 0.0);
    /// let delta_time = 1.0 / 50.0;
    /// let new_position = smooth_component.smooth_position(current, target, delta_time);
    /// // After this, new_position should be Vec2{ x: 506.88, y: 0.0, z: 0.0 };
    /// ```
    pub fn smooth_position(&self, position: Vec3, target: Vec3, delta_time: f32) -> Vec3 {
        let error = position.distance(target);
        let new_pos = if error < self.end_threshold {
            target // teleport to target when we are this close
        } else {
            let near_threshold = self.near_threshold.unwrap_or(self.smoothing_speed);
            let far_threshold = self.far_threshold.unwrap_or(self.smoothing_speed * 10.);
            let direction = (target - position).normalize();

            if self.near_slowdown && error < near_threshold {
                position + (delta_time * self.near_modifier * error).min(error) * direction // keep clamped to not overshoot
            } else if self.far_speedup && error > far_threshold {
                // delta_time * (smoothing_speed + (far_modifier * (target_distance - far_threshold)))
                position + (delta_time * (self.smoothing_speed + (self.far_modifier * (error - far_threshold)))).min(error) * direction
            } else {
                position + (delta_time * self.smoothing_speed).min(error) * direction
            }
        };

        new_pos
    }
}

impl PSmoothing {
    pub fn smooth_position(&self, position: Vec3, target: Vec3, delta_time: f32) -> Vec3 {
        let error = position.distance(target);
        if error < self.end_threshold {
            position
        } else {
            let direction = (target - position).normalize();
            (self.modifier * error * delta_time).min(error) * direction + position
        }
    }
}

#[derive(Component, Default)]
pub enum PositionSmoothing {
    #[default]
    None,
    P(PSmoothing),
    PFP(PFPSmoothing),
}

impl PositionSmoothing {
    pub fn smooth_position(&self, position: Vec3, target: Vec3, delta_time: f32) -> Vec3 {
        match self {
            PositionSmoothing::None => target,
            PositionSmoothing::P(p) => p.smooth_position(position, target, delta_time),
            PositionSmoothing::PFP(pfp) => pfp.smooth_position(position, target, delta_time),
        }
    }
}



impl Plugin for PositionSmoothingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::update_smoothed);
    }
}

impl PositionSmoothingPlugin {
    fn update_smoothed(mut query: Query<(&mut Transform, &PositionSmoothing, &TargetPosition)>, time: Res<Time>) {
        for (mut transform, smooth_position, target_position) in query.iter_mut() {
            transform.translation = smooth_position.smooth_position(transform.translation, target_position.0, time.delta_seconds());
        }
    }
}
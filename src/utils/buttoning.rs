use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::TextLayoutInfo;
use bevy::window::PrimaryWindow;
use std::marker::PhantomData;

pub struct ButtoningPlugin;

#[derive(Default, Component)]
pub struct TextButton;

#[derive(Default, Component)]
pub struct EnableHoverTest;

#[derive(Default, Component)]
pub struct Clickable;

///
/// This component should only be added during the initialization of entities with other components
///
#[derive(Component)]
pub struct GenericBoundingBox {
    rect: Rect,
}

#[derive(Default, Component)]
pub struct Hovered;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ButtonClickType {
    Pressed,
    Released,
}

#[derive(Event)]
pub struct ButtonClickEvent {
    pub entity: Entity,
    pub mouse_position: Vec2,
    pub mouse_button: MouseButton,
    pub click_type: ButtonClickType,
}

impl Plugin for ButtoningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                setup_bounding_boxes,
                (TextButton::update_bounding_boxes)
                    .before(update_hoverers)
                    .after(setup_bounding_boxes),
                update_hoverers,
            ),
        );
        app.add_systems(Update, update_clicks);
        app.add_event::<ButtonClickEvent>();
    }
}

fn update_hoverers(
    mut commands: Commands,
    query: Query<(Entity, Has<Hovered>, &GenericBoundingBox), With<EnableHoverTest>>,
    windowq: Query<&Window, With<PrimaryWindow>>,
    cameraq: Query<(&Camera, &GlobalTransform)>,
) {
    if windowq.is_empty() || cameraq.is_empty() {
        return;
    }
    if let Some(cursor_position) = windowq.single().cursor_position() {
        let (camera, ctransform) = cameraq.single();
        let cursor_position = camera
            .viewport_to_world_2d(ctransform, cursor_position)
            .unwrap_or(cursor_position);

        for (entity, was_hovered, bounding_box) in query.iter() {
            let hovered = bounding_box.rect.contains(cursor_position);
            if hovered && !was_hovered {
                commands.entity(entity).insert(Hovered);
            } else if !hovered && was_hovered {
                commands.entity(entity).remove::<Hovered>();
            }
        }
    }
}

fn update_clicks(
    query: Query<Entity, (With<Hovered>, With<Clickable>)>,
    mut event_writer: EventWriter<ButtonClickEvent>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    windowq: Query<&Window, With<PrimaryWindow>>,
    cameraq: Query<(&Camera, &GlobalTransform)>,
) {
    if windowq.is_empty() || cameraq.is_empty() {
        return;
    }
    if let Some(cursor_position) = windowq.single().cursor_position() {
        let (camera, ctransform) = cameraq.single();
        let cursor_position = camera
            .viewport_to_world_2d(ctransform, cursor_position)
            .unwrap_or(cursor_position); // need this to check actual cursor position

        for entity in query.iter() {
            for mouse_button in mouse_buttons.get_just_pressed().cloned() {
                event_writer.send(ButtonClickEvent {
                    entity,
                    mouse_position: cursor_position,
                    mouse_button,
                    click_type: ButtonClickType::Pressed,
                });
            }

            for mouse_button in mouse_buttons.get_just_released().cloned() {
                event_writer.send(ButtonClickEvent {
                    entity,
                    mouse_position: cursor_position,
                    mouse_button,
                    click_type: ButtonClickType::Released,
                });
            }
        }
    }
}

fn setup_bounding_boxes(
    mut commands: Commands,
    query: Query<Entity, (With<TextButton>, Without<GenericBoundingBox>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((EnableHoverTest, GenericBoundingBox { rect: Rect::EMPTY }, Clickable));
    }
}

impl TextButton {
    fn update_bounding_boxes(
        mut query: Query<
            (
                &mut GenericBoundingBox,
                &TextLayoutInfo,
                &Anchor,
                &Transform,
            ),
            With<TextButton>,
        >,
    ) {
        for (mut bounding_box, text_info, anchor, transform) in query.iter_mut() {
            let anchor_point = anchor.as_vec();
            let size = text_info.logical_size;
            let inner_offset = size * anchor_point;
            let center = transform.translation.xy() - inner_offset;
            bounding_box.rect = Rect::from_center_size(center, size);
        }
    }
}

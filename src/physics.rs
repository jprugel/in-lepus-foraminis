use bevy::prelude::*;
use bevy::math::Vec3A;

#[derive(Event)]
pub struct OnCollisionEvent(Collision);
pub struct Collision {
    entity_a: Entity, // The entity checking for collisions.
    entity_b: Entity, // The entity that collided into collider a.
}

#[derive(Component, Debug, Default, PartialEq, Reflect)]
pub struct Collider {
    pub center: Vec3A,
    pub radius: Vec3A,
}

impl Collider {
    pub fn new(center: Vec3A, radius: Vec3A) -> Self {
        Self {
            center,
            radius,
        }
    }
}

pub fn update_collider_center(
    mut query: Query<(&mut Collider, &Transform)>
) {
    for (mut collider, transform) in query.iter_mut() {
        collider.center = transform.translation.into();
    }
}

pub fn detect_collisions(
    query: Query<(Entity, &Collider)>,
    mut event: EventWriter<OnCollisionEvent>
) {
    for (entity_a, a) in query.iter() {
        for (entity_b, b) in query.iter() {
            if entity_a.index() == entity_b.index() { break }

            let x_axis_delta = a.center.x - b.center.x; 
            let y_axis_delta = a.center.y - b.center.y;

            let sum_x_size = a.radius.x + b.radius.x;
            let sum_y_size = a.radius.y + b.radius.y;

            if (-sum_x_size..sum_x_size).contains(&x_axis_delta) 
            && (-sum_y_size..sum_y_size).contains(&y_axis_delta) {
                event.send(OnCollisionEvent(Collision {
                    entity_a,
                    entity_b,
                }));
            }
        }
    }
}

pub fn debug_collisions(
    mut event_reader: EventReader<OnCollisionEvent>
) {
    for event in event_reader.read() {
        info!("collision event between {} and {}", event.0.entity_a.index(), event.0.entity_b.index());
    }
}

pub fn draw_collider_gizmos(
    query: Query<&Collider>,
    mut gizmos: Gizmos,
) {
    for collider in query.iter() {
        gizmos.rect_2d(
            Vec2::new(collider.center.x, collider.center.y),
            0.,
            Vec2::new(collider.radius.x * 2., collider.radius.y * 2.),
            Color::YELLOW,
        );
    }
}

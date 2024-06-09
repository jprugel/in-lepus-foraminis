use bevy::prelude::*;
use bevy::math::Vec3A;

#[derive(Event, Deref, DerefMut)]
pub struct OnCollisionEvent(Collision);

pub struct Collision {
    entity_a: Entity, // The entity checking for collisions.
    entity_b: Entity, // The entity that collided into collider a.
    normal: Vec3,
    penetration_depth: f32,
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
                //Calculate depth
                let x_penetration = x_axis_delta - sum_x_size;
                let y_penetration = y_axis_delta - sum_y_size;
                let penetration_depth = x_penetration.min(y_penetration);

                event.send(OnCollisionEvent(Collision {
                    entity_a,
                    entity_b,
                    normal: (b.center - a.center).normalize_or_zero().into(),
                    penetration_depth,
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

#[derive(Component, Reflect, Debug)]
pub struct Rigidbody {
    pub mass: f32,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub drag: f32,
    pub angular_drag: f32,
    pub net_force: Vec3,
    pub net_angular_force: Vec3,
    pub freeze_position: BVec3,
    pub freeze_rotation: BVec3,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            mass: 1.,
            velocity: Vec3::default(),
            angular_velocity: Vec3::default(),
            drag: 0.01,
            angular_drag: 0.01,
            net_force: Vec3::default(),
            net_angular_force: Vec3::default(),
            freeze_position: BVec3::default(),
            freeze_rotation: BVec3::default(),
        }
    }
}

impl Rigidbody {
    fn new(
        mass: f32,
        velocity: Vec3,
        angular_velocity: Vec3,
        drag: f32,
        angular_drag: f32,
        net_force: Vec3,
        net_angular_force: Vec3,
        freeze_position: BVec3,
        freeze_rotation: BVec3
) -> Self {
        Self {
            mass,
            velocity,
            angular_velocity,
            drag,
            angular_drag,
            net_force,
            net_angular_force,
            freeze_position,
            freeze_rotation,
        }
    }

    pub fn add_force(&mut self, force_type: ForceType, force: Vec3) {
        match force_type {
            ForceType::Continuous => self.velocity += force,
            ForceType::Impulse    => self.velocity = force,
        };
    }
}

pub enum ForceType {
    Continuous,
    Impulse
}

pub fn simulate_rigidbodies(mut query: Query<(&mut Rigidbody, &mut Transform)>) {
    for (rigidbody, mut transform) in query.iter_mut() {
        let rigidbody = rigidbody.into_inner();

        rigidbody.velocity += rigidbody.net_force;
        transform.translation += rigidbody.velocity;
        rigidbody.net_force = Vec3::ZERO;
    }
}

pub fn collision_response(
    mut event_reader: EventReader<OnCollisionEvent>,
    mut query: Query<&mut Rigidbody>
) {
    for collision in event_reader.read() {
        if let Ok([a, b]) = query.get_many_mut([collision.entity_a, collision.entity_b]) {
            let a = a.into_inner();
            let b = b.into_inner();
            let e = 1.; // Coefficient of restitution
            let relative_velocity = a.velocity - b.velocity;
            let velocity_of_collision = -(1. + e) * relative_velocity * collision.normal;
            let impulse = velocity_of_collision
                / ( 1. / a.mass + 1. / b.mass);

            a.velocity += (1. / a.mass) * impulse * collision.normal;
            b.velocity -= (1. / b.mass) * impulse * collision.normal;
        }
    }
}

use bevy::a11y::accesskit::ListStyle::Circle;
use bevy::prelude::*;
use bevy::sprite::{ MaterialMesh2dBundle };

#[derive(Component, Default)]
pub struct Physics {
    speed: Vec2,
    acc: Vec2
}

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    base: MaterialMesh2dBundle<ColorMaterial>,
    physics: Physics
}

pub struct BaseParticle {
    radius: f32,
    pos: Vec3
}

impl BaseParticle {
    pub fn toBundle(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> ParticleBundle {
        return ParticleBundle {
            base: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
            physics: default()
        }
    }
}

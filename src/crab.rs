use bevy::prelude::*;
use crate::states::GameState;
use crate::beat::BeatResource;
use crate::battle::{BattleResource, HitResult};

pub struct CrabPlugin;

impl Plugin for CrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_crabs)
            .add_systems(Update, (
                animate_crabs,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnExit(GameState::Playing), despawn_crabs);
    }
}

#[derive(Component)]
pub struct Crab {
    pub is_player: bool,
    pub base_y: f32,
}

#[derive(Component)]
pub struct CrabBody;

fn spawn_crabs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Player crab (blue/teal) at z=0
    spawn_crab(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 0.0),
        Color::rgb(0.0, 0.8, 0.8),
        true,
        0.0,
    );

    // Opponent crab (orange/red) at z=22, facing toward player
    spawn_crab(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, 22.0),
        Color::rgb(1.0, 0.4, 0.1),
        false,
        std::f32::consts::PI,
    );
}

fn spawn_crab(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    color: Color,
    is_player: bool,
    y_rotation: f32,
) {
    let body_material = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });

    let eye_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    });

    let pupil_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.0),
        ..default()
    });

    let claw_material = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });

    let dark_color = match is_player {
        true => Color::rgb(0.0, 0.5, 0.5),
        false => Color::rgb(0.8, 0.2, 0.0),
    };

    let leg_material = materials.add(StandardMaterial {
        base_color: dark_color,
        ..default()
    });

    // Body - flattened sphere
    let body_mesh = meshes.add(Sphere::new(1.0).mesh().uv(32, 18));
    // Eye stalk
    let eye_stalk_mesh = meshes.add(Cylinder::new(0.05, 0.3));
    // Eye sphere
    let eye_sphere_mesh = meshes.add(Sphere::new(0.15).mesh().uv(16, 8));
    // Pupil
    let pupil_mesh = meshes.add(Sphere::new(0.07).mesh().uv(8, 4));
    // Claw arm
    let claw_arm_mesh = meshes.add(Cylinder::new(0.1, 0.5));
    // Claw tip
    let claw_tip_mesh = meshes.add(Sphere::new(0.2).mesh().uv(16, 8));
    // Leg
    let leg_mesh = meshes.add(Cylinder::new(0.06, 0.4));

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position)
                .with_rotation(Quat::from_rotation_y(y_rotation)),
            ..default()
        },
        Crab { is_player, base_y: position.y },
    ))
    .with_children(|parent| {
        // Body
        parent.spawn((
            PbrBundle {
                mesh: body_mesh,
                material: body_material.clone(),
                transform: Transform::from_xyz(0.0, 0.5, 0.0)
                    .with_scale(Vec3::new(1.0, 0.6, 0.8)),
                ..default()
            },
            CrabBody,
        ));

        // Left eye stalk
        parent.spawn(PbrBundle {
            mesh: eye_stalk_mesh.clone(),
            material: leg_material.clone(),
            transform: Transform::from_xyz(-0.35, 0.95, -0.3)
                .with_rotation(Quat::from_rotation_z(0.2)),
            ..default()
        });
        // Left eye
        parent.spawn(PbrBundle {
            mesh: eye_sphere_mesh.clone(),
            material: eye_material.clone(),
            transform: Transform::from_xyz(-0.45, 1.15, -0.3),
            ..default()
        });
        // Left pupil
        parent.spawn(PbrBundle {
            mesh: pupil_mesh.clone(),
            material: pupil_material.clone(),
            transform: Transform::from_xyz(-0.52, 1.15, -0.38),
            ..default()
        });

        // Right eye stalk
        parent.spawn(PbrBundle {
            mesh: eye_stalk_mesh.clone(),
            material: leg_material.clone(),
            transform: Transform::from_xyz(0.35, 0.95, -0.3)
                .with_rotation(Quat::from_rotation_z(-0.2)),
            ..default()
        });
        // Right eye
        parent.spawn(PbrBundle {
            mesh: eye_sphere_mesh.clone(),
            material: eye_material.clone(),
            transform: Transform::from_xyz(0.45, 1.15, -0.3),
            ..default()
        });
        // Right pupil
        parent.spawn(PbrBundle {
            mesh: pupil_mesh.clone(),
            material: pupil_material.clone(),
            transform: Transform::from_xyz(0.52, 1.15, -0.38),
            ..default()
        });

        // Left claw arm
        parent.spawn(PbrBundle {
            mesh: claw_arm_mesh.clone(),
            material: claw_material.clone(),
            transform: Transform::from_xyz(-1.2, 0.5, -0.2)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        // Left claw tip
        parent.spawn(PbrBundle {
            mesh: claw_tip_mesh.clone(),
            material: claw_material.clone(),
            transform: Transform::from_xyz(-1.6, 0.5, -0.2)
                .with_scale(Vec3::new(1.0, 0.7, 0.7)),
            ..default()
        });

        // Right claw arm
        parent.spawn(PbrBundle {
            mesh: claw_arm_mesh.clone(),
            material: claw_material.clone(),
            transform: Transform::from_xyz(1.2, 0.5, -0.2)
                .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        // Right claw tip
        parent.spawn(PbrBundle {
            mesh: claw_tip_mesh.clone(),
            material: claw_material.clone(),
            transform: Transform::from_xyz(1.6, 0.5, -0.2)
                .with_scale(Vec3::new(1.0, 0.7, 0.7)),
            ..default()
        });

        // Legs (4 per side)
        for i in 0..4 {
            let leg_z = -0.1 + i as f32 * 0.25;

            // Left legs
            parent.spawn(PbrBundle {
                mesh: leg_mesh.clone(),
                material: leg_material.clone(),
                transform: Transform::from_xyz(-1.1, 0.1, leg_z)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4)),
                ..default()
            });

            // Right legs
            parent.spawn(PbrBundle {
                mesh: leg_mesh.clone(),
                material: leg_material.clone(),
                transform: Transform::from_xyz(1.1, 0.1, leg_z)
                    .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4)),
                ..default()
            });
        }
    });
}

fn animate_crabs(
    beat: Res<BeatResource>,
    battle: Res<BattleResource>,
    mut crab_query: Query<(&Crab, &mut Transform)>,
) {
    let bob_height = 0.15;
    let bob_y = (beat.beat_pulse * std::f32::consts::TAU).sin() * bob_height;

    for (crab, mut transform) in crab_query.iter_mut() {
        // Bob up and down with beat
        transform.translation.y = crab.base_y + bob_y;

        // Flash on hit - slight scale change
        if battle.hit_flash_timer > 0.0 {
            let flash = match battle.last_hit {
                Some(HitResult::Miss) if !crab.is_player => 1.0,
                Some(HitResult::Miss) => {
                    1.0 + battle.hit_flash_timer * 0.2
                }
                Some(_) if !crab.is_player => {
                    1.0 + battle.hit_flash_timer * 0.1
                }
                _ => 1.0,
            };
            if crab.is_player && matches!(battle.last_hit, Some(HitResult::Miss)) {
                let shake = (battle.hit_flash_timer * 30.0).sin() * 0.05;
                transform.translation.x = shake;
            } else if !crab.is_player && !matches!(battle.last_hit, Some(HitResult::Miss)) {
                transform.translation.y = crab.base_y + bob_y + battle.hit_flash_timer * 0.1;
            }
            transform.scale = Vec3::splat(flash);
        } else {
            transform.scale = Vec3::ONE;
            if crab.is_player {
                transform.translation.x = 0.0;
            }
        }
    }
}

fn despawn_crabs(
    mut commands: Commands,
    query: Query<Entity, With<Crab>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

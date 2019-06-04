// Library
use specs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
use vek::*;

// Crate
use crate::{
    comp::{
        phys::{ForceUpdate, Ori, Pos, Vel},
        Animation, AnimationInfo, Attacking, Control, Gliding, HealthSource, Jumping, Respawning,
        Stats,
    },
    state::{DeltaTime, Time, Uid},
    terrain::TerrainMap,
    vol::{ReadVol, Vox},
};

// Basic ECS AI agent system
pub struct Sys;

impl<'a> System<'a> for Sys {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Uid>,
        Read<'a, Time>,
        Read<'a, DeltaTime>,
        ReadExpect<'a, TerrainMap>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Ori>,
        WriteStorage<'a, AnimationInfo>,
        WriteStorage<'a, Stats>,
        ReadStorage<'a, Control>,
        WriteStorage<'a, Jumping>,
        WriteStorage<'a, Respawning>,
        WriteStorage<'a, Gliding>,
        WriteStorage<'a, Attacking>,
        WriteStorage<'a, ForceUpdate>,
    );

    fn run(
        &mut self,
        (
            entities,
            uids,
            time,
            dt,
            terrain,
            positions,
            mut velocities,
            mut orientations,
            mut animation_infos,
            mut stats,
            mut controls,
            mut jumps,
            mut respawns,
            mut glides,
            mut attacks,
            mut force_updates,
        ): Self::SystemData,
    ) {
        for (entity, pos, control, stats, mut ori, mut vel) in (
            &entities,
            &positions,
            &controls,
            &stats,
            &mut orientations,
            &mut velocities,
        )
            .join()
        {
            // Disable while dead TODO: Replace with client states
            if stats.is_dead {
                continue;
            }

            // Handle held-down control
            let on_ground = terrain
                .get((pos.0 - Vec3::unit_z() * 0.1).map(|e| e.floor() as i32))
                .map(|vox| !vox.is_empty())
                .unwrap_or(false)
                && vel.0.z <= 0.0;

            let (gliding, friction) = if on_ground {
                // TODO: Don't hard-code this.
                // Apply physics to the player: acceleration and non-linear deceleration.
                vel.0 += Vec2::broadcast(dt.0) * control.move_dir * 200.0;

                if jumps.get(entity).is_some() {
                    vel.0.z += 16.0;
                    jumps.remove(entity);
                }

                (false, 0.15)
            } else {
                // TODO: Don't hard-code this.
                // Apply physics to the player: acceleration and non-linear deceleration.
                vel.0 += Vec2::broadcast(dt.0) * control.move_dir * 10.0;

                if glides.get(entity).is_some() && vel.0.z < 0.0 {
                    // TODO: Don't hard-code this.
                    let anti_grav = 9.81 * 3.95 + vel.0.z.powf(2.0) * 0.2;
                    vel.0.z +=
                        dt.0 * anti_grav * Vec2::<f32>::from(vel.0 * 0.15).magnitude().min(1.0);

                    (true, 0.008)
                } else {
                    (false, 0.015)
                }
            };

            // Friction
            vel.0 -= Vec2::broadcast(dt.0)
                * 50.0
                * vel.0.map(|e| {
                    (e.abs() * friction * (vel.0.magnitude() * 0.1 + 0.5))
                        .min(e.abs() * dt.0 * 50.0)
                        .copysign(e)
                })
                * Vec3::new(1.0, 1.0, 0.0);

            if vel.0.magnitude_squared() != 0.0 {
                ori.0 = vel.0.normalized() * Vec3::new(1.0, 1.0, 0.0);
            }

            let animation = if on_ground {
                if control.move_dir.magnitude() > 0.01 {
                    Animation::Run
                } else if attacks.get(entity).is_some() {
                    Animation::Attack
                } else {
                    Animation::Idle
                }
            } else if glides.get(entity).is_some() {
                Animation::Gliding
            } else {
                Animation::Jump
            };

            let last = animation_infos
                .get_mut(entity)
                .cloned()
                .unwrap_or(AnimationInfo::default());
            let changed = last.animation != animation;

            animation_infos.insert(
                entity,
                AnimationInfo {
                    animation,
                    time: if changed { 0.0 } else { last.time },
                    changed,
                },
            );
        }

        for (entity, &uid, pos, ori, attacking) in
            (&entities, &uids, &positions, &orientations, &mut attacks).join()
        {
            if !attacking.applied {
                for (b, pos_b, mut stat_b, mut vel_b) in
                    (&entities, &positions, &mut stats, &mut velocities).join()
                {
                    // Check if it is a hit
                    if entity != b
                        && !stat_b.is_dead
                        && pos.0.distance_squared(pos_b.0) < 50.0
                        && ori.0.angle_between(pos_b.0 - pos.0).to_degrees() < 70.0
                    {
                        // Deal damage
                        stat_b.hp.change_by(-10, HealthSource::Attack { by: uid }); // TODO: variable damage and weapon
                        vel_b.0 += (pos_b.0 - pos.0).normalized() * 10.0;
                        vel_b.0.z = 15.0;
                        force_updates.insert(b, ForceUpdate);
                    }
                }
                attacking.applied = true;
            }
        }
    }
}
use super::SpawnRules;
use crate::{
    block::block_from_structure,
    column::ColumnSample,
    sim::WorldSim,
    site::{
        settlement::building::{
            archetype::keep::{Attr, Keep as KeepArchetype},
            Archetype, Branch, Ori,
        },
        BlockMask,
    },
    util::{attempt, Grid, RandomField, Sampler, CARDINALS, DIRS},
};
use common::{
    assets,
    astar::Astar,
    comp,
    generation::{ChunkSupplement, EntityInfo},
    npc,
    spiral::Spiral2d,
    store::{Id, Store},
    terrain::{Block, BlockKind, Structure, TerrainChunkSize},
    vol::{BaseVol, ReadVol, RectSizedVol, RectVolSize, Vox, WriteVol},
};
use core::{f32, hash::BuildHasherDefault};
use fxhash::FxHasher64;
use lazy_static::lazy_static;
use rand::prelude::*;
use std::sync::Arc;
use vek::*;

struct Keep {
    offset: Vec2<i32>,
    locus: i32,
    height: i32,
    is_tower: bool,
    alt: i32,
}

struct Tower {
    offset: Vec2<i32>,
    alt: i32,
}

pub struct Castle {
    origin: Vec2<i32>,
    alt: i32,
    seed: u32,
    radius: i32,
    towers: Vec<Tower>,
    keeps: Vec<Keep>,
    rounded_towers: bool,
}

pub struct GenCtx<'a, R: Rng> {
    sim: Option<&'a mut WorldSim>,
    rng: &'a mut R,
}

impl Castle {
    #[allow(clippy::let_and_return)] // TODO: Pending review in #587
    pub fn generate(wpos: Vec2<i32>, sim: Option<&mut WorldSim>, rng: &mut impl Rng) -> Self {
        let mut ctx = GenCtx { sim, rng };

        let boundary_towers = ctx.rng.gen_range(5, 10);
        let keep_count = ctx.rng.gen_range(1, 4);
        let boundary_noise = ctx.rng.gen_range(-2i32, 8).max(1) as f32;

        let radius = 150;

        let this = Self {
            origin: wpos,
            alt: ctx
                .sim
                .as_ref()
                .and_then(|sim| sim.get_alt_approx(wpos))
                .unwrap_or(0.0) as i32
                + 6,
            seed: ctx.rng.gen(),
            radius,

            towers: (0..boundary_towers)
                .map(|i| {
                    let angle = (i as f32 / boundary_towers as f32) * f32::consts::PI * 2.0;
                    let dir = Vec2::new(angle.cos(), angle.sin());
                    let dist = radius as f32 + ((angle * boundary_noise).sin() - 1.0) * 40.0;

                    let mut offset = (dir * dist).map(|e| e as i32);
                    // Try to move the tower around until it's not intersecting a path
                    for i in (1..80).step_by(5) {
                        if ctx
                            .sim
                            .as_ref()
                            .and_then(|sim| sim.get_nearest_path(wpos + offset))
                            .map(|(dist, _, _, _)| dist > 24.0)
                            .unwrap_or(true)
                        {
                            break;
                        }
                        offset = (dir * dist)
                            .map(|e| (e + ctx.rng.gen_range(-1.0, 1.0) * i as f32) as i32);
                    }

                    Tower {
                        offset,
                        alt: ctx
                            .sim
                            .as_ref()
                            .and_then(|sim| sim.get_alt_approx(wpos + offset))
                            .unwrap_or(0.0) as i32
                            + 2,
                    }
                })
                .collect(),
            rounded_towers: ctx.rng.gen(),
            keeps: (0..keep_count)
                .map(|i| {
                    let angle = (i as f32 / keep_count as f32) * f32::consts::PI * 2.0;
                    let dir = Vec2::new(angle.cos(), angle.sin());
                    let dist =
                        (radius as f32 + ((angle * boundary_noise).sin() - 1.0) * 40.0) * 0.3;

                    let locus = ctx.rng.gen_range(20, 26);
                    let offset = (dir * dist).map(|e| e as i32);
                    let height = ctx.rng.gen_range(25, 70).clamped(30, 48);

                    Keep {
                        offset,
                        locus,
                        height,
                        is_tower: true,
                        alt: ctx
                            .sim
                            .as_ref()
                            .and_then(|sim| sim.get_alt_approx(wpos + offset))
                            .unwrap_or(0.0) as i32
                            + 2,
                    }
                })
                .collect(),
        };

        this
    }

    pub fn contains_point(&self, wpos: Vec2<i32>) -> bool {
        let lpos = wpos - self.origin;
        for i in 0..self.towers.len() {
            let tower0 = &self.towers[i];
            let tower1 = &self.towers[(i + 1) % self.towers.len()];

            if lpos.determine_side(Vec2::zero(), tower0.offset) > 0
                && lpos.determine_side(Vec2::zero(), tower1.offset) <= 0
                && lpos.determine_side(tower0.offset, tower1.offset) > 0
            {
                return true;
            }
        }

        false
    }

    pub fn get_origin(&self) -> Vec2<i32> { self.origin }

    pub fn radius(&self) -> f32 { 1200.0 }

    #[allow(clippy::needless_update)] // TODO: Pending review in #587
    pub fn spawn_rules(&self, wpos: Vec2<i32>) -> SpawnRules {
        SpawnRules {
            trees: wpos.distance_squared(self.origin) > self.radius.pow(2),
            ..SpawnRules::default()
        }
    }

    pub fn apply_to<'a>(
        &'a self,
        wpos2d: Vec2<i32>,
        mut get_column: impl FnMut(Vec2<i32>) -> Option<&'a ColumnSample<'a>>,
        vol: &mut (impl BaseVol<Vox = Block> + RectSizedVol + ReadVol + WriteVol),
    ) {
        for y in 0..vol.size_xy().y as i32 {
            for x in 0..vol.size_xy().x as i32 {
                let offs = Vec2::new(x, y);

                let wpos2d = wpos2d + offs;
                let rpos = wpos2d - self.origin;

                if rpos.magnitude_squared() > (self.radius + 64).pow(2) {
                    continue;
                }

                let col_sample = if let Some(col) = get_column(offs) {
                    col
                } else {
                    continue;
                };

                // Inner ground
                if self.contains_point(wpos2d) {
                    let surface_z = col_sample.alt as i32;
                    for z in -5..3 {
                        let pos = Vec3::new(offs.x, offs.y, surface_z + z);

                        if z >= 0 {
                            if vol.get(pos).unwrap().kind() != BlockKind::Water {
                                let _ = vol.set(pos, Block::empty());
                            }
                        } else {
                            let _ = vol.set(
                                pos,
                                Block::new(
                                    BlockKind::Normal,
                                    col_sample.sub_surface_color.map(|e| (e * 255.0) as u8),
                                ),
                            );
                        }
                    }
                }

                let (wall_dist, wall_pos, wall_alt, wall_ori, towers) = (0..self.towers.len())
                    .map(|i| {
                        let tower0 = &self.towers[i];
                        let tower1 = &self.towers[(i + 1) % self.towers.len()];

                        let wall = LineSegment2 {
                            start: tower0.offset.map(|e| e as f32),
                            end: tower1.offset.map(|e| e as f32),
                        };

                        let projected = wall
                            .projected_point(rpos.map(|e| e as f32))
                            .map(|e| e.floor() as i32);

                        let tower0_dist = tower0
                            .offset
                            .map(|e| e as f32)
                            .distance(projected.map(|e| e as f32));
                        let tower1_dist = tower1
                            .offset
                            .map(|e| e as f32)
                            .distance(projected.map(|e| e as f32));
                        let tower_lerp = tower0_dist / (tower0_dist + tower1_dist);
                        let wall_ori = if (tower0.offset.x - tower1.offset.x).abs()
                            < (tower0.offset.y - tower1.offset.y).abs()
                        {
                            Ori::East
                        } else {
                            Ori::North
                        };

                        (
                            wall.distance_to_point(rpos.map(|e| e as f32)) as i32,
                            projected,
                            Lerp::lerp(tower0.alt as f32, tower1.alt as f32, tower_lerp) as i32,
                            wall_ori,
                            [tower0, tower1],
                        )
                    })
                    .min_by_key(|x| x.0)
                    .unwrap();
                let border_pos = (wall_pos - rpos).map(|e| e.abs());
                let wall_normal = (rpos - wall_pos).map(|e| e as f32);
                let wall_rpos = if wall_ori == Ori::East {
                    rpos
                } else {
                    rpos.yx()
                };
                let head_space = col_sample
                    .path
                    .map(|(dist, _, path, _)| path.head_space(dist))
                    .unwrap_or(0);

                // Apply the dungeon entrance
                let wall_sample = if let Some(col) = get_column(offs + wall_pos - rpos) {
                    col
                } else {
                    col_sample
                };

                let keep_archetype = KeepArchetype {
                    flag_color: Rgb::new(200, 80, 40),
                };

                for z in -10..64 {
                    let wpos = Vec3::new(wpos2d.x, wpos2d.y, col_sample.alt as i32 + z);

                    // Boundary wall
                    let wall_z = wpos.z - wall_alt;
                    if z < head_space {
                        continue;
                    }

                    let mut mask = keep_archetype.draw(
                        Vec3::from(wall_rpos) + Vec3::unit_z() * wpos.z - wall_alt,
                        wall_dist,
                        Vec2::new(border_pos.reduce_max(), border_pos.reduce_min()),
                        rpos - wall_pos,
                        wall_z,
                        wall_ori,
                        4,
                        0,
                        &Attr {
                            height: 16,
                            is_tower: false,
                            ridged: false,
                            rounded: true,
                        },
                    );

                    // Boundary towers
                    for tower in &self.towers {
                        let tower_wpos = Vec3::new(
                            self.origin.x + tower.offset.x,
                            self.origin.y + tower.offset.y,
                            tower.alt,
                        );
                        let tower_locus = 10;

                        let border_pos = (tower_wpos - wpos).xy().map(|e| e.abs());
                        mask = mask.resolve_with(keep_archetype.draw(
                            if (tower_wpos.x - wpos.x).abs() < (tower_wpos.y - wpos.y).abs() {
                                wpos - tower_wpos
                            } else {
                                Vec3::new(
                                    wpos.y - tower_wpos.y,
                                    wpos.x - tower_wpos.x,
                                    wpos.z - tower_wpos.z,
                                )
                            },
                            border_pos.reduce_max() - tower_locus,
                            Vec2::new(border_pos.reduce_max(), border_pos.reduce_min()),
                            (wpos - tower_wpos).xy(),
                            wpos.z - tower.alt,
                            Ori::East,
                            tower_locus,
                            0,
                            &Attr {
                                height: 28,
                                is_tower: true,
                                ridged: false,
                                rounded: self.rounded_towers,
                            },
                        ));
                    }

                    // Keeps
                    for keep in &self.keeps {
                        let keep_wpos = Vec3::new(
                            self.origin.x + keep.offset.x,
                            self.origin.y + keep.offset.y,
                            keep.alt,
                        );

                        let border_pos = (keep_wpos - wpos).xy().map(|e| e.abs());
                        mask = mask.resolve_with(keep_archetype.draw(
                            if (keep_wpos.x - wpos.x).abs() < (keep_wpos.y - wpos.y).abs() {
                                wpos - keep_wpos
                            } else {
                                Vec3::new(
                                    wpos.y - keep_wpos.y,
                                    wpos.x - keep_wpos.x,
                                    wpos.z - keep_wpos.z,
                                )
                            },
                            border_pos.reduce_max() - keep.locus,
                            Vec2::new(border_pos.reduce_max(), border_pos.reduce_min()),
                            (wpos - keep_wpos).xy(),
                            wpos.z - keep.alt,
                            Ori::East,
                            keep.locus,
                            0,
                            &Attr {
                                height: keep.height,
                                is_tower: keep.is_tower,
                                ridged: true,
                                rounded: self.rounded_towers,
                            },
                        ));
                    }

                    if let Some(block) = mask.finish() {
                        let _ = vol.set(Vec3::new(offs.x, offs.y, wpos.z), block);
                    }
                }
            }
        }
    }

    #[allow(clippy::or_fun_call)] // TODO: Pending review in #587
    pub fn apply_supplement<'a>(
        &'a self,
        rng: &mut impl Rng,
        wpos2d: Vec2<i32>,
        _get_column: impl FnMut(Vec2<i32>) -> Option<&'a ColumnSample<'a>>,
        supplement: &mut ChunkSupplement,
    ) {
        // TODO
    }
}
use super::{
    super::{vek::*, Animation},
    CharacterSkeleton, SkeletonAttr,
};
use common::{
    comp::item::{Hands, ToolKind},
    states::wielding::StageSection,
};
use std::f32::consts::PI;

pub struct AlphaAnimation;

impl Animation for AlphaAnimation {
    type Dependency = (
        Option<ToolKind>,
        Option<ToolKind>,
        f32,
        f64,
        Option<StageSection>,
    );
    type Skeleton = CharacterSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"character_alpha\0";

    #[cfg_attr(feature = "be-dyn-lib", export_name = "character_alpha")]
    #[allow(clippy::approx_constant)] // TODO: Pending review in #587
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (active_tool_kind, second_tool_kind, velocity, _global_time, stage_section): Self::Dependency,
        anim_time: f64,
        rate: &mut f32,
        skeleton_attr: &SkeletonAttr,
    ) -> Self::Skeleton {
        *rate = 1.0;
        let mut next = (*skeleton).clone();

        let lab = 1.0;

        let foot = (((1.0)
            / (0.2
                + 0.8
                    * ((anim_time as f32 * lab as f32 * 2.0 * velocity).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * lab as f32 * 2.0 * velocity).sin());
        let slowersmooth = (anim_time as f32 * lab as f32 * 4.0).sin();
        let accel_med = 1.0 - (anim_time as f32 * 16.0 * lab as f32).cos();
        let accel_slow = 1.0 - (anim_time as f32 * 12.0 * lab as f32).cos();
        let accel_fast = 1.0 - (anim_time as f32 * 24.0 * lab as f32).cos();
        let decel = (anim_time as f32 * 16.0 * lab as f32).min(PI / 2.0).sin();
        let push = anim_time as f32 * lab as f32 * 4.0;
        let slow = (((5.0)
            / (0.4 + 4.6 * ((anim_time as f32 * lab as f32 * 9.0).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * lab as f32 * 9.0).sin());
        let quick = (((5.0)
            / (0.4 + 4.6 * ((anim_time as f32 * lab as f32 * 18.0).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * lab as f32 * 18.0).sin());
        let slower = (((1.0)
            / (0.0001 + 0.999 * ((anim_time as f32 * lab as f32 * 4.0).sin()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * lab as f32 * 4.0).sin());
        let slowax = (((5.0)
            / (0.1 + 4.9 * ((anim_time as f32 * lab as f32 * 4.0 + 1.9).cos()).powf(2.0 as f32)))
        .sqrt())
            * ((anim_time as f32 * lab as f32 * 4.0 + 1.9).cos());

        let movement = anim_time as f32 * 1.0;
        let test = (anim_time as f32 * 1.75).sin();

        if let Some(ToolKind::Sword(_)) = active_tool_kind {
            next.l_hand.position = Vec3::new(-0.75, -1.0, 2.5);
            next.l_hand.orientation = Quaternion::rotation_x(1.47) * Quaternion::rotation_y(-0.2);
            next.l_hand.scale = Vec3::one() * 1.04;
            next.r_hand.position = Vec3::new(0.75, -1.5, -0.5);
            next.r_hand.orientation = Quaternion::rotation_x(1.47) * Quaternion::rotation_y(0.3);
            next.r_hand.scale = Vec3::one() * 1.05;
            next.main.position = Vec3::new(0.0, 0.0, 2.0);
            next.main.orientation = Quaternion::rotation_x(-0.1)
                * Quaternion::rotation_y(0.0)
                * Quaternion::rotation_z(0.0);

            if let Some(stage_section) = stage_section {
                match stage_section {
                    StageSection::Buildup => {
                        //println!("{:.3} build", anim_time);
                        next.control.position =
                            Vec3::new(-7.0, 7.0 + movement * -4.0, 2.0 + movement * 1.0);
                        next.control.orientation = Quaternion::rotation_x(movement * -0.5)
                            * Quaternion::rotation_y(movement * -1.0)
                            * Quaternion::rotation_z(movement * -1.2);

                        next.chest.orientation = Quaternion::rotation_z(movement * 1.5);
                        next.head.orientation = Quaternion::rotation_z(movement * -0.9);
                    },
                    StageSection::Swing => {
                        //println!("{:.3} swing", anim_time);
                        next.control.position = Vec3::new(-7.0, 3.0 + movement * 8.0, 3.0);
                        next.control.orientation =
                            Quaternion::rotation_x(-0.5 + movement * -1.0 * 0.0)
                                * Quaternion::rotation_y(-1.0 + movement * -0.6)
                                * Quaternion::rotation_z(-1.2 + movement * 1.3);

                        next.chest.orientation = Quaternion::rotation_z(1.5 + test * -3.0);
                        next.head.orientation = Quaternion::rotation_z(-0.9 + test * 2.5);
                        //next.head.orientation = Quaternion::rotation_z(-test
                        // * 0.8); next.chest.
                        // orientation = Quaternion::rotation_x(test * 0.15)
                        //* Quaternion::rotation_y(movement * 0.3)
                        //* Quaternion::rotation_z(movement * 1.5);
                        //next.belt.orientation = Quaternion::rotation_z(test2
                        // * 0.5); next.shorts.
                        // orientation = Quaternion::rotation_z(test2 * 1.5);
                        // next.torso.orientation = Quaternion::rotation_z(test2
                        // * 7.2);
                    },
                    StageSection::Recover | StageSection::Combo => {
                        //println!("{:.3} recover", anim_time);
                        next.control.position = Vec3::new(-7.0, 7.0, 2.0);
                        next.control.orientation = Quaternion::rotation_x(0.0)
                            * Quaternion::rotation_y(-1.57 + movement * 1.0)
                            * Quaternion::rotation_z(0.0);
                        next.control.scale = Vec3::one();
                        next.chest.orientation = Quaternion::rotation_y(0.0)
                            * Quaternion::rotation_z(-1.57 + movement * 0.5);
                        next.head.orientation =
                            Quaternion::rotation_y(0.0) * Quaternion::rotation_z(1.57);
                    },
                }
            }
        }

        match active_tool_kind {
            //TODO: Inventory
            Some(ToolKind::Dagger(_)) => {
                next.head.position =
                    Vec3::new(0.0, -2.0 + skeleton_attr.head.0, skeleton_attr.head.1);
                next.head.orientation = Quaternion::rotation_z(slow * -0.25)
                    * Quaternion::rotation_x(0.0 + slow * 0.15)
                    * Quaternion::rotation_y(slow * -0.15);
                next.head.scale = Vec3::one() * skeleton_attr.head_scale;

                next.chest.position = Vec3::new(0.0, skeleton_attr.chest.0, skeleton_attr.chest.1);
                next.chest.orientation = Quaternion::rotation_z(slow * 0.4)
                    * Quaternion::rotation_x(0.0 + slow * -0.2)
                    * Quaternion::rotation_y(slow * 0.2);
                next.chest.scale = Vec3::one();

                next.belt.position = Vec3::new(0.0, skeleton_attr.belt.0, skeleton_attr.belt.1);
                next.belt.orientation = next.chest.orientation * -0.3;

                next.shorts.position =
                    Vec3::new(0.0, skeleton_attr.shorts.0, skeleton_attr.shorts.1);
                next.shorts.orientation = next.chest.orientation * -0.45;

                // TODO: Fix animation
                next.l_hand.position = Vec3::new(0.0, 0.0, 0.0);
                next.l_hand.orientation = Quaternion::rotation_x(0.0);
                next.l_hand.scale = Vec3::one() * 1.12;

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_x(0.0);

                next.l_control.position = Vec3::new(-10.0 + push * 5.0, 6.0 + push * 5.0, 2.0);
                next.l_control.orientation = Quaternion::rotation_x(-1.4 + slow * 0.4)
                    * Quaternion::rotation_y(slow * -1.3)
                    * Quaternion::rotation_z(1.4 + slow * -0.5);
                next.l_control.scale = Vec3::one();

                next.r_hand.position = Vec3::new(0.0, 0.0, 0.0);
                next.r_hand.orientation = Quaternion::rotation_x(0.0);
                next.r_hand.scale = Vec3::one() * 1.12;

                next.second.position = Vec3::new(0.0, 0.0, 0.0);
                next.second.orientation = Quaternion::rotation_x(0.0);

                next.r_control.position = Vec3::new(8.0, 0.0, 0.0);
                next.r_control.orientation = Quaternion::rotation_x(0.0);
                next.r_control.scale = Vec3::one();

                // next.r_control.position = Vec3::new(-10.0 + push * 5.0, 6.0 + push * 5.0,
                // 2.0); next.r_control.orientation =
                // Quaternion::rotation_x(-1.4 + slow * 0.4)
                //     * Quaternion::rotation_y(slow * -1.3)
                //     * Quaternion::rotation_z(1.4 + slow * -0.5);
                // next.r_control.scale = Vec3::one();

                // next.r_hand.position = Vec3::new(0.75, -1.5, -5.5);
                // next.r_hand.orientation = Quaternion::rotation_x(1.27);
                // next.r_hand.scale = Vec3::one() * 1.05;

                // next.control.position = Vec3::new(-10.0 + push * 5.0, 6.0 + push * 5.0, 2.0);
                // next.control.orientation = Quaternion::rotation_x(-1.4 + slow * 0.4)
                //     * Quaternion::rotation_y(slow * -1.3)
                //     * Quaternion::rotation_z(1.4 + slow * -0.5);
                // next.control.scale = Vec3::one();

                next.l_foot.position = Vec3::new(
                    -skeleton_attr.foot.0,
                    slow * -3.0 + quick * 3.0 - 4.0,
                    skeleton_attr.foot.2,
                );
                next.l_foot.orientation = Quaternion::rotation_x(slow * 0.6)
                    * Quaternion::rotation_y((slow * -0.2).max(0.0));
                next.l_foot.scale = Vec3::one();

                next.r_foot.position = Vec3::new(
                    skeleton_attr.foot.0,
                    slow * 3.0 + quick * -3.0 + 5.0,
                    skeleton_attr.foot.2,
                );
                next.r_foot.orientation = Quaternion::rotation_x(slow * -0.6)
                    * Quaternion::rotation_y((slow * 0.2).min(0.0));
                next.r_foot.scale = Vec3::one();

                next.lantern.orientation =
                    Quaternion::rotation_x(slow * -0.7 + 0.4) * Quaternion::rotation_y(slow * 0.4);

                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.orientation = Quaternion::rotation_z(0.0)
                    * Quaternion::rotation_x(0.0)
                    * Quaternion::rotation_y(0.0);
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
            },
            Some(ToolKind::Axe(_)) => {
                next.head.position = Vec3::new(
                    0.0 + slowax * 2.0,
                    0.0 + skeleton_attr.head.0 + slowax * -2.0,
                    skeleton_attr.head.1,
                );
                next.head.orientation = Quaternion::rotation_z(slowax * 0.25)
                    * Quaternion::rotation_x(0.0 + slowax * 0.2)
                    * Quaternion::rotation_y(slowax * 0.2);
                next.head.scale = Vec3::one() * skeleton_attr.head_scale;

                next.chest.position = Vec3::new(0.0, 0.0, 7.0);
                next.chest.orientation = Quaternion::rotation_z(slowax * 0.2)
                    * Quaternion::rotation_x(0.0 + slowax * 0.2)
                    * Quaternion::rotation_y(slowax * 0.2);
                next.chest.scale = Vec3::one();

                next.belt.position = Vec3::new(0.0, 0.0, -2.0);
                next.belt.orientation = next.chest.orientation * -0.2;

                next.shorts.position = Vec3::new(0.0, 0.0, -5.0);
                next.shorts.orientation = next.chest.orientation * -0.15;

                next.l_hand.position = Vec3::new(-4.0, 3.0, 2.0);
                next.l_hand.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_z(3.14 - 0.3)
                    * Quaternion::rotation_y(-0.8);
                next.l_hand.scale = Vec3::one() * 1.08;
                next.r_hand.position = Vec3::new(-2.5, 9.0, 0.0);
                next.r_hand.orientation = Quaternion::rotation_x(-0.3)
                    * Quaternion::rotation_z(3.14 - 0.3)
                    * Quaternion::rotation_y(-0.8);
                next.r_hand.scale = Vec3::one() * 1.06;
                next.main.position = Vec3::new(-6.0, 10.0, -5.0);
                next.main.orientation = Quaternion::rotation_x(1.27)
                    * Quaternion::rotation_y(-0.3)
                    * Quaternion::rotation_z(-0.8);

                next.lantern.orientation = Quaternion::rotation_x(slowax * -0.7 + 0.4)
                    * Quaternion::rotation_y(slowax * 0.4);

                next.control.position = Vec3::new(0.0, 0.0 + slowax * 8.2, 6.0);
                next.control.orientation = Quaternion::rotation_x(0.8)
                    * Quaternion::rotation_y(-0.3)
                    * Quaternion::rotation_z(-0.7 + slowax * -1.9);
                next.control.scale = Vec3::one();
                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.orientation = Quaternion::rotation_z(0.0)
                    * Quaternion::rotation_x(0.0)
                    * Quaternion::rotation_y(0.0);
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
            },
            Some(ToolKind::Hammer(_)) => {
                next.l_hand.position = Vec3::new(-12.0, 0.0, 0.0);
                next.l_hand.orientation =
                    Quaternion::rotation_x(-0.0) * Quaternion::rotation_y(0.0);
                next.l_hand.scale = Vec3::one() * 1.08;
                next.r_hand.position = Vec3::new(3.0, 0.0, 0.0);
                next.r_hand.orientation = Quaternion::rotation_x(0.0) * Quaternion::rotation_y(0.0);
                next.r_hand.scale = Vec3::one() * 1.06;
                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_x(0.0)
                    * Quaternion::rotation_y(-1.57)
                    * Quaternion::rotation_z(1.57);

                next.head.position =
                    Vec3::new(0.0, -2.0 + skeleton_attr.head.0, skeleton_attr.head.1);
                next.head.orientation = Quaternion::rotation_z(slower * 0.03)
                    * Quaternion::rotation_x(slowersmooth * 0.1)
                    * Quaternion::rotation_y(slower * 0.05 + slowersmooth * 0.06)
                    * Quaternion::rotation_z((slowersmooth * -0.4).max(0.0));
                next.head.scale = Vec3::one() * skeleton_attr.head_scale;

                next.chest.position = Vec3::new(0.0, 0.0, 7.0);
                next.chest.orientation =
                    Quaternion::rotation_z(slower * 0.18 + slowersmooth * 0.15)
                        * Quaternion::rotation_x(0.0 + slower * 0.18 + slowersmooth * 0.15)
                        * Quaternion::rotation_y(slower * 0.18 + slowersmooth * 0.15);

                next.belt.position = Vec3::new(0.0, 0.0, -2.0);
                next.belt.orientation =
                    Quaternion::rotation_z(slower * -0.1 + slowersmooth * -0.075)
                        * Quaternion::rotation_x(0.0 + slower * -0.1)
                        * Quaternion::rotation_y(slower * -0.1);

                next.shorts.position = Vec3::new(0.0, 0.0, -5.0);
                next.shorts.orientation =
                    Quaternion::rotation_z(slower * -0.1 + slowersmooth * -0.075)
                        * Quaternion::rotation_x(0.0 + slower * -0.1)
                        * Quaternion::rotation_y(slower * -0.1);

                next.lantern.orientation = Quaternion::rotation_x(slower * -0.7 + 0.4)
                    * Quaternion::rotation_y(slower * 0.4);

                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.orientation = Quaternion::rotation_z(0.0);
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

                if velocity > 0.5 {
                    next.l_foot.position =
                        Vec3::new(-skeleton_attr.foot.0, foot * -6.0, skeleton_attr.foot.2);
                    next.l_foot.orientation = Quaternion::rotation_x(foot * -0.4)
                        * Quaternion::rotation_z((slower * 0.3).max(0.0));
                    next.l_foot.scale = Vec3::one();

                    next.r_foot.position =
                        Vec3::new(skeleton_attr.foot.0, foot * 6.0, skeleton_attr.foot.2);
                    next.r_foot.orientation = Quaternion::rotation_x(foot * 0.4)
                        * Quaternion::rotation_z((slower * 0.3).max(0.0));
                    next.r_foot.scale = Vec3::one();
                    next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                    next.torso.orientation =
                        Quaternion::rotation_z(0.0) * Quaternion::rotation_x(-0.15);
                    next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
                } else {
                    next.l_foot.position = Vec3::new(
                        -skeleton_attr.foot.0,
                        -2.5,
                        skeleton_attr.foot.2 + (slower * 2.5).max(0.0),
                    );
                    next.l_foot.orientation = Quaternion::rotation_x(slower * -0.2 - 0.2)
                        * Quaternion::rotation_z((slower * 1.0).max(0.0));
                    next.l_foot.scale = Vec3::one();

                    next.r_foot.position = Vec3::new(
                        skeleton_attr.foot.0,
                        3.5 - slower * 2.0,
                        skeleton_attr.foot.2,
                    );
                    next.r_foot.orientation = Quaternion::rotation_x(slower * 0.1)
                        * Quaternion::rotation_z((slower * 0.5).max(0.0));
                    next.r_foot.scale = Vec3::one();
                    next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                    next.torso.orientation = Quaternion::rotation_z(0.0);
                    next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
                }

                //next.control.position = Vec3::new(-4.0, 3.0 + slower * 2.0, 5.0 + slower *
                // 5.0); next.control.orientation = Quaternion::rotation_x()
                //    * Quaternion::rotation_y(0.0)
                //    * Quaternion::rotation_z(1.4);
                next.control.scale = Vec3::one();
                next.control.position = Vec3::new(-8.0, 7.0, 1.0);
                next.control.orientation = Quaternion::rotation_x(-1.5 + slower * 1.5)
                    * Quaternion::rotation_y(slowersmooth * 0.35 - 0.3)
                    * Quaternion::rotation_z(1.4 + slowersmooth * 0.2);
                next.control.scale = Vec3::one();

                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.orientation = Quaternion::rotation_z(0.0);
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
            },
            Some(ToolKind::Staff(_)) => {
                next.head.position = Vec3::new(
                    0.0,
                    0.0 + skeleton_attr.head.0, /* + decel * 0.8 */
                    // Had some clipping issues
                    skeleton_attr.head.1,
                );
                next.head.orientation = Quaternion::rotation_z(decel * 0.25)
                    * Quaternion::rotation_x(0.0 + decel * 0.1)
                    * Quaternion::rotation_y(decel * -0.1);

                next.chest.orientation = Quaternion::rotation_z(decel * -0.2)
                    * Quaternion::rotation_x(0.0 + decel * -0.2)
                    * Quaternion::rotation_y(decel * 0.2);

                next.belt.orientation = Quaternion::rotation_z(decel * -0.1)
                    * Quaternion::rotation_x(0.0 + decel * -0.1)
                    * Quaternion::rotation_y(decel * 0.1);

                next.shorts.position = Vec3::new(0.0, 0.0, -5.0);
                next.shorts.orientation = Quaternion::rotation_z(decel * -0.08)
                    * Quaternion::rotation_x(0.0 + decel * -0.08)
                    * Quaternion::rotation_y(decel * 0.08);
                next.l_hand.position = Vec3::new(0.0, 1.0, 0.0);
                next.l_hand.orientation = Quaternion::rotation_x(1.27);
                next.l_hand.scale = Vec3::one() * 1.05;
                next.r_hand.position = Vec3::new(0.0, 0.0, 10.0);
                next.r_hand.orientation = Quaternion::rotation_x(1.27);
                next.r_hand.scale = Vec3::one() * 1.05;
                next.main.position = Vec3::new(0.0, 6.0, -4.0);
                next.main.orientation = Quaternion::rotation_x(-0.3);

                next.control.position = Vec3::new(-8.0 - slow * 1.0, 3.0 - slow * 5.0, 0.0);
                next.control.orientation = Quaternion::rotation_x(-1.2)
                    * Quaternion::rotation_y(slow * 1.5)
                    * Quaternion::rotation_z(1.4 + slow * 0.5);
                next.control.scale = Vec3::one();
                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
            },
            Some(ToolKind::Shield(_)) => {
                next.head.position = Vec3::new(
                    0.0,
                    0.0 + skeleton_attr.head.0 + decel * 0.8,
                    skeleton_attr.head.1,
                );
                next.head.orientation = Quaternion::rotation_z(decel * 0.25)
                    * Quaternion::rotation_x(0.0 + decel * 0.1)
                    * Quaternion::rotation_y(decel * -0.1);
                next.head.scale = Vec3::one() * skeleton_attr.head_scale;

                next.chest.position = Vec3::new(0.0, 0.0, 7.0);
                next.chest.orientation = Quaternion::rotation_z(decel * -0.2)
                    * Quaternion::rotation_x(0.0 + decel * -0.2)
                    * Quaternion::rotation_y(decel * 0.2);

                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;

                next.belt.position = Vec3::new(0.0, 0.0, 0.0);
                next.belt.orientation = Quaternion::rotation_z(decel * -0.1)
                    * Quaternion::rotation_x(0.0 + decel * -0.1)
                    * Quaternion::rotation_y(decel * 0.1);

                next.shorts.position = Vec3::new(0.0, 0.0, 0.0);
                next.belt.orientation = Quaternion::rotation_z(decel * -0.08)
                    * Quaternion::rotation_x(0.0 + decel * -0.08)
                    * Quaternion::rotation_y(decel * 0.08);

                next.l_control.position =
                    Vec3::new(-8.0 + accel_slow * 10.0, 8.0 + accel_fast * 3.0, 0.0);
                next.l_control.orientation = Quaternion::rotation_z(-0.8)
                    * Quaternion::rotation_x(0.0 + accel_med * -0.8)
                    * Quaternion::rotation_y(0.0 + accel_med * -0.4);

                next.l_hand.position = Vec3::new(0.0, 0.0, 0.0);
                next.l_hand.orientation = Quaternion::rotation_x(0.0);
                next.l_hand.scale = Vec3::one() * 1.01;

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_z(0.0);

                next.r_control.position = Vec3::new(8.0, 0.0, 0.0);
                next.r_control.orientation = Quaternion::rotation_x(0.0);

                next.r_hand.position = Vec3::new(0.0, 0.0, 0.0);
                next.r_hand.orientation = Quaternion::rotation_x(0.0);
                next.r_hand.scale = Vec3::one() * 1.01;

                next.second.position = Vec3::new(0.0, 0.0, 0.0);
                next.second.orientation = Quaternion::rotation_x(0.0);
            },
            Some(ToolKind::Debug(_)) => {
                next.head.position = Vec3::new(
                    0.0,
                    -2.0 + skeleton_attr.head.0 + decel * 0.8,
                    skeleton_attr.head.1,
                );
                next.head.orientation = Quaternion::rotation_x(0.0);
                next.head.scale = Vec3::one() * skeleton_attr.head_scale;

                next.chest.position = Vec3::new(0.0, 0.0, 7.0);
                next.chest.orientation = Quaternion::rotation_z(decel * -0.2)
                    * Quaternion::rotation_x(0.0 + decel * -0.2)
                    * Quaternion::rotation_y(decel * 0.2);

                next.l_hand.position =
                    Vec3::new(-8.0 + accel_slow * 10.0, 8.0 + accel_fast * 3.0, 0.0);
                next.l_hand.orientation = Quaternion::rotation_z(-0.8)
                    * Quaternion::rotation_x(accel_med * -0.8)
                    * Quaternion::rotation_y(accel_med * -0.4);
                next.l_hand.scale = Vec3::one() * 1.01;

                next.r_hand.position =
                    Vec3::new(-8.0 + accel_slow * 10.0, 8.0 + accel_fast * 3.0, -2.0);
                next.r_hand.orientation = Quaternion::rotation_z(-0.8)
                    * Quaternion::rotation_x(accel_med * -0.8)
                    * Quaternion::rotation_y(accel_med * -0.4);
                next.r_hand.scale = Vec3::one() * 1.01;

                next.main.position =
                    Vec3::new(-8.0 + accel_slow * 10.0, 8.0 + accel_fast * 3.0, 0.0);
                next.main.orientation = Quaternion::rotation_z(-0.8)
                    * Quaternion::rotation_x(0.0 + accel_med * -0.8)
                    * Quaternion::rotation_y(0.0 + accel_med * -0.4);
                next.main.scale = Vec3::one();
                next.torso.position = Vec3::new(0.0, 0.0, 0.1) * skeleton_attr.scaler;
                next.torso.orientation = Quaternion::rotation_x(0.0);
                next.torso.scale = Vec3::one() / 11.0 * skeleton_attr.scaler;
            },
            _ => {},
        }
        next.lantern.position = Vec3::new(
            skeleton_attr.lantern.0,
            skeleton_attr.lantern.1,
            skeleton_attr.lantern.2,
        );
        next.lantern.scale = Vec3::one() * 0.65;
        next.l_shoulder.scale = Vec3::one() * 1.1;
        next.r_shoulder.scale = Vec3::one() * 1.1;
        next.glider.position = Vec3::new(0.0, 0.0, 10.0);
        next.glider.scale = Vec3::one() * 0.0;
        next.l_control.scale = Vec3::one();
        next.r_control.scale = Vec3::one();

        next.second.scale = match (
            active_tool_kind.map(|tk| tk.hands()),
            second_tool_kind.map(|tk| tk.hands()),
        ) {
            (Some(Hands::OneHand), Some(Hands::OneHand)) => Vec3::one(),
            (_, _) => Vec3::zero(),
        };

        next
    }
}

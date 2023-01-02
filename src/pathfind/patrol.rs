use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Patrol {
    pub can_patrol: bool,
    pub patrol_timer: Timer,
    pub patrol_pause_timer: Timer,
    pub target: Vec2,

    pub lost_target: bool,
    pub lose_notice_timer: Timer
}

impl Default for Patrol {
    fn default() -> Self {
        Self {
            can_patrol: true,
            patrol_timer: Timer::from_seconds(12.0, TimerMode::Once),
            patrol_pause_timer: Timer::from_seconds(3.0, TimerMode::Once),
            target: Vec2::ZERO,

            lost_target: false,
            lose_notice_timer: Timer::from_seconds(1.0, TimerMode::Once)
        }
    }
}

impl Patrol {
    pub fn lose_target(&mut self) {
        self.lost_target = true;
        self.lose_notice_timer.reset();
    }

    pub fn patrol<
        PickTarget: FnMut(&mut Patrol),
        Move: FnMut(&mut Patrol),
        NoTarget: FnMut(&mut Patrol)
    >(
        &mut self,
        mut pick_target_fn: PickTarget,
        mut move_fn: Move,
        mut no_target_fn: NoTarget
    ) {
        if self.lose_notice_timer.just_finished() || self.patrol_pause_timer.just_finished() {
            pick_target_fn(self);
        } else if self.patrol_pause_timer.finished() && self.lose_notice_timer.finished() {
            move_fn(self);
        } else {
            no_target_fn(self);
        }
    }
}
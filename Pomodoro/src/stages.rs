/*
Author: lvxw

      :::    :::     ::: :::    ::: :::       :::
     :+:    :+:     :+: :+:    :+: :+:       :+:
    +:+    +:+     +:+  +:+  +:+  +:+       +:+
   +#+    +#+     +:+   +#++:+   +#+  +:+  +#+
  +#+     +#+   +#+   +#+  +#+  +#+ +#+#+ +#+
 #+#      #+#+#+#   #+#    #+#  #+#+# #+#+#
########## ###     ###    ###   ###   ###

WARNING: This program has been written by lvxw (https://github.com/lvzrr) and any and all unauthoraised plagiarism is strictly prohibited
*/
use chrono::{DateTime, Local, Timelike};
use std::time;

use notify_rust::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Stage {
    Study,
    Break,
}

pub struct Pomodoro {
    pub currentphase: Stage,
    pub stage_remtime: time::Duration,
    pub phasetime: time::Instant,
    pub elapsed: time::Duration,
    pub currentdate: DateTime<Local>,
    pub cycles: u8,
}

impl Stage {
    pub fn disp(&self) -> &str {
        match *self {
            Self::Study => "Study",
            Self::Break => "Break",
        }
    }
}

pub fn notify(mode: &str) {
    let text: &str = &format!("Wake up! It's time to {}", mode);
    let _noti = Notification::new()
        .appname("Pomodoro")
        .summary(text)
        .auto_icon()
        .timeout(Timeout::Milliseconds(6000))
        .sound_name("Default")
        .show()
        .unwrap();
}

pub fn format_duration(elapsed: time::Duration, stagetime: time::Duration) -> String {
    // Total time difference in seconds
    let remaining = stagetime.as_secs() as i64 - elapsed.as_secs() as i64;

    // Ensure time doesn't go negative
    let remaining = if remaining > 0 { remaining } else { 0 };

    // Calculate minutes and seconds
    let minutes = remaining / 60;
    let seconds = remaining % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

impl Pomodoro {
    pub fn run(&mut self) {
        // Update stage remaining time based on current phase
        self.stage_remtime = match self.currentphase {
            Stage::Study => time::Duration::from_secs(1200),
            Stage::Break => time::Duration::from_secs(300),
        };

        self.currentdate = Local::now().with_nanosecond(0).unwrap();

        // Calculate elapsed time since the last phase started
        self.elapsed = self.phasetime.elapsed();

        if self.elapsed >= self.stage_remtime {
            // Switch phases and update the state
            match self.currentphase {
                Stage::Study => {
                    self.phasetime = time::Instant::now();
                    self.currentphase = Stage::Break;
                    self.cycles += 1;
                    notify(self.currentphase.disp());
                }
                Stage::Break => {
                    self.phasetime = time::Instant::now();
                    self.currentphase = Stage::Study;
                    notify(self.currentphase.disp());
                }
            }
        }
    }
}

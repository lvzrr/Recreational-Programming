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
use chrono::{Local, Timelike};
use crossterm::{cursor, terminal, terminal::ClearType, ExecutableCommand};
use std::io::stdout;
use std::time;
mod stages;
use stages::*;

fn main() {
    let _local = Local::now();
    let mut pom = Pomodoro {
        elapsed: time::Duration::new(0, 0),
        currentphase: Stage::Study,
        stage_remtime: time::Duration::from_secs(0),
        phasetime: time::Instant::now(),
        currentdate: Local::now().with_nanosecond(0).unwrap(),
        cycles: 1,
    };

    // Clear the terminal

    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(ClearType::All))
        .expect("Failed to clear terminal");

    let (mut prewidth, mut preheight) = terminal::size().expect("Failed to get terminal size");
    let mut prephase = pom.currentphase;
    loop {
        pom.run();

        // Get terminal size
        let (width, height) = terminal::size().expect("Failed to get terminal size");

        if (width != prewidth) || (height != preheight) || (prephase != pom.currentphase) {
            stdout
                .execute(terminal::Clear(ClearType::All))
                .expect("Failed to clear terminal");

            prewidth = width;
            preheight = height;
            prephase = pom.currentphase;
        }

        // Emoji positioning
        let emoji = match pom.currentphase {
            Stage::Study => "(｡>﹏<)",
            Stage::Break => "(｡˃ ᵕ ˂)",
        };
        let emoji_x = width / 2 - emoji.len() as u16 / 2;
        let emoji_y = height / 2;

        // Print the emoji in the center
        stdout
            .execute(cursor::MoveTo(emoji_x, emoji_y))
            .expect("Failed to move cursor to emoji");
        print!("{}", emoji);

        let text = "「https://github.com/lvzrr/Pomodoro 」";
        let text_x = width / 2 - text.len() as u16 / 2;

        stdout
            .execute(cursor::MoveTo(text_x, 3))
            .expect("Failed to move cursor to emoji");
        print!("{}", text);

        stdout
            .execute(cursor::MoveTo(0, 3))
            .expect("Failed to move cursor to bottom");
        print!("ᓚ₍ ^. ̫ .^₎");

        let emoji_4x = width / 4;
        let emoji_4y = height - 3;

        if width > 200 {
            stdout
                .execute(cursor::MoveTo(1, emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| {} |", pom.currentdate);

            stdout
                .execute(cursor::MoveTo(emoji_4x + 10, emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| Phase : {} |", pom.currentphase.disp());

            stdout
                .execute(cursor::MoveTo(emoji_4x * 2, emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!(
                "| Time remaining to {} phase: {} |",
                match pom.currentphase {
                    Stage::Study => "break",
                    Stage::Break => "study",
                },
                format_duration(pom.elapsed, pom.stage_remtime)
            );

            stdout
                .execute(cursor::MoveTo((emoji_4x * 3) + 20, emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| Pomodoros: {} |", pom.cycles);

            stdout
                .execute(cursor::MoveTo(emoji_4x * 8, emoji_4y + 14))
                .expect("Failed to move cursor to bottom");
            print!("");
        } else {
            stdout
                .execute(cursor::MoveTo(1, emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| {} |", pom.currentdate);

            stdout
                .execute(cursor::MoveTo(emoji_4x + (width / 4), emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| {} |", pom.currentphase.disp());

            stdout
                .execute(cursor::MoveTo((emoji_4x * 2) + (width / 5), emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| {} |", format_duration(pom.elapsed, pom.stage_remtime));

            stdout
                .execute(cursor::MoveTo((emoji_4x * 3) + (width / 6), emoji_4y))
                .expect("Failed to move cursor to bottom");
            print!("| {} |", pom.cycles);

            stdout
                .execute(cursor::MoveTo(width, height))
                .expect("Failed to move cursor to bottom");
            print!("");
        }
        std::thread::sleep(time::Duration::from_millis(100));
    }
}

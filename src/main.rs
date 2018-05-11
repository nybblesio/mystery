extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;

fn main() {
    initscr();
    //cbreak();
    start_color();
    raw();
    keypad(stdscr(), true);
    noecho();

    let mut x = 0;
    let mut y = 0;

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let title = format!(" Mystery | Dark & Grimy Motel Room");
    let title_bar = format!(
        "{:1$} Score: {2:06} | Moves: {3:03} ",
        title,
        (max_x as usize - (title.len() - 6)),
        0,
        0);
    let title_win = newwin(1, max_x, 0, 0);
    leaveok(title_win, true);
    wmove(title_win, 0, 0);
    wattron(title_win, A_BOLD() | A_REVERSE());
    wprintw(title_win, &title_bar);
    wattroff(title_win, A_BOLD() | A_REVERSE());
    wrefresh(title_win);

    let story_win = newwin(max_y - 1, max_x, 1, 0);
    scrollok(story_win, true);
    wmove(story_win, 0, 0);

    let mut escape_seq = String::new();

    loop {
        let ch = wgetch(story_win);
        getyx(story_win, &mut y, &mut x);
        if ch == 'q' as i32 {
            break;
        } else if ch == '\n' as i32 {
            wprintw(story_win, "\n\n");
            let text = "You slowly awake. Your head really hurts.  The room slowly comes into focus. It's disgusting.\n";
            wprintw(
                story_win,
                &fill(text, max_x as usize));
            wprintw(story_win, "\n> ");
        } else if ch == 127 {
            if x > 2 {
                mvwdelch(story_win, y, x - 1);
            }
        } else if ch == '~' as i32 {
            wdelch(story_win);
        } else {
            if ch == 27 {
                for _ in 0..2 {
                    let ch = wgetch(story_win);
                    escape_seq.push((ch as u8) as char);
                }
            }

            if escape_seq.len() > 0 {
                if escape_seq == "[C" {
                    if x < 64 {
                        wmove(story_win, y, x + 1);
                    }
                } else if escape_seq == "[D" {
                    if x > 2 {
                        wmove(story_win, y, x - 1);
                    }
                }
                escape_seq.clear();
            } else {
                if x < 64 {
                    waddch(story_win, ch as u32);
                }
            }
        }
        wrefresh(story_win);
    }

    endwin();
}

extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;
use std::collections::HashMap;

struct HeaderWindow {
    game_title:String,
    room_name:String,
    score:u32,
    moves:u32,
    max_x:i32,
    max_y:i32,
    win:WINDOW
}

impl HeaderWindow {
    fn new(game_title:String) -> HeaderWindow {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        HeaderWindow {
            game_title,
            room_name: String::from("The Void"),
            score: 0,
            moves: 0,
            max_x,
            max_y,
            win: newwin(1, max_x, 0, 0)
        }
    }

    fn room_name(&mut self, name:&str) {
        self.room_name = String::from(name);
        self.update();
    }

    fn increment_moves(&mut self) {
        self.moves += 1;
    }

    fn update(&mut self) {
        let title = format!(" {} | {}", self.game_title, self.room_name);
        let title_bar = format!(
            "{:1$} Score: {2:06} | Moves: {3:03} ",
            title,
            (self.max_x as usize - (title.len() - 6)),
            self.score,
            self.moves);
        leaveok(self.win, true);
        wmove(self.win, 0, 0);
        wattron(self.win, A_BOLD() | A_REVERSE());
        wprintw(self.win, &title_bar);
        wattroff(self.win, A_BOLD() | A_REVERSE());
        wrefresh(self.win);
    }
}

struct StoryWindow {
    max_x:i32,
    max_y:i32,
    win:WINDOW,
    input_seq:String,
    escape_seq:String,
    room:RoomIdentifier,
    header_window:HeaderWindow,
    rooms:HashMap<RoomIdentifier, Room>,
}

impl StoryWindow {
    fn new(header_window:HeaderWindow) -> StoryWindow {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let story_window = StoryWindow {
            max_x,
            max_y,
            header_window,
            rooms: HashMap::new(),
            input_seq: String::new(),
            escape_seq: String::new(),
            room: RoomIdentifier::None,
            win: newwin(max_y - 1, max_x, 1, 0)
        };
        scrollok(story_window.win, true);
        wmove(story_window.win, 0, 0);
        story_window
    }

    fn define_rooms(&mut self) {
        self.rooms.insert(
            RoomIdentifier::None,
            Room::new(
                RoomIdentifier::None,
                String::from("The Void"),
                String::from("I'm in the void. So this is the end, huh?"),
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
            ));

        self.rooms.insert(
            RoomIdentifier::GrimyHotelRoom,
            Room::new(
                RoomIdentifier::GrimyHotelRoom,
                String::from("Grimy & Dark Hotel Room"),
                String::from("I'm in a dark, seedy, and extremely dirty hotel room.  My head is killing me."),
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::HotelHallway,
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
            ));

        self.rooms.insert(
            RoomIdentifier::HotelHallway,
            Room::new(
                RoomIdentifier::HotelHallway,
                String::from("Grimy & Dark Hotel's Hallway"),
                String::from("If it's possible, this hallway is even darker and more disgusting than that hotel room. Who am I?"),
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::None,
                RoomIdentifier::GrimyHotelRoom,
                RoomIdentifier::None,
                RoomIdentifier::None,
            ));
    }

    fn room(&mut self, room_id:RoomIdentifier) {
        self.room = room_id;
    }

    fn look_at_room(&mut self) {
        let get_room_result = self.rooms.get(&self.room);
        let room = match get_room_result {
            None => self.rooms.get(&RoomIdentifier::None).unwrap(),
            Some(room) => room
        };

        self.header_window.room_name(room.name.as_str());

        let mut description_seq = String::new();
        description_seq.push_str(room.description.as_str());
        description_seq.push('\n');

        wprintw(
            self.win,
            &fill(description_seq.as_str(), self.max_x as usize));
    }

    fn prompt(&mut self) {
        wprintw(self.win, "\n> ");
        self.input_seq.clear();
    }

    fn move_west(&mut self) -> RoomIdentifier {
        let get_room_result = self.rooms.get(&self.room);
        let room = match get_room_result {
            None => {
                return RoomIdentifier::None;
            },
            Some(room) => room
        };
        room.west
    }

    fn move_east(&mut self) -> RoomIdentifier {
        let get_room_result = self.rooms.get(&self.room);
        let room = match get_room_result {
            None => {
                return RoomIdentifier::None;
            },
            Some(room) => room
        };
        room.east
    }

    fn run(&mut self) {
        let mut x = 0;
        let mut y = 0;

        self.header_window.update();
        self.look_at_room();
        self.prompt();

        loop {
            let ch = wgetch(self.win);
            getyx(self.win, &mut y, &mut x);
            if ch == '\n' as i32 {
                let mut force_look = false;

                wprintw(self.win, "\n\n");
                let result = match self.input_seq.as_ref() {
                    "quit" => {
                        break;
                    }

                    "look" => {
                        self.look_at_room();
                        None
                    }

                    "west" => {
                        let new_room_id = self.move_west();
                        if new_room_id == RoomIdentifier::None {
                            Some("I don't appear to be able to go in that direction.\n")
                        } else {
                            force_look = true;
                            self.room = new_room_id;
                            self.header_window.increment_moves();
                            Some("OK, heading west.\n")
                        }
                    }

                    "east" => {
                        let new_room_id = self.move_east();
                        if new_room_id == RoomIdentifier::None {
                            Some("I don't appear to be able to go in that direction.\n")
                        } else {
                            force_look = true;
                            self.room = new_room_id;
                            self.header_window.increment_moves();
                            Some("OK, heading east.\n")
                        }
                    }

                    "up" => {
                        Some("I don't appear to be able to go in that direction.\n")
                    }

                    "down" => {
                        Some("I don't appear to be able to go in that direction.\n")
                    }

                    "north" => {
                        Some("I don't appear to be able to go in that direction.\n")
                    }

                    "south" => {
                        Some("I don't appear to be able to go in that direction.\n")
                    }

                    _ => {
                        Some("I don't understand you, friend.\n")
                    }
                };

                match result {
                    None => {},
                    Some(message) => {
                        wprintw(self.win, message);
                    }
                };

                if force_look {
                    self.look_at_room();
                }

                self.prompt();
            } else if ch == 127 {
                if x > 2 {
                    mvwdelch(self.win, y, x - 1);
                }
            } else if ch == '~' as i32 {
                wdelch(self.win);
            } else {
                if ch == 27 {
                    for _ in 0..2 {
                        let ch = wgetch(self.win);
                        self.escape_seq.push((ch as u8) as char);
                    }
                }

                if self.escape_seq.len() > 0 {
                    if self.escape_seq == "[C" {
                        if x < 64 {
                            wmove(self.win, y, x + 1);
                        }
                    } else if self.escape_seq == "[D" {
                        if x > 2 {
                            wmove(self.win, y, x - 1);
                        }
                    }
                    self.escape_seq.clear();
                } else {
                    if x < 64 {
                        waddch(self.win, ch as u32);
                        self.input_seq.push((ch as u8) as char);
                    }
                }
            }
            wrefresh(self.win);
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum RoomIdentifier {
    None,
    GrimyHotelRoom,
    HotelHallway,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Room {
    id:RoomIdentifier,
    name:String,
    description:String,
    up:RoomIdentifier,
    down:RoomIdentifier,
    west:RoomIdentifier,
    east:RoomIdentifier,
    north:RoomIdentifier,
    south:RoomIdentifier,
}

impl Room {
    fn new(
            id:RoomIdentifier,
            name:String,
            description:String,
            up:RoomIdentifier,
            down:RoomIdentifier,
            west:RoomIdentifier,
            east:RoomIdentifier,
            north:RoomIdentifier,
            south:RoomIdentifier) -> Room {
        Room {
            id,
            name,
            description,
            up,
            down,
            west,
            east,
            north,
            south
        }
    }
}

fn main() {
    initscr();
    start_color();
    raw();
    keypad(stdscr(), true);
    noecho();

    let header_window:HeaderWindow = HeaderWindow::new(
        String::from("Mystery"));

    let mut story_window:StoryWindow = StoryWindow::new(header_window);
    story_window.define_rooms();
    story_window.room(RoomIdentifier::GrimyHotelRoom);
    story_window.run();

    endwin();
}

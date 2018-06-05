extern crate ncurses;
extern crate textwrap;

use ncurses::*;
use textwrap::fill;
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum ItemIdentifier {
    Flashlight
}

struct Item {
    id: ItemIdentifier,
    description: String,
}

impl Item {
    fn new(id: ItemIdentifier, description: String) -> Item {
        Item {
            id,
            description
        }
    }
}

struct Player {
    moves: u32,
    score: u32,
    location: RoomIdentifier,
    items: Vec<ItemIdentifier>,
}

impl Player {
    fn new() -> Player {
        Player {
            score: 0,
            moves: 0,
            items: Vec::new(),
            location: RoomIdentifier::None,
        }
    }

    fn made_move(&mut self) {
        self.moves += 1;
    }

    fn earned_points(&mut self, points: u32) {
        self.score += points;
    }
}

struct InteractiveStory {
    max_x: i32,
    max_y: i32,
    player: Player,
    input_seq: String,
    game_title: String,
    story_win: WINDOW,
    header_win: WINDOW,
    escape_seq: String,
    items: HashMap<ItemIdentifier, Item>,
    rooms: HashMap<RoomIdentifier, Room>,
}

impl InteractiveStory {
    fn new() -> InteractiveStory {
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        let story = InteractiveStory {
            max_x,
            max_y,
            player: Player::new(),
            items: HashMap::new(),
            rooms: HashMap::new(),
            input_seq: String::new(),
            escape_seq: String::new(),
            header_win: newwin(1, max_x, 0, 0),
            game_title: String::from("Mystery"),
            story_win: newwin(max_y - 1, max_x, 1, 0)
        };
        scrollok(story.story_win, true);
        wmove(story.story_win, 0, 0);
        story
    }

    fn initialize(&mut self) {
        self.define_items();
        self.define_rooms();
        self.player.location = RoomIdentifier::GrimyHotelRoom;
    }

    fn define_items(&mut self) {
        self.items.insert(
            ItemIdentifier::Flashlight,
            Item::new(
                ItemIdentifier::Flashlight,
                String::from("A sleek and modern flashlight")
            )
        );
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

    fn look_at_room(&mut self) {
        self.update_header();

        let get_room_result = self.rooms.get(&self.player.location);
        let room = match get_room_result {
            None => self.rooms.get(&RoomIdentifier::None).unwrap(),
            Some(room) => room
        };

        let mut description_seq = String::new();
        description_seq.push_str(room.description.as_str());
        description_seq.push('\n');

        wprintw(
            self.story_win,
            &fill(description_seq.as_str(), self.max_x as usize));
    }

    fn prompt(&mut self) {
        wprintw(self.story_win, "\n> ");
        self.input_seq.clear();
    }

    fn move_west(&mut self) -> RoomIdentifier {
        let get_room_result = self.rooms.get(&self.player.location);
        let room = match get_room_result {
            None => {
                return RoomIdentifier::None;
            },
            Some(room) => room
        };
        room.west
    }

    fn move_east(&mut self) -> RoomIdentifier {
        let get_room_result = self.rooms.get(&self.player.location);
        let room = match get_room_result {
            None => {
                return RoomIdentifier::None;
            },
            Some(room) => room
        };
        room.east
    }

    fn update_header(&mut self) {
        let get_room_result = self.rooms.get(&self.player.location);
        let room = match get_room_result {
            None => self.rooms.get(&RoomIdentifier::None).unwrap(),
            Some(room) => room
        };

        let title = format!(" {} | {}", self.game_title, room.name);
        let title_bar = format!(
            "{:1$} Score: {2:06} | Moves: {3:03} ",
            title,
            (self.max_x as usize - (title.len() - 6)),
            self.player.score,
            self.player.moves);
        leaveok(self.header_win, true);
        wmove(self.header_win, 0, 0);
        wattron(self.header_win, A_BOLD() | A_REVERSE());
        wprintw(self.header_win, &title_bar);
        wattroff(self.header_win, A_BOLD() | A_REVERSE());
        wrefresh(self.header_win);
    }

    fn run(&mut self) {
        let mut x = 0;
        let mut y = 0;

        self.look_at_room();
        self.prompt();

        loop {
            let ch = wgetch(self.story_win);
            getyx(self.story_win, &mut y, &mut x);
            if ch == '\n' as i32 {
                let mut force_look = false;

                wprintw(self.story_win, "\n\n");
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
                            self.player.location = new_room_id;
                            self.player.made_move();
                            Some("OK, heading west.\n")
                        }
                    }

                    "east" => {
                        let new_room_id = self.move_east();
                        if new_room_id == RoomIdentifier::None {
                            Some("I don't appear to be able to go in that direction.\n")
                        } else {
                            force_look = true;
                            self.player.location = new_room_id;
                            self.player.made_move();
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
                        wprintw(self.story_win, message);
                    }
                };

                if force_look {
                    self.look_at_room();
                }

                self.prompt();
            } else if ch == 127 {
                if x > 2 {
                    mvwdelch(self.story_win, y, x - 1);
                }
            } else if ch == '~' as i32 {
                wdelch(self.story_win);
            } else {
                if ch == 27 {
                    for _ in 0..2 {
                        let ch = wgetch(self.story_win);
                        self.escape_seq.push((ch as u8) as char);
                    }
                }

                if self.escape_seq.len() > 0 {
                    if self.escape_seq == "[C" {
                        if x < 64 {
                            wmove(self.story_win, y, x + 1);
                        }
                    } else if self.escape_seq == "[D" {
                        if x > 2 {
                            wmove(self.story_win, y, x - 1);
                        }
                    }
                    self.escape_seq.clear();
                } else {
                    if x < 64 {
                        waddch(self.story_win, ch as u32);
                        self.input_seq.push((ch as u8) as char);
                    }
                }
            }
            wrefresh(self.story_win);
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
    id: RoomIdentifier,
    name: String,
    description: String,
    up: RoomIdentifier,
    down: RoomIdentifier,
    west: RoomIdentifier,
    east: RoomIdentifier,
    north: RoomIdentifier,
    south: RoomIdentifier,
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

    let mut story: InteractiveStory = InteractiveStory::new();
    story.initialize();
    story.run();

    endwin();
}

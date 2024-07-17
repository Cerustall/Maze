use crossterm::{
    terminal::{size, enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{poll, read, Event, KeyCode, KeyEventKind},
    cursor::{MoveTo, Hide, Show},
    style::{SetBackgroundColor, Color, ResetColor},
    execute,
};
use std::{
    io::{stdout, Result},
    thread,
    time
};

//Tile options
const WALL: char = '#';
const BLANK: char = ' ';
const PLAYER: char = '@';

//Fullscreen terminal dimensions
const SIZE_X: usize = 260;
const SIZE_Y: usize = 39;

//Player
#[derive(Clone)]
struct Player{
    pos_x: u32,
    pos_y: u32
}

//Options for tiles to be displayed
#[derive(Clone)]
enum TileType {
    Wall,
    Blank,
    Player
}

//Actual tiles stored in array
#[derive(Clone)]
struct Tile{
    ty: TileType,
    visited: bool
}

fn draw_screen(map: Vec<Tile>){
    let length = map.len();
    let size = size().unwrap();

    for y in 0..(size.1.into()){
        match map[y].ty{
            TileType::Wall => print!("{} ", SetBackgroundColor(Color::White)),
            TileType::Blank => print!("{} ", ResetColor),
            TileType::Player => print!("{} ", SetBackgroundColor(Color::Red))
        }
    }
}

fn main() {
    enable_raw_mode();
    execute!(stdout(), EnterAlternateScreen);

    let player = Player{
        pos_x: 1,
        pos_y: 1,
    };
    
    //Get current terminal dimensions
    let size = size().unwrap();

    for n in 0..size.0.into(){
        print!("{} ", SetBackgroundColor(Color::White));
    }

    /*
    let mut map: Vec<Tile> = vec![Tile{ty: TileType::Wall, visited: false}; ((size.0)*(size.1)) as usize];

    draw_screen(map);

    */
    let ten_millis = time::Duration::from_millis(5000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    execute!(stdout(), LeaveAlternateScreen);
    print!("{}", Show);
    disable_raw_mode();
}
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

type Grid = Vec<Vec<Tile>>;

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
    visited: bool,
    color: Color
}

fn set_screen(mut map: Grid) -> Grid{
    for y in 0..map.len(){
        for x in 0..y{
            map[x][y].ty = TileType::Wall;
            map[x][y].visited = false;
        }
    }
    return map;
}

fn draw_screen(map: Grid){
    for y in map{
        for x in y{
            match x.ty{
                TileType::Wall => print!("{} ", SetBackgroundColor(Color::White)),
                TileType::Blank => print!("{} ", ResetColor),
                TileType::Player => print!("{} ", SetBackgroundColor(Color::Red))
            }
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

    let mut map: Grid = vec![vec![Tile{ty: TileType::Wall, visited: false, color: Color::White}; size.0.into()]; size.1.into()];

    map = set_screen(map);
    draw_screen(map);

    let ten_millis = time::Duration::from_millis(5000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    execute!(stdout(), LeaveAlternateScreen);
    print!("{}", Show);
    disable_raw_mode();
}
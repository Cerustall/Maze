use crossterm::{
    cursor::{Hide, Show, MoveTo}, execute, style::{Color, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, ClearType, EnterAlternateScreen, LeaveAlternateScreen,Clear,},
    event::{poll, read, Event, KeyCode, KeyEventKind}
};
use std::{
    io::stdout
};
use rand::prelude::*;

type Grid = Vec<Vec<Tile>>;

const FRAME_DELAY: u64 = 17;

#[derive(Clone,PartialEq)]
enum TileType{
    Wall,
    Passage,
}

#[derive(Clone)]
struct Tile{
    ty: TileType,
    occupied: bool,
    visited: bool
}

#[derive(Clone)]
struct Player{
    pos_x: usize,
    pos_y: usize
}

fn set_screen(map: &mut Grid, size: (u16, u16)) -> Grid{
    let mut top_or_bottom: bool;
    let mut either_side: bool;
    let mut x_counter: u16 = 0;
    let mut y_counter: u16 = 0;
    
    for y in &mut *map{
        for x in y{
            top_or_bottom = false;
            either_side = false;

            if (x_counter == 0) || (x_counter == size.0-1){
                top_or_bottom = true;
            }
            if (y_counter == 0) || (y_counter == size.1-1){
                either_side = true;
            }
            
            if either_side{
                x.ty = TileType::Wall;
            }else if top_or_bottom{
                x.ty = TileType::Wall;
            }else if (x_counter % 2 == 0) || (y_counter % 2 == 0) {
                x.ty = TileType::Wall;
            }else{
                x.ty = TileType::Passage;
            }

            x_counter += 1;
        }
        x_counter = 0;
        y_counter += 1;
    }
    return map.to_vec();
}

fn gen_maze(map: &mut Grid, size: (u16, u16)) -> Grid{
    let mut top_or_bottom: bool;
    let mut either_side: bool;
    let mut x_counter: u16 = 0;
    let mut y_counter: u16 = 0;
    let mut direction: bool;
    //true = right, false = down
    let mut rng = rand::thread_rng();
    let mut decider: u16 = 2;

    for y in 0..size.0-2{
        for x in 0..size.1-2{
            top_or_bottom = false;
            either_side = false;
            direction = true;
            decider = rng.gen_range(0..2);

            if (x == 0) || (x == size.0-1){
                top_or_bottom = true;
            }
            if (y == 0) || (y == size.1-1){
                either_side = true;
            }
            if decider == 0 {
                direction = false
            }else{
                direction = true;
            }

            //map[x as usize][y as usize].ty == TileType::Passage <- to go in brackets of below if statement if fix for generation doesnt work

            if !top_or_bottom && !either_side && (y%2 != 0){
                match direction{
                    true => map[(x+1) as usize][y as usize].ty = TileType::Passage,
                    false => map[x as usize][(y_counter + 1) as usize].ty = TileType::Passage
                }
            }
            x_counter += 1;
        }
        x_counter = 0;
        y_counter += 1;
    }
    map.to_vec()
}

fn draw_screen(map: &Grid){
    for y in map{
        for x in y{
            match x.ty{
                TileType::Passage => match x.occupied{
                    true => print!("{} ", SetBackgroundColor(Color::Red)),
                    false => print!("{} ", ResetColor)
                }
                TileType::Wall => print!("{} ", SetBackgroundColor(Color::White))
            }
        }
    }
}

fn main(){
    //Raw mode and alternate screen
    enable_raw_mode();
        execute!(stdout(), EnterAlternateScreen);
        print!("{}", Hide);
     
    //Player starting position top left
    let mut player: Player = Player{
        pos_x: 1,
        pos_y: 1,
    };
    

    //Get terminal dimensions
    let size: (u16, u16) = size().unwrap();

    //Game 'map,' 2D vector with dimensions equal to terminal window game is run in
    let mut map: Grid = vec![vec![Tile{ty: TileType::Passage, occupied: false, visited: false}; size.0.into()]; size.1.into()];
    map[player.pos_y][player.pos_x].occupied = true;
    
    set_screen(&mut map, size);
    gen_maze(&mut map, size);
    
    draw_screen(&map);
    loop{
        print!("{}{}", MoveTo(0,0), Hide);
        if poll(std::time::Duration::from_millis(FRAME_DELAY)).expect("REASON") { 
            if let Ok(Event::Key(key)) = read() {
                if key.kind == KeyEventKind::Press
                    && (
                        key.code == KeyCode::Char('q')
                        || key.code == KeyCode::Char('c')
                        || key.code == KeyCode::Esc
                        )
                {
                    break;
                } 
                
                if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('s') 
                && (map[player.pos_y+1][player.pos_x].ty != TileType::Wall)
                {
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_y += 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('d') 
                && (map[player.pos_y][player.pos_x+1].ty != TileType::Wall)
                {
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_x += 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('w') 
                && (map[player.pos_y-1][player.pos_x].ty != TileType::Wall)
                {
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_y -= 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('a') 
                && (map[player.pos_y][player.pos_x-1].ty != TileType::Wall)
                {
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_x -= 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                }
            }
        }
    }
    execute!(stdout(), LeaveAlternateScreen);
    print!("{}", Show);
    disable_raw_mode();
}
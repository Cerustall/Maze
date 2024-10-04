use crossterm::{
    cursor::{Hide, Show, MoveTo}, execute, style::{Color, ResetColor, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, ClearType::Purge, EnterAlternateScreen, LeaveAlternateScreen,Clear,},
    event::{poll, read, Event, KeyCode, KeyEventKind},
    ExecutableCommand
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
    Win,
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
    *map = set_win_area(map, size);
    map.to_vec()
}

fn set_win_area(map: &mut Grid, size: (u16, u16)) -> Grid{
    map[((size.1 - 2 ) as usize)][((size.0 - 2) as usize)].ty = TileType::Win;
    map[((size.1 - 3 ) as usize)][((size.0 - 2) as usize)].ty = TileType::Passage;
    map[((size.1 - 2 ) as usize)][((size.0 - 3) as usize)].ty = TileType::Passage;
    map[((size.1 - 3 ) as usize)][((size.0 - 3) as usize)].ty = TileType::Passage;
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
                TileType::Wall => print!("{} ", SetBackgroundColor(Color::White)),
                TileType::Win => print!("{} ", SetBackgroundColor(Color::Green))
            }
        }
    }
}

fn has_won(pos_x: usize, pos_y: usize, size: (u16, u16)) -> bool{
    if (pos_x == (size.0 - 2) as usize) && (pos_y == (size.1 - 2) as usize) {
        return true;
    }
    return false;
}

fn you_won(map: Grid){
    for y in map{
        for x in y{
            print!("{} ", SetBackgroundColor(Color::Black))
        }
    }
    stdout().execute(Clear(crossterm::terminal::ClearType::All));
    print!("{}{}", MoveTo(0,0), Hide);
    println!("You won! Well played!");
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
    
    let mut won: bool = false;

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
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('z'){
                    won = true;
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
                    if has_won(player.pos_x, player.pos_y, size) {won = true; break;}

                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('d') 
                && (map[player.pos_y][player.pos_x+1].ty != TileType::Wall)
                {
                    
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_x += 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                    if has_won(player.pos_x, player.pos_y, size) {won = true; break;}
                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('w') 
                && (map[player.pos_y-1][player.pos_x].ty != TileType::Wall)
                {
                   
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_y -= 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                    if has_won(player.pos_x, player.pos_y, size) {won = true; break;}
                }else if key.kind == KeyEventKind::Press 
                && key.code == KeyCode::Char('a') 
                && (map[player.pos_y][player.pos_x-1].ty != TileType::Wall)
                {
                    
                    map[player.pos_y][player.pos_x].occupied = false;
                    player.pos_x -= 1;
                    map[player.pos_y][player.pos_x].occupied = true;
                    draw_screen(&map);
                    if has_won(player.pos_x, player.pos_y, size) {won = true; break;}
                }
            }
        }
    }

    if won {you_won(map);}

    loop{
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
            }
        }
    }
execute!(stdout(), LeaveAlternateScreen);
print!("{}", Show);
disable_raw_mode();
}
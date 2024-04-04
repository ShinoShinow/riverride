use std::{ io::{ stdout, Stdout, Write}, thread::sleep, time::Duration, vec};
use crossterm::{
    cursor::{Hide, MoveTo, Show}, event::{self, poll, read, Event, KeyCode, KeyEvent}, execute, queue, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, terminal::{disable_raw_mode, enable_raw_mode, Clear}, ExecutableCommand, QueueableCommand
};
use rand::{thread_rng, Rng};

struct World {
    player_c: u16,
    player_l: u16,
    maxc: u16,
    maxl: u16,
    map: Vec<(u16 , u16)>,
    died: bool,
    next_right: u16,
    next_left: u16
}

fn draw(sc: &mut Stdout,worldr: &mut World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.flush()?;

    //draw the map
    for l in 0..=worldr.map.len() - 1 {
        sc.queue(MoveTo(0,l as u16))?;
        sc.queue(Print("+".repeat(worldr.map[l].0 as usize)))?;
        sc.queue(MoveTo(worldr.map[l].1,l as u16))?;
        let usizen = worldr.maxc - worldr.map[l].1;
        sc.queue(Print("+".repeat(usizen as usize)))?;
        sc.flush()?;
    }

    //draw the player
    sc.queue(MoveTo(worldr.player_c,worldr.player_l))?;
    sc.queue(Print("P"))?;
    sc.queue(Hide)?;
    sc.flush()?;

    Ok(())
}

fn physics(mut world: World) -> std::io::Result<World> {
    //check if player die
    if world.player_c <= world.map[world.player_l as usize].0 {
        world.died = true;
    }
    if world.player_c >= world.map[world.player_l as usize].1 - 1 {
        world.died = true;
    }
    //shift the map
    for l in (0 .. world.map.len()-1).rev() {
        world.map[l+1].0 = world.map[l].0;
        world.map[l+1].1 = world.map[l].1;
    }
    
    if world.map[0].0 < world.next_left {
        world.map[0].0 += 1
    }
    if world.map[0].0 > world.next_left {
        world.map[0].0 -= 1
    }
    if world.map[0].1 > world.next_right {
        world.map[0].1 -= 1
    }
    if world.map[0].1 < world.next_right {
        world.map[0].1 += 1
    }
    let mut rng = thread_rng();
    if world.next_left == world.map[0].0 && rng.gen_range(0..10) >= 7 {
        world.next_left = rng.gen_range(world.next_left-5..world.next_left+5)
    }
    if world.next_right == world.map[0].1 && rng.gen_range(0..10) >= 7  {
        world.next_right = rng.gen_range(world.next_right-5..world.next_right+5)
    }
    if world.next_right - world.next_left < 3 { // todo: check abs
        world.next_right += 3;
    }

    Ok(world)
}

fn main() -> std::io::Result<()> {

    //init screen
    let mut screen = stdout();
    let (maxc,maxl) = crossterm::terminal::size()?;
    enable_raw_mode()?;


    //init game
    let mut world = World {
        player_c: maxc / 2,
        player_l: maxl - 1,
        maxc,
        maxl,
        map : vec![((maxc/2) - 7,(maxc/2) + 8);maxl as usize],
        died: false,
        next_left: maxc/2 - 14,
        next_right: maxc/2 + 15
    };

    while !world.died {
        if poll(Duration::from_millis(10))? {
            let readr = read().unwrap();
            while poll(Duration::from_millis(0)).unwrap() {
                let _ = read();
            }            
            match readr {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('w') => {
                            if world.player_l > 1 {
                                world.player_l -= 1
                            }
                        },
                        KeyCode::Char('a') => { 
                            if world.player_c > 1 {
                                world.player_c -= 1
                            }
                        },
                        KeyCode::Char('s') => { 
                            if world.player_l < maxl -1 {
                                world.player_l += 1
                            }
                        },
                        KeyCode::Char('d') => { 
                            if world.player_c < maxc - 1 {
                                world.player_c += 1
                            }
                        },
                        KeyCode::Esc => { break; }
                        _ => {}
                    }
                },
                _ => {

                }
            }
        if world.died {
            screen.queue(MoveTo((world.maxc / 2) -4,world.maxl /2)).unwrap();
            screen.queue(Print("You Die!!")).unwrap();
            screen.flush().unwrap();
            sleep(Duration::from_secs(3));
            screen.queue(Clear(crossterm::terminal::ClearType::All)).unwrap();
            screen.flush().unwrap();
            screen.execute(Print("Thanks for playing!!")).unwrap();
            break;
        }
        }

        world = physics(world).unwrap();
        draw(&mut screen,&mut world)?; 
        sleep(Duration::from_millis(100));
    }
    screen.queue(Show)?;
    screen.flush()?;
    disable_raw_mode()?;
    Ok(())
}

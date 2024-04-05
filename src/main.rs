use std::{ io::{ stdout, Stdout, Write}, thread::sleep, time::Duration, vec};
use crossterm::{cursor::{Hide, MoveTo, Show}, event::{poll, read, Event, KeyCode,}, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear}, ExecutableCommand, QueueableCommand};
use rand::{ thread_rng, Rng};

struct Enemy {
    line: u16,
    cols: u16
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         
struct Bullet {
    l: u16,
    c: u16,
    range: u16
}

enum PlayerStatus {
    Alive,
    Dead,   
}

struct World {
    player_c: u16,
    player_l: u16,
    maxc: u16,
    maxl: u16,
    map: Vec<(u16 , u16)>,
    status: PlayerStatus,
    next_right: u16,
    next_left: u16,
    enemy : Vec<Enemy>,
    bullets: Vec<Bullet>,
    score: u16,
    fuel: u16,
    timer: u8
}

fn draw(sc: &mut Stdout,world: &mut World) -> std::io::Result<()> {
    sc.queue(Clear(crossterm::terminal::ClearType::All))?;
    sc.flush()?;

    //draw the map
    for l in 0..=world.map.len() - 1 {
        sc.queue(MoveTo(0,l as u16))?;
        sc.queue(Print("+".repeat(world.map[l].0 as usize)))?;
        sc.queue(MoveTo(world.map[l].1,l as u16))?;
        let usizen = world.maxc - world.map[l].1;
        sc.queue(Print("+".repeat(usizen as usize)))?;
        sc.flush()?;
    }

    //draw score and fuel
    sc.queue(MoveTo(2,2))?;
    sc.queue(Print(format!("Score: {}", world.score)))?;
    sc.queue(MoveTo(2,3))?;
    sc.queue(Print(format!("Fuel: {}", world.fuel)))?;
    sc.flush()?;

    //draw the player
    sc.queue(MoveTo(world.player_c,world.player_l))?;
    sc.queue(Print("P"))?;
    sc.queue(Hide)?;
    sc.flush()?;

    //draw enemy
    for i in &world.enemy {
        sc.queue(MoveTo(i.cols,i.line)).unwrap();
        sc.queue(Print('E')).unwrap();
    }
    sc.flush().unwrap();

    //draw bullet
    if world.bullets.len() > 0 {
        sc.queue(MoveTo(world.bullets[0].c,world.bullets[0].l + 1))?;
        sc.queue(Print('|'))?;
        sc.queue(MoveTo(world.bullets[0].c,world.bullets[0].l))?;
        sc.queue(Print('^'))?;
        sc.flush()?;
    }

    Ok(())
}
fn physics(mut world: World) -> std::io::Result<World> {
    //check if player die
    if world.player_c <= world.map[world.player_l as usize].0 {
        world.status = PlayerStatus::Dead;
    }
    if world.player_c >= world.map[world.player_l as usize].1 - 1 {
        world.status = PlayerStatus::Dead;
    }
    if world.fuel == 0 {
        world.status = PlayerStatus::Dead;
    }

    //check enemy hit somthing
    for i in (0 .. world.enemy.len()).rev() {
        if world.player_c == world.enemy[i].cols && world.player_l == world.enemy[i].line {
            world.status = PlayerStatus::Dead;
        }
        for j in (0 .. world.bullets.len()).rev() {
            if world.bullets[j].l < 3 {
                if world.enemy[i].line == world.bullets[j].l || world.enemy[i].line == world.bullets[j].l + 1{
                    if world.enemy[i].cols == world.bullets[j].c {
                        world.enemy.remove(i);
                        world.score += 10;
                    }                
                }
            }else {
                if (world.enemy[i].line == world.bullets[j].l || world.enemy[i].line == world.bullets[j].l - 1) || world.enemy[i].line == world.bullets[j].l + 1{
                    if world.enemy[i].cols == world.bullets[j].c {
                        world.enemy.remove(i);
                        world.score += 10;
                    }                
                }
            }
        }
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
    if world.next_right.abs_diff( world.next_left) < 4 {
        world.next_right += 3;
    }

    //enemy init
    if rng.gen_range(1 ..= 100) < 11 {
        let randint_c = rng.gen_range(world.map[0].0 + 1 .. world.map[0].1 - 1);
        let temp = Enemy {
            line: 0,
            cols: randint_c
        };
        world.enemy.push(temp);
    }

    for i in &mut world.enemy {
        i.line += 1;
    }

    //enemy delete
    if world.enemy.len() > 0 {
        for i in (0 ..= world.enemy.len()-1).rev() {
            if world.enemy[i].line > world.maxl - 1 {
                world.enemy.remove(i);
            }
        }
    }

    //move bullet
    if world.bullets.len() >= 1 {
        if world.bullets[0].range == 0 {
            world.bullets.remove(0);
        }else {
            if world.bullets[0].l < 2 {
                world.bullets[0].range = 0;
            }else {
                world.bullets[0].l -= 2;
                world.bullets[0].range -= 1;  
            } 
        }
    }
    // fuel consumption
    if world.timer == 20 {
        world.fuel -= 1;
        world.timer = 0;
    }
    world.timer += 1;
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
        status: PlayerStatus::Alive,
        next_left: maxc/2 - 13,
        next_right: maxc/2 + 14,
        enemy: vec![],
        bullets: vec![],
        score: 0,
        fuel: 20,
        timer: 0
    };

    loop {
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
                        KeyCode::Char(' ') => {
                            if world.bullets.len() < 1 {    
                                let temp = Bullet {
                                    l : world.player_l,
                                    c : world.player_c,
                                    range: 9
                                };
                                world.bullets.push(temp);  
                            }
                        },
                        KeyCode::Esc => { break; },
                        _ => {}
                    }
                },
                _ => {

                }
            }
        }
        //dead
        match world.status {
            PlayerStatus::Dead => {
                screen.queue(MoveTo((world.maxc / 2) -4,world.maxl /2)).unwrap();
                screen.queue(Print("You Die!!")).unwrap();
                screen.flush().unwrap();
                sleep(Duration::from_secs(3));
                screen.queue(Clear(crossterm::terminal::ClearType::All)).unwrap();
                screen.flush().unwrap();
                screen.execute(Print("Thanks for playing!!")).unwrap();
                break;
            },
            _ => {}
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
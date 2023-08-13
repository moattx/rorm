// game logic
use std::io;

use crossterm::{
    cursor::MoveTo, style::Print, terminal::size, terminal::Clear, terminal::ClearType,
    ExecutableCommand,
};
use rand::Rng;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Game<'a> {
    pub running: bool,
    pub score: u16,
    sout: &'a io::Stdout,
    worm_body: Vec<(u16, u16)>, //x, y
    worm_headx: u16,
    worm_heady: u16,
    worm_direction: Direction,
    maxx: u16,
    maxy: u16,
    foody: u16,
    foodx: u16,
    food: u16,
    upper: bool,
}

impl<'a> Game<'a> {
    pub fn new(stdout: &'a io::Stdout) -> Game<'a> {
        let (maxx, maxy) = size().unwrap();
        Self {
            running: true,
            score: 0,
            sout: stdout,
            worm_body: Vec::new(),
            worm_headx: maxx / 2,
            worm_heady: maxy / 2,
            worm_direction: Direction::Right,
            maxx,
            maxy,
            foody: 0,
            foodx: 0,
            food: 0,
            upper: false,
        }
    }
    pub fn quit(&mut self) {
        self.running = !self.running;
    }
    pub fn display(&mut self) {
        self.sout.execute(Clear(ClearType::All));
        self.display_worm();
        self.update_food();
        self.display_food();
    }

    pub fn update(&mut self) {
        // check ClearType again
        self.sout.execute(Clear(ClearType::All));

        if self.check_borders() {
            self.quit();
            return;
        }

        if self.check_eaten() {
            self.add_worm_part();
            self.update_score();
            self.update_food();
        }

        self.display_score();
        self.display_food();
        // update the worm part's cords
        self.update_worm();
        // then display them
        self.display_worm();
    }

    fn check_eaten(&mut self) -> bool {
        (self.worm_headx, self.worm_heady) == (self.foodx, self.foody)
    }

    fn add_worm_part(&mut self) {
        let new_body = match self.worm_direction {
            Direction::Up => (self.worm_headx, self.worm_heady + 1),
            Direction::Right => (self.worm_headx - 1, self.worm_heady),
            Direction::Down => (self.worm_headx, self.worm_heady - 1),
            Direction::Left => (self.worm_headx + 1, self.worm_heady),
        };

        for _ in 0..=self.food {
            self.worm_body.push(new_body);
        }
    }

    fn update_score(&mut self) {
        self.score = self.score + self.food;
    }

    fn display_score(&mut self) {
        if self.score != 0 {
            self.sout.execute(MoveTo(self.maxx / 2, 0)).unwrap();
            let printout = format!("Score: {}", self.score);
            self.sout.execute(Print(printout)).unwrap();
        }
    }

    fn display_worm(&mut self) {
        // display body parts
        for (partx, party) in &self.worm_body {
            self.sout.execute(MoveTo(*partx, *party)).unwrap();
            self.sout.execute(Print("o")).unwrap();
        }
        // display head
        self.sout
            .execute(MoveTo(self.worm_headx, self.worm_heady))
            .unwrap();
        self.sout.execute(Print("@")).unwrap();
    }

    fn update_worm(&mut self) {
        if self.upper {
            // assign the first body part with the head's cords - worm_direction and then save the
            // body part's previous cords
            // when it goes to the second body part, it'll get the previous body part's new (not
            // the old) cords and does the same cords calculation cords - worm_direction and etc
            //
            // 1. loop through body parts
            // 2. assign first body part with a var outside the loop that did the calculation first
            // 3. save the calculation in a var outside loop
            //
            // 1. loop
            // 2. do the calculations with a outside mutable var that has head vars
            // 3. set the outside var with cords of body part
            // 4. set body part
            
            /*
            let mut egobrox = self.worm_headx;
            let mut egobroy = self.worm_heady;
            */
            
            let (mut savex, mut savey) = (self.worm_headx, self.worm_heady);

            for i in 0..self.worm_body.len() {
/*
                if (self.worm_direction == Direction::Right || self.worm_direction == Direction::Left && i != 9) || (self.worm_direction == Direction::Up || self.worm_direction == Direction::Down && i != 5){
                */

                /*
                let cal = match self.worm_direction {
                        Direction::Up => {
                            if i != 5 {
                            (savex, savey + 1)
                            }
                            /*else{
                            (savex - 1, savey)
                            }
                            */
                        },
                        Direction::Right => {
                            if i != 9 {
                            (savex - 1, savey)
                            }
                            //else{
                            //(savex, savey + 1)
                            //}
                        },
                        Direction::Down => {
                            if i != 5 {
                            (savex, savey - 1)
                            }
                            //else{
                            //(savex + 1, savey)
                            //}
                        },
                        Direction::Left => 
                        {
                            if i != 9 {
                                (savex + 1, savey)
                            }
                            //else{
                            //    (savex, savey - 1)
                            //}
                        },

                };
            */
                /*
                }else{
                    let cal = (savex, savey);
                }
                */

                //(savex, savey) = self.worm_body[i];

                let cal = match self.worm_direction {
                        Direction::Up => (savex, savey + 1),
                        Direction::Right => (savex - 1, savey),
                        Direction::Down => (savex, savey - 1),
                        Direction::Left => (savex + 1, savey),
                };



                (savex, savey) = cal;
                self.worm_body[i] = cal;

                /*
                let (previousx, previousy) = (egobrox, egobroy);

                let (getterx, gettery) = self.worm_body[i];

                let (egobrox, egobroy) = match self.worm_direction {
                    Direction::Up => (getterx, gettery + 1),
                    Direction::Right => (getterx - 1, gettery),
                    Direction::Down => (getterx, gettery - 1),
                    Direction::Left => (getterx + 1, gettery),
                };
                // now previous
                self.worm_body[i] = (previousx, previousy);
                */

                if (self.worm_headx, self.worm_heady) == (savex, savey) {
                    self.quit();
                    return;
                }
            }
        } else {
            let mut egobro = (self.worm_headx, self.worm_heady);
            for i in 0..self.worm_body.len() {
                let previous = egobro;
                egobro = self.worm_body[i];
                self.worm_body[i] = previous;

                if (self.worm_headx, self.worm_heady) == egobro {
                    self.quit();
                    return;
                }
            }
        }
    }

    fn display_food(&mut self) {
        self.sout.execute(MoveTo(self.foodx, self.foody)).unwrap();
        self.sout.execute(Print(self.food)).unwrap();
    }
    fn update_food(&mut self) {
        let foodx = rand::thread_rng().gen_range(1..(self.maxx - 1));
        let foody = rand::thread_rng().gen_range(1..(self.maxy - 1));
        let food = rand::thread_rng().gen_range(1..10);

        self.foodx = foodx;
        self.foody = foody;
        self.food = food;
    }

    fn check_borders(&mut self) -> bool {
        self.worm_heady == (self.maxy - 1)
            || self.worm_heady == 1
            || self.worm_headx == (self.maxx - 1)
            || self.worm_headx == 1
    }

    pub fn go_right(&mut self) {
        self.upper = false;
        self.worm_direction = Direction::Right;
        self.worm_headx = self.worm_headx + 1;
    }

    pub fn go_left(&mut self) {
        self.upper = false;
        self.worm_direction = Direction::Left;
        self.worm_headx = self.worm_headx - 1;
    }

    pub fn go_up(&mut self) {
        self.upper = false;
        self.worm_direction = Direction::Up;
        self.worm_heady = self.worm_heady - 1;
    }

    pub fn go_down(&mut self) {
        self.upper = false;
        self.worm_direction = Direction::Down;
        self.worm_heady = self.worm_heady + 1;
    }

    pub fn upper_go_right(&mut self) {
        self.upper = true;
        self.worm_direction = Direction::Right;

        //https://man.netbsd.org/worm.6
        self.worm_headx = self.worm_headx + 9;
    }

    pub fn upper_go_left(&mut self) {
        self.upper = true;
        self.worm_direction = Direction::Left;

        // https://man.netbsd.org/worm.6
        self.worm_headx = self.worm_headx - 9;
    }

    pub fn upper_go_up(&mut self) {
        self.upper = true;
        self.worm_direction = Direction::Up;

        //https://man.netbsd.org/worm.6
        self.worm_heady = self.worm_heady - 5;
    }

    pub fn upper_go_down(&mut self) {
        self.upper = true;
        self.worm_direction = Direction::Down;
        //https://man.netbsd.org/worm.6
        self.worm_heady = self.worm_heady + 5;
    }
}

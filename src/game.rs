// game logic
use std::collections::VecDeque;
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
    worm_body: VecDeque<(u16, u16)>, //x, y
    worm_headx: u16,
    worm_heady: u16,
    worm_direction: Direction,
    maxx: u16,
    maxy: u16,
    foody: u16,
    foodx: u16,
    food: u16,
    upper_move: u16,
}

impl<'a> Game<'a> {
    pub fn new(stdout: &'a io::Stdout) -> Game<'a> {
        //let (maxx, maxy) = size().unwrap() as (i32, i32);
        let (maxx, maxy) = size().unwrap();
        Self {
            running: true,
            score: 0,
            sout: stdout,
            //worm_body: Vec::new(),
            worm_body: VecDeque::new(),
            worm_headx: maxx / 2,
            worm_heady: maxy / 2,
            worm_direction: Direction::Right,
            //maxx,
            maxx,
            maxy,
            //maxy,
            foody: 0,
            foodx: 0,
            food: 0,
            upper_move: 0,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
    pub fn display(&mut self) -> std::io::Result<()> {
        self.sout.execute(Clear(ClearType::All))?;
        self.draw_border();
        self.display_worm();
        self.update_food();
        self.display_food();
        Ok(())
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }
        if self.check_borders() {
            self.quit();
            return;
        }

        if self.check_eaten() {
            self.upper_move = 0;
            self.add_worm_part();
            self.update_score();
            self.update_food();
            self.display_score();
            self.display_food();
        }

        self.display_worm();

        if self.upper_move != 0 {
            self.upper_move -= 1;
            self.go_forward();
            self.update();
        }
    }

    fn draw_border(&mut self) {
        for i in 0..self.maxy {
            self.sout.execute(MoveTo(self.maxx - 2, i)).unwrap();
            self.sout.execute(Print("|")).unwrap();
            self.sout.execute(MoveTo(1, i)).unwrap();
            self.sout.execute(Print("|")).unwrap();
        }

        for i in 0..(self.maxx - 2) {
            if i > 1 {
                self.sout.execute(MoveTo(i, self.maxy)).unwrap();
                self.sout.execute(Print("-")).unwrap();
                self.sout.execute(MoveTo(i, 0)).unwrap();
                self.sout.execute(Print("-")).unwrap();
            }
        }
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

        for _ in 1..=self.food {
            self.worm_body.push_front(new_body);
        }
    }

    fn update_score(&mut self) {
        self.score += self.food;
    }

    fn display_score(&mut self) {
        if self.score != 0 {
            self.sout.execute(MoveTo(self.maxx / 2, 0)).unwrap();
            let printout = format!("Score: {}", self.score);
            self.sout.execute(Print(printout)).unwrap();
        }
    }

    fn display_worm(&mut self) {
        if let Some((tailx, taily)) = self.worm_body.back() {
            self.sout.execute(MoveTo(*tailx, *taily)).unwrap();
            self.sout.execute(Print(" ")).unwrap();
        }
        self.worm_body.pop_back();

        for (partx, party) in self.worm_body.iter() {
            self.sout.execute(MoveTo(*partx, *party)).unwrap();
            self.sout.execute(Print("o")).unwrap();
            if (self.worm_headx, self.worm_heady) == (*partx, *party) {
                self.quit();
                return;
            }
        }

        self.worm_body
            .push_front((self.worm_headx, self.worm_heady));
        self.sout
            .execute(MoveTo(self.worm_headx, self.worm_heady))
            .unwrap();
        self.sout.execute(Print("@")).unwrap();
        self.sout
            .execute(MoveTo(self.worm_headx, self.worm_heady))
            .unwrap();
    }

    fn display_food(&mut self) {
        self.sout.execute(MoveTo(self.foodx, self.foody)).unwrap();
        self.sout.execute(Print(self.food)).unwrap();
    }
    fn update_food(&mut self) {
        let foodx = rand::thread_rng().gen_range(2..(self.maxx - 2));
        let foody = rand::thread_rng().gen_range(2..(self.maxy - 2));
        let food = rand::thread_rng().gen_range(1..9);

        self.foodx = foodx;
        self.foody = foody;
        self.food = food;
    }

    fn check_borders(&mut self) -> bool {
        self.worm_heady == (self.maxy - 1)
            || self.worm_heady == 0
            || self.worm_headx == (self.maxx - 2)
            || self.worm_headx == 1
    }

    pub fn go_forward(&mut self) {
        (self.worm_headx, self.worm_heady) = match self.worm_direction {
            Direction::Up => (self.worm_headx, self.worm_heady - 1),
            Direction::Right => (self.worm_headx + 1, self.worm_heady),
            Direction::Down => (self.worm_headx, self.worm_heady + 1),
            Direction::Left => (self.worm_headx - 1, self.worm_heady),
        };
    }

    pub fn go_right(&mut self) {
        self.worm_direction = Direction::Right;
        self.worm_headx += 1;
    }

    pub fn go_left(&mut self) {
        self.worm_direction = Direction::Left;
        self.worm_headx -= 1;
    }

    pub fn go_up(&mut self) {
        self.worm_direction = Direction::Up;
        self.worm_heady -= 1;
    }

    pub fn go_down(&mut self) {
        self.worm_direction = Direction::Down;
        self.worm_heady += 1;
    }

    pub fn upper_go_right(&mut self) {
        self.upper_move = 9;
        self.worm_direction = Direction::Right;

        //https://man.netbsd.org/worm.6
        self.worm_headx += 1;
    }

    pub fn upper_go_left(&mut self) {
        self.upper_move = 9;
        self.worm_direction = Direction::Left;

        // https://man.netbsd.org/worm.6
        self.worm_headx -= 1;
    }

    pub fn upper_go_up(&mut self) {
        self.upper_move = 5;
        self.worm_direction = Direction::Up;

        //https://man.netbsd.org/worm.6
        self.worm_heady -= 1;
    }

    pub fn upper_go_down(&mut self) {
        self.upper_move = 5;
        self.worm_direction = Direction::Down;
        //https://man.netbsd.org/worm.6
        self.worm_heady += 1;
    }
}

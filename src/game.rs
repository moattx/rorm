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
        }
    }
    pub fn quit(&mut self) {
        self.running = !self.running;
    }
    pub fn display(&mut self) {
        self.display_worm();
        self.update_food();
        self.display_food();
    }

    pub fn update(&mut self) {
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
        self.worm_direction = Direction::Right;
        self.worm_headx = self.worm_headx + 1;
    }

    pub fn go_left(&mut self) {
        self.worm_direction = Direction::Left;
        self.worm_headx = self.worm_headx - 1;
    }

    pub fn go_up(&mut self) {
        self.worm_direction = Direction::Up;
        self.worm_heady = self.worm_heady - 1;
    }

    pub fn go_down(&mut self) {
        self.worm_direction = Direction::Down;
        self.worm_heady = self.worm_heady + 1;
    }
}

// game logic
use std::collections::VecDeque;
use std::io;

// TODO: replace style::Print with custom?

use crossterm::{
    cursor::MoveTo, style::Print, terminal::size, terminal::Clear, terminal::ClearType,
    ExecutableCommand,
};
use rand::Rng;

// https://man.netbsd.org/worm.6
const UPPER_Y: u16 = 5;
const UPPER_X: u16 = 9;

type Point = (u16, u16); // x, y

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
    worm_body: VecDeque<Point>, //TODO: just make worm_body worm
    worm_direction: Direction,  // TODO: just make worm_direction direction
    screen: Point,
    food: Point,
    food_value: u16,
    upper_move: u16,
}

impl<'a> Game<'a> {
    pub fn new(stdout: &'a io::Stdout) -> Game<'a> {
        let (maxx, maxy) = size().unwrap();
        Self {
            running: true,
            score: 0,
            sout: stdout,
            worm_body: VecDeque::new(),
            worm_direction: Direction::Right,
            screen: (maxx, maxy),
            food: (0, 0),
            food_value: 0,
            upper_move: 0,
        }
    }
    pub fn quit(&mut self) {
        self.running = false;

        // XXX: for tty
        self.sout.execute(MoveTo(0, 0)).unwrap();
        self.sout.execute(Clear(ClearType::All)).unwrap();
    }
    pub fn display(&mut self) -> std::io::Result<()> {
        self.sout.execute(Clear(ClearType::All))?;
        self.draw_border();
        self.display_worm((self.screen.0 / 2, self.screen.1 / 2));
        self.update_food();
        self.display_food();
        Ok(())
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        let new_head = self.get_new_head();

        if self.check_collisions(new_head) {
            self.quit();
            return;
        }

        if self.check_eaten(new_head) {
            self.upper_move = 0;
            self.add_worm_part(new_head);
            self.update_score();
            self.update_food();
            self.display_score();
            self.display_food();
        } else {
            self.pop_tail();
        }

        self.display_worm(new_head);

        if self.upper_move != 0 {
            self.upper_move -= 1;
            self.update();
        }
    }
    fn check_collisions(&mut self, new_head: Point) -> bool {
        for part in &self.worm_body {
            if new_head == *part {
                return true;
            }
        }

        if new_head.1 == (self.screen.1 - 1)
            || new_head.1 == 0
            || new_head.0 == (self.screen.0 - 2)
            || new_head.0 == 1
        {
            return true;
        }

        false
    }

    fn get_new_head(&mut self) -> Point {
        let head = self.worm_body.front().unwrap().clone();
        let new_head = match self.worm_direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };
        new_head
    }

    fn draw_border(&mut self) {
        for i in 0..self.screen.1 {
            self.sout.execute(MoveTo(self.screen.0 - 2, i)).unwrap();
            self.sout.execute(Print("|")).unwrap();
            self.sout.execute(MoveTo(1, i)).unwrap();
            self.sout.execute(Print("|")).unwrap();
        }

        for i in 0..(self.screen.0 - 2) {
            if i > 1 {
                self.sout.execute(MoveTo(i, self.screen.1)).unwrap();
                self.sout.execute(Print("-")).unwrap();
                self.sout.execute(MoveTo(i, 0)).unwrap();
                self.sout.execute(Print("-")).unwrap();
            }
        }
    }

    fn check_eaten(&mut self, new_head: Point) -> bool {
        new_head == self.food
    }

    fn add_worm_part(&mut self, new_body: Point) {
        for _ in 2..=self.food_value {
            self.worm_body.push_front(new_body);
        }
    }

    fn update_score(&mut self) {
        self.score += self.food_value;
    }

    fn display_score(&mut self) {
        if self.score != 0 {
            self.sout.execute(MoveTo(self.screen.0 / 2, 0)).unwrap();
            let printout = format!("Score: {}", self.score);
            self.sout.execute(Print(printout)).unwrap();
        }
    }

    fn pop_tail(&mut self) {
        if let Some((tailx, taily)) = self.worm_body.back() {
            self.sout.execute(MoveTo(*tailx, *taily)).unwrap();
            self.sout.execute(Print(" ")).unwrap();
        }
        self.worm_body.pop_back();
    }
    fn display_worm(&mut self, new_head: Point) {
        for (partx, party) in self.worm_body.iter() {
            self.sout.execute(MoveTo(*partx, *party)).unwrap();
            self.sout.execute(Print("o")).unwrap();
        }

        self.worm_body.push_front(new_head);
        self.sout.execute(MoveTo(new_head.0, new_head.1)).unwrap();
        self.sout.execute(Print("@")).unwrap();
        self.sout.execute(MoveTo(new_head.0, new_head.1)).unwrap();
    }

    fn display_food(&mut self) {
        self.sout.execute(MoveTo(self.food.0, self.food.1)).unwrap();
        self.sout.execute(Print(self.food_value)).unwrap();
    }
    fn update_food(&mut self) {
        let mut foodx = rand::thread_rng().gen_range(2..(self.screen.0 - 2));
        let mut foody = rand::thread_rng().gen_range(2..(self.screen.1 - 2));
        let food = rand::thread_rng().gen_range(1..9);

        for part in &self.worm_body {
            if (foodx, foody) == *part {
                foodx = rand::thread_rng().gen_range(2..(self.screen.0 - 2));
                foody = rand::thread_rng().gen_range(2..(self.screen.1 - 2));
                continue;
            }
        }

        self.food = (foodx, foody);
        self.food_value = food;
    }

    pub fn go_forward(&mut self) {
        self.update();
    }

    pub fn go_right(&mut self) {
        self.worm_direction = Direction::Right;
    }

    pub fn go_left(&mut self) {
        self.worm_direction = Direction::Left;
    }

    pub fn go_up(&mut self) {
        self.worm_direction = Direction::Up;
    }

    pub fn go_down(&mut self) {
        self.worm_direction = Direction::Down;
    }

    pub fn upper_go_right(&mut self) {
        self.upper_move = UPPER_X;
        self.worm_direction = Direction::Right;
    }

    pub fn upper_go_left(&mut self) {
        self.upper_move = UPPER_X;
        self.worm_direction = Direction::Left;
    }

    pub fn upper_go_up(&mut self) {
        self.upper_move = UPPER_Y;
        self.worm_direction = Direction::Up;
    }

    pub fn upper_go_down(&mut self) {
        self.upper_move = UPPER_Y;
        self.worm_direction = Direction::Down;
    }
}

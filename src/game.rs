// game logic

use std::io::{self, Write};
use crossterm::cursor::Hide;
use crossterm::cursor::Show;

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{DisableBlinking, EnableBlinking, MoveTo, MoveUp, RestorePosition, SavePosition},
    terminal::Clear,
    terminal::ClearType,
    terminal::size,
};
use rand::Rng;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::style::{self, Stylize};


pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

pub struct Game<'a>{
    pub running: bool,
    sout: &'a io::Stdout,
    worm_body: Vec<(u16,u16)>, //x, y
    //worm_body_len: usize,
    worm_headx: u16,
    worm_heady: u16,
    worm_direction: Direction,
    maxx: u16,
    maxy: u16,
    foody: u16,
    foodx: u16,
    food: u16,
}

impl<'a> Game<'a>{
    pub fn new(stdout: &'a io::Stdout) -> Game<'a>{
        let (maxx, maxy) = size().unwrap();
        Self{
            running: true,
            sout: stdout,
            worm_body: vec![((maxx / 2) - 1, (maxy / 2))],
            //worm_body_len: 0,
            worm_headx: maxx / 2,
            worm_heady: maxy / 2,
            worm_direction: Direction::Right,
            maxx: maxx,
            maxy: maxy,
            foody: 0,
            foodx: 0,
            food: 0,
        }
    }
    pub fn quit(&mut self){
        self.running = !self.running;
    }
    pub fn display(&mut self){
        self.display_worm();
        self.update_food();
        self.display_food();
    }

    pub fn update(&mut self){
        self.sout.execute(Clear(ClearType::All));

        if self.check_borders() {
            return
        }

        if self.check_eaten(){
            self.add_worm_part();
            self.update_food();
        }

        self.display_food();

        // update the worm part's cords
        self.update_worm();
        // then display them
        self.display_worm();
    }

    fn check_eaten(&mut self) -> bool {
        if (self.worm_headx, self.worm_heady) == (self.foodx, self.foody) {
            true
        }else{
            false
        }
    }

    fn add_worm_part(&mut self){
        //self.worm_body[self.worm_body.len() + 1] = (self.worm_headx, self.worm_heady);
        if self.worm_body.is_empty(){
            self.worm_body.push([self.worm_headx, self.worm_heady].into());
        }else{
            //let (previousx, previousy) = self.worm_body[self.worm_body.len()];
            //let mut egobro = (self.worm_headx, self.worm_heady);

            let new_body = match self.worm_direction {
              Direction::Up => (self.worm_headx, self.worm_heady - 1),
              Direction::Right => (self.worm_headx + 1, self.worm_heady),
              Direction::Down => (self.worm_headx , self.worm_heady + 1),
              Direction::Left => (self.worm_headx - 1, self.worm_heady),
            };
            for i in 1..=self.food{
                self.worm_body.push(new_body);
            }
        }
    }

    fn display_worm(&mut self){
        // display head
        self.sout.execute(MoveTo(self.worm_headx, self.worm_heady)).unwrap();
        self.sout.execute(Print("@")).unwrap();

        // display body parts
        for (partx, party) in &self.worm_body{
            self.sout.execute(MoveTo(*partx, *party)).unwrap();
            self.sout.execute(Print("o")).unwrap();
        }
        // go back to head
        self.sout.execute(MoveTo(self.worm_headx, self.worm_heady)).unwrap();
    }

    fn update_worm(&mut self){
            if self.worm_body.is_empty(){
                return
            }
            let mut egobro = (self.worm_headx, self.worm_heady);
            for i in 0..self.worm_body.len() {
                 let previous = egobro;
                 egobro = self.worm_body[i];
                 self.worm_body[i] = previous;
            }
    }

    fn display_food(&mut self){
            self.sout.execute(MoveTo(self.foodx, self.foody)).unwrap();
            self.sout.execute(Print(self.food)).unwrap();
    }
    fn update_food(&mut self){
            let foodx = rand::thread_rng().gen_range(0..self.maxx);
            let foody = rand::thread_rng().gen_range(0..self.maxy);
            let food = rand::thread_rng().gen_range(1..10);

            self.foodx = foodx;
            self.foody = foody;
            self.food = food;

            /*
            self.sout.execute(MoveTo(numx, numy)).unwrap();
            self.sout.execute(Print(numfood)).unwrap();
            */

    }
    fn check_borders(&mut self) -> bool{
        if self.worm_headx == self.maxx || self.worm_headx == 0 {
            self.quit();
            true
        }else if self.worm_heady == self.maxy || self.worm_heady == 0 {
            self.quit();
            true
        }else{
            false
        }
    }

    pub fn go_right(&mut self){
            self.worm_headx = self.worm_headx + 1;
    }

    pub fn go_left(&mut self){
            self.worm_headx = self.worm_headx - 1;
    }

    pub fn go_up(&mut self){
            self.worm_heady = self.worm_heady - 1;
    }

    pub fn go_down(&mut self){
            self.worm_heady = self.worm_heady + 1;
    }
}

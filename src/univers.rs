use rand::Rng;
use crate::univers::State::{Alive, Dead};

#[derive(Debug, Eq, PartialEq)]
pub struct Universe {
    pub universe: Vec<Vec<Cell>>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Cell {
pub x: usize,
pub y: usize,
pub state: State
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum State{
    Alive,
    Dead
}

impl Universe {

    pub fn generate(size: usize) -> Universe{
        
        let mut rng = rand::thread_rng();
        
        let mut universe = Vec::with_capacity(size);

        for x in 0..size {
            
            let mut row = Vec::with_capacity(size);
            
            for y in 0..size {
                
                let state = match rng.gen::<bool>() {
                    true => Alive,
                    false => Dead
                };
                
                row.push(Cell { x, y, state });
            }
            universe.push(row);
        }

        Universe { universe }
    }
    
    pub fn out_min (&self, value:usize) -> bool {

        usize::overflowing_sub(value, 1).1
    }

    pub fn out_max(&self, value:usize) -> bool {

        (value + 1) > (self.universe.len() - 1)
    }

    pub fn apply_rules(&self, x:usize, y:usize)-> State{

        let nb_voisin_alive = Cell::voisin_by_state(self, x, y, Alive).len();

        match &self.universe[x][y].state {
            Alive => {
                match nb_voisin_alive {
                    2 | 3 => Alive,
                    _ => Dead,
                }
            },
            Dead => {
                match nb_voisin_alive {
                    3 => Alive,
                    _ => Dead

                }
            }
        }
    }

    pub fn calculate_future(&self) -> Universe{

        let mut universe_future = Universe{ universe: Vec::new() };

        for (x, line) in self.universe.iter().enumerate() {

            universe_future.universe.push(Vec::new());

            for(y, _) in line.iter().enumerate(){
                let state = Self::apply_rules(self, x, y);
                universe_future.universe[x].push(Cell{x, y, state})
            }

        }

        universe_future
    }
}

impl Cell {
    pub fn voisin_by_state(universe: &Universe, x:usize, y:usize, state: State) -> Vec<&Cell> {
        let voisins = Self::find_voisin(universe, x, y);
        let mut result: Vec<&Cell> = Vec::new();
        for voisin in voisins{
            if voisin.state == state{
                result.push(voisin)
            }
        }
        result
    }
    pub fn find_voisin (universe: &Universe,x:usize,y:usize) -> Vec<&Cell>{
        let mut mes_voisins:Vec<&Cell> = Vec::new();
        //gauche
        if !universe.out_min(y){
            mes_voisins.push(&universe.universe[x][y-1]);
        }

        //droite
        if !universe.out_max(y) {
            mes_voisins.push(&universe.universe[x][y+1])
        }

        //haut
        if !universe.out_min(x) {
            mes_voisins.push(&universe.universe[x-1][y])
        }

        //haut gauche
        if !universe.out_min(x) && !universe.out_min(y){
            mes_voisins.push(&universe.universe[x-1][y-1])
        }

        //haut droite
        if !universe.out_min(x) && !universe.out_max(y){
            mes_voisins.push(&universe.universe[x-1][y+1])
        }

        //bas
        if !universe.out_max(x) {
            mes_voisins.push(&universe.universe[x+1][y])
        }

        //bas gauche
        if !universe.out_max(x) && !universe.out_min(y) {
            mes_voisins.push(&universe.universe[x+1][y-1])
        }

        //bas droite
        if !universe.out_max(x) && !universe.out_max(y) {
            mes_voisins.push(&universe.universe[x+1][y+1])
        }

        return mes_voisins;
    }
}
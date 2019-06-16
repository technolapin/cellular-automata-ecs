/* ARCHITECTURE DÉSIRÉE

on créé un objet voisinages + transition qui quand on lui donne un vec de cellule retourne l'étape suivant
En ECS.

Le vec des cellules est une ressource

les voisinages (cluster de cellules) sont des entitées



*/

#[derive(Default, Copy, Clone)]
struct Cell {
    state: u8,
}

impl Cell {
    fn to_string(&self) -> String {
        format!("{}", self.state)
    }
}

struct Cells {
    width: usize,
    height: usize,
    vec: Vec<Cell>,
}

impl Cells {
    fn new(w: usize, h: usize) -> Self {
        let mut v = Vec::new();
        v.reserve_exact(w * h);
        for i in 0..w * h {
            v.push(Cell::default());
        }
        Cells {
            width: w,
            height: h,
            vec: v,
        }
    }

    fn read(&self, i: usize) -> &Cell {
        &self.vec[i]
    }
    fn write(&mut self, i: usize, cell: Cell) {
        self.vec[i] = cell;
    }

    fn read_pos(&self, x: usize, y: usize) -> &Cell {
        self.read(x + self.width * y)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.read_pos(x, y).to_string());
            }
            println!();
        }
    }
}

struct Cluster {
    neighborhood: Vec<usize>,
    pos: usize, // the cell's position in the neighborhood
}
use specs::Component;
use specs::DenseVecStorage;
use specs::ReadStorage;
use specs::System;
impl Component for Cluster {
    type Storage = DenseVecStorage<Self>;
}

struct TransitionSystem;
impl<'a> System<'a> for TransitionSystem {
    type SystemData = (ReadStorage<'a, Cluster>);
    fn run(&mut self, datas: Self::SystemData) {}
}

fn automate(data: Cells) -> Cells {
    let moore = vec![
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let woop: Vec<usize> = (0..data.vec.len())
        .map(|i| {
            let x0 = i % data.width;
            let y0 = i / data.height;
            moore.iter().filter_map(|&(x, y)| {
                if x < 0 || y < 0 || (x as usize) >= data.width || y as usize >= data.height {
                    None
                } else {
                    Some((x as usize) + data.width * (y as usize))
                }
            })
        })
        .collect();

    println!("{:?}", woop);
    Cells {
        width: data.width,
        height: data.height,

        vec: data
            .vec
            .iter()
            .zip(0..data.vec.len())
            .map(|(&cell, i)| cell)
            .collect(),
    }
}

fn main() {
    let v = Cells::new(8, 8);
    v.print();
    automate(v);
}

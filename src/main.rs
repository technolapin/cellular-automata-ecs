/* ARCHITECTURE DÉSIRÉE

on créé un objet voisinages + transition qui quand on lui donne un vec de cellule retourne l'étape suivant
En ECS.

Le vec des cellules est une ressource

les voisinages (cluster de cellules) sont des entitées



*/

#[derive(Default, Copy, Clone, Debug)]
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
    fn write_pos(&mut self, x: usize, y: usize, cell: Cell)
    {
        self.write(x + self.width * y, cell)
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.read_pos(x, y).to_string());
            }
            println!();
        }
    }

    fn step(&self) -> Self
    {
        let moore = vec![
            (-1isize, -1isize),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        Cells{
            width: self.width,
            height: self.height,
            vec:
            (0..self.vec.len())
                .map(|i| {
                    let x0 = i % self.width;
                    let y0 = i / self.width;
                    moore.iter().filter_map(|&(v_x, v_y)| {
                        let x = x0 as isize + v_x;
                        let y = y0 as isize + v_y;
                        if x < 0 || y < 0 || (x as usize) >= self.width || y as usize >= self.height {
                            None
                        } else {
                            Some(self.vec[(x as usize) + self.width * (y as usize)].state)
                        }
                    }).fold(0, |somme, ngh| somme+ngh)
                }).zip(0..self.vec.len()).map(
                    |(sum, i)| Cell{state:
                                    match sum
                                    {
                                        2 => self.vec[i].state,
                                        3 => 1,
                                        _ => 0
                                    }}
                )
                .collect()
        }

    }
    fn evolve(&self, n: usize)
    {
        self.print();
        match n
        {
            0 => (),
            _ => self.step().evolve(n-1)
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


fn main() {
    const ON: Cell = Cell{state: 1};
    const OFF: Cell = Cell{state: 0};
    let mut v = Cells::new(19, 8);
    v.write_pos(2, 1, ON);
    v.write_pos(2, 2, ON);
    v.write_pos(2, 3, ON);
    v.write_pos(1, 3, ON);
    v.write_pos(0, 2, ON);

    v.evolve(20);
}

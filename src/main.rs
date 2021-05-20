use std::collections::HashSet;

// https://qiita.com/ref3000/items/af18a4532123c22a19a4
type State = Vec<Vec<bool>>;
type Field = Vec<Vec<u32>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct P {
    x: usize,
    y: usize
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Mino {
    ps: HashSet<P>,
    width: usize,
    height: usize
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    x: i32,
    y: i32,
    num: usize,
    is_border: bool
}

#[derive(Clone, Debug)]
struct Node {
    cells: Vec<Cell>,
    nums: Vec<usize>
}

fn enum_polyomino(n: usize) -> Vec<Mino> {
    let mut stack: Vec<Node> = vec![Node {
        cells: vec![Cell{
            x: 0,
            y: 0,
            num: 1,
            is_border: false
        }],
        nums: vec![1]
    }];

    let mut res: Vec<Mino> = vec![];

    while stack.len() > 0 {
        next_step(n, &mut stack, &mut res);
    }

    res
}

fn extract(node: &Node) -> Mino {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    let mut res: HashSet<P> = vec![].into_iter().collect();

    for cell in &node.cells {
        if cell.is_border {
            min_x = std::cmp::min(min_x, cell.x);
            min_y = std::cmp::min(min_y, cell.y);
            max_x = std::cmp::max(max_x, cell.x);
            max_y = std::cmp::max(max_y, cell.y);
        }
    }

    for cell in &node.cells {
        if cell.is_border {
            res.insert(P {
                x: (cell.x-min_x) as usize,
                y: (cell.y-min_y) as usize
            });
        }
    }

    Mino {
        ps: res,
        width: (max_x-min_x+1) as usize,
        height: (max_y-min_y+1) as usize
    }
}

fn next_step(n: usize, stack: &mut Vec<Node>, res: &mut Vec<Mino>) {
    if stack.len() > n || stack[stack.len()-1].nums.len() == 0 {
        stack.pop();
    } else {
        let length = stack.len();
        let num = stack[length-1].nums.pop().unwrap();
        let mut new_node = stack[stack.len()-1].clone();
        neighbor(&mut new_node, num);
        res.push(extract(&new_node));
        stack.push(new_node);
    }
}

fn neighbor(node: &mut Node, num: usize) {
    let cells = node.cells.clone();
    let mut nx = 0;
    let mut ny = 0;

    for mut cell in &mut node.cells {
        if cell.num == num {
            cell.is_border = true;
            nx = cell.x;
            ny = cell.y;
        }
    }

    let mut length: usize = cells.len();
    
    let dx: Vec<i32> = vec![0, -1, 1, 0];
    let dy: Vec<i32> = vec![-1, 0, 0, 1];

    for i in 0..4 {
        let x = nx + dx[i];
        let y = ny + dy[i];

        if !(y > 0 || (y == 0 && x >= 0)) {
            continue;
        }

        if cells.clone().into_iter().any(|e| e.x == x && e.y == y) {
            continue;
        }

        length += 1;

        node.cells.push(Cell {
            x: x,
            y: y,
            num: length,
            is_border: false
        });

        node.nums.push(length);
    }
}

fn duplicate(grps: &Vec<Vec<Mino>>) -> Vec<Vec<Mino>> {
    let mut mino_size: usize = 0;
    let mut res: Vec<Vec<Mino>> = vec![vec![]; grps.len()];

    for nmino in grps {
        let mut used: HashSet<Vec<P>> = vec![].into_iter().collect();

        for mino1 in nmino.iter() {
            for mino2 in nmino.iter() {
                for x in (-(mino1.width as i32))..((mino2.width+mino1.width+1) as i32) {
                    for y in (-(mino1.height as i32))..((mino2.height+mino1.height+1) as i32) {
                        let mino = concat(x, y, mino1, mino2);
                        let mut vps = mino.ps.clone().into_iter().collect::<Vec<_>>();
                        vps.sort();

                        if !connected(&mino) || mino.ps.len() != mino_size*2 || used.contains(&vps) {
                            continue;
                        }

                        used.insert(vps);

                        show_mino(&mino);
                        println!("---------------->");
                        println!("{:?}", mino);
                        println!("----------------");

                        res[mino_size].push(mino);
                    }
                }
            }
        }

        mino_size += 1;
    }

    res
}

fn concat(x: i32, y: i32, mino1: &Mino, mino2: &Mino) -> Mino {
    #[derive(Debug, PartialEq, Eq, Hash)]
    struct IP {
        x: i32,
        y: i32
    }

    let mut ips: HashSet<IP> = vec![].into_iter().collect();

    for p in mino1.ps.iter() {
        ips.insert(IP {
            x: p.x as i32,
            y: p.y as i32
        });
    }

    let mut min_x = 0;
    let mut min_y = 0;

    for p in mino2.ps.iter() {
        ips.insert(IP {
            x: (p.x as i32)-x,
            y: (p.y as i32)-y
        });

        min_x = std::cmp::min(min_x, (p.x as i32)-x);
        min_y = std::cmp::min(min_y, (p.y as i32)-y);
    }

    let mut ps: HashSet<P> = vec![].into_iter().collect();
    let mut width = 0;
    let mut height = 0;

    for ip in ips.iter() {
        ps.insert(P {
            x: (ip.x-min_x) as usize,
            y: (ip.y-min_y) as usize
        });

        width = std::cmp::max(width, (ip.x-min_x+1) as usize);
        height = std::cmp::max(height, (ip.y-min_y+1) as usize);
    }

    let res = Mino {
        ps: ps,
        width: width,
        height: height
    };
    
    res
}

fn dfs(index: usize, ps: &mut Vec<P>, width: usize, height: usize) {
    let p = ps[index].clone();
    let dx: Vec<i32> = vec![0, -1, 1, 0];
    let dy: Vec<i32> = vec![-1, 0, 0, 1];

    ps.remove(index);

    for i in 0..4 {
        let x = (p.x as i32) + dx[i];
        let y = (p.y as i32) + dy[i];

        if x < 0 || x >= (width as i32) || y < 0 || y >= (height as i32) {

            continue;
        }
        let res = ps.clone().into_iter().position(|e| e.x == (x as usize) && e.y == (y as usize));

        match res {
            Some(next_index) => dfs(next_index, ps, width, height),
            None => {}
        }
    }
}

fn connected(mino: &Mino) -> bool {
    let mut cur = mino.clone();

    let mut ps = cur.ps.into_iter().collect::<Vec<_>>();

    dfs(0, &mut ps, cur.width, cur.height);

    ps.len() == 0
}

fn show_mino(mino: &Mino) {
    let spaces = (0..mino.width).map(|_| ' ').collect::<Vec<char>>();

    let mut res: Vec<Vec<char>> = vec![spaces; mino.height];

    for p in mino.ps.iter() {
        res[p.y][p.x] = '#';
    }

    for line in &res {
        println!("{}", line.into_iter().collect::<String>());
    }
}

fn show_grp_mino(grps: &Vec<Vec<Mino>>) {
    let mut sum = 0;

    for i in 1..grps.len() {
        println!("{}: {}", i, grps[i].len());
        sum += grps[i].len();
    }

    println!("{}", sum);
}

fn tile(state: Vec<bool>, flds: Vec<Field>) {
    
}

fn main() {
    let n = 5;
    let xs = enum_polyomino(n);
    let mut grps: Vec<Vec<Mino>> = vec![Vec::<Mino>::new(); n+1];

    for mino in &xs {
        //show_mino(mino);
        //println!("-------------");

        grps[mino.ps.len()].push(mino.clone());
    }

    show_grp_mino(&grps);

    let dgrps = duplicate(&grps);

    show_grp_mino(&dgrps);

    //println!("{:?}", xs);
}
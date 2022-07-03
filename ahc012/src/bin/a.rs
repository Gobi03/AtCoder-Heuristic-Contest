#[allow(unused_imports)]
use proconio::marker::{Chars, Isize1, Usize1};
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::Write;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
use std::time::SystemTime;

#[allow(dead_code)]
const LIMIT_TIME: usize = 3_000; // ms
const K: usize = 100; // カット数
const RADIUS: isize = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}
#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Self {
        Coord { x: p.0, y: p.1 }
    }
    fn from_usize_pair(p: (usize, usize)) -> Self {
        Coord {
            x: p.0 as isize,
            y: p.1 as isize,
        }
    }

    // fn _in_field(&self) -> bool {
    //     (0 <= self.x && self.x < SIDE as isize) && (0 <= self.y && self.y < H as isize)
    // }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    // マンハッタン距離
    fn distance(&self, that: &Self) -> isize {
        (self.x - that.x).abs() + (self.y - that.y).abs()
    }

    // 四則演算
    fn plus(&self, that: &Self) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Self) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn access_matrix<'a, T>(&'a self, mat: &'a Vec<Vec<T>>) -> &'a T {
        &mat[self.y as usize][self.x as usize]
    }

    fn set_matrix<T>(&self, mat: &mut Vec<Vec<T>>, e: T) {
        mat[self.y as usize][self.x as usize] = e;
    }

    // user define
    fn to_str(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}

// => (b, max_piece)
fn make_b(input: &Input, out: &Vec<(Coord, Coord)>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    // ケーキのビースに乗ってる苺の番号一覧の一覧
    let mut pieces = vec![(0..input.n).collect::<Vec<_>>()];
    for (p, q) in out {
        let Coord { x: px, y: py } = p;
        let Coord { x: qx, y: qy } = q;

        let mut new_pieces = vec![];
        // 各ピースを切断面で二分割
        for piece in pieces {
            let mut left = vec![];
            let mut right = vec![];
            for j in piece {
                let (x, y) = input.xy[j];
                let side = (qx - px) as i64 * (y - py) as i64 - (qy - py) as i64 * (x - px) as i64;
                if side > 0 {
                    left.push(j);
                } else if side < 0 {
                    right.push(j);
                }
            }
            if left.len() > 0 {
                new_pieces.push(left);
            }
            if right.len() > 0 {
                new_pieces.push(right);
            }
        }
        pieces = new_pieces;
    }

    // 年数毎にカウント
    let mut b = vec![0; 10];
    for piece in &pieces {
        if piece.len() <= 10 {
            b[piece.len() - 1] += 1;
        }
    }

    pieces.sort_by_key(|e| -(e.len() as isize));
    (b, pieces[0].clone(), pieces[1].clone())
}

fn compute_score(input: &Input, b: &Vec<usize>) -> isize {
    // スコア計算
    let mut num = 0;
    let mut den = 0;
    for d in 0..10 {
        num += input.a[d].min(b[d]);
        den += input.a[d];
    }
    let score = (1e6 * num as f64 / den as f64).round() as isize;

    score
}

fn compute_shortage(input: &Input, b: &Vec<usize>) -> isize {
    let mut shortage = 0;
    for d in 0..10 {
        if b[d] < input.a[d] {
            shortage += input.a[d] - b[d];
        }
    }

    shortage as isize
}

struct Input {
    n: usize,
    a: Vec<usize>,
    xy: Vec<(isize, isize)>,
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();
    let mut rng = thread_rng();

    input! {
        n: usize, // マックス5,500?
        _: usize,
        a: [usize; 10], // [1, 100]
        xy: [(isize, isize); n],
    }

    let input = Input { n, a, xy };

    let mut best_score = 10000000000;
    let mut best_out = vec![];

    while system_time.elapsed().unwrap().as_millis() < (LIMIT_TIME - 100) as u128 {
        let mut out = Vec::with_capacity(K);
        for i in 0..=K {
            if i == 0 {
                let px = rng.gen_range(-RADIUS, RADIUS);
                let py = rng.gen_range(-RADIUS, RADIUS);
                let qx = rng.gen_range(-RADIUS, RADIUS);
                let qy = rng.gen_range(-RADIUS, RADIUS);
                let p = Coord::new((px, py));
                let q = Coord::new((qx, qy));

                out.push((p, q));
            } else {
                let (b, piece_a, piece_b) = make_b(&input, &out);
                let score = compute_shortage(&input, &b);
                if score < best_score {
                    // ベストスコアの更新
                    best_score = score;
                    best_out = out.clone();
                }

                if i == K {
                    break;
                }

                let p = central_point(&input, &piece_a);
                let q = central_point(&input, &piece_b);

                out.push((p, q))
            }

            if system_time.elapsed().unwrap().as_millis() > (LIMIT_TIME - 100) as u128 {
                break;
            }
        }
    }

    // output
    print_out(&best_out);

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

fn print_out(out: &Vec<(Coord, Coord)>) {
    println!("{}", out.len());
    for (p, q) in out {
        println!("{} {}", p.to_str(), q.to_str());
    }
}

fn make_random_2p(rng: &mut ThreadRng) -> (Coord, Coord) {
    let px = rng.gen_range(-RADIUS, RADIUS);
    let py = rng.gen_range(-RADIUS, RADIUS);
    let qx = rng.gen_range(-RADIUS, RADIUS);
    let qy = rng.gen_range(-RADIUS, RADIUS);
    let p = Coord::new((px, py));
    let q = Coord::new((qx, qy));

    (p, q)
}

fn central_point(input: &Input, piece: &Vec<usize>) -> Coord {
    let mut acc_x = 0;
    let mut acc_y = 0;
    for n in piece {
        let (x, y) = input.xy[*n];

        acc_x += x;
        acc_y += y;
    }

    let nx = (acc_x as f64 / piece.len() as f64).round() as isize;
    let ny = (acc_y as f64 / piece.len() as f64).round() as isize;

    Coord::new((nx, ny))
}

fn eval(input: &Input, b: &Vec<usize>) -> f64 {
    let mut res = 0.0;
    for d in 3..10 {
        if b[d] < input.a[d] {
            res += (input.a[d] - b[d]) as f64 * (d - 2) as f64 * 0.3;
        }
    }

    res
}

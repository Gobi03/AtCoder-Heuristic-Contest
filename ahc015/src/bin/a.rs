#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(unused_imports)]
use rand::rngs::ThreadRng;
#[allow(unused_imports)]
use rand::seq::SliceRandom;
#[allow(unused_imports)]
use rand::{thread_rng, Rng};
#[allow(unused_imports)]
use std::io::Write;
use std::time::SystemTime;

macro_rules! mat {
	($($e:expr),*) => { Vec::from(vec![$($e),*]) };
	($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
	($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
	($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

const TURN: usize = 100;
const N: usize = 10;
const CANDY_KIND: usize = 3;
const DIRS: [char; 4] = ['F', 'B', 'L', 'R'];
const DIJ: [(usize, usize); 4] = [(1, 0), (0, 1), (!0, 0), (0, !0)];

const LAST_SEARCH_TURN: usize = 93;

struct Input {
    fs: Vec<usize>,
    candy_nums: Vec<usize>,
    ps: Vec<usize>,
}
impl Input {
    fn new(f: Vec<usize>) -> Self {
        let mut candy_nums: Vec<usize> = vec![0; CANDY_KIND + 1];
        for e in &f {
            candy_nums[*e] += 1;
        }

        Self {
            fs: f,
            candy_nums,
            ps: vec![],
        }
    }

    // 一番数が多いキャンディの種類番号を返す
    fn max_num_candy(&self) -> usize {
        let mut hoge = 0;
        let mut res = 0;

        for i in 1..=CANDY_KIND {
            if hoge < self.candy_nums[i] {
                hoge = self.candy_nums[i];
                res = i;
            }
        }

        res
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub board: Vec<Vec<usize>>,
    pub t: usize,
}

// 打ち手を返す
fn simulate(st: &State, input: &Input) -> char {
    let mut res = 'c';
    let mut best = 0;

    for &dir in &DIRS {
        let mut next_st = st.clone();
        next_st.apply_move(dir);

        let hoge = simulate_put(next_st, input);
        if hoge > best {
            best = hoge;
            res = dir;
        }
    }

    res
}
fn simulate_put(st: State, input: &Input) -> usize {
    // 全置き方で進めて、最も悪い値を返す
    let mut acc = std::usize::MAX;
    for p in 1..=(TURN - st.t) {
        let mut next_st = st.clone();
        next_st.apply_put(p, input);
        let hoge = simulate_move(next_st, input);
        acc += hoge;
    }

    acc / (TURN - st.t)
}
fn simulate_move(st: State, input: &Input) -> usize {
    if st.t == TURN - 1 {
        st.compute_score()
    } else {
        // 4パターンのムーブを試して、一番良い値を返す
        let mut best = 0;
        for &dir in &DIRS {
            let mut next_st = st.clone();
            let _ = next_st.apply_move(dir);

            let hoge = simulate_put(next_st, input);
            if hoge > best {
                best = hoge;
            }
        }

        best
    }
}

impl State {
    fn new() -> Self {
        let board = mat![0; N; N];

        Self { board, t: 0 }
    }

    fn apply_put(&mut self, p: usize, input: &Input) {
        // 入力お菓子の置き場所の検索
        let mut cnt = 0;
        for i in 0..N {
            for j in 0..N {
                if self.board[i][j] == 0 {
                    cnt += 1;
                    if cnt == p {
                        self.board[i][j] = input.fs[self.t];
                    }
                }
            }
        }
    }

    fn apply_move(&mut self, dir: char) -> Result<(), String> {
        match dir {
            'L' => {
                for i in 0..N {
                    let mut k = 0;
                    for j in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'R' => {
                for i in 0..N {
                    let mut k = N - 1;
                    for j in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[i][k] = self.board[i][j];
                            if k != j {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            'F' => {
                for j in 0..N {
                    let mut k = 0;
                    for i in 0..N {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k += 1;
                        }
                    }
                }
            }
            'B' => {
                for j in 0..N {
                    let mut k = N - 1;
                    for i in (0..N).rev() {
                        if self.board[i][j] != 0 {
                            self.board[k][j] = self.board[i][j];
                            if k != i {
                                self.board[i][j] = 0;
                            }
                            k -= 1;
                        }
                    }
                }
            }
            _ => {
                return Err(format!("Illegal output: {}", dir));
            }
        }

        self.t += 1;

        Ok(())
    }

    // 分子部分の算出
    fn compute_score(&self) -> usize {
        // スコア計算 分子部分。
        let mut visited = mat![false; N; N];
        let mut num = 0;
        for i in 0..N {
            for j in 0..N {
                if !visited[i][j] && self.board[i][j] != 0 {
                    visited[i][j] = true;
                    let c = self.board[i][j];
                    let mut size = 1;
                    let mut stack = vec![(i, j)];
                    while let Some((i, j)) = stack.pop() {
                        for &(di, dj) in &DIJ {
                            let i2 = i + di;
                            let j2 = j + dj;
                            if i2 < N && j2 < N && !visited[i2][j2] && self.board[i2][j2] == c {
                                visited[i2][j2] = true;
                                stack.push((i2, j2));
                                size += 1;
                            }
                        }
                    }
                    num += size * size;
                }
            }
        }

        num
    }
}

fn main() {
    let system_time = SystemTime::now();
    let mut _rng = thread_rng();

    // tool
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    // input
    let f = (0..TURN).map(|_| sc.read::<usize>()).collect::<Vec<_>>();
    let mut input = Input::new(f);

    let mut st = State::new();

    /* action */
    // 右に寄せる対象
    let target_candy = input.max_num_candy();
    let second = (target_candy % CANDY_KIND) + 1;
    let _third = (second % CANDY_KIND) + 1;
    let mut ok_flag = true; // 上下動しても良し

    for ti in 0..TURN {
        let p: usize = sc.read();
        input.ps.push(p);

        st.apply_put(p, &input);

        let ans = if ti <= LAST_SEARCH_TURN {
            if input.fs[ti + 1] == target_candy {
                ok_flag = false;
                'L'
            } else {
                if !ok_flag {
                    ok_flag = true;
                    'R'
                }
                // 上下動させる
                else if input.fs[ti + 1] == second {
                    'F'
                } else {
                    'B'
                }
            }
        }
        // なに返しても一緒
        else if ti == TURN - 1 {
            'R' // dummy
        }
        // 探索
        else {
            simulate(&st, &input)
        };

        let _ = st.apply_move(ans);

        println!("{}", ans);
    }

    // eprintln!("{}", st.compute_score());

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

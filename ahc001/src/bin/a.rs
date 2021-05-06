#[allow(unused_imports)]
use proconio::marker::Chars;
use proconio::{fastout, input};

#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;

use rand::{thread_rng, Rng};
use std::time::SystemTime;

const TIMEOUT_MS: u128 = 5_000;
const SIDE: usize = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl Coord {
    fn new(p: (isize, isize)) -> Coord {
        Coord { x: p.0, y: p.1 }
    }

    fn in_field(pos: Self) -> bool {
        (0 <= pos.x && pos.x <= SIDE as isize) && (0 <= pos.y && pos.y <= SIDE as isize)
    }

    // ペアへの変換
    fn to_pair(&self) -> (isize, isize) {
        (self.x as isize, self.y as isize)
    }

    // マンハッタン距離
    fn distance(&self, that: Coord) -> isize {
        let dist_x = max(self.x, that.x) - min(self.x, that.x);
        let dist_y = max(self.y, that.y) - min(self.y, that.y);
        dist_x + dist_y
    }

    // 四則演算
    fn plus(&self, that: &Coord) -> Self {
        Coord::new((self.x + that.x, self.y + that.y))
    }
    fn minus(&self, that: &Coord) -> Self {
        Coord::new((self.x - that.x, self.y - that.y))
    }

    fn mk_4dir(&self) -> Vec<Self> {
        let (ix, iy) = self.to_pair();
        let delta = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        delta
            .iter()
            .map(|&(dx, dy)| Coord::new((ix + dx, iy + dy)))
            .filter(|&pos| Coord::in_field(pos))
            .collect()
    }
}

struct Request {
    spot: Coord,
    area: usize,
}

struct Input {
    n: usize,
    requests: Vec<Request>, // (希望座標, 希望サイズ)
}
impl Input {
    fn new(n: usize, xyr: Vec<(usize, usize, usize)>) -> Input {
        let mut requests = Vec::new();
        for (x, y, r) in xyr {
            requests.push(Request {
                spot: Coord::new((x as isize, y as isize)),
                area: r,
            });
        }
        Input { n, requests }
    }
}

enum RectSide {
    Width,
    Height,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rectangle {
    leftup: Coord,
    rightdown: Coord,
}

impl Rectangle {
    fn new(leftup: Coord, rightdown: Coord) -> Self {
        Rectangle { leftup, rightdown }
    }

    fn calc_area(&self) -> isize {
        (self.rightdown.x - self.leftup.x) * (self.rightdown.y - self.leftup.y)
    }

    fn in_field(&self) -> bool {
        self.leftup.x >= 0
            && self.leftup.y >= 0
            && self.rightdown.x <= SIDE as isize
            && self.rightdown.y <= SIDE as isize
    }

    fn is_valid(&self) -> bool {
        self.in_field() && self.leftup.x < self.rightdown.x && self.leftup.y < self.rightdown.y
    }

    // 四則演算
    fn plus(&self, delta: &Self) -> Self {
        Self::new(
            Coord::new((
                self.leftup.x + delta.leftup.x,
                self.leftup.y + delta.leftup.y,
            )),
            Coord::new((
                self.rightdown.x + delta.rightdown.x,
                self.rightdown.y + delta.rightdown.y,
            )),
        )
    }

    fn minus(&self, delta: &Self) -> Self {
        Self::new(
            Coord::new((
                self.leftup.x - delta.leftup.x,
                self.leftup.y - delta.leftup.y,
            )),
            Coord::new((
                self.rightdown.x - delta.rightdown.x,
                self.rightdown.y - delta.rightdown.y,
            )),
        )
    }

    fn does_include_point(&self, point: &Coord) -> bool {
        let &Coord { x, y } = point;
        self.leftup.x <= x && x < self.rightdown.x && self.leftup.y <= y && y < self.rightdown.y
    }

    fn does_include_rect(&self, that: &Rectangle) -> bool {
        let in_x_overwrapped = self.leftup.x < that.rightdown.x && self.rightdown.x > that.leftup.x;
        let in_y_overwrapped = self.leftup.y < that.rightdown.y && self.rightdown.y > that.leftup.y;
        in_x_overwrapped && in_y_overwrapped
    }

    fn calc_score(&self, id: usize, input: &Input) -> f64 {
        let request = &input.requests[id];
        if self.does_include_point(&request.spot) {
            let area = self.calc_area() as f64;
            let score = 1.0
                - (1.0 - (request.area as f64).min(area) / (request.area as f64).max(area)).powi(2);
            score
        } else {
            0.0
        }
    }

    // 長辺を返す。正方形ならNoneを返す。
    fn long_side(&self) -> Option<RectSide> {
        let w = self.rightdown.x - self.leftup.x;
        let h = self.rightdown.y - self.leftup.y;
        if w > h {
            Some(RectSide::Width)
        } else if w < h {
            Some(RectSide::Height)
        } else {
            None
        }
    }
}

// 一方向に1広げることを表現する
enum Expander {
    ToLeft,
    ToUp,
    ToRight,
    ToDown,
}

impl Expander {
    fn pushed_delta(&self) -> Rectangle {
        match self {
            Self::ToLeft => Rectangle::new(Coord::new((0, 0)), Coord::new((-1, 0))),
            Self::ToUp => Rectangle::new(Coord::new((0, 0)), Coord::new((0, -1))),
            Self::ToRight => Rectangle::new(Coord::new((1, 0)), Coord::new((0, 0))),
            Self::ToDown => Rectangle::new(Coord::new((0, 1)), Coord::new((0, 0))),
        }
    }

    fn push_delta(&self) -> Rectangle {
        match self {
            Self::ToLeft => Rectangle::new(Coord::new((-1, 0)), Coord::new((0, 0))),
            Self::ToUp => Rectangle::new(Coord::new((0, -1)), Coord::new((0, 0))),
            Self::ToRight => Rectangle::new(Coord::new((0, 0)), Coord::new((1, 0))),
            Self::ToDown => Rectangle::new(Coord::new((0, 0)), Coord::new((0, 1))),
        }
    }

    fn make_prob_overwrapped<'a>(&self, output: &'a Output, rect: &Rectangle) -> &'a Vec<usize> {
        match self {
            Expander::ToLeft => &output.rightside_hash[rect.leftup.x as usize],
            Expander::ToUp => &output.bottomside_hash[rect.leftup.y as usize],
            Expander::ToRight => &output.leftside_hash[rect.rightdown.x as usize],
            Expander::ToDown => &output.topside_hash[rect.rightdown.y as usize],
        }
    }

    fn elems() -> [Self; 4] {
        [
            Expander::ToLeft,
            Expander::ToUp,
            Expander::ToRight,
            Expander::ToDown,
        ]
    }
}

struct Output {
    input: Input,
    results: Vec<Rectangle>, // (leftup, rightdown)
    score: f64,
    // 辺軸からid列へのハッシュを持つ
    leftside_hash: Vec<Vec<usize>>,
    topside_hash: Vec<Vec<usize>>,
    rightside_hash: Vec<Vec<usize>>,
    bottomside_hash: Vec<Vec<usize>>,
}

impl Output {
    fn new(input: Input) -> Self {
        let mut results = Vec::new();

        let mut leftside_hash = vec![Vec::new(); SIDE + 1];
        let mut topside_hash = vec![Vec::new(); SIDE + 1];
        let mut rightside_hash = vec![Vec::new(); SIDE + 1];
        let mut bottomside_hash = vec![Vec::new(); SIDE + 1];

        // 希望地点に面積1で置く
        for (id, req) in input.requests.iter().enumerate() {
            let Request { spot: pos, area: _ } = req;
            let area1_rect = Rectangle::new(pos.clone(), pos.clone().plus(&Coord::new((1, 1))));
            results.push(area1_rect);

            leftside_hash[area1_rect.leftup.x as usize].push(id);
            topside_hash[area1_rect.leftup.y as usize].push(id);
            rightside_hash[area1_rect.rightdown.x as usize].push(id);
            bottomside_hash[area1_rect.rightdown.y as usize].push(id);
        }

        let mut output = Output {
            input,
            results,
            score: 0.0,
            leftside_hash,
            topside_hash,
            rightside_hash,
            bottomside_hash,
        };
        for id in 0..output.input.n {
            output.score += output.get_current_score(id);
        }

        // リクエストサイズを超えるところまで、正方形型に広げていく
        let mut flag = true;
        let dirs = Expander::elems();
        let mut di = 0;
        while flag {
            flag = false;
            let dir = &dirs[di];
            let delta = dir.push_delta();

            for id in 0..output.input.n {
                let rect = output.results[id];
                if rect.calc_area() < output.input.requests[id].area as isize {
                    let new_rect = rect.plus(&delta);

                    if !new_rect.in_field() {
                        continue;
                    }

                    let prob_overwrapped = dir.make_prob_overwrapped(&output, &rect);
                    let overwrapped = prob_overwrapped
                        .iter()
                        .any(|&i| output.results[i].does_include_rect(&new_rect));
                    if !overwrapped {
                        if !flag {
                            flag = true;
                        }
                        output.update_results(id, new_rect);
                    }
                }
            }

            di += 1;
            di %= 4;
        }

        output
    }

    fn remove_item<T: Eq>(v: &mut Vec<T>, e: &T) {
        let index = v.iter().position(|elem| *elem == *e).unwrap();
        v.remove(index);
    }

    // 正規のアップデートかのチェックはされてる前提
    // todo: 座標からidへのマッピングを更新
    fn update_results(&mut self, id: usize, next_rect: Rectangle) {
        let cur_rect = self.results[id];

        self.score -= self.results[id].calc_score(id, &self.input);
        self.score += next_rect.calc_score(id, &self.input);
        self.results[id] = next_rect;

        Self::remove_item(&mut self.leftside_hash[cur_rect.leftup.x as usize], &id);
        self.leftside_hash[next_rect.leftup.x as usize].push(id);
        Self::remove_item(&mut self.topside_hash[cur_rect.leftup.y as usize], &id);
        self.topside_hash[next_rect.leftup.y as usize].push(id);
        Self::remove_item(&mut self.rightside_hash[cur_rect.rightdown.x as usize], &id);
        self.rightside_hash[next_rect.rightdown.x as usize].push(id);
        Self::remove_item(
            &mut self.bottomside_hash[cur_rect.rightdown.y as usize],
            &id,
        );
        self.bottomside_hash[next_rect.rightdown.y as usize].push(id);
    }

    fn calc_game_score(&self) -> usize {
        (self.score / self.input.n as f64 * 1e9).round() as usize
    }

    fn get_current_score(&self, id: usize) -> f64 {
        self.results[id].calc_score(id, &self.input)
    }

    // return (other_changes, score_diff). 押し広げられない場合はNone
    fn expand(
        &self,
        cur_rect: &Rectangle,
        next_rect: &Rectangle,
        expander: Expander,
    ) -> Option<(Vec<(usize, Rectangle)>, f64)> {
        let prob_overwrapped = expander.make_prob_overwrapped(&self, &cur_rect);

        let mut other_changes = Vec::<(usize, Rectangle)>::new(); // (id, next_rect)
        let mut score_diff = 0.0;

        for &i in prob_overwrapped {
            let other = self.results[i];
            if other.does_include_rect(&next_rect) {
                let next_other = other.plus(&expander.pushed_delta());
                // 押し広げた結果、不正な長方形を生まないか？
                if !next_other.is_valid() {
                    return None;
                }
                score_diff +=
                    next_other.calc_score(i, &self.input) - other.calc_score(i, &self.input);
                other_changes.push((i, next_other));
            }
        }

        Some((other_changes, score_diff))
    }
}

fn format_result_row(rect: &Rectangle) -> String {
    let Rectangle { leftup, rightdown } = rect;
    format!("{} {} {} {}", leftup.x, leftup.y, rightdown.x, rightdown.y)
}

fn print_result_row(rect: &Rectangle) {
    println!("{}", format_result_row(&rect));
}

fn annealing(output: &mut Output, system_time: &SystemTime) {
    // 開始温度(スコア差の最大値にすると良さそう。開始直後に35%くらいの確率でこの差量を受け入れる)
    let start_temp: f64 = 1e-3;
    // 終了温度(終盤に悪化遷移を35%程度許容できる値にすると良さそう)
    let end_temp: f64 = 5e-6;

    let strat_time = system_time.elapsed().unwrap().as_millis();
    let tl: f64 = ((TIMEOUT_MS - strat_time) - 4100) as f64; // 焼きなまし時間(ミリ秒)

    let mut rng = thread_rng();

    let mut temp;
    // 初期値をセット
    let mut best_score = output.score;
    let mut best_out = output.results.clone();

    const LOOP_NUM: usize = 1000;
    let mut loop_cnt = 0;

    loop {
        let spent_time_rate = (system_time.elapsed().unwrap().as_millis() - strat_time) as f64 / tl; // (0.0, 1.0)
        if spent_time_rate >= 1.0 {
            break;
        }
        // 温度。段々下がっていく。
        temp = start_temp + (end_temp - start_temp) * spent_time_rate;

        for _ in 0..LOOP_NUM {
            let id = rng.gen_range(0, output.input.n);
            let rect = &output.results[id];
            let cur_score = rect.calc_score(id, &output.input);

            /* 変更処理の実行 */
            if rng.gen_bool(1.0) {
                /* 一辺を動かす */
                let direction = rng.gen_range(0, 2); // leftup, rightdown

                let next_rect = if direction == 0 {
                    let next_reprs = rect.leftup.mk_4dir();
                    let i = rng.gen_range(0, next_reprs.len());
                    Rectangle::new(next_reprs[i], rect.rightdown)
                } else {
                    let next_reprs = rect.rightdown.mk_4dir();
                    let i = rng.gen_range(0, next_reprs.len());
                    Rectangle::new(rect.leftup, next_reprs[i])
                };

                if !next_rect.is_valid() {
                    continue;
                }

                let expander = {
                    if next_rect.leftup.x == rect.leftup.x - 1 {
                        Some(Expander::ToLeft)
                    } else if next_rect.leftup.y == rect.leftup.y - 1 {
                        Some(Expander::ToUp)
                    } else if next_rect.rightdown.x == rect.rightdown.x + 1 {
                        Some(Expander::ToRight)
                    } else if next_rect.rightdown.y == rect.rightdown.y + 1 {
                        Some(Expander::ToDown)
                    } else {
                        None
                    }
                };

                let next_score = next_rect.calc_score(id, &output.input);
                let mut score_diff = next_score - cur_score;

                let mut other_changes = Vec::new(); // (id, next_rect)

                if let Some(expander) = expander {
                    match output.expand(rect, &next_rect, expander) {
                        None => continue,
                        Some((o_changes, diff)) => {
                            other_changes = o_changes;
                            score_diff += diff;
                        }
                    };
                }

                // スコアが増すか、`e^(score差 / T)` の確率にヒットしたら
                // `score差` が負の数なのが肝
                if score_diff > 0.0 || rng.gen_bool(f64::exp(score_diff / temp)) {
                    output.update_results(id, next_rect);
                    for (i, other) in other_changes {
                        output.update_results(i, other);
                    }
                }
            } else {
                /* 短い辺を縮めて、長い辺を伸ばす */
                let shrink_dir_b = rng.gen_range(0, 2); // 0: left or up, 1: right or down
                let expand_dir_b = rng.gen_range(0, 2); // 0: left or up, 1: right or down

                let (shrink_dir, expand_dir) = match rect.long_side() {
                    Some(RectSide::Width) | None => (
                        // shrink
                        if shrink_dir_b == 0 {
                            Expander::ToUp
                        } else {
                            Expander::ToDown
                        },
                        // expand
                        if expand_dir_b == 0 {
                            Expander::ToLeft
                        } else {
                            Expander::ToRight
                        },
                    ),
                    Some(RectSide::Height) => (
                        if shrink_dir_b == 0 {
                            Expander::ToLeft
                        } else {
                            Expander::ToRight
                        },
                        if expand_dir_b == 0 {
                            Expander::ToUp
                        } else {
                            Expander::ToDown
                        },
                    ),
                };

                let pre_next_rect = rect.minus(&shrink_dir.push_delta());
                let next_rect = pre_next_rect.plus(&expand_dir.push_delta());
                if !next_rect.is_valid() {
                    continue;
                }

                let mut score_diff = next_rect.calc_score(id, &output.input) - cur_score;
                let other_changes: Vec<(usize, Rectangle)> =
                    match output.expand(&pre_next_rect, &next_rect, expand_dir) {
                        None => {
                            continue;
                        }
                        Some((other_changes, sc_diff)) => {
                            score_diff += sc_diff;
                            other_changes
                        }
                    };

                // todo: DRYにする
                // スコアが増すか、`e^(score差 / T)` の確率にヒットしたら
                // `score差` が負の数なのが肝
                if score_diff > 0.0 || rng.gen_bool(f64::exp(score_diff / temp)) {
                    output.update_results(id, next_rect);
                    for (i, other) in other_changes {
                        output.update_results(i, other);
                    }
                }
            }

            if output.score > best_score {
                // ベストスコアの更新
                best_score = output.score;
                best_out = output.results.clone();
            }
        }

        loop_cnt += 1;

        // 出力
        for res in &output.results {
            print_result_row(&res);
        }
    }

    eprintln!("{} * {}回ループ", loop_cnt, LOOP_NUM);

    output.results = best_out;
    output.score = best_score;
}

#[fastout]
fn main() {
    let system_time = SystemTime::now();

    input! {
        n: usize,
        xyr: [(usize, usize, usize); n],
    }

    let input = Input::new(n, xyr);
    let mut output = Output::new(input);

    eprintln!(
        "start annealing at {}ms.",
        system_time.elapsed().unwrap().as_millis()
    );
    annealing(&mut output, &system_time);

    eprintln!("{}", output.score / output.input.n as f64);

    // 出力
    for res in output.results {
        print_result_row(&res);
    }

    eprintln!("{}ms", system_time.elapsed().unwrap().as_millis());
}

use std::fs;
use std::io::Write;

#[derive(Clone)]
struct Graphics {
	screen_w: f64,
	screen_h: f64,

	data: String,

	sr: f64,
	sg: f64,
	sb: f64,
	sa: f64,

	fr: f64,
	fg: f64,
	fb: f64,
	fa: f64,
}

#[allow(dead_code)]
impl Graphics {
	fn new() -> Self {
		Self {
			screen_w: 1.0,
			screen_h: 1.0,
			data: String::from(""),
			sr: 0.0,
			sg: 0.0,
			sb: 0.0,
			sa: 0.0,
			fr: 1.0,
			fg: 1.0,
			fb: 1.0,
			fa: 1.0,
		}
	}

	fn screen(&mut self, width: f64, height: f64) {
		self.screen_w = width;
		self.screen_h = height;
	}

	fn clear(&mut self) {
		self.data = String::from("");
	}

	fn stroke_rgb(&mut self, r: f64, g: f64, b: f64) {
		self.stroke_rgba(r, g, b, 1.0)
	}

	fn stroke_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
		self.sr = r;
		self.sg = g;
		self.sb = b;
		self.sa = a;
	}

	fn no_stroke(&mut self) {
		self.stroke_rgba(0.0, 0.0, 0.0, 0.0)
	}

	fn fill_rgb(&mut self, r: f64, g: f64, b: f64) {
		self.fill_rgba(r, g, b, 1.0)
	}

	fn fill_rgba(&mut self, r: f64, g: f64, b: f64, a: f64) {
		self.fr = r;
		self.fg = g;
		self.fb = b;
		self.fa = a;
	}

	fn no_fill(&mut self) {
		self.fill_rgba(0.0, 0.0, 0.0, 0.0)
	}

	fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
		self.data = format!(
			"{}{}",
			self.data,
			format!(
				"<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" {}/>\n",
				x1,
				y1,
				x2,
				y2,
				self.stroke()
			)
		)
	}

	fn rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
		self.data = format!(
			"{}{}",
			self.data,
			format!(
				"<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" {} {}/>\n",
				x,
				y,
				w,
				h,
				self.stroke(),
				self.fill()
			)
		)
	}

	fn text(&mut self, str: String, x: f64, y: f64, size: f64) {
		self.data =
			format!(
				"{}{}",
				self.data,
				format!(
				"<text text-anchor=\"middle\" x=\"{}\" y=\"{}\" font-size=\"{}\" {} >{}</text>\n",
				x, y, size, self.fill(), str
			)
			)
	}

	fn dump(
		&self,
		id: Option<String>,
		style: Option<String>,
		width_px: Option<usize>,
		height_px: Option<isize>,
	) -> String {
		let mut res = String::from("<svg ");
		if let Some(id) = id {
			res = format!("{}{}", res, format!("id=\"{}\" ", id))
		};
		if let Some(style) = style {
			res = format!("{}{}", res, format!("style=\"{}\" ", style));
		};
		if let Some(width_px) = width_px {
			res = format!("{}{}", res, format!("width_px=\"{}\" ", width_px));
		};
		if let Some(height_px) = height_px {
			res = format!("{}{}", res, format!("height_px=\"{}\" ", height_px));
		};

		let s = format!(
			"viewBox=\"-1 -1 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n{}</svg>",
			self.screen_w + 2.0,
			self.screen_h + 2.0,
			self.data
		);
		res = format!("{}{}", res, s);

		res
	}

	fn stroke(&self) -> String {
		format!(
			"stroke=\"{}\" stroke-opacity=\"{}\"",
			Self::rgb(self.sr, self.sg, self.sb),
			self.sa
		)
	}

	fn fill(&self) -> String {
		format!(
			"fill=\"{}\" fill-opacity=\"{}\"",
			Self::rgb(self.fr, self.fg, self.fb),
			self.fa
		)
	}

	fn rgb(r: f64, g: f64, b: f64) -> String {
		format!(
			"rgb({},{},{})",
			(r * 255.0).round(),
			(g * 255.0).round(),
			(b * 255.0).round()
		)
	}
}

struct Movie {
	svgs: Vec<String>,
}

#[allow(dead_code)]
impl Movie {
	fn new() -> Self {
		Self { svgs: vec![] }
	}

	fn clear(&mut self) {
		self.svgs.clear();
	}

	fn add_frame(&mut self, g: Graphics) {
		self.svgs.push(g.dump(
			Some(format!("f{}", self.svgs.len())),
			Some(String::from(
				"display:none;pointer-events:none;user-select:none;",
			)),
			None,
			None,
		))
	}

	// 相対パスでも可
	fn add_file(&mut self, file_path: String) {
		self.svgs.push(format!(
			"<img id=\"f{}\" style=\"display:none;pointer-events:none;user-select:none;\" src=\"{}\" width=\"1000\" height=\"1000\">",
			self.svgs.len(),
			file_path
		))
	}

	fn dump_html(&self, fps: usize) -> String {
		let mut s: String = String::from("<html><body><div id=\"text\">loading...</div>\n");

		// SVG の挿入
		for svg in &self.svgs {
			s += svg;
		}

		let t = format!(
			"<script>
				let numFrames = {}, fps = {};",
			self.svgs.len(),
			fps
		);
		s = format!("{}\n{}", s, t);

		let hoge = "
			let text = document.getElementById(\"text\");
			let frames = [];
			for (let i = 0; i < numFrames; i++) {
				let f = document.getElementById(\"f\" + i);
				frames.push(f);
				f.style.display = \"none\";
			}
			let currentFrame = 0;
			let playing = true;
			setInterval(() => {
				if (!playing) return;
				text.innerText = (currentFrame + 1) + \" / \" + numFrames;
				frames[(currentFrame - 1 + numFrames) % numFrames].style.display = \"none\";
				frames[currentFrame].style.display = null;
				currentFrame = (currentFrame + 1) % numFrames;
				if (currentFrame == 0) playing = false;
			}, 1000 / fps);
			window.onmousedown = e => { if (e.button == 0) playing = true; };
			</script></body></html>\n";

		s += hoge;

		s
	}
}

fn main() {
	let mut mov = Movie::new();
	// mov.add_file(String::from("tools/out.svg"));

	let mut g = Graphics::new();
	let (w, h) = (500.0, 300.0);
	let mut x = 100.0;
	let mut y = 100.0;
	let mut vx = 20.0;
	let mut vy = 25.0;

	// スクリーンの大きさを設定
	g.screen(w, h);

	for _ in 0..60 * 5 {
		// 画面消去
		g.clear();
		// 枠を黒に、塗りを灰色に
		g.stroke_rgb(0.0, 0.0, 0.0);
		g.fill_rgb(0.5, 0.5, 0.5);
		// 四角を描画
		g.rect(x - 50.0, y - 50.0, 100.0, 100.0);
		// 座標を青で表示
		g.fill_rgb(0.0, 0.0, 1.0);
		g.text(format!("X={}, Y={}", x, y), w / 2.0, 100.0, 16.0);
		// フレームを追加
		mov.add_frame(g.clone());
		// 四角を動かす
		x += vx;
		y += vy;
		// 重力
		vy += 2.0;
		// 壁で跳ね返す
		if x < 50.0 && vx < 0.0 || x > w - 50.0 && vx > 0.0 {
			vx = -vx * 0.8
		};
		if y < 50.0 && vy < 0.0 || y > h - 50.0 && vy > 0.0 {
			vy = -vy * 0.8
		};
	}

	let mut f = fs::File::create("movie.html").unwrap();
	f.write_all(mov.dump_html(60).as_bytes()).unwrap();
}

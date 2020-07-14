use nannou::prelude::*;
use rand::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    sort: Vec<Vec<f32>>,
    rng: ThreadRng,
    min_size: f32,
    max_size: f32,
}

fn model(_app: &App) -> Model {
    _app.new_window().size(900, 600).view(view).build().unwrap();
    let mut m = Model {
        sort: Vec::new(),
        rng: rand::thread_rng(),
        min_size: 5.0,
        max_size: 25.0,
    };

    let n = 20;
    let mut initial = Vec::with_capacity(n);
    for _ in 0..n {
        initial.push(m.rng.gen_range(m.min_size, m.max_size));
    }
    m.sort.push(initial);
    bubble_sort(&mut m.sort);
    println!("{:?}", m.sort.len());
    m
}

// fn update(_app: &App, _model: &mut Model, _update: Update) {
// }

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = _app.draw();
    let w = _app.window_rect().pad(20.0);

    let h_spacing = w.w() / _model.sort.len() as f32;
    for (x_idx, v) in _model.sort.iter().enumerate() {
        let n = v.len();
        let v_spacing = w.h() / n as f32;
        // println!("{}, {}, {}", w.bottom(), n, v_spacing);
        for (y_idx, sz) in v.iter().enumerate() {
            let x = w.left() + (h_spacing / 2.0) + (x_idx as f32 * h_spacing);
            let y = w.bottom() + (v_spacing / 2.0) + (y_idx as f32 * v_spacing);
            let c = map_range(*sz, _model.min_size, _model.max_size, 0.25, 0.75);
            draw.ellipse()
                .x_y(x, y)
                .w_h(*sz, *sz)
                .color(rgb(255.0, c, c));
            // .stroke(BLACK)
            // .stroke_weight(1.5);
        }
    }

    draw.to_frame(_app, &frame).unwrap();
}

fn bubble_sort(vec: &mut Vec<Vec<f32>>) {
    if vec.len() != 1 {
        panic!("vec must contain a single element")
    }
    let mut swapped;
    let l = vec[0].len();
    let mut curr_vec = 0;
    for _ in 0..l {
        swapped = false;
        for i in 0..(l - 1) {
            let mut v = vec[curr_vec].to_vec();
            if v[i] > v[i + 1] {
                let temp = v[i + 1];
                v[i + 1] = v[i];
                v[i] = temp;
                swapped |= true;
            }
            vec.push(v);
            curr_vec += 1;
        }
        if !swapped {
            return;
        }
    }
}

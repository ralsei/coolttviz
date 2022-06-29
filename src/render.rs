use glium::backend::Facade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::DepthTexture2d;
use glium::uniforms::SamplerBehavior;
use glium::*;
use imgui::*;
use imgui_glium_renderer::Texture;
use nalgebra::{Perspective3, Unit};
use std::rc::Rc;

use crate::camera;
use crate::cube;
use crate::label;
use crate::messages;
use crate::{linalg, system};

pub struct Scene {
    pub camera: camera::Camera,

    pub cube: cube::Cube,

    pub program: glium::Program,

    pub dims: Vec<String>,
    pub labels: Vec<label::Label>,
    pub context: String,
}

fn init_scene(display: &Display, msg: &messages::DisplayGoal) -> Scene {
    let camera = camera::Camera::new();

    let program = program!(display, 140 => {
        vertex: include_str!("../resources/shader.vert"),
        fragment: include_str!("../resources/shader.frag")
    })
    .unwrap();

    let labels = msg
        .labels
        .iter()
        .map(|lbl| label::Label::new(&msg.dims, lbl))
        .collect();
    let cube = cube::Cube::new(display, &msg.dims, 1.0);

    Scene {
        camera,
        program,
        cube,
        labels,
        dims: msg.dims.clone(),
        context: msg.context.clone(),
    }
}

fn render_cube<S: Surface>(ui: &Ui, display: &Display, scene: &mut Scene, target: &mut S) {
    let [width, height] = ui.io().display_size;

    let eye = scene.camera.eye();
    let view = scene.camera.view();

    let aspect = width / height;
    let fov = 45.0_f32.to_radians();
    let projection = Perspective3::new(aspect, fov, 0.1, 100.0);

    let view_proj = projection.to_homogeneous() * view.to_homogeneous();
    let mvp = view_proj * scene.cube.model.to_homogeneous();

    scene.cube.render(view_proj, &scene.program, target);

    for lbl in &scene.labels {
        lbl.render(mvp, ui);
    }

    let mouse_view_point =
        view.inverse() * linalg::world_coords(projection, ui.io().display_size, ui.io().mouse_pos);
    let direction = Unit::new_normalize(eye - mouse_view_point);

    let isects = scene.cube.intersections(eye, *direction);
    if let Some((_, face)) = isects.first() {
        scene
            .cube
            .render_face(face, view_proj, &scene.program, target);
        ui.tooltip(|| {
            let mut s = String::new();
            for (nm, d) in &face.dims {
                s.push_str(&format!("{} = {}\n", nm, if *d { 1 } else { 0 }));
            }
            ui.text(s);
        });
    };
}

// [TODO: Hazel; 2022-06-29] This probably shouldn't exist? Or at least it needs to be refactored.
// Getting the borrow checker to accept this was a pain in the shit, which is why I'm leaving it
// as dead code for now.
fn register_cube<'a, F: Facade>(
    tex_rc_ref: &'a Rc<Texture2d>,
    depthtex: &'a DepthTexture2d,
    facade: &F,
    textures: &'a mut Textures<Texture>,
) -> (TextureId, SimpleFrameBuffer<'a>) {
    let tex_rc = tex_rc_ref.clone();
    let another_tex_rc = tex_rc.clone();
    let fb = SimpleFrameBuffer::with_depth_buffer(facade, &**tex_rc_ref, depthtex).unwrap();

    let texture = Texture {
        texture: another_tex_rc,

        sampler: SamplerBehavior {
            magnify_filter: uniforms::MagnifySamplerFilter::Linear,
            minify_filter: uniforms::MinifySamplerFilter::Linear,
            ..Default::default()
        },
    };

    (textures.insert(texture), fb)
}

fn render_frame(ui: &Ui, display: &Display, scene: &mut Scene, target: &mut Frame) {
    let [_, height] = ui.io().display_size;

    render_cube(ui, display, scene, target);

    let ctx = unsafe { ImStr::from_utf8_with_nul_unchecked(scene.context.as_bytes()) };
    Window::new("Context")
        .position([0.0, 0.0], Condition::Always)
        .size([200.0, height], Condition::Appearing)
        .size_constraints([100.0, height], [400.0, height])
        .title_bar(false)
        .collapsible(false)
        .build(ui, || {
            ui.text_wrapped(ctx);
        });
}

fn handle_input(ui: &Ui, scene: &mut Scene) {
    let io = ui.io();
    if !io.want_capture_mouse {
        let [delta_x, delta_y] = io.mouse_delta;
        if ui.is_mouse_down(MouseButton::Left) {
            scene.camera.rotate_azimuth(delta_x / 300.0);
            scene.camera.rotate_polar(delta_y / 300.0);
        }
        scene.camera.zoom(0.1_f32 * io.mouse_wheel);
    }
}

fn handle_message(msg: messages::Message, display: &Display, scene: &mut Scene) {
    match msg {
        messages::Message::DisplayGoal(goal) => *scene = init_scene(display, &goal),
    }
}

pub fn render() {
    let mut system = system::init(3001, file!());
    let dims = vec![
        "i".to_string(),
        "j".to_string(),
        "k".to_string(),
        "l".to_string(),
    ];

    let ctx = "Welcome to coolttviz!\nPlease add a #viz hole to your code to start visualizing your goals.\0";
    let scene = init_scene(
        &system.display,
        &messages::DisplayGoal {
            dims,
            labels: vec![],
            context: ctx.to_string(),
        },
    );

    system.main_loop(
        scene,
        handle_message,
        move |_, display, scene, target, ui| {
            handle_input(ui, scene);
            render_frame(ui, display, scene, target);
        },
    );
}

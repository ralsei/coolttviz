use glium::backend::Facade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::DepthTexture2d;
use glium::uniforms::SamplerBehavior;
use glium::*;
use imgui::*;
use imgui_glium_renderer::{Texture, Renderer};
use nalgebra::{Perspective3, Unit};
use std::rc::Rc;

use crate::camera;
use crate::camera::Camera;
use crate::cube;
use crate::label;
use crate::messages;
use crate::{linalg, system};

pub struct Scene {
    camera: camera::Camera,

    cube: cube::Cube,

    program: glium::Program,

    dims: Vec<String>,
    labels: Vec<label::Label>,

    context: String,
    textures: Vec<TextureId>,
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

fn render_cube_static<S: Surface>(size: [f32; 2], scene: &Scene, target: &mut S) {
    let [width, height] = size;

    // [HACK: Amber; 2022-06-30] i'm not sure if we should be using the scene's camera,
    // or a different camera
    let view = scene.camera.view();

    let aspect = width / height;
    let fov = 45.0_f32.to_radians();
    let projection = Perspective3::new(aspect, fov, 0.1, 100.0);

    let view_proj = projection.to_homogeneous() * view.to_homogeneous();

    scene.cube.render(view_proj, &scene.program, target);
}

fn init_scene(display: &Display, msg: &messages::DisplayGoal, renderer: &mut Renderer) -> Scene {
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

    // [TODO: Amber; 2022-06-30] make this actually read stuff from the context (hard!)
    // [TODO: Amber; 2022-06-30] make this adjust based on sidebar length (double hard!)
    let texture = glium::texture::Texture2d::empty_with_format(
        display,
        glium::texture::UncompressedFloatFormat::F32F32F32F32,
        glium::texture::MipmapsOption::NoMipmap,
        200,
        200
    ).unwrap();
    let depth_texture = glium::texture::DepthTexture2d::empty_with_format(
        display,
        glium::texture::DepthFormat::F32,
        glium::texture::MipmapsOption::NoMipmap,
        200,
        200
    ).unwrap();
    let tex_rc = Rc::new(texture);

    let (tex_id, mut fb) = register_cube(&tex_rc, &depth_texture, display, renderer.textures());
    
    let scene =
        Scene {
            camera,
            program,
            cube,
            labels,
            dims: msg.dims.clone(),
            context: msg.context.clone(),
            textures: vec![tex_id],
        };

    render_cube_static([200.0, 200.0], &scene, &mut fb);
    
    scene
}

fn render_cube<S: Surface>(ui: &Ui, display: &Display, scene: &mut Scene, target: &mut S) {
    render_cube_static(ui.io().display_size, scene, target);

    let [width, height] = ui.io().display_size;

    let eye = scene.camera.eye();
    let view = scene.camera.view();

    let aspect = width / height;
    let fov = 45.0_f32.to_radians();
    let projection = Perspective3::new(aspect, fov, 0.1, 100.0);

    let view_proj = projection.to_homogeneous() * view.to_homogeneous();
    let mvp = view_proj * scene.cube.model.to_homogeneous(); 

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
            let draw_list = ui.get_window_draw_list();

            ui.text_wrapped(ctx);

            ui.invisible_button("spoingus", [200.0, 200.0]);
            
            draw_list.add_image(scene.textures[0], ui.item_rect_min(), ui.item_rect_max()).build();
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

fn handle_message(msg: messages::Message, display: &Display, scene: &mut Scene, renderer: &mut Renderer) {
    match msg {
        messages::Message::DisplayGoal(goal) => *scene = init_scene(display, &goal, renderer),
    }
}

pub fn render() {
    let mut system = system::init(3001, file!());
    let dims = vec![
        "i".to_string(),
        "j".to_string(),
        "k".to_string(),
//        "l".to_string(),
    ];

    let ctx = "Welcome to coolttviz!\nPlease add a #viz hole to your code to start visualizing your goals.\0";
    let scene = init_scene(
        &system.display,
        &messages::DisplayGoal {
            dims,
            labels: vec![],
            context: ctx.to_string(),
        },
        &mut system.renderer
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

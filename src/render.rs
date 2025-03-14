use glium::backend::Facade;
use glium::framebuffer::SimpleFrameBuffer;
use glium::texture::DepthTexture2d;
use glium::uniforms::SamplerBehavior;
use glium::*;
use imgui::*;
use imgui_glium_renderer::{Renderer, Texture};
use nalgebra::{Perspective3, Unit};
use std::rc::Rc;

use crate::camera;
use crate::cube;
use crate::label;
use crate::messages;
use crate::messages::CubeMessage;
use crate::{linalg, system};

pub struct LabeledCube {
    name: String,
    cube: cube::Cube,
    dims: Vec<String>,
    labels: Vec<label::Label>,
    texture_id: Option<TextureId>,
}

pub struct Scene {
    camera: camera::Camera,

    main_cube: LabeledCube,

    program: glium::Program,

    context: String,
    sidebar_cubes: Vec<LabeledCube>,
}

pub enum CubeIndex {
    MainCube,
    SidebarCube(usize),
}

fn register_cube<'a, F: Facade>(
    tex_rc_ref: &'a Rc<Texture2d>,
    depthtex: &'a DepthTexture2d,
    facade: &F,
    textures: &'a mut Textures<Texture>,
) -> (TextureId, SimpleFrameBuffer<'a>) {
    let tex_rc = tex_rc_ref.clone();
    let fb = SimpleFrameBuffer::with_depth_buffer(facade, &**tex_rc_ref, depthtex).unwrap();

    let texture = Texture {
        texture: tex_rc,

        sampler: SamplerBehavior {
            magnify_filter: uniforms::MagnifySamplerFilter::Linear,
            minify_filter: uniforms::MinifySamplerFilter::Linear,
            ..Default::default()
        },
    };

    (textures.insert(texture), fb)
}

fn render_cube_labelless<S: Surface>(
    size: [f32; 2],
    scene: &mut Scene,
    idx: CubeIndex,
    target: &mut S,
) {
    let [width, height] = size;
    let lc = match idx {
        CubeIndex::MainCube => &scene.main_cube,
        CubeIndex::SidebarCube(n) => &scene.sidebar_cubes[n],
    };

    // [HACK: Amber; 2022-06-30] i'm not sure if we should be using the scene's camera,
    // or a different camera

    // ... we should be using a diferent camera
    // [HACK: June; 2022-07-14] So what this is doing is using the camera at the front-facing view
    // This is probably not optimal. In my mind for 1 and 2-cubes, we should be rendering at the front,
    // But for 3 and 4-cubes, we should be rendering at an angle

    // We /could/ construct a camera here, but that wouldn't be great, since for the main cube we actually
    // do want to use the scene's dynamic camera
    // Alternatively we could just write a view matrix when the index is SidebarCube which would be fine
    let view = scene.camera.view();

    let aspect = width / height;
    let fov = 45.0_f32.to_radians();
    let projection = Perspective3::new(aspect, fov, 0.1, 100.0);

    let view_proj = projection.to_homogeneous() * view.to_homogeneous();

    lc.cube.render(view_proj, &scene.program, target);
}

fn cubes_from_context(
    display: &Display,
    scene: &mut Scene,
    cubes: &[CubeMessage],
    renderer: &mut Renderer,
) {
    let size = [200.0, 200.0];
    let white = [1.0, 1.0, 1.0, 1.0];

    for (i, cm) in cubes.iter().enumerate() {
        let dims = cm.dims.clone();
        let labels = cm
            .labels
            .iter()
            .map(|lbl| label::Label::new(&dims, lbl))
            .collect();

        let cube = cube::Cube::new(display, &dims, 1.0, white);

        let texture = glium::texture::Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32F32F32F32,
            glium::texture::MipmapsOption::NoMipmap,
            size[0] as u32,
            size[1] as u32,
        )
        .unwrap();
        let depth_texture = glium::texture::DepthTexture2d::empty_with_format(
            display,
            glium::texture::DepthFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            size[0] as u32,
            size[1] as u32,
        )
        .unwrap();
        let tex_rc = Rc::new(texture);

        let (texture_id, mut fb) =
            register_cube(&tex_rc, &depth_texture, display, renderer.textures());

        let labeled_cube = LabeledCube {
            name: cm.id.clone(),
            cube,
            dims,
            labels,
            texture_id: Some(texture_id),
        };

        scene.sidebar_cubes.push(labeled_cube);
        render_cube_labelless(size, scene, CubeIndex::SidebarCube(i), &mut fb);
    }
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
    let black = [0.0, 0.0, 0.0, 1.0];
    let cube = cube::Cube::new(display, &msg.dims, 1.0, black);

    let mut scene = Scene {
        camera,
        program,
        main_cube: LabeledCube {
            name: "".to_string(),
            cube,
            labels,
            dims: msg.dims.clone(),
            texture_id: None,
        },
        context: msg.context.clone(),
        sidebar_cubes: Vec::new(),
    };

    cubes_from_context(display, &mut scene, &msg.cubes, renderer);

    scene
}

fn render_cube<S: Surface>(ui: &Ui, scene: &mut Scene, target: &mut S) {
    render_cube_labelless(ui.io().display_size, scene, CubeIndex::MainCube, target);

    let [width, height] = ui.io().display_size;

    let eye = scene.camera.eye();
    let view = scene.camera.view();

    let aspect = width / height;
    let fov = 45.0_f32.to_radians();
    let projection = Perspective3::new(aspect, fov, 0.1, 100.0);

    let view_proj = projection.to_homogeneous() * view.to_homogeneous();
    let mvp = view_proj * scene.main_cube.cube.model.to_homogeneous();

    for lbl in &scene.main_cube.labels {
        lbl.render(mvp, ui);
    }

    let mouse_view_point =
        view.inverse() * linalg::world_coords(projection, ui.io().display_size, ui.io().mouse_pos);
    let direction = Unit::new_normalize(eye - mouse_view_point);

    let isects = scene.main_cube.cube.intersections(eye, *direction);
    if let Some((_, face)) = isects.first() {
        scene
            .main_cube
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

fn render_frame(ui: &Ui, scene: &mut Scene, target: &mut Frame) {
    let [_, height] = ui.io().display_size;

    render_cube(ui, scene, target);

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

            for sc in &scene.sidebar_cubes {
                if !sc.dims.is_empty() {
                    ui.text_wrapped(format!("Cube: {}", sc.name));

                    ui.invisible_button(
                        "",
                        [ui.content_region_avail()[0], ui.content_region_avail()[0]],
                    );

                    draw_list
                        .add_image(
                            sc.texture_id.unwrap(),
                            ui.item_rect_min(),
                            ui.item_rect_max(),
                        )
                        .build();
                }
            }
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

fn handle_message(
    msg: messages::Message,
    display: &Display,
    scene: &mut Scene,
    renderer: &mut Renderer,
) {
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
        "l".to_string(),
    ];

    let ctx = "Please add a #viz or #edit hole to your code to start visualizing your goals.\0";
    let scene = init_scene(
        &system.display,
        &messages::DisplayGoal {
            dims,
            labels: vec![],
            context: ctx.to_string(),
            cubes: vec![],
        },
        &mut system.renderer,
    );

    system.main_loop(scene, handle_message, move |_, _, scene, target, ui| {
        handle_input(ui, scene);
        render_frame(ui, scene, target);
    });
}

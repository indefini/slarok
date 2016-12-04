#![feature(box_syntax)]

extern crate glutin;
extern crate dormin;
//extern crate libc;

use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {

    let res = dormin::resource::ResourceGroup::new();
    let mut scene = dormin::scene::Scene::new_from_file("scene/aaa.scene", &res);
    let camera = if let Some(ref c) = scene.camera {
        c.clone()
    }
    else {
        return;
    };

    {
        //let mut cm = component::Manager::new();
        let mut cm = dormin::component::manager::COMP_MGR.lock().unwrap();
        cm.register_component("player_behavior", dormin::component::player::player_new);
        cm.register_component(
            "armature_animation",
            dormin::component::armature_animation::new);
    }

    scene.init_components(&res);

    let w = camera.borrow().data.width as u32;
    let h = camera.borrow().data.height as u32;

    let builder = glutin::WindowBuilder::new()
        .with_dimensions(w,h)
        .with_title("test".to_owned())
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0)))
        ;
    //let window = glutin::Window::new().unwrap();
    let window = builder.build().unwrap();
    unsafe { window.make_current() };


    let mut render = box dormin::render::GameRender::new(camera.clone(), Rc::new(res));

    render.resize(w as i32,h as i32);

    unsafe { dormin::render::cypher_init_simple(); }
    render.init();

    let mut input = dormin::input::Input::new();
    let load = Arc::new(Mutex::new(0));

    let mut quit = false;
    while !quit {
        input.clear();
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => quit = true,// break,
                glutin::Event::KeyboardInput(a,b,c) => {
                    input.add_key(b);
                }
            _ => ()
            }
        }

        scene.update(0.01f64, &input);
        unsafe {dormin::render::cypher_draw_start(w as i32, h as i32); }

        render.draw(&scene.objects, load.clone());
        unsafe {dormin::render::cypher_draw_end(); }
        window.swap_buffers();
    }
}

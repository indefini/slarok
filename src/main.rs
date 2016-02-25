#![feature(box_syntax)]

extern crate glutin;
extern crate dormin;
//extern crate libc;

use std::rc::Rc;

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_dimensions(300,200)
        .with_title("test".to_owned())
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0)))
        ;
    //let window = glutin::Window::new().unwrap();
    let window = builder.build().unwrap();
    unsafe { window.make_current() };

    /*
    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    for event in window.wait_events() {
        //unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        window.swap_buffers();

        match event {
            glutin::Event::Closed => break,
            _ => ()
        }
    }
    */

    let factory = dormin::factory::Factory::new();
    let res = dormin::resource::ResourceGroup::new();
    let mut scene = dormin::scene::Scene::new_from_file("scene/simple.scene", &res);
    let camera = if let Some(ref c) = scene.camera {
        c.clone()
    }
    else {
        return;
    };

    let mut render = box dormin::render::GameRender::new(camera, Rc::new(res));

    render.resize(300,200);

    render.init();

    let mut quit = false;
    while !quit {

        scene.update(0.01f64);
        render.draw(&scene.objects);
        //here draw something
        window.swap_buffers();
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => quit = true,// break,
            _ => ()
            }
        }
    }
}

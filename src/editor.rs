use std::ffi::c_void;
use log::*;
use std::sync::Arc;
use std::borrow::Borrow;
use std::thread;

use crate::x_handle::XHandle;
use crate::parameters::Parameters;

pub struct Editor {
    is_open: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    x_handle: Box<XHandle>,
    window_handle: u32,
    draw_context: u32,
    parameters: Arc<Parameters>,
}

impl Editor {
    pub fn new(x_handle: Box<XHandle>, parameters: Arc<Parameters>) -> Self {
        info!("GuiVstEditor::new()");

        Self {
            is_open: false,
            x: 0,
            y: 0,
            width: 1000,
            height: 1000,
            x_handle,
            window_handle: 0,
            draw_context: 0,
            parameters,
        }
    }

    fn create_draw_context(&mut self, parent: u32) {
        info!("GuiVstEditor::create_draw_context()");
        let conn = self.x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.x_handle.screen_num() as usize).unwrap();

        self.draw_context = conn.generate_id();
        let draw_context = self.draw_context;

        xcb::create_gc(conn.borrow(), draw_context, parent, &[
            (xcb::GC_FOREGROUND, screen.white_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);
    }

    fn create_window(&mut self, parent: u32) {
        info!("GuiVstEditor::create_window()");

        self.create_draw_context(parent);

        let conn = self.x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.x_handle.screen_num() as usize).unwrap();

        self.window_handle = conn.generate_id();
        xcb::create_window(&conn,
                           xcb::COPY_FROM_PARENT as u8,
                           self.window_handle,
                           parent,
                           self.x as i16,
                           self.y as i16,
                           self.width as u16,
                           self.height as u16,
                           0,
                           xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                           screen.root_visual(), &[
                (xcb::CW_BACK_PIXEL, screen.black_pixel()),
                (xcb::CW_EVENT_MASK,
                    xcb::EVENT_MASK_EXPOSURE |
                    xcb::EVENT_MASK_BUTTON_PRESS |
                    xcb::EVENT_MASK_BUTTON_RELEASE
                ),
            ]
        );
        xcb::map_window(&conn, self.window_handle);
        conn.flush();

        self.draw_editor();

        // Start handling events on this connection.
        thread::spawn(move || {
            Editor::handle_events(conn);
        });
    }

    pub fn draw_editor(&mut self) {
        //info!("GuiVstEditor::draw_editor()");

        let conn = self.x_handle.conn();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.x_handle.screen_num() as usize).unwrap();

        // Clear screen
        xcb::change_gc(
            conn.borrow(),
            self.draw_context,
            &[
                (xcb::GC_FOREGROUND, screen.black_pixel()),
                (xcb::GC_BACKGROUND, screen.black_pixel()),
                (xcb::GC_FILL_STYLE, xcb::FILL_STYLE_SOLID),
            ]
        );
        xcb::poly_fill_rectangle(
            conn.borrow(),
            self.window_handle,
            self.draw_context,
            &[xcb::Rectangle::new(0, 0, 1000, 1000)],
        );

        // Draw parameters on screen
        xcb::change_gc(
            conn.borrow(),
            self.draw_context,
            &[
                (xcb::GC_FOREGROUND, screen.white_pixel()),
                (xcb::GC_BACKGROUND, screen.white_pixel()),
                (xcb::GC_FILL_STYLE, xcb::FILL_STYLE_SOLID),
            ]
        );
        let rectangle_borders = vec!(
            xcb::Rectangle::new(50, 300, 900, 100),
            xcb::Rectangle::new(50, 600, 900, 100),
        );
        let rectangle_values = vec!(
            xcb::Rectangle::new(50, 300, (self.parameters.param1.get() * 900.0) as u16, 100),
            xcb::Rectangle::new(50, 600, (self.parameters.param2.get() * 900.0) as u16, 100),
        );
        xcb::poly_rectangle(
            conn.borrow(),
            self.window_handle,
            self.draw_context,
            &rectangle_borders,
        );
        xcb::poly_fill_rectangle(
            conn.borrow(),
            self.window_handle,
            self.draw_context,
            &rectangle_values,
        );

        // Flush the request
        conn.flush();
    }

    fn handle_events(conn: Arc<xcb::Connection>) {
        loop {
            let wait = conn.wait_for_event();
            if let Some(event) = wait {
                match event.response_type() {
                    xcb::BUTTON_PRESS => {
                        let event = unsafe { xcb::cast_event::<xcb::ButtonPressEvent>(&event) };
                        let button = event.detail();

                        // Left mouse button only
                        if button == 1 {
                            info!("Button press at: ({}, {})", event.event_x(), event.event_y());
                        }
                    },
                    xcb::BUTTON_RELEASE => {
                        let event = unsafe { xcb::cast_event::<xcb::ButtonReleaseEvent>(&event) };
                        let button = event.detail();

                        // Left mouse button only
                        if button == 1 {
                            info!("Button release at: ({}, {})", event.event_x(), event.event_y());
                        }
                    },
                    _ => {
                    }
                }
            }
        }
    }
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        info!("Editor::size()");
        (self.width, self.height)
    }

    fn position(&self) -> (i32, i32) {
        info!("Editor::position()");
        (self.x, self.y)
    }

    fn close(&mut self) {
        info!("Editor::close()");
        self.is_open = false;
    }

    fn open(&mut self, parent: *mut c_void) {
        info!("Editor::open()");
        self.create_window(parent as u32);
    }

    fn is_open(&mut self) -> bool {
        info!("Editor::is_open()");
        self.is_open
    }
}
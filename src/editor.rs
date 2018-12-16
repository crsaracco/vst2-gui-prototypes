use std::ffi::c_void;
use log::*;

pub struct Editor {
    is_open: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    x_connection: Box<xcb::Connection>,
    screen_num: i32,
    window_handle: u32,
    draw_context: u32,
    polyline: Vec<xcb::Point>,
}

impl Editor {
    pub fn new() -> Self {
        info!("GuiVstEditor::new()");

        let mut polyline: Vec<xcb::Point> = vec![];

        polyline.push(xcb::Point::new(50, 10 ));
        polyline.push(xcb::Point::new(5, 20 ));
        polyline.push(xcb::Point::new(25, -20 ));
        polyline.push(xcb::Point::new(10, 10 ));

        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

        Self {
            is_open: false,
            x: 0,
            y: 0,
            width: 1000,
            height: 1000,
            x_connection: Box::new(conn),
            screen_num,
            window_handle: 0,
            draw_context: 0,
            polyline,
        }
    }

    fn create_draw_context(&mut self, parent: u32) {
        info!("GuiVstEditor::create_draw_context()");
        let conn = self.x_connection.as_ref();

        self.draw_context = conn.generate_id();
        let draw_context = self.draw_context;

        xcb::create_gc(conn, draw_context, parent, &[
            (xcb::GC_FOREGROUND, self.get_screen().white_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);
    }

    fn get_screen(&self) -> xcb::StructPtr<'_, xcb::ffi::xcb_screen_t> {
        let conn = self.x_connection.as_ref();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();
        screen
    }

    fn create_window(&mut self, parent: u32) {
        info!("GuiVstEditor::create_window()");
        info!("Parent: {}", parent);

        self.create_draw_context(parent);

        self.window_handle = self.x_connection.generate_id();
        xcb::create_window(&self.x_connection,
                           xcb::COPY_FROM_PARENT as u8,
                           self.window_handle,
                           parent,
                           self.x as i16,
                           self.y as i16,
                           self.width as u16,
                           self.height as u16,
                           0,
                           xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                           self.get_screen().root_visual(), &[
                (xcb::CW_BACK_PIXEL, self.get_screen().black_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
            ]
        );
        xcb::map_window(&self.x_connection, self.window_handle);
        self.x_connection.flush();

        self.draw_editor();
    }

    pub fn draw_editor(&mut self) {
        info!("GuiVstEditor::draw_editor()");

        let conn = self.x_connection.as_ref();
        xcb::poly_line(
            conn,
            xcb::COORD_MODE_PREVIOUS as u8,
            self.window_handle,
            self.draw_context,
            &self.polyline
        );

        // Flush the request
        conn.flush();
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
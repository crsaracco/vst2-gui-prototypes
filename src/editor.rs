use std::ffi::c_void;
use log::*;

pub struct Editor {
    is_open: bool,
    polyline: Vec<xcb::Point>,
    x_connection: Option<xcb::Connection>,
    screen_num: i32,
    window_handle: u32,
    draw_context: u32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Editor {
    pub fn new() -> Self {
        info!("GuiVstEditor::new()");

        let mut polyline: Vec<xcb::Point> = vec![];

        polyline.push(xcb::Point::new(50, 10 ));
        polyline.push(xcb::Point::new(5, 20 ));
        polyline.push(xcb::Point::new(25, -20 ));
        polyline.push(xcb::Point::new(10, 10 ));

        Self {
            is_open: false,
            polyline,
            x_connection: None,
            screen_num: 0,
            window_handle: 0,
            draw_context: 0,
            x: 0,
            y: 0,
            width: 1000,
            height: 1000,
        }
    }

    fn create_connection(&mut self) {
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        self.x_connection = Some(conn);
        self.screen_num = screen_num;
    }

    fn create_window(&mut self, parent: u32) {
        info!("GuiVstEditor::create_window()");
        info!("Parent: {}", parent);

        self.create_connection();

        let conn = self.x_connection.as_mut().unwrap();

        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();

        self.draw_context = conn.generate_id();

        xcb::create_gc(&conn, self.draw_context, parent, &[
            (xcb::GC_FOREGROUND, screen.black_pixel()),
            (xcb::GC_GRAPHICS_EXPOSURES, 0),
        ]);

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
                (xcb::CW_BACK_PIXEL, screen.white_pixel()),
                (xcb::CW_EVENT_MASK,
                 xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
            ]
        );
        xcb::map_window(&conn, self.window_handle);
        conn.flush();

        self.draw_editor();
    }

    pub fn draw_editor(&mut self) {
        info!("GuiVstEditor::draw_editor()");

        // Since we draw whenever the host calls `get_editor()`, it's possible that the window
        // wasn't created yet. Don't attempt to draw if we don't have a window yet.
        if self.x_connection.is_some() {
            let conn = self.x_connection.as_mut().unwrap();
            xcb::poly_line(&conn,
                           xcb::COORD_MODE_PREVIOUS as u8,
                           self.window_handle,
                           self.draw_context,
                           &self.polyline
            );

            // Flush the request
            conn.flush();
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
        self.x_connection = None;
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
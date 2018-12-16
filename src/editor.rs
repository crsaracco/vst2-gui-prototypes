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
    param1_value: f32,
    param2_value: f32,
}

impl Editor {
    pub fn new() -> Self {
        info!("GuiVstEditor::new()");

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
            param1_value: 0.0,
            param2_value: 0.0,
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
        info!("GuiVstEditor::get_screen()");
        let conn = self.x_connection.as_ref();
        let setup = conn.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();
        screen
    }

    fn create_window(&mut self, parent: u32) {
        info!("GuiVstEditor::create_window()");

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
                (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS),
            ]
        );
        xcb::map_window(&self.x_connection, self.window_handle);
        self.x_connection.flush();

        self.draw_editor();
    }

    pub fn draw_editor(&mut self) {
        info!("GuiVstEditor::draw_editor()");

        let conn = self.x_connection.as_ref();

        // Clear screen
        xcb::change_gc(
            conn,
            self.draw_context,
            &[
                (xcb::GC_FOREGROUND, self.get_screen().black_pixel()),
                (xcb::GC_BACKGROUND, self.get_screen().black_pixel()),
                (xcb::GC_FILL_STYLE, xcb::FILL_STYLE_SOLID),
            ]
        );
        xcb::poly_fill_rectangle(
            conn,
            self.window_handle,
            self.draw_context,
            &[xcb::Rectangle::new(0, 0, 1000, 1000)],
        );

        // Draw parameters on screen
        xcb::change_gc(
            conn,
            self.draw_context,
            &[
                (xcb::GC_FOREGROUND, self.get_screen().white_pixel()),
                (xcb::GC_BACKGROUND, self.get_screen().white_pixel()),
                (xcb::GC_FILL_STYLE, xcb::FILL_STYLE_SOLID),
            ]
        );
        let rectangle_borders = vec!(
            xcb::Rectangle::new(50, 300, 900, 100),
            xcb::Rectangle::new(50, 600, 900, 100),
        );
        let rectangle_values = vec!(
            xcb::Rectangle::new(50, 300, (self.param1_value * 900.0) as u16, 100),
            xcb::Rectangle::new(50, 600, (self.param2_value * 900.0) as u16, 100),
        );
        xcb::poly_rectangle(
            conn,
            self.window_handle,
            self.draw_context,
            &rectangle_borders,
        );
        xcb::poly_fill_rectangle(
            conn,
            self.window_handle,
            self.draw_context,
            &rectangle_values,
        );


        // Flush the request
        conn.flush();
    }

    pub fn change_param1_value(&mut self, value: f32) {
        info!("GuiVstEditor::change_param1_value({})", value);
        self.param1_value = value;
    }

    pub fn change_param2_value(&mut self, value: f32) {
        info!("GuiVstEditor::change_param2_value({})", value);
        self.param2_value = value;
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
pub struct XHandle {
    pub conn: xcb::base::Connection,
    pub screen_num: i32,
}

impl XHandle {
    pub fn new() -> Self {
        let (conn, screen_num) = xcb::base::Connection::connect(None).unwrap();

        Self {
            conn,
            screen_num,
        }
    }
}
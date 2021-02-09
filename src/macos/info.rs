use super::iokit as io;

impl crate::InfoContext for super::Context {
    fn mouse_location(&self) -> Result<(i32, i32), Self::Error> {
        unsafe {
            let struct_ptr = self.fb_address as *const io::StdFBShmem_t;
            let loc_ptr: *const io::IOGPoint = &(*struct_ptr).cursorLoc;
            let loc = std::ptr::read_volatile(loc_ptr);
            Ok((loc.x as i32, loc.y as i32))
        }
    }

    fn screen_size(&self) -> Result<(i32, i32), Self::Error> {
        unsafe {
            let struct_ptr = self.fb_address as *const io::StdFBShmem_t;
            let bounds_ptr: *const io::IOGBounds = &(*struct_ptr).screenBounds;
            let bounds = std::ptr::read_volatile(bounds_ptr);
            Ok(((bounds.maxx - bounds.minx) as i32, (bounds.maxy - bounds.miny) as i32))
        }
    }
}
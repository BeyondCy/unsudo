extern crate libc;

fn main() {
    unsafe {
        let groupcount = libc::getgroups(0, 0 as *mut _);
        let mut groups = Vec::<u32>::with_capacity(groupcount as _);
        groups.extend((0 as u32)..(groupcount as u32));
        libc::getgroups(groupcount, groups.as_mut_ptr());
        let newgroups = groups.drain(..).filter(|&x| x != 10).collect::<Vec<u32>>();
        libc::setgroups(newgroups.len(), newgroups.as_ptr());
        libc::system([std::env::args().nth(1).unwrap_or("bash".to_string()).as_bytes(), &[0 as u8]].concat().as_ptr() as *const _);
    }
}

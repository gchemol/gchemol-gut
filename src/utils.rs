// [[file:../gut.note::*utils][utils:1]]
/// Sleep a few seconds
pub fn sleep(t: f64) {
    std::thread::sleep(std::time::Duration::from_secs_f64(t));
}
// utils:1 ends here

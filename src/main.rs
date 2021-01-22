#![feature(alloc_error_hook)]

fn main() {
    std::alloc::set_alloc_error_hook(my_alloc_error_hook);

    let mut v: Vec<u8> = Vec::with_capacity(1024 * 1024 * 1024 * 100);
    for i in (0..v.len()).step_by(4096) {
        v[i] = 1u8;
    }

    println!("Sleeping ...");
    std::thread::sleep(std::time::Duration::from_secs(60));
}

fn my_alloc_error_hook(layout: std::alloc::Layout) {
    eprintln!("memory allocation of {} bytes failed", layout.size());

    for debug_file_path in ["/proc/self/maps", "/proc/self/limits", "/proc/self/oom_adj", "/proc/self/oom_score", "/proc/self/oom_score_adj", "/proc/self/smaps", "/proc/self/stat", "/proc/self/statm", "/proc/self/status"].iter() {
        use std::io::Read;
    
        match std::fs::File::open(debug_file_path) {
            Ok(mut file) => {
                let mut file_content = String::new();
                if let Err(err) = file.read_to_string(&mut file_content) {
                    eprintln!("debug_file_path: {} cannot read: {:?}", debug_file_path, err);
                    continue;
                }
                eprintln!("debug_file_path: {}\ncontent: {}", debug_file_path, file_content);
            },
            Err(err) => {
                eprintln!("debug_file_path: {} cannot open: {:?}", debug_file_path, err);
                continue;
            }
        }
    }
}

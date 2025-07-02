use fs_extra::dir;
use std::time::Instant;
use screen_capture::Window;
// use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;


fn normalized(filename: &str) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

fn main() {
    let start = Instant::now();
    let windows = Window::all().unwrap();
    
    dir::create_all("target/windows", true).unwrap();

    let mut i = 0;
    for window_in_loop in windows {
        if window_in_loop.is_minimized().unwrap() {
            continue;
        }

        if window_in_loop.title().unwrap() == "windows – Datei-Explorer" {

            let window = Window::from(window_in_loop.hwnd().unwrap());

            println!(
                "Window: {:?} {:?} {:?}",
                window.title().unwrap(),
                (
                    window.x().unwrap(),
                    window.y().unwrap(),
                    window.width().unwrap(),
                    window.height().unwrap()
                ),
                (
                    window.is_minimized().unwrap(),
                    window.is_maximized().unwrap()
                )
            );
    
            let image = window.capture_image().unwrap();
            image
                .save(format!(
                    "target/windows/window-{}-{}.png",
                    i,
                    normalized(&window.title().unwrap())
                ))
                .unwrap();    

        }


        i += 1;
    }


    println!("Finished in: {:?}", start.elapsed());
}

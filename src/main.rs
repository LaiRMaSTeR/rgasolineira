use rustautogui::{self, RustAutoGui};
use std::{env, fs, io::{self, BufRead, Write}, path::{Path, PathBuf}, thread, time::{Duration, Instant}, vec::Vec};
use clap::{command, Arg, ArgAction};
use chrono::{Timelike, Local};
use image::{
    imageops::{resize, FilterType::{CatmullRom}},
    ImageBuffer, Luma};
use reqwest::blocking;
use rand;



struct Position {
    x: u32,
    y: u32
}

fn construct_position(pos: (i32, i32)) -> Position {
    Position{x: pos.0 as u32, y: pos.1 as u32}
}

struct Config {
    username: String,
    password: String,
    username_pos: Position,
    password_pos: Position,
    next_day_pos: Position,
    logout_pos: Position
}


fn distance(pos1: &Position, pos2: &Position) -> f32 {
    let out: f32 = ((pos1.x as i32 - pos2.x as i32) * (pos1.x as i32 - pos2.x as i32) + (pos1.y as i32 - pos2.y as i32) * (pos1.y as i32 - pos2.y as i32)) as f32;
    out.sqrt()
}


fn save_config(path: &PathBuf, rgui: &RustAutoGui) {
    let mut buffer: String = String::new();
    let mut coords: (i32, i32);
    let mut file = fs::File::create(path).expect("Nie można otworzyć pliku!");
    print!("Wprowadź nazwę użytkownika: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    writeln!(file, "{}", buffer.trim()).expect("Nie można zapisać do pliku!");
    buffer.clear();
    print!("Wprowadź hasło: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    writeln!(file, "{}", buffer.trim()).expect("Nie można zapisać do pliku!");
    print!("Ustaw kursor nad polem z nazwą użytkownika");
    io::stdout().flush().unwrap();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        coords = rgui.get_mouse_position().unwrap();
        if buffer.contains('\n') { break };
    }
    writeln!(file, "{},{}", coords.0, coords.1).expect("Nie można zapisać do pliku!");
    buffer.clear();
    print!("Ustaw kursor nad polem z hasłem");
    io::stdout().flush().unwrap();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        coords = rgui.get_mouse_position().unwrap();
        if buffer.contains('\n') { break };
    }
    writeln!(file, "{},{}", coords.0, coords.1).expect("Nie można zapisać do pliku!");
    buffer.clear();
    print!("Ustaw kursor nad polem z następnym dniem");
    io::stdout().flush().unwrap();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        coords = rgui.get_mouse_position().unwrap();
        if buffer.contains('\n') { break };
    }
    writeln!(file, "{},{}", coords.0, coords.1).expect("Nie można zapisać do pliku!");
    buffer.clear();
    print!("Ustaw kursor nad polem z przyciskiem wyloguj");
    io::stdout().flush().unwrap();
    loop {
        io::stdin().read_line(&mut buffer).unwrap();
        coords = rgui.get_mouse_position().unwrap();
        if buffer.contains('\n') { break };
    }
    writeln!(file, "{},{}", coords.0, coords.1).expect("Nie można zapisać do pliku!");
    buffer.clear();
}


fn load_config(path: &PathBuf) -> Config {
    let file = fs::File::open(path).expect("Nie znaleziono pliku credentials.conf!");
    let mut lines: Vec<String> = Vec::new();
    for e in io::BufReader::new(file).lines() {
        lines.push(e.unwrap());
    }
    let username_pos: Vec<&str> = lines[2].split(',').collect();
    let password_pos: Vec<&str> = lines[3].split(',').collect();
    let next_day_pos: Vec<&str> = lines[4].split(',').collect();
    let logout_pos: Vec<&str> = lines[5].split(',').collect();
    Config { username: lines[0].parse().unwrap(),
        password: lines[1].parse().unwrap(),
        username_pos: Position{ x: username_pos[0].parse().unwrap(), y: username_pos[1].parse().unwrap() },
        password_pos: Position{ x: password_pos[0].parse().unwrap(), y: password_pos[1].parse().unwrap() },
        next_day_pos: Position{ x: next_day_pos[0].parse().unwrap(), y: next_day_pos[1].parse().unwrap() },
        logout_pos: Position{ x: logout_pos[0].parse().unwrap(), y: logout_pos[1].parse().unwrap() } }
}


fn save_movies(path: &PathBuf, rgui: &RustAutoGui) {
    let mut buffer: String = String::new();
    let mut coords: (i32, i32);
    let mut last_coords: (i32, i32) = (0, 0);
    let mut file = fs::File::create(path).expect("Nie można otworzyć pliku!");
    println!("Wpisz enter by zapisać koordynaty. Wciśnij enter dwukrotnie, by zakończyć.");
    loop {
        coords = rgui.get_mouse_position().unwrap();
        if buffer.contains('\n') {
            if coords == last_coords { break };
            writeln!(file, "{},{}", coords.0, coords.1).expect("Nie można zapisać do pliku!");
            println!("Zapisano koordynaty {} {}", coords.0, coords.1);
            buffer.clear();
            last_coords = coords;
        }
        io::stdin().read_line(&mut buffer).unwrap();
    }
}


fn load_movies(path: &PathBuf) -> Vec<Position> {
    let file = fs::File::open(path).expect("Nie znaleziono pliku movies.conf!");
    let mut movies: Vec<Position> = Vec::new();
    for e in io::BufReader::new(file).lines() {
        let temp: Vec<u32> = e.unwrap().split(',').map(|x: &str| x.parse().unwrap()).collect();
        movies.push(Position { x: temp[0], y: temp[1] });
    }
    movies
}


fn log_in(conf: &Config, rgui: &RustAutoGui) {
    rgui.move_mouse_to_pos(conf.username_pos.x, conf.username_pos.y, 0.2).unwrap();
    rgui.left_click().unwrap();
    for c in conf.username.chars() {
        rgui.keyboard_input(&String::from(c)).unwrap();
        thread::sleep(Duration::from_millis(rand::random_range(30..40)));
    }
    thread::sleep(Duration::from_millis(100));
    rgui.move_mouse_to_pos(conf.password_pos.x, conf.password_pos.y, rand::random_range(0.05..0.1)).unwrap();
    rgui.left_click().unwrap();
    for c in conf.password.chars() {
        rgui.keyboard_input(&String::from(c)).unwrap();
        thread::sleep(Duration::from_millis(rand::random_range(30..40)));
    }
    rgui.keyboard_command("enter").unwrap();
    thread::sleep(Duration::from_millis(100));
    rgui.move_mouse_to_pos(conf.next_day_pos.x, conf.next_day_pos.y, rand::random_range(0.1..0.2)).unwrap();
    rgui.left_click().unwrap();
    println!("Zalogowano")
}


fn log_out(conf: &Config, rgui: &RustAutoGui) {
    thread::sleep(Duration::from_secs(rand::random_range(4..=8)));
    rgui.move_mouse_to_pos(conf.logout_pos.x, conf.logout_pos.y, rand::random_range(0.05..0.15)).unwrap();
    rgui.left_click().unwrap();
    println!("Wylogowano");
}


fn reserve_movies(movies: &Vec<Position>, rgui: &mut RustAutoGui, diagonal: &f32) {
    let mut current_pos: Position;
    let mut time: f32;
    for movie in movies {
        current_pos = construct_position(rgui.get_mouse_position().unwrap());
        time = distance(&current_pos, movie) / diagonal.sqrt() / 3.;
        rgui.move_mouse_to_pos(movie.x, movie.y, time).unwrap();
        thread::sleep(Duration::from_millis(rand::random_range(15..30)));
        rgui.left_click().unwrap();
        thread::sleep(Duration::from_millis(rand::random_range(10..30)));
        println!("Przesunięto do {} {} w {} s", movie.x, movie.y, time);
    }
}


fn wait_till(hour: u32, minute: u32, second: u32, text: &str) {
    let start_time: u32 = hour * 3600 + minute * 60 + second;
    let mut time_diff: u32;
    let mut time: chrono::DateTime<Local> = chrono::Local::now();
    while !(hour == time.hour() && minute == time.minute() && second <= time.second()) {
        if hour < time.hour() || (hour == time.hour() && minute < time.minute()) {
            time_diff = 24 * 3600 + start_time - time.hour() * 3600 - time.minute() * 60 - time.second();
        }
        else {
            time_diff = start_time - time.hour() * 3600 - time.minute() * 60 - time.second();
        }
        print!("\r{text} {:0>2}:{:0>2}:{:0>2}", time_diff / 3600, time_diff % 3600 / 60, time_diff % 3600 % 60);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(10));
        time = chrono::Local::now();
    }
    println!("");
}


fn download_marker(path: &PathBuf) {
    if !fs::exists(path).unwrap() {
        println!("Pobieram znacznik do {:?}", path);
        let mut file: fs::File = fs::File::create(path).unwrap();
        let mut image = blocking::get("https://raw.github.com/LaiRMaSTeR/rgasolineira/master/markers/marker.png").unwrap();
        image.copy_to(&mut file).unwrap();
    }
}


fn check_movies_novelty(new_movies: &Vec<(u32, u32, f32)>, movies: &mut Vec<(u32, u32, f32)>) {
    if movies.len() == 0 {
        for movie in new_movies {
            movies.push(*movie);
        }
    }
    else {
        let mut novelty: bool;
        for movie in new_movies {
            novelty = true;
            for old in &mut *movies {
                if (movie.0 as i32 - old.0 as i32).abs() < 10 && (movie.1 as i32 - old.1 as i32).abs() < 5 { novelty = false; }
            }
            if novelty { movies.push(*movie); }
        }
    }
}


fn detect_movies(path: &PathBuf, rgui: &mut RustAutoGui) -> Vec<Position> {
    println!("\nSzukam zaznaczonych filmów");
    let time: Instant = Instant::now();
    let img: image::ImageReader<io::BufReader<fs::File>> = image::ImageReader::open(path).unwrap();
    let img: image::DynamicImage = img.decode().unwrap();
    let gray_image: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    for scaling in 14..40 {
        let scaled_image: ImageBuffer<Luma<u8>, Vec<u8>> = resize(&gray_image, &gray_image.width() * scaling / 100, &gray_image.height() * scaling / 100, CatmullRom);
        rgui.store_template_from_imagebuffer(scaled_image, None, rustautogui::MatchMode::Segmented, &scaling.to_string()).unwrap();
    }
    let mut locations: Option<Vec<(u32, u32, f32)>>;
    let mut movies: Vec<(u32, u32, f32)> = vec![];
    for scaling in 14..40 {
        locations = rgui.find_stored_image_on_screen(0.9, &scaling.to_string()).unwrap();
        match locations {
            Some(x) => check_movies_novelty(&x, &mut movies),
            None => continue
        }
    }
    println!("");
    let mut out: Vec<Position> = vec![];
    for movie in movies {
        out.push(Position { x: movie.0 - 30, y: movie.1 });
    }
    println!("Znaleziono {} filmy w {:.2?}", &out.len(), time.elapsed());
    out
}


fn main() {
    let matches = command!()
        .arg(Arg::new("config").short('c').long("config").action(ArgAction::SetTrue))
        .arg(Arg::new("set_movies").short('m').long("set_movies").action(ArgAction::SetTrue))
        .arg(Arg::new("debug").short('d').long("debug").action(ArgAction::SetFalse))
        .arg(Arg::new("auto_mode").short('a').long("auto_mode").action(ArgAction::SetTrue))
        .get_matches();

    let mut rgui: rustautogui::RustAutoGui = RustAutoGui::new(false).unwrap();
    let screen: (i32, i32) = rgui.get_screen_size();
    let diagonal: f32 = (screen.0 * screen.0 + screen.1 * screen.1) as f32;
    let config_dir: PathBuf = [env::current_exe().unwrap().parent().unwrap(), Path::new(".rgasolineira")].iter().collect();
    if !fs::exists(&config_dir).unwrap() {
        println!("Tworzenie folderu konfiguracyjnego: {:?}", &config_dir);
        fs::create_dir(&config_dir).expect("Nie można utworzyć folderu konfiguracyjnego!");
    }
    let config_path: PathBuf = config_dir.join("credentials.conf");
    let marker_path: PathBuf = config_dir.join("marker.png");

    if matches.get_flag("config") {
        save_config(&config_path, &rgui);
        download_marker(&marker_path);
    }
    else if matches.get_flag("set_movies") {
        let movies_path: PathBuf = config_dir.join("movies.conf");
        save_movies(&movies_path, &rgui);
    }
    else if matches.get_flag("auto_mode") {
        let config: Config = load_config(&config_path);
        let mut movies: Vec<Position>;
        loop {
            if matches.get_flag("debug") { wait_till(8, 27, rand::random_range(0..60), "Logowanie za"); } // 8:28:16
            log_in(&config, &rgui);
            movies = detect_movies(&marker_path, &mut rgui);
            let current_pos = construct_position(rgui.get_mouse_position().unwrap()); // so first movie will be clicked on right after 8:30
            rgui.move_mouse_to_pos(&movies[0].x - 10, &movies[0].y + 10, distance(&current_pos, &movies[0]) / diagonal.sqrt() / 2.).unwrap();
            if matches.get_flag("debug") { wait_till(8, 30, 0, "Rezerwacje za"); } // 8:29:59
            if movies.len() > 0 { reserve_movies(&movies, &mut rgui, &diagonal); }
            log_out(&config, &rgui);
            movies.clear();

            if !matches.get_flag("debug") { break; }
        }
    }
    else {
        let movies_path: PathBuf = config_dir.join("movies.conf");
        let config: Config = load_config(&config_path);
        let movies: Vec<Position> = load_movies(&movies_path);

        if matches.get_flag("debug") { wait_till(8, 28, rand::random_range(0..60), "Logowanie za"); } // 8:28:16
        log_in(&config, &rgui);
        let current_pos = construct_position(rgui.get_mouse_position().unwrap()); // so first movie will be clicked on right after 8:30
        rgui.move_mouse_to_pos(&movies[0].x - 10, &movies[0].y + 10, distance(&current_pos, &movies[0]) / diagonal.sqrt() / 2.).unwrap();
        if matches.get_flag("debug") { wait_till(8, 30, 0, "Rezerwacje za"); } // 8:29:59
        reserve_movies(&movies, &mut rgui, &diagonal);
        log_out(&config, &rgui);
    }
}

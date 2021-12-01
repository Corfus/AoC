use std::env;
use std::fs;
use std::fs::Metadata;
use std::path::Path;
use tera::{Context, Tera};
use walkdir::{WalkDir};

static TEMPLATE_PATH: &'static str = "templates";

fn extract_date() -> Option<(u16, u8)> {
    let args: Vec<String> = env::args().collect();
    let year_opt = args.get(1);
    let day_opt = args.get(2);

    year_opt
        .and_then(|year| year.parse::<u16>().ok())
        .and_then(|year| {
            day_opt.and_then(|day| day.parse::<u8>().ok())
                .map(|day| (year, day))
        })
}

fn create_project_folder(year: u16, day: u8) -> Option<String> {
    let p = format!("../{}/{}/src", year, day);
    fs::create_dir_all(p.clone()).ok().map(|_| p)
}

fn get_two_digits_int(n: u8) -> String {
    format!("{:02}", n)
}

fn get_folder_content() -> Vec<String> {
    WalkDir::new(TEMPLATE_PATH).into_iter()
        .filter_map(|r| r.ok()
            .and_then(|e| e.metadata().ok().map(|m| (e, m))))
        .filter_map(|(e, m)| if m.is_file() { Some(e) } else { None })
        .map(|e| format!("{}", e.path().display()).replace(&format!("{}/", TEMPLATE_PATH), ""))
        .collect::<Vec<_>>()
}

fn main() {
    extract_date()
        .and_then(|(year, day)| {
            create_project_folder(year, day)
                .and_then(|_| {
                    Tera::new(&format!("{}/**/*", TEMPLATE_PATH))
                        .ok()
                        .and_then(|t| {
                            let day_two_digits = get_two_digits_int(day);
                            let mut context = Context::new();
                            context.insert("year", &year);
                            context.insert("day", &day);
                            context.insert("day_two_digits", &day_two_digits);

                            get_folder_content()
                                .iter()
                                .map(|name| {
                                    t.render(name, &context)
                                        .ok()
                                        .and_then(|content| {
                                            fs::write(Path::new(&format!("../{}/{}/{}",year, day, name)), &content).map_err(|err| println!("{:?}", err)).ok()
                                        })
                                })
                                .collect::<Option<()>>()
                        })
                })
        });
}

use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Serialize)]
struct SystemInfo {
    available_memory: u64,
    total_memory: u64,
    used_memory: u64,
    used_memory_percent: f64,
    core_count: usize,
    cpu_usage: f32,
    total_swap: u64,
    used_swap: u64,
    used_swap_percent: f64,
    uptime: u64,
    load_average: LoadAvg,
}

#[derive(Serialize)]
struct LoadAvg {
    one: f64,
    five: f64,
    fifteen: f64,
}

async fn sysinfo() -> impl Responder {
    let mut system = System::new_all();
    system.refresh_all();

    let available_memory = system.available_memory() as f64;
    let total_memory = system.total_memory() as f64;
    let used_memory = total_memory - available_memory;
    let used_memory_percent = used_memory / total_memory * 100.0;
    let core_count = system.physical_core_count().unwrap_or(0);
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let total_swap = system.total_swap() as f64;
    let used_swap = system.used_swap() as f64;
    let used_swap_percent = used_swap / total_swap * 100.0;
    let uptime = system.uptime();
    let load_average = system.load_average();

    let memory_unit = if total_memory > 1073741824.0 {
        "GB"
    } else {
        "MB"
    };

    let system_info = json!({
        "available_memory": format!("{:.2} {}", available_memory / 1073741824.0, memory_unit),
        "total_memory": format!("{:.2} {}", total_memory / 1073741824.0, memory_unit),
        "used_memory": format!("{:.2} {}", used_memory / 1073741824.0, memory_unit),
        "used_memory_percent": format!("{:.2}%", used_memory_percent),
        "core_count": core_count,
        "cpu_usage": format!("{:.2}%", cpu_usage),
        "total_swap": format!("{:.2} {}", total_swap / 1073741824.0, memory_unit),
        "used_swap": format!("{:.2} {}", used_swap / 1073741824.0, memory_unit),
        "used_swap_percent": format!("{:.2}%", used_swap_percent),
        "uptime": uptime,
        "load_average": {
            "one": format!("{:.2}", load_average.one),
            "five": format!("{:.2}", load_average.five),
            "fifteen": format!("{:.2}", load_average.fifteen),
        }
    });

    let response = system_info.to_string();

    println!("=> disks:");
    for disk in system.disks() {
        println!("{:?}", disk);
    }

    
    println!("=> components:");
    for component in system.components() {
        println!("{:?}", component);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

async fn root(req: HttpRequest) -> impl Responder {
    let user_agent = req
        .headers()
        .get("User-Agent")
        .map_or("", |header| header.to_str().unwrap_or(""));

    let mut system = System::new_all();
    system.refresh_all();

    let available_memory = system.available_memory() as f64;
    let total_memory = system.total_memory() as f64;
    let used_memory = total_memory - available_memory;
    let used_memory_percent = used_memory / total_memory * 100.0;
    let core_count = system.physical_core_count().unwrap_or(0);
    let cpu_usage = system.global_cpu_info().cpu_usage();
    let total_swap = system.total_swap() as f64;
    let used_swap = system.used_swap() as f64;
    let used_swap_percent = used_swap / total_swap * 100.0;
    let uptime = system.uptime();
    let load_average = system.load_average();

    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;
    let formatted_uptime = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    if user_agent.contains("curl") {
        HttpResponse::Ok().body(format!(
            "
+----------------------+-----------+
|        Data          |   Value   |
+----------------------+-----------+
| Available Memory     | {:>8}  |
| Total Memory         | {:>8}  |
| Used Memory          | {:>8}  |
| Used Memory Percent  | {:>6}%   |
| Core Count           | {:>8}  |
| CPU Usage            | {:>6}%   |
| Total Swap           | {:>8}  |
| Used Swap            | {:>8}  |
| Used Swap Percent    | {:>6}%   |
| Uptime               | {:>8}  |
| Load Average (1)     | {:>8.2}  |
| Load Average (5)     | {:>8.2}  |
| Load Average (15)    | {:>8.2}  |
+----------------------+-----------+
",
            format!("{:.2} GB", available_memory / 1073741824.0),
            format!("{:.2} GB", total_memory / 1073741824.0),
            format!("{:.2} GB", used_memory / 1073741824.0),
            format!("{:.2}", used_memory_percent),
            core_count,
            format!("{:.2}", cpu_usage),
            format!("{:.2} GB", total_swap / 1073741824.0),
            format!("{:.2} GB", used_swap / 1073741824.0),
            format!("{:.2}", used_swap_percent),
            formatted_uptime,
            load_average.one,
            load_average.five,
            load_average.fifteen,
        ))
    } else {
        let path: PathBuf = PathBuf::new().join("src/index.html");
        NamedFile::open(path).unwrap().into_response(&req)
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(root))
            .route("/api/sysinfo", web::get().to(sysinfo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

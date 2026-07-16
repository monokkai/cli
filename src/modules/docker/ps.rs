use colored::Colorize;
use std::process::Command;

struct Column {
    header: &'static str,
    width: usize,
}

const COLUMNS: &[Column] = &[
    Column {
        header: "ID",
        width: 14,
    },
    Column {
        header: "NAME",
        width: 24,
    },
    Column {
        header: "IMAGE",
        width: 28,
    },
    Column {
        header: "STATUS",
        width: 20,
    },
    Column {
        header: "PORTS",
        width: 28,
    },
    Column {
        header: "CREATED",
        width: 20,
    },
];

pub fn handle() {
    let output = Command::new("docker")
        .args([
            "ps",
            "-a",
            "--format",
            "{{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}\t{{.RunningFor}}",
        ])
        .output();

    match output {
        Err(_) => {
            eprintln!("{}", "❌ Docker is not running or not installed".red());
            return;
        }
        Ok(out) if !out.status.success() => {
            let err = String::from_utf8_lossy(&out.stderr);
            eprintln!("{} {}", "❌".red(), err.trim().red());
            return;
        }
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let rows: Vec<Vec<&str>> = stdout
                .lines()
                .map(|line| line.splitn(6, '\t').collect())
                .collect();

            print_header();
            print_separator();

            if rows.is_empty() || (rows.len() == 1 && rows[0].is_empty()) {
                println!("  {}", "No containers found".yellow());
            } else {
                for row in &rows {
                    print_row(row);
                }
            }

            print_separator();
            println!(
                "{} {} container(s)",
                "ℹ".blue(),
                rows.iter().filter(|r| !r.is_empty()).count()
            );
        }
    }
}

fn print_header() {
    println!();
    let header: String = COLUMNS
        .iter()
        .map(|c| {
            format!(
                "{:<width$}",
                c.header.bold().blue().to_string(),
                width = c.width + 10
            )
        })
        .collect::<Vec<_>>()
        .join("  ");
    println!("  {}", header);
}

fn print_separator() {
    let sep: String = COLUMNS
        .iter()
        .map(|c| "─".repeat(c.width))
        .collect::<Vec<_>>()
        .join("──");
    println!("  {}", sep.dimmed());
}

fn print_row(fields: &[&str]) {
    let get = |i: usize| fields.get(i).copied().unwrap_or("").trim();

    let id = truncate(get(0), COLUMNS[0].width);
    let name = truncate(get(1), COLUMNS[1].width);
    let image = truncate(get(2), COLUMNS[2].width);
    let status = get(3);
    let ports = truncate(get(4), COLUMNS[4].width);
    let created = truncate(get(5), COLUMNS[5].width);

    let status_colored = if status.starts_with("Up") {
        format!("{:<20}", status).green().to_string()
    } else if status.starts_with("Exited") {
        format!("{:<20}", status).red().to_string()
    } else {
        format!("{:<20}", status).yellow().to_string()
    };

    println!(
        "  {:<14}  {:<24}  {:<28}  {}  {:<28}  {:<20}",
        id.cyan(),
        name.white(),
        image.dimmed(),
        status_colored,
        ports.yellow(),
        created.dimmed(),
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max - 1])
    } else {
        s.to_string()
    }
}

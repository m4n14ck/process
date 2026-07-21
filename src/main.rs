use sysinfo::{System, Users};
use std::io::{self, Write};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    println!("+--------------------------------------------------+");
    println!("|           SYSTEM PROCESS MONITOR                 |");
    println!("+--------------------------------------------------+");
    println!("| Options:                                         |");
    println!("| 1. List all processes                            |");
    println!("| 2. List top processes by CPU                     |");
    println!("| 3. List top processes by memory                  |");
    println!("| 4. Filter processes by name                      |");
    println!("| 5. Real-time monitoring (refresh every 2s)       |");
    println!("| 6. System summary                                |");
    println!("| 7. Exit                                          |");
    println!("+--------------------------------------------------+");
    println!();

    loop {
        print!("\n[?] Choose option (1-7): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input");
        let choice = choice.trim();

        match choice {
            "1" => {
                list_all_processes();
                wait_for_enter();
                clear_screen();
            }
            "2" => {
                list_top_cpu_processes();
                wait_for_enter();
                clear_screen();
            }
            "3" => {
                list_top_memory_processes();
                wait_for_enter();
                clear_screen();
            }
            "4" => {
                filter_processes_by_name();
                wait_for_enter();
                clear_screen();
            }
            "5" => {
                real_time_monitoring();
                clear_screen();
            }
            "6" => {
                show_system_summary();
                wait_for_enter();
                clear_screen();
            }
            "7" => {
                println!("\n[*] Goodbye!");
                break;
            }
            _ => {
                println!("[!] Invalid option. Try again.");
                wait_for_enter();
                clear_screen();
            }
        }
    }
}

fn list_all_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut users = Users::new();
    users.refresh_list();

    println!("\n+-------------------------------------------+");
    println!("|        ALL RUNNING PROCESSES              |");
    println!("+-------------------------------------------+");
    println!(
        "{:<8} {:<30} {:<8} {:<12} {:<20}",
        "PID", "NAME", "CPU%", "MEM (KB)", "USER"
    );
    println!("+----------+-------------------------------+----------+------------+----------------------+");

    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| a.1.name().cmp(b.1.name()));

    for (pid, process) in processes {
        let pid_val = pid.as_u32();
        let name = process.name();
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory() / 1024;

        let user_info = get_user_name(process, &mut users);

        println!(
            "{:<8} {:<30} {:<8.2}% {:<12} {:<20}",
            pid_val,
            truncate_text(name, 30),
            cpu_usage,
            format_memory(memory_usage),
            truncate_text(&user_info, 20)
        );
    }
    println!("+----------+-------------------------------+----------+------------+----------------------+");
    println!("[+] Total processes: {}", sys.processes().len());
}

fn list_top_cpu_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut users = Users::new();
    users.refresh_list();

    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

    println!("\n+-------------------------------------------+");
    println!("|       TOP 20 PROCESSES BY CPU             |");
    println!("+-------------------------------------------+");
    println!(
        "{:<8} {:<30} {:<8} {:<12} {:<20}",
        "PID", "NAME", "CPU%", "MEM (KB)", "USER"
    );
    println!("+----------+-------------------------------+----------+------------+----------------------+");

    for (pid, process) in processes.iter().take(20) {
        let pid_val = pid.as_u32();
        let name = process.name();
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory() / 1024;

        let user_info = get_user_name(process, &mut users);

        println!(
            "{:<8} {:<30} {:<8.2}% {:<12} {:<20}",
            pid_val,
            truncate_text(name, 30),
            cpu_usage,
            format_memory(memory_usage),
            truncate_text(&user_info, 20)
        );
    }
    println!("+----------+-------------------------------+----------+------------+----------------------+");
}

fn list_top_memory_processes() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut users = Users::new();
    users.refresh_list();

    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.memory().cmp(&a.1.memory()));

    println!("\n+-------------------------------------------+");
    println!("|     TOP 20 PROCESSES BY MEMORY            |");
    println!("+-------------------------------------------+");
    println!(
        "{:<8} {:<30} {:<8} {:<12} {:<20}",
        "PID", "NAME", "CPU%", "MEM (KB)", "USER"
    );
    println!("+----------+-------------------------------+----------+------------+----------------------+");

    for (pid, process) in processes.iter().take(20) {
        let pid_val = pid.as_u32();
        let name = process.name();
        let cpu_usage = process.cpu_usage();
        let memory_usage = process.memory() / 1024;

        let user_info = get_user_name(process, &mut users);

        println!(
            "{:<8} {:<30} {:<8.2}% {:<12} {:<20}",
            pid_val,
            truncate_text(name, 30),
            cpu_usage,
            format_memory(memory_usage),
            truncate_text(&user_info, 20)
        );
    }
    println!("+----------+-------------------------------+----------+------------+----------------------+");
}

fn filter_processes_by_name() {
    print!("\n[>] Enter process name to search: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let search_term = input.trim().to_lowercase();

    if search_term.is_empty() {
        println!("[!] No search term entered.");
        return;
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut users = Users::new();
    users.refresh_list();

    let mut found = false;

    println!("\n+-------------------------------------------+");
    println!("|       PROCESSES MATCHING '{}'            |", search_term);
    println!("+-------------------------------------------+");
    println!(
        "{:<8} {:<30} {:<8} {:<12} {:<20}",
        "PID", "NAME", "CPU%", "MEM (KB)", "USER"
    );
    println!("+----------+-------------------------------+----------+------------+----------------------+");

    for (pid, process) in sys.processes() {
        let name = process.name().to_lowercase();
        if name.contains(&search_term) {
            found = true;
            let pid_val = pid.as_u32();
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory() / 1024;

            let user_info = get_user_name(process, &mut users);

            println!(
                "{:<8} {:<30} {:<8.2}% {:<12} {:<20}",
                pid_val,
                truncate_text(process.name(), 30),
                cpu_usage,
                format_memory(memory_usage),
                truncate_text(&user_info, 20)
            );
        }
    }

    if !found {
        println!("[!] No processes found matching '{}'", search_term);
    }
    println!("+----------+-------------------------------+----------+------------+----------------------+");
}

fn real_time_monitoring() {
    println!("\n[*] Real-time monitoring started");
    println!("[+] Press Ctrl+C to stop monitoring and return to menu\n");

    // Create a flag to track if we should continue
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("\n\n[*] Stopping real-time monitoring...");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        let mut sys = System::new_all();
        sys.refresh_all();

        // Clear screen and show top 10 processes
        clear_screen();
        
        println!("+--------------------------------------------------+");
        println!("|       REAL-TIME MONITORING (Top 10 CPU)          |");
        println!("+--------------------------------------------------+");
        println!(
            "{:<8} {:<30} {:<8} {:<12} {:<20}",
            "PID", "NAME", "CPU%", "MEM (KB)", "USER"
        );
        println!("+----------+-------------------------------+----------+------------+----------------------+");

        let mut users = Users::new();
        users.refresh_list();

        let mut processes: Vec<_> = sys.processes().iter().collect();
        processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

        for (pid, process) in processes.iter().take(10) {
            let pid_val = pid.as_u32();
            let name = process.name();
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory() / 1024;

            let user_info = get_user_name(process, &mut users);

            println!(
                "{:<8} {:<30} {:<8.2}% {:<12} {:<20}",
                pid_val,
                truncate_text(name, 30),
                cpu_usage,
                format_memory(memory_usage),
                truncate_text(&user_info, 20)
            );
        }
        println!("+----------+-------------------------------+----------+------------+----------------------+");
        println!("[+] Press Ctrl+C to stop monitoring");

        // Sleep for 2 seconds, checking if we should stop
        for _ in 0..20 {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    // Reset Ctrl+C handler to default
    // This prevents the handler from interfering with other parts of the program
    let _ = ctrlc::set_handler(|| {
        // Do nothing, just reset
    });
    
    println!("[+] Returning to main menu...");
    std::thread::sleep(Duration::from_millis(1000));
}

fn show_system_summary() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n+--------------------------------------------------+");
    println!("|              SYSTEM SUMMARY                      |");
    println!("+--------------------------------------------------+");
    println!("| System Information:                              |");
    println!("+--------------------------------------------------+");
    
    // System info
    println!("| Hostname:        {:30} |", 
             System::host_name().unwrap_or_else(|| "Unknown".to_string()));
    println!("| OS:              {:30} |", 
             System::name().unwrap_or_else(|| "Unknown".to_string()));
    println!("| Kernel:          {:30} |", 
             System::kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("| OS Version:      {:30} |", 
             System::os_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("+--------------------------------------------------+");
    
    // Memory info
    println!("| Memory Information:                              |");
    println!("+--------------------------------------------------+");
    println!("| Total Memory:    {:30} |", 
             format_memory(sys.total_memory() / 1024));
    println!("| Used Memory:     {:30} |", 
             format_memory(sys.used_memory() / 1024));
    println!("| Free Memory:     {:30} |", 
             format_memory(sys.free_memory() / 1024));
    println!("| Memory Usage:    {:29.1}% |", 
             (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0);
    println!("+--------------------------------------------------+");
    
    // CPU info
    println!("| CPU Information:                                |");
    println!("+--------------------------------------------------+");
    println!("| CPU Cores:       {:30} |", 
             sys.physical_core_count().unwrap_or(0));
    println!("| Total Processes: {:30} |", 
             sys.processes().len());
    println!("+--------------------------------------------------+");
    
    // Load average
    let load = System::load_average();
    println!("| Load Average:                                   |");
    println!("+--------------------------------------------------+");
    println!("| 1 minute:        {:30.2} |", load.one);
    println!("| 5 minutes:       {:30.2} |", load.five);
    println!("| 15 minutes:      {:30.2} |", load.fifteen);
    println!("+--------------------------------------------------+");
}

fn get_user_name(process: &sysinfo::Process, users: &mut Users) -> String {
    if let Some(uid) = process.user_id() {
        if let Some(user) = users.get_user_by_id(uid) {
            return user.name().to_string();
        } else {
            return format!("UID: {:?}", uid);
        }
    }
    String::from("N/A")
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len - 3])
    }
}

fn format_memory(kb: u64) -> String {
    if kb < 1024 {
        format!("{} KB", kb)
    } else if kb < 1024 * 1024 {
        format!("{:.1} MB", kb as f64 / 1024.0)
    } else {
        format!("{:.2} GB", kb as f64 / (1024.0 * 1024.0))
    }
}

fn wait_for_enter() {
    print!("\n[>] Press Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status();
    } else {
        let _ = std::process::Command::new("clear")
            .status();
    }
    
    print_menu();
}

fn print_menu() {
    println!("+--------------------------------------------------+");
    println!("|           SYSTEM PROCESS MONITOR                 |");
    println!("+--------------------------------------------------+");
    println!("| Options:                                         |");
    println!("| 1. List all processes                           |");
    println!("| 2. List top processes by CPU                   |");
    println!("| 3. List top processes by memory                |");
    println!("| 4. Filter processes by name                    |");
    println!("| 5. Real-time monitoring (refresh every 2s)    |");
    println!("| 6. System summary                              |");
    println!("| 7. Exit                                       |");
    println!("+--------------------------------------------------+");
    println!();
}
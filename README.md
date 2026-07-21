## About

**Process Ops** is an interactive command-line system process monitor written in Rust. It provides real-time insights into running processes, including CPU usage, memory consumption, and user details, all presented through a clean and intuitive ASCII interface.

### Key Features

- **Process Listing** - View all running processes with detailed information
- **Performance Metrics** - Identify top CPU and memory consumers
- **Smart Search** - Find processes by name with instant filtering
- **Live Monitoring** - Watch system activity update every 2 seconds
- **System Overview** - Get complete system health summary
- **Cross-Platform** - Works on Windows, Linux, and macOS

Built for system administrators, developers, and power users who need quick, reliable access to process information directly from their terminal.

...
📖 Usage
When you run the program, you'll see an interactive menu with the following options:
+--------------------------------------------------+
|           SYSTEM PROCESS MONITOR                 |
+--------------------------------------------------+
| Options:                                         |
| 1. List all processes                           |
| 2. List top processes by CPU                   |
| 3. List top processes by memory                |
| 4. Filter processes by name                    |
| 5. Real-time monitoring (refresh every 2s)    |
| 6. System summary                              |
| 7. Exit                                       |
+--------------------------------------------------+


2. Search Processes by Name
text
[?] Choose option (1-7): 4

[>] Enter process name to search: chrome

+-------------------------------------------+
|       PROCESSES MATCHING 'chrome'          |
+-------------------------------------------+
PID      NAME                           CPU%   MEM (KB)     USER
+----------+-------------------------------+----------+------------+----------------------+
9012     chrome                         8.50%   156.7 MB    user
9013     chrome                         12.30%  245.1 MB    user
3. Real-time Monitoring
text
[?] Choose option (1-7): 5

[*] Real-time monitoring started
[+] Press Ctrl+C to stop monitoring and return to menu


+--------------------------------------------------+
|       REAL-TIME MONITORING (Top 10 CPU)          |
+--------------------------------------------------+
PID      NAME                           CPU%   MEM (KB)     USER
+----------+-------------------------------+----------+------------+----------------------+
5678     rust-analyzer                  15.30%  89.2 MB     user
9012     chrome                         8.50%   156.7 MB    user
...

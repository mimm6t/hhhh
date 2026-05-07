#!/usr/bin/env python3
"""
rustFrida Web UI Server
提供 Web 界面来管理 rustfrida server 并保持其持久化运行
"""

import subprocess
import threading
import json
import time
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs
import os
import signal

RUSTFRIDA_BIN = "/data/adb/modules/rustfrida-kernelsu/bin/rustfrida"
RUSTFRIDA_PORT = 27042
WEBUI_PORT = 8080

class RustFridaManager:
    def __init__(self):
        self.process = None
        self.output_lines = []
        self.max_lines = 1000
        
    def start(self):
        if self.process and self.process.poll() is None:
            return {"status": "already_running", "pid": self.process.pid}
        
        try:
            self.process = subprocess.Popen(
                [RUSTFRIDA_BIN, "--server", "--rpc-port", f"0.0.0.0:{RUSTFRIDA_PORT}"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                bufsize=1
            )
            
            # 启动输出读取线程
            threading.Thread(target=self._read_output, daemon=True).start()
            
            time.sleep(2)
            if self.process.poll() is not None:
                return {"status": "failed", "error": "Process exited immediately"}
            
            return {"status": "started", "pid": self.process.pid}
        except Exception as e:
            return {"status": "error", "error": str(e)}
    
    def stop(self):
        if not self.process or self.process.poll() is not None:
            return {"status": "not_running"}
        
        try:
            self.process.stdin.write("exit\n")
            self.process.stdin.flush()
            self.process.wait(timeout=5)
            return {"status": "stopped"}
        except:
            self.process.kill()
            return {"status": "killed"}
    
    def send_command(self, cmd):
        if not self.process or self.process.poll() is not None:
            return {"status": "error", "error": "Process not running"}
        
        try:
            self.process.stdin.write(cmd + "\n")
            self.process.stdin.flush()
            return {"status": "sent"}
        except Exception as e:
            return {"status": "error", "error": str(e)}
    
    def get_status(self):
        if not self.process:
            return {"running": False}
        
        poll = self.process.poll()
        return {
            "running": poll is None,
            "pid": self.process.pid if poll is None else None,
            "output_lines": len(self.output_lines)
        }
    
    def get_output(self, lines=50):
        return self.output_lines[-lines:]
    
    def _read_output(self):
        while self.process and self.process.poll() is None:
            try:
                line = self.process.stdout.readline()
                if line:
                    self.output_lines.append(line.rstrip())
                    if len(self.output_lines) > self.max_lines:
                        self.output_lines = self.output_lines[-self.max_lines:]
            except:
                break

manager = RustFridaManager()

class WebUIHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass  # 禁用日志
    
    def do_GET(self):
        parsed = urlparse(self.path)
        
        if parsed.path == "/":
            self.send_html()
        elif parsed.path == "/api/status":
            self.send_json(manager.get_status())
        elif parsed.path == "/api/output":
            params = parse_qs(parsed.query)
            lines = int(params.get("lines", [50])[0])
            self.send_json({"output": manager.get_output(lines)})
        else:
            self.send_error(404)
    
    def do_POST(self):
        parsed = urlparse(self.path)
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8')
        
        try:
            data = json.loads(body) if body else {}
        except:
            data = {}
        
        if parsed.path == "/api/start":
            self.send_json(manager.start())
        elif parsed.path == "/api/stop":
            self.send_json(manager.stop())
        elif parsed.path == "/api/command":
            cmd = data.get("command", "")
            self.send_json(manager.send_command(cmd))
        else:
            self.send_error(404)
    
    def send_json(self, data):
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        self.wfile.write(json.dumps(data).encode())
    
    def send_html(self):
        html = """<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>rustFrida Web UI</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: monospace; background: #1e1e1e; color: #d4d4d4; padding: 20px; }
        .container { max-width: 1200px; margin: 0 auto; }
        h1 { color: #4ec9b0; margin-bottom: 20px; }
        .status { background: #252526; padding: 15px; border-radius: 5px; margin-bottom: 20px; }
        .status-item { display: inline-block; margin-right: 20px; }
        .status-label { color: #858585; }
        .status-value { color: #4ec9b0; font-weight: bold; }
        .running { color: #4ec9b0; }
        .stopped { color: #f48771; }
        .controls { margin-bottom: 20px; }
        button { background: #0e639c; color: white; border: none; padding: 10px 20px; 
                 border-radius: 3px; cursor: pointer; margin-right: 10px; font-size: 14px; }
        button:hover { background: #1177bb; }
        button:disabled { background: #3e3e42; cursor: not-allowed; }
        .command-input { background: #252526; border: 1px solid #3e3e42; color: #d4d4d4; 
                        padding: 10px; width: 400px; border-radius: 3px; margin-right: 10px; }
        .output { background: #1e1e1e; border: 1px solid #3e3e42; padding: 15px; 
                 border-radius: 5px; height: 500px; overflow-y: auto; font-size: 13px; }
        .output-line { margin: 2px 0; white-space: pre-wrap; }
        .quick-commands { margin-bottom: 20px; }
        .quick-btn { background: #2d2d30; padding: 8px 15px; font-size: 12px; }
        .quick-btn:hover { background: #3e3e42; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🦀 rustFrida Web UI</h1>
        
        <div class="status" id="status">
            <span class="status-item">
                <span class="status-label">Status:</span>
                <span class="status-value" id="status-text">Loading...</span>
            </span>
            <span class="status-item">
                <span class="status-label">PID:</span>
                <span class="status-value" id="pid-text">-</span>
            </span>
            <span class="status-item">
                <span class="status-label">RPC Port:</span>
                <span class="status-value">""" + str(RUSTFRIDA_PORT) + """</span>
            </span>
        </div>
        
        <div class="controls">
            <button id="start-btn" onclick="startServer()">▶ Start Server</button>
            <button id="stop-btn" onclick="stopServer()">⏹ Stop Server</button>
            <button onclick="clearOutput()">🗑 Clear Output</button>
        </div>
        
        <div class="quick-commands">
            <strong>Quick Commands:</strong>
            <button class="quick-btn" onclick="sendCommand('list')">list</button>
            <button class="quick-btn" onclick="sendCommand('help')">help</button>
            <button class="quick-btn" onclick="sendCommand('spawn com.android.settings')">spawn settings</button>
        </div>
        
        <div style="margin-bottom: 10px;">
            <input type="text" id="command-input" class="command-input" 
                   placeholder="Enter command (e.g., spawn com.app, attach 1234, list)" 
                   onkeypress="if(event.key==='Enter') sendCommand()">
            <button onclick="sendCommand()">Send Command</button>
        </div>
        
        <div class="output" id="output"></div>
    </div>
    
    <script>
        let autoScroll = true;
        
        async function startServer() {
            const res = await fetch('/api/start', {method: 'POST'});
            const data = await res.json();
            alert(data.status === 'started' ? 'Server started!' : 'Error: ' + (data.error || data.status));
            updateStatus();
        }
        
        async function stopServer() {
            if (!confirm('Stop rustFrida server?')) return;
            const res = await fetch('/api/stop', {method: 'POST'});
            const data = await res.json();
            alert('Server stopped');
            updateStatus();
        }
        
        async function sendCommand(cmd) {
            if (!cmd) cmd = document.getElementById('command-input').value;
            if (!cmd) return;
            
            const res = await fetch('/api/command', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({command: cmd})
            });
            const data = await res.json();
            
            if (data.status === 'error') {
                alert('Error: ' + data.error);
            } else {
                document.getElementById('command-input').value = '';
            }
        }
        
        async function updateStatus() {
            const res = await fetch('/api/status');
            const data = await res.json();
            
            const statusText = document.getElementById('status-text');
            const pidText = document.getElementById('pid-text');
            const startBtn = document.getElementById('start-btn');
            const stopBtn = document.getElementById('stop-btn');
            
            if (data.running) {
                statusText.textContent = 'Running';
                statusText.className = 'status-value running';
                pidText.textContent = data.pid;
                startBtn.disabled = true;
                stopBtn.disabled = false;
            } else {
                statusText.textContent = 'Stopped';
                statusText.className = 'status-value stopped';
                pidText.textContent = '-';
                startBtn.disabled = false;
                stopBtn.disabled = true;
            }
        }
        
        async function updateOutput() {
            const res = await fetch('/api/output?lines=100');
            const data = await res.json();
            const output = document.getElementById('output');
            const shouldScroll = autoScroll && (output.scrollHeight - output.scrollTop - output.clientHeight < 50);
            
            output.innerHTML = data.output.map(line => 
                `<div class="output-line">${escapeHtml(line)}</div>`
            ).join('');
            
            if (shouldScroll) {
                output.scrollTop = output.scrollHeight;
            }
        }
        
        function clearOutput() {
            document.getElementById('output').innerHTML = '';
        }
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        // 自动更新
        setInterval(updateStatus, 2000);
        setInterval(updateOutput, 1000);
        updateStatus();
        updateOutput();
        
        // Enter 键发送命令
        document.getElementById('command-input').focus();
    </script>
</body>
</html>"""
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.end_headers()
        self.wfile.write(html.encode())

def main():
    print(f"Starting rustFrida Web UI on port {WEBUI_PORT}...")
    print(f"Open http://localhost:{WEBUI_PORT} in your browser")
    
    # 自动启动 rustfrida
    print("Auto-starting rustFrida server...")
    result = manager.start()
    print(f"Result: {result}")
    
    server = HTTPServer(("0.0.0.0", WEBUI_PORT), WebUIHandler)
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down...")
        manager.stop()
        server.shutdown()

if __name__ == "__main__":
    main()

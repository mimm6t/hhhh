#!/usr/bin/env python3
"""
rustFrida Manager - LSPosed-style Module Manager
完整的应用管理、Hook 脚本管理、日志查看系统
"""

import subprocess
import threading
import json
import time
import os
import re
from http.server import HTTPServer, BaseHTTPRequestHandler
from urllib.parse import urlparse, parse_qs

RUSTFRIDA_BIN = "/data/adb/modules/rustfrida-kernelsu/bin/rustfrida"
SCRIPTS_DIR = "/data/adb/rustfrida/scripts"
HOOKS_CONFIG = "/data/adb/rustfrida/hooks.json"
RUSTFRIDA_PORT = 27042
WEBUI_PORT = 8080

os.makedirs(SCRIPTS_DIR, exist_ok=True)

class AppManager:
    """应用管理器"""
    def get_installed_apps(self):
        try:
            result = subprocess.run(
                ["pm", "list", "packages", "-3"],
                capture_output=True, text=True, timeout=10
            )
            apps = []
            for line in result.stdout.strip().split('\n'):
                if line.startswith('package:'):
                    pkg = line.replace('package:', '')
                    apps.append({
                        "package": pkg,
                        "name": self._get_app_name(pkg),
                        "enabled": self._is_hook_enabled(pkg)
                    })
            return sorted(apps, key=lambda x: x['name'])
        except:
            return []
    
    def _get_app_name(self, package):
        try:
            result = subprocess.run(
                ["pm", "dump", package],
                capture_output=True, text=True, timeout=5
            )
            for line in result.stdout.split('\n'):
                if 'applicationInfo' in line or 'label' in line:
                    match = re.search(r'label=([^\s]+)', line)
                    if match:
                        return match.group(1)
            return package.split('.')[-1]
        except:
            return package.split('.')[-1]
    
    def _is_hook_enabled(self, package):
        if not os.path.exists(HOOKS_CONFIG):
            return False
        try:
            with open(HOOKS_CONFIG) as f:
                config = json.load(f)
                return package in config.get('enabled', [])
        except:
            return False
    
    def get_running_apps(self):
        try:
            result = subprocess.run(
                ["ps", "-A"],
                capture_output=True, text=True, timeout=5
            )
            apps = set()
            for line in result.stdout.split('\n'):
                parts = line.split()
                if len(parts) > 8 and '.' in parts[-1]:
                    apps.add(parts[-1])
            return list(apps)
        except:
            return []

class ScriptManager:
    """Hook 脚本管理器"""
    def list_scripts(self):
        scripts = []
        if os.path.exists(SCRIPTS_DIR):
            for f in os.listdir(SCRIPTS_DIR):
                if f.endswith('.js'):
                    path = os.path.join(SCRIPTS_DIR, f)
                    scripts.append({
                        "name": f,
                        "size": os.path.getsize(path),
                        "modified": os.path.getmtime(path)
                    })
        return scripts
    
    def get_script(self, name):
        path = os.path.join(SCRIPTS_DIR, name)
        if os.path.exists(path):
            with open(path) as f:
                return f.read()
        return None
    
    def save_script(self, name, content):
        path = os.path.join(SCRIPTS_DIR, name)
        with open(path, 'w') as f:
            f.write(content)
        return True
    
    def delete_script(self, name):
        path = os.path.join(SCRIPTS_DIR, name)
        if os.path.exists(path):
            os.remove(path)
            return True
        return False

class HookManager:
    """Hook 配置管理器"""
    def get_config(self):
        if os.path.exists(HOOKS_CONFIG):
            with open(HOOKS_CONFIG) as f:
                return json.load(f)
        return {"enabled": [], "hooks": {}}
    
    def save_config(self, config):
        with open(HOOKS_CONFIG, 'w') as f:
            json.dump(config, f, indent=2)
    
    def enable_hook(self, package, script):
        config = self.get_config()
        if package not in config['enabled']:
            config['enabled'].append(package)
        config['hooks'][package] = script
        self.save_config(config)
    
    def disable_hook(self, package):
        config = self.get_config()
        if package in config['enabled']:
            config['enabled'].remove(package)
        if package in config['hooks']:
            del config['hooks'][package]
        self.save_config(config)
    
    def get_hook_script(self, package):
        config = self.get_config()
        return config.get('hooks', {}).get(package)

class RustFridaManager:
    def __init__(self):
        self.process = None
        self.sessions = {}
        self.output_lines = []
        self.max_lines = 2000
        
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
            
            threading.Thread(target=self._read_output, daemon=True).start()
            time.sleep(2)
            
            if self.process.poll() is not None:
                return {"status": "failed", "error": "Process exited"}
            
            return {"status": "started", "pid": self.process.pid}
        except Exception as e:
            return {"status": "error", "error": str(e)}
    
    def spawn_with_hook(self, package, script_name):
        """Spawn 应用并加载 Hook 脚本"""
        script_path = os.path.join(SCRIPTS_DIR, script_name)
        if not os.path.exists(script_path):
            return {"status": "error", "error": "Script not found"}
        
        cmd = f"spawn {package} -l {script_path}"
        return self.send_command(cmd)
    
    def attach_with_hook(self, pid, script_name):
        """附加到进程并加载 Hook 脚本"""
        script_path = os.path.join(SCRIPTS_DIR, script_name)
        if not os.path.exists(script_path):
            return {"status": "error", "error": "Script not found"}
        
        cmd = f"attach {pid} -l {script_path}"
        return self.send_command(cmd)
    
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
            "sessions": len(self.sessions),
            "output_lines": len(self.output_lines)
        }
    
    def get_output(self, lines=100):
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

app_mgr = AppManager()
script_mgr = ScriptManager()
hook_mgr = HookManager()
frida_mgr = RustFridaManager()

class WebUIHandler(BaseHTTPRequestHandler):
    def log_message(self, format, *args):
        pass
    
    def do_GET(self):
        parsed = urlparse(self.path)
        
        if parsed.path == "/":
            self.send_html()
        elif parsed.path == "/api/apps":
            self.send_json(app_mgr.get_installed_apps())
        elif parsed.path == "/api/running":
            self.send_json(app_mgr.get_running_apps())
        elif parsed.path == "/api/scripts":
            self.send_json(script_mgr.list_scripts())
        elif parsed.path.startswith("/api/script/"):
            name = parsed.path.split("/")[-1]
            content = script_mgr.get_script(name)
            if content:
                self.send_json({"content": content})
            else:
                self.send_error(404)
        elif parsed.path == "/api/status":
            self.send_json(frida_mgr.get_status())
        elif parsed.path == "/api/output":
            params = parse_qs(parsed.query)
            lines = int(params.get("lines", [100])[0])
            self.send_json({"output": frida_mgr.get_output(lines)})
        elif parsed.path == "/api/hooks":
            self.send_json(hook_mgr.get_config())
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
            self.send_json(frida_mgr.start())
        elif parsed.path == "/api/spawn":
            pkg = data.get("package")
            script = data.get("script")
            self.send_json(frida_mgr.spawn_with_hook(pkg, script))
        elif parsed.path == "/api/attach":
            pid = data.get("pid")
            script = data.get("script")
            self.send_json(frida_mgr.attach_with_hook(pid, script))
        elif parsed.path == "/api/hook/enable":
            hook_mgr.enable_hook(data.get("package"), data.get("script"))
            self.send_json({"status": "ok"})
        elif parsed.path == "/api/hook/disable":
            hook_mgr.disable_hook(data.get("package"))
            self.send_json({"status": "ok"})
        elif parsed.path == "/api/script/save":
            script_mgr.save_script(data.get("name"), data.get("content"))
            self.send_json({"status": "ok"})
        elif parsed.path == "/api/script/delete":
            script_mgr.delete_script(data.get("name"))
            self.send_json({"status": "ok"})
        elif parsed.path == "/api/command":
            self.send_json(frida_mgr.send_command(data.get("command", "")))
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
    <title>rustFrida Manager</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; 
               background: #f5f5f5; }
        .header { background: #6200ea; color: white; padding: 16px 20px; 
                  box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        .header h1 { font-size: 20px; font-weight: 500; }
        .tabs { background: white; display: flex; border-bottom: 1px solid #e0e0e0; }
        .tab { padding: 16px 24px; cursor: pointer; border-bottom: 2px solid transparent; 
               transition: all 0.3s; }
        .tab:hover { background: #f5f5f5; }
        .tab.active { border-bottom-color: #6200ea; color: #6200ea; font-weight: 500; }
        .content { padding: 20px; }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
        .card { background: white; border-radius: 8px; padding: 16px; margin-bottom: 16px; 
                box-shadow: 0 1px 3px rgba(0,0,0,0.1); }
        .app-item { padding: 12px; border-bottom: 1px solid #f0f0f0; display: flex; 
                    align-items: center; justify-content: space-between; }
        .app-item:last-child { border-bottom: none; }
        .app-info { flex: 1; }
        .app-name { font-weight: 500; margin-bottom: 4px; }
        .app-package { font-size: 12px; color: #666; }
        .switch { position: relative; width: 48px; height: 24px; }
        .switch input { opacity: 0; width: 0; height: 0; }
        .slider { position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0;
                  background-color: #ccc; transition: .4s; border-radius: 24px; }
        .slider:before { position: absolute; content: ""; height: 18px; width: 18px; left: 3px;
                        bottom: 3px; background-color: white; transition: .4s; border-radius: 50%; }
        input:checked + .slider { background-color: #6200ea; }
        input:checked + .slider:before { transform: translateX(24px); }
        .btn { background: #6200ea; color: white; border: none; padding: 10px 20px; 
               border-radius: 4px; cursor: pointer; font-size: 14px; }
        .btn:hover { background: #7c4dff; }
        .btn-secondary { background: #757575; }
        .btn-secondary:hover { background: #616161; }
        .script-list { max-height: 400px; overflow-y: auto; }
        .script-item { padding: 12px; border-bottom: 1px solid #f0f0f0; display: flex;
                      justify-content: space-between; align-items: center; }
        .editor { width: 100%; height: 400px; font-family: monospace; padding: 12px;
                 border: 1px solid #ddd; border-radius: 4px; resize: vertical; }
        .status-bar { background: #e8f5e9; padding: 12px; border-radius: 4px; margin-bottom: 16px; }
        .status-bar.error { background: #ffebee; }
        .log-output { background: #1e1e1e; color: #d4d4d4; padding: 16px; border-radius: 4px;
                     height: 400px; overflow-y: auto; font-family: monospace; font-size: 13px; }
        .search-box { width: 100%; padding: 12px; border: 1px solid #ddd; border-radius: 4px;
                     margin-bottom: 16px; font-size: 14px; }
        .empty-state { text-align: center; padding: 40px; color: #999; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🦀 rustFrida Manager</h1>
    </div>
    
    <div class="tabs">
        <div class="tab active" onclick="switchTab('apps')">应用列表</div>
        <div class="tab" onclick="switchTab('scripts')">Hook 脚本</div>
        <div class="tab" onclick="switchTab('logs')">日志</div>
        <div class="tab" onclick="switchTab('settings')">设置</div>
    </div>
    
    <div class="content">
        <!-- 应用列表 -->
        <div id="apps-tab" class="tab-content active">
            <div class="card">
                <input type="text" class="search-box" id="app-search" placeholder="搜索应用..." onkeyup="filterApps()">
                <div id="apps-list"></div>
            </div>
        </div>
        
        <!-- Hook 脚本 -->
        <div id="scripts-tab" class="tab-content">
            <div class="card">
                <button class="btn" onclick="showNewScript()">+ 新建脚本</button>
                <div class="script-list" id="scripts-list"></div>
            </div>
            
            <div id="script-editor" class="card" style="display:none;">
                <h3>编辑脚本</h3>
                <input type="text" id="script-name" placeholder="脚本名称 (例: hook.js)" style="width:100%; padding:8px; margin:10px 0;">
                <textarea id="script-content" class="editor" placeholder="// JavaScript Hook 代码"></textarea>
                <button class="btn" onclick="saveScript()">保存</button>
                <button class="btn btn-secondary" onclick="cancelEdit()">取消</button>
            </div>
        </div>
        
        <!-- 日志 -->
        <div id="logs-tab" class="tab-content">
            <div class="status-bar" id="status-info">
                <strong>状态:</strong> <span id="status-text">加载中...</span>
            </div>
            <div class="card">
                <button class="btn" onclick="clearLogs()">清空日志</button>
                <div class="log-output" id="log-output"></div>
            </div>
        </div>
        
        <!-- 设置 -->
        <div id="settings-tab" class="tab-content">
            <div class="card">
                <h3>服务控制</h3>
                <button class="btn" onclick="startServer()">启动服务</button>
                <button class="btn btn-secondary" onclick="stopServer()">停止服务</button>
            </div>
            <div class="card">
                <h3>关于</h3>
                <p>rustFrida Manager v1.0</p>
                <p>基于 rustFrida 的 LSPosed 风格管理器</p>
            </div>
        </div>
    </div>
    
    <script>
        let apps = [];
        let scripts = [];
        let hooks = {};
        
        function switchTab(tab) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            event.target.classList.add('active');
            document.getElementById(tab + '-tab').classList.add('active');
            
            if (tab === 'apps') loadApps();
            if (tab === 'scripts') loadScripts();
            if (tab === 'logs') updateLogs();
        }
        
        async function loadApps() {
            const res = await fetch('/api/apps');
            apps = await res.json();
            const hooksRes = await fetch('/api/hooks');
            hooks = await hooksRes.json();
            renderApps();
        }
        
        function renderApps() {
            const list = document.getElementById('apps-list');
            const search = document.getElementById('app-search').value.toLowerCase();
            const filtered = apps.filter(app => 
                app.name.toLowerCase().includes(search) || 
                app.package.toLowerCase().includes(search)
            );
            
            if (filtered.length === 0) {
                list.innerHTML = '<div class="empty-state">没有找到应用</div>';
                return;
            }
            
            list.innerHTML = filtered.map(app => `
                <div class="app-item">
                    <div class="app-info">
                        <div class="app-name">${app.name}</div>
                        <div class="app-package">${app.package}</div>
                    </div>
                    <label class="switch">
                        <input type="checkbox" ${app.enabled ? 'checked' : ''} 
                               onchange="toggleHook('${app.package}', this.checked)">
                        <span class="slider"></span>
                    </label>
                </div>
            `).join('');
        }
        
        function filterApps() {
            renderApps();
        }
        
        async function toggleHook(pkg, enabled) {
            if (enabled) {
                const script = prompt('选择 Hook 脚本 (输入脚本名称):');
                if (!script) return;
                await fetch('/api/hook/enable', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({package: pkg, script: script})
                });
                alert('Hook 已启用，下次启动应用时生效');
            } else {
                await fetch('/api/hook/disable', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({package: pkg})
                });
            }
            loadApps();
        }
        
        async function loadScripts() {
            const res = await fetch('/api/scripts');
            scripts = await res.json();
            renderScripts();
        }
        
        function renderScripts() {
            const list = document.getElementById('scripts-list');
            if (scripts.length === 0) {
                list.innerHTML = '<div class="empty-state">还没有 Hook 脚本</div>';
                return;
            }
            
            list.innerHTML = scripts.map(s => `
                <div class="script-item">
                    <div>
                        <strong>${s.name}</strong>
                        <span style="color:#666; font-size:12px; margin-left:10px;">
                            ${(s.size / 1024).toFixed(1)} KB
                        </span>
                    </div>
                    <div>
                        <button class="btn" style="padding:6px 12px; font-size:12px;" 
                                onclick="editScript('${s.name}')">编辑</button>
                        <button class="btn btn-secondary" style="padding:6px 12px; font-size:12px;" 
                                onclick="deleteScript('${s.name}')">删除</button>
                    </div>
                </div>
            `).join('');
        }
        
        function showNewScript() {
            document.getElementById('script-name').value = '';
            document.getElementById('script-content').value = `// Hook 模板
console.log("[*] Script loaded");

Java.ready(function() {
    console.log("[*] Java runtime ready");
    
    // 在这里编写你的 Hook 代码
});`;
            document.getElementById('script-editor').style.display = 'block';
        }
        
        async function editScript(name) {
            const res = await fetch('/api/script/' + name);
            const data = await res.json();
            document.getElementById('script-name').value = name;
            document.getElementById('script-content').value = data.content;
            document.getElementById('script-editor').style.display = 'block';
        }
        
        async function saveScript() {
            const name = document.getElementById('script-name').value;
            const content = document.getElementById('script-content').value;
            if (!name) {
                alert('请输入脚本名称');
                return;
            }
            await fetch('/api/script/save', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({name, content})
            });
            alert('脚本已保存');
            cancelEdit();
            loadScripts();
        }
        
        function cancelEdit() {
            document.getElementById('script-editor').style.display = 'none';
        }
        
        async function deleteScript(name) {
            if (!confirm('确定删除脚本 ' + name + '?')) return;
            await fetch('/api/script/delete', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({name})
            });
            loadScripts();
        }
        
        async function updateLogs() {
            const res = await fetch('/api/output?lines=200');
            const data = await res.json();
            const output = document.getElementById('log-output');
            output.innerHTML = data.output.map(line => 
                `<div>${escapeHtml(line)}</div>`
            ).join('');
            output.scrollTop = output.scrollHeight;
            
            const statusRes = await fetch('/api/status');
            const status = await statusRes.json();
            const statusText = document.getElementById('status-text');
            const statusInfo = document.getElementById('status-info');
            if (status.running) {
                statusText.textContent = `运行中 (PID: ${status.pid})`;
                statusInfo.className = 'status-bar';
            } else {
                statusText.textContent = '已停止';
                statusInfo.className = 'status-bar error';
            }
        }
        
        function clearLogs() {
            document.getElementById('log-output').innerHTML = '';
        }
        
        async function startServer() {
            const res = await fetch('/api/start', {method: 'POST'});
            const data = await res.json();
            alert(data.status === 'started' ? '服务已启动' : '启动失败: ' + (data.error || data.status));
        }
        
        async function stopServer() {
            alert('服务由 Web UI 管理，无需手动停止');
        }
        
        function escapeHtml(text) {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        // 自动更新
        setInterval(() => {
            const activeTab = document.querySelector('.tab-content.active').id;
            if (activeTab === 'logs-tab') updateLogs();
        }, 2000);
        
        // 初始化
        loadApps();
    </script>
</body>
</html>"""
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.end_headers()
        self.wfile.write(html.encode())

def main():
    print(f"Starting rustFrida Manager on port {WEBUI_PORT}...")
    print(f"Open http://localhost:{WEBUI_PORT} in your browser")
    
    print("Auto-starting rustFrida server...")
    result = frida_mgr.start()
    print(f"Result: {result}")
    
    server = HTTPServer(("0.0.0.0", WEBUI_PORT), WebUIHandler)
    
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down...")
        server.shutdown()

if __name__ == "__main__":
    main()

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

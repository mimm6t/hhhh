#!/usr/bin/env python3
import subprocess
import shutil
import zipfile
import os
from pathlib import Path

PATH_BASE = Path(__file__).parent.resolve()
PATH_BUILD = PATH_BASE / "build"
PATH_RUSTFRIDA = PATH_BASE.parent / "rustFrida-master"

def build_rustfrida():
    """构建 rustfrida 二进制"""
    print("[1/4] Building loader shellcode...")
    subprocess.run(
        ["python3", "build_helpers.py"],
        cwd=PATH_RUSTFRIDA / "loader",
        check=True
    )
    
    print("[2/4] Building agent...")
    subprocess.run(
        ["cargo", "build", "-p", "agent", "--release"],
        cwd=PATH_RUSTFRIDA,
        check=True
    )
    
    print("[3/4] Building rustfrida...")
    subprocess.run(
        ["cargo", "build", "-p", "rust_frida", "--release"],
        cwd=PATH_RUSTFRIDA,
        check=True
    )
    
    return PATH_RUSTFRIDA / "target/aarch64-linux-android/release/rustfrida"

def create_module(version: str):
    """创建模块结构"""
    print("[4/4] Creating module...")
    
    PATH_BUILD.mkdir(exist_ok=True)
    tmp = PATH_BUILD / "tmp"
    
    if tmp.exists():
        shutil.rmtree(tmp)
    
    shutil.copytree(PATH_BASE / "base", tmp)
    
    (tmp / "bin").mkdir(exist_ok=True)
    (tmp / "config").mkdir(exist_ok=True)
    (tmp / "logs").mkdir(exist_ok=True)
    
    return tmp

def package_module(tmp: Path, rustfrida_bin: Path, version: str):
    """打包模块"""
    # 复制二进制
    shutil.copy2(rustfrida_bin, tmp / "bin/rustfrida")
    (tmp / "bin/rustfrida").chmod(0o755)
    
    # 创建 files 目录用于 customize.sh
    files_dir = tmp / "files"
    files_dir.mkdir(exist_ok=True)
    shutil.copy2(rustfrida_bin, files_dir / "rustfrida")
    
    # 打包
    zip_path = PATH_BUILD / f"rustFrida-KernelSU-{version}.zip"
    
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for root, dirs, files in os.walk(tmp):
            for file in files:
                file_path = Path(root) / file
                arcname = file_path.relative_to(tmp)
                zf.write(file_path, arcname)
    
    shutil.rmtree(tmp)
    print(f"\n✅ Module created: {zip_path}")
    return zip_path

def main():
    version = "0.16.10"
    
    # 构建
    rustfrida_bin = build_rustfrida()
    
    # 打包
    tmp = create_module(version)
    package_module(tmp, rustfrida_bin, version)

if __name__ == "__main__":
    main()

import subprocess
import sys
import shutil
import os

git_repo = "https://github.com/jibrildeoculos/emojirs_ls"

ROOT = os.getcwd()

if sys.platform == "win32":
    local_install = os.path.join(os.environ["LOCALAPPDATA"], "Programs", "emojirs_ls")
else:
    local_install = "/usr/local/bin/"

def clone():
    print("Cloning Repository...")
    if not os.path.exists("emojirs_ls"):
        subprocess.run(f"git clone {git_repo}", shell=True, check=True)
    else:
        print("Repository already exists. Skipping clone.")

def build():
    print("Building...")
    os.chdir("emojirs_ls")
    subprocess.run("cargo build --release", shell=True, check=True)
    os.chdir(ROOT)

def install():
    print("Installing to", local_install)
    os.makedirs(local_install, exist_ok=True)

    src = (
        "emojirs_ls/target/release/emojirs_ls.exe"
        if sys.platform == "win32"
        else "emojirs_ls/target/release/emojirs_ls"
    )
    dest = os.path.join(local_install, os.path.basename(src))

    if os.path.exists(dest):
        os.remove(dest)

    shutil.move(src, dest)
    print("Installed:", dest)

def path():
    print("Adding to PATH...")
    if sys.platform == "win32":
        command = f'setx PATH "%PATH%;{local_install}"'
        subprocess.run(command, shell=True)
        print("Restart CMD to apply PATH changes.")

if __name__ == "__main__":
    clone()
    build()
    install()
    if sys.platform == "win32":
        path()

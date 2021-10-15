import sys
import toml
import json

config = toml.load(sys.argv[1])

includes = []
for target in config["toolchain"]["targets"]:
    im_arch = ""
    
    x64 = False
    x86 = False
    aarch64 = False
    arm = False
    
    windows = False
    android = False
    linux = False
    mac = False
    tool = ""
    output = ""
    profile = config["toolchain"]["profile"]

    # figure out the arch of target
    if "x86_64" in target:
        im_arch = "x64"
        x64 = True
    elif "i686" in target:
        im_arch = "x86"
        x86 = True
    elif "aarch64" in target:
        aarch64 = True
    elif "arm" in target:
        arm = True
    
    # handle different systems
    if "windows" in target:
        windows = True
        output = "kmagick.dll"
        tool = "build-win.ps1"
    elif "android" in target:
        android = True
        output = "libkmagick.so"
        tool = "build-android.ps1"
    elif "linux" in target:
        linux = True
        output = "libkmagick.so"
    elif "apple" in target:
        mac = True

    # tell it which arch to compile for
    if x64:
        tool += " -arch x86_64"
    elif x86:
        tool += " -arch x86"
    elif arm:
        tool += " -arch arm"
    elif aarch64:
        tool += " -arch aarch64"

    release_tool = tool
    release_tool += " -release"

    t = {
        "target": target,
        "windows": windows,
        "android": android,
        "linux": linux,
        "mac": mac,
        "im_arch": im_arch,
        "output": output,
        "profile": profile,
        "debug": {
            "tool": tool
        },
        "release": {
            "tool": release_tool
        }
    }

    includes.append(t)

matrixConfig = {
    "target": config["toolchain"]["targets"],
    "include": includes
}

print("::set-output name=matrix::" + json.dumps(matrixConfig))

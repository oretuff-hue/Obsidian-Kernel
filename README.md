**Not Compilled!**
**Not tested!**
**Unstable**

# Obsidian Kernel
*v0.0.3-pre-alpha-rc1*
> Formerly Kernel Route

## 1. What is Obsidian Kernel?

Is an open-source kernel written from scratch in Rust. Can be used as the kernel of your own project or in RoadingOS

## 2. What is RoadingOS?

Is an operating system that aims to correct things that Linux and other systems ignore. RoadingOS don't use POSIX or Unix legacy. There are diferent directories structure, diferent commands and others.

### 2.1 RoadingOS directories structure

- /system: kernels, DEs, WMs, graphic servers, shell, binaries, libraries and other essential components to make the system
- /packages: Packages of the type Packages (topic 2.3), the executables and binaries for reconstruction.
- /programs: Packages of the type Programs (topic 2.3), the executables and binaries for reconstruction.
- /config: Configurations of all the system, of one or all the users, packages of all types and can be copied to other installations.
- /boot: Bootloaders and init systems.
- /user: This folder contains the personal folder of all the users, including the root (or administrator).

### 2.2 What elements make up RoadingOS?

Roading will be composed of several components:

- **Obsidian**: the kernel.
- **Excavator**: the bootloader.
- **Asphalt**: the init system.
- **LibOsmium**: the libc.
- **Nickel**: essential utilities 
(Diamond, compact files and folders,
Drill, network manager,
Titanium, package manager,
Makeiso, create and extract isos,
Brick, compiller,
BetterGit, TUI Git,
Malachite, GUI file manager,
Pyrite, TUI file manager,
Paving, GUI text editor and
Iridium, TUI text editor,
Molybdenum, multimedia player (videos and audios),
Tecnecium, disk manager,
Polaris, configurations app).
- **Steamroller**: the shell.
- **Jade**: the desktop environment.
- **Loam**: the graphic server.
- **Chromium**: the browser.
- **Basalt Suit**: the app suit, contains apps for Office, PDFs, LaTeX, audio/video/image edit, convert files to other file types, translate codes and IDE (Writer, like Word,
Sheets, like Excel, with support to Python and JS macros,
Slides, like PowerPoint,
PDFs, a reader and editor of PDFs,
LaTeX, to create documents and sites using LaTeX,
Morph, convert files to other files (like mp4 to mkv),
Code Morph, translate codes to other languages (like Rust to C),
Videos, lives, video edit and record,
Studio, professional video edit,
Audio, audio recorder and editor,
Images, image edit,
Designer, 3D modeling,
Sketch, like KolourPaint, Pinta or Microsoft Paint,
Sketch 3D, like Paint 3D,
Cadence, like AutoCad but just 2D,
Manganese, like Revit, 2D and 3D, 
Wonderterface, app, DE and site designs,
Cobalt Code Studio, IDE) 

### 2.3 RoadingOS package types

- Package of the type Package: CLI apps, TUI apps, themes, fonts and others. Is stored in /packages.
- Package of the type Program: GUI apps. Is stored in /programs.
- Package of the type System: Dependencies, libraries, DEs, WMs, kernels, graphic servers, shells and others. Is stored in /system/installed-by-user. 

## 3. Usage of Obsidian

You can use the Obsidian Kernel as the base of your own OS, it's called Obsidian distribution. Feel free to modify the Obsidian Kernel or integrate with your own projects. But you need to read the LICENSE.md (or, if you know the terms of the Apache License 2.0, not really need) and read the NOTICE.md.

## 4. License Note

If you want to redistribute the Obsidian or publish your distribution or modification of Obsidian you need to read the LICENSE.md and the NOTICE.md.

## 5. Glossary 

Kernel: Core of a Operating System
CLI: only-text interface, like apt, neofetch or git.
TUI: menus on terminal, don't need a desktop environment or window manager, like aptitude, elinks or nano.
GUI: graphic interface, where has a mouse pointer, colors, buttons and more, like Chrome or Spotify.
 
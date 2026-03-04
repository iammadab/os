# Run

## Prerequisites

- Install bootimage:

```bash
cargo install bootimage
```

- Install QEMU (`qemu-system-x86_64`) from your distro packages.
- Install a VNC viewer (for headless QEMU display), e.g. TigerVNC or Remmina.

## Run

```bash
cargo run
```

## View Display

- If QEMU opens a local window, use that window.
- If QEMU reports a VNC server (for example on `:5900`), connect with a VNC viewer:

```bash
vncviewer 127.0.0.1:5900
```

- In Remmina, create a VNC connection to `127.0.0.1:5900`.

## Arch Linux Notes

- Install QEMU:

```bash
sudo pacman -S qemu-system-x86
```

- Install TigerVNC:

```bash
sudo pacman -S tigervnc
```

- If Remmina has no VNC plugin, install missing libraries:

```bash
sudo pacman -S libvncserver gtk-vnc
```

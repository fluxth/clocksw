# clocksw

smart led wall clock display for raspberry pi

# Building

Dependencies for cross-compiling `clocksw`:
- a linux environment
- rustup
- clang
- llvm
- lld
- python3
- Raspberry Pi's `rootfs`
- [rpi-rgb-led-matrix](https://github.com/hzeller/rpi-rgb-led-matrix) (already cross-compiled for arm)

With all the dependencies installed, do:

```
make
```

For release builds, do `make release`.

---
&copy; fluxth 2020, All rights reserved. This program is licensed under GNU GPL v3.

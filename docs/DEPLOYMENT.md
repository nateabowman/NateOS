# NateOS Deployment Guide

## Prerequisites

- QEMU or physical x86_64 hardware
- Bootloader (GRUB or custom)
- Build tools (Rust, cargo)

## Building for Production

1. Clone the repository:
```bash
git clone <repository-url>
cd NateOS
```

2. Build the release kernel:
```bash
cargo build --release
```

3. The kernel binary will be at:
```
target/x86_64-nateos/release/nateos
```

## Deployment Options

### QEMU

Run in QEMU for testing:
```bash
qemu-system-x86_64 \
    -kernel target/x86_64-nateos/release/nateos \
    -serial stdio \
    -no-reboot \
    -no-shutdown
```

### Physical Hardware

1. Create bootable media (USB or CD)
2. Copy kernel to boot partition
3. Configure bootloader (GRUB) to load kernel
4. Boot from media

## Configuration

### Kernel Parameters

Kernel parameters can be passed via bootloader configuration:
- `aslr=on|off` - Enable/disable ASLR
- `stack_protection=on|off` - Enable/disable stack protection
- `profiling=on|off` - Enable/disable profiling

### System Limits

Edit `src/stability/resource_monitor.rs` to adjust:
- Maximum memory usage
- Maximum process count
- Watchdog timeout

## Monitoring

### Performance Metrics

Access performance metrics via:
- Profiler API
- Resource monitor
- System call statistics

### Logging

Kernel logs are output to:
- Serial port (COM1)
- VGA console
- Audit log (in-memory)

## Troubleshooting

### Kernel Panic

If kernel panics:
1. Check serial output for error message
2. Review panic handler output
3. Check memory map configuration
4. Verify bootloader compatibility

### Boot Issues

If kernel doesn't boot:
1. Verify multiboot2 compliance
2. Check memory map from bootloader
3. Ensure correct entry point
4. Verify linker script

### Performance Issues

If experiencing performance problems:
1. Enable profiler
2. Check scheduler load
3. Monitor memory usage
4. Review I/O scheduler settings

## Production Checklist

- [ ] Build with release optimizations
- [ ] Strip debug symbols (optional)
- [ ] Configure security features
- [ ] Set appropriate system limits
- [ ] Test on target hardware
- [ ] Document configuration
- [ ] Create backup/recovery plan


# gpustat

<del>It fail to compile due to [https://github.com/alexcrichton/backtrace-rs/issues/136](https://github.com/alexcrichton/backtrace-rs/issues/136).</del>

<del>Stop developing for a while.</del>

Fixed by running
```
cargo update -p libc
```

## Compilation

Check [Cldfire/nvml-wrapper](https://github.com/Cldfire/nvml-wrapper)

> The NVML library can be found at /usr/lib/nvidia-<driver-version>/libnvidia-ml.so; on my system with driver version 375.51 installed, this puts the library at /usr/lib/nvidia-375/libnvidia-ml.so. You will need to create a symbolic link:

```
sudo ln -s /usr/lib/nvidia-<driver-version>/libnvidia-ml.so /usr/lib
```

## Limitation

This project is for machine learning, so it shows computation processes only. Your graphic processes won't be taken into account.
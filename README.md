# rpi-video-rs

### Camera Device Installation

#### 1. Adds `bcm2835-v4l2` to the bottom of file `/etc/modules`.

```
sudo echo bcm2835-v4l2 >> /etc/modules
```

#### 2. Activates the camera in configuration.

```
sudo raspi-config
```

#### 3. Follows the prompt to reboot Raspberry PI.

#### 4. You could use the command `raspivid` (or the below C version code) to test recording a `H264` video.

```
raspivid -o test_video.h264
```

### C Version Code

#### 1. Enters into folder `tools/c_ver` and runs `make` to build the command `rpi_video.out`.

```
pushd tools/c_ver
make
```

#### 2. Runs the `rpi_video.out` to record a sample video.

```
./rpi_video.out
```


rustup target add arm-unknown-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf

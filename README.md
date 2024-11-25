# touch_sampling
设置一加13的触控采样率，通过读取fas-rs配置文件实现
### 编译
#### 基础环境配置
- 下载容器
```shell
pkg install proot -y; pkg install proot-distro -y; proot-distro add archlinux
```

- 登录容器

```shell
proot-distro login archlinux
```

- 更新源

```shell
yes | pacman -Sy
```

- 安装依赖
```shell
yes | pacman -S llvm clang python glibc make cmake
```
安装glibc是防止以下问题:
```
= note: cc: /usr/lib/libc.so.6: version `GLIBC_2.36' not found (required by cc)
      cc: /usr/lib/libc.so.6: version `GLIBC_2.38' not found (required by cc)
```
- 安装rust
> 默认为nightly，default
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile default -y

rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

rustup component add rust-src

cargo install cargo-ndk
```
```
- 下载android NDK
- aarch64架构(termux请使用此工具链):

  https://github.com/Lzhiyong/termux-ndk/releases

- x86_64架构：
  https://github.com/android/ndk/releases/latest
```
- 下载完毕后，把下载好的zip改名为ndk.zip，随后按照以下代码设置
```shell
mkdir ~/ndk_temp 2>/dev/null
unzip ndk.zip -d ~/ndk_temp
mv ~/ndk_temp/*/* ~/ndk_temp
```
- 随后设置环境变量
```shell
export ANDROID_NDK_HOME=$(realpath ~/ndk_temp)
export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
```
全部设置完毕后，执行`python3 ./make.py build --release --nightly` 即可


### 换源
- rustup
```shell
export RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
export RUSTUP_UPDATE_ROOT="https://mirrors.tuna.tsinghua.edu.cn/rustup/rustup"
```

- cargo
创建一个`.cargo`目录，创建文件`config.toml`
```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'tuna'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 北京外国语大学
[source.bfsu]
registry = "https://mirrors.bfsu.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
```

# StratoVirt：
StratoVirt是计算产业中面向云数据中心的企业级虚拟化平台，实现了一套架构统一支持虚拟机、容器、Serverless三种场景。StratoVirt在轻量低噪、软硬协同、Rust语言级安全等方面具备关键技术竞争优势。

StratoVirt预留了接口和设计来支持更多特性，未来甚至向标准虚拟化演进。

## 如何开始

### 环境准备
在编译StratoVirt前，请确保Rust语言环境和Cargo软件已经安装成功。如果没有安装，请参考以下链接的指导进行安装：

https://www.rust-lang.org/tools/install

### 编译软件
为了编译StratoVirt，需要先克隆代码工程，然后执行编译命令，如下：
```sh
$ git clone https://gitee.com/openeuler/stratovirt.git
$ cd stratovirt
$ cargo build --release
```
可以在`target/release/stratovirt`路径下找到生成的二进制文件

### 运行软件
为了快速上手StratoVirt，需要准备
* PE格式的Linux内核镜像
* EXT4格式的rootfs镜像

```shell
# 如果-api-channel的socket文件已经存在，请先删除它
$ ./target/release/stratovirt \
    -kernel /path/to/kernel \
    -append console=ttyS0 root=/dev/vda reboot=k panic=1 \
    -drive file=/path/to/rootfs,id=rootfs,readonly=off \
    -api-channel unix:/path/to/socket \
    -serial stdio
```

关于制作rootfs镜像、编译内核镜像以及编译StratoVirt的详细指导，请参考[StratoVirt Quickstart](./docs/quickstart.md)。

StratoVirt所支持更多特性，详细指导请参考[Configuration Guidebook](docs/config_guidebook.md)。

## 设计
想获取更多的StratoVirt核心架构设计信息，请参考[StratoVirt design](./docs/design.md)。

## 如何贡献
我们非常欢迎新贡献者的加入，并且非常乐意为新的贡献者提供指导和帮助。
StratoVirt遵循Rust语言编程规范，请参考以下链接：

https://github.com/rust-dev-tools/fmt-rfcs/tree/master/guide

https://github.com/rust-lang/rust-clippy

如果你想获取更多关于StratoVirt的信息，请参考以下链接：

https://gitee.com/openeuler/stratovirt/wikis

## 许可
StratoVirt使用Mulan PSL v2开源协议许可
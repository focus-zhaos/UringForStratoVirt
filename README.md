# rust-iouring for StratoVirt

#### 介绍
基于stratovirt的后端虚拟块设备，实现对异步读写组件iouring的支持。

1. 基于crate libc，封装425-427号系统调用与相关iouring的数据结构、宏定义与枚举类型。

2. 实现异步上下文结构体UringContext(util/src/aio/uring.rs), 其成员函数分别实现setup(), enter()和register()功能，具体来说：
    - 构造函数new()：调用sys_io_uring_setup()初始化iouring，注册eventfd，并完成sq,cq,sqe三段内存的映射，构造异步上下文UringContext。
    - 请求提交submit(): 接收Iocb相关参数(file_fd, iovec, offset等)，构造并向sqe尾部(tail)添加请求，调用sys_io_uring_enter()提交。 
    - 获取结果get_buffs(): 由handle调用，从cq_head开始读取cqes的返回数据(user_data, res)，打包成EventResult结构返回。

3. 按需修改部分mod.rs和block.rs文件中代码逻辑。
4. sample_program内为示例代码的实现。

#### 项目目录树

```
util/src
├── aio
│   ├── mod.rs
│   └── uring.rs
device_model/src
├── virtio
│   └── block.rs
sample_program/src
├── main.rs
├── testFile.txt

```

#### 执行原理
![UringForStratovirt执行原理](https://images.gitee.com/uploads/images/2021/0402/120249_bd29ee3c_8342032.png "Untitled Diagram (1).png")

#### 说明
代码基于kernel 5.3，libc "0.2.71"版本撰写。虚拟机编译与运行方式同原生系统，详见[stratovirt](https://gitee.com/openeuler/stratovirt)


#### 可用性测试
1. 示例代码（stratovirt/sample_program）：示例代码使用iouring读取一个小于512字节的文件（测试文件位于stratovirt/sample_program/src/testFile.txt）。您可以通过修改testFile.txt文件来输入测试数据。

```
[zs@localhost sample_program]$ cargo build & cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/sample_program_Q17`
[Sample Result]: 2020 openEuler Contest - Q17
```


#### 作者简历
赵盛，南开大学计算机学院。

技术栈：QEMU设备虚拟化，Linux内存管理。

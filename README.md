# 14-NBJL挤牙膏大师

#### 介绍
基于stratovirt的后端虚拟块设备，实现对异步读写组件iouring的支持。
1. 基于crate libc，封装425-427号系统调用与相关iouring的数据结构、宏定义与枚举类型。
2. 实现异步上下文结构体UringContext, 其成员函数分别实现setup(), enter()和register()功能，具体来说：
    - 构造函数new()：调用sys_io_uring_setup()初始化iouring，注册eventfd，并完成sq,cq,sqe三段内存的映射，构造异步上下文UringContext。
    - 请求提交submit(): 接收Iocb相关参数(file_fd, iovec, offset等)，构造并向sqe尾部(tail)添加请求，调用sys_io_uring_enter()提交。 

#### 项目目录树

```
util/src
├── aio
│   ├── libaio.rs
│   ├── mod.rs
│   ├── raw.rs
│   └── uring.rs
├── ...
```


#### 执行原理

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明
本项目基于stratovirt开发，编译与运行方式同原生系统，详见[openEuler / stratovirt](https://gitee.com/openeuler/stratovirt?_from=gitee_search)

#### 作者简历
TOPIC_ID:14, TEAM_ID:1996338335, TEAM_NAME:NBJL挤牙膏大师.

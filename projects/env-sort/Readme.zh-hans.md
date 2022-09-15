Env Sort
=====================

强迫症福音, 帮你的环境变量排序

1. 排序环境变量, 可以区分用户环境变量和系统环境变量
2. 删除重复的 PATH 环境
3. 删除失效的 PATH 环境
4. PATH 大小写改正成实际路径的大小写, 前提是该路径存在

---

## Install

如果已经安装了 rust 环境, 可以直接用 cargo 安装.

```sh
cargo install env-sort -f
env-sort     # 预览执行
env-sort -e  # 实际执行
```

如果没有安装 rust 环境, 可以直接下载最新 [release](https://github.com/oovm/env-sort/releases), 然后在同目录执行.

## Options

```yaml
usage: env-sort.exe [OPTIONS]

options:
  -e, --execute 
      执行模式, 是否实际写入环境变量
      默认关闭

  -v, --verify
      验证模式, 是否验证路径是否存在
      默认开启

  -h, --help
      查看帮助信息

  -V, --version
      查看版本信息
```




## Problem

首次运行可能触发你电脑上的杀毒软件, 比如这样:

![Virus Error](../../.github/assets/virus.png)

这是因为 Windows 的环境变量在注册表里, 直接点允许即可.

正常运行结果如下: 

![](https://user-images.githubusercontent.com/17541209/203804749-83f81a45-e613-4a68-89cc-30948c051cfd.png)

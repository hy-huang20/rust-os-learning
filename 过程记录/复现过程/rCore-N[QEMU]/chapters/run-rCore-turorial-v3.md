# 运行 rCore-Tutorial-v3

rCore-Tutorial-v3 [指导书](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html)

## 1. 概述

因为需要参考这篇文章：[rcore 引入异步运行时 embassy](https://github.com/lighkLife/rcore-blog/issues/1)，因此需要先将 rCore-Tutorial-v3/ch9 在本地跑起来，以下记录运行过程以及踩过的坑。

## 2. 过程记录

理论上运行以下指令即可：（前提是没有遇到任何问题）

```
# 在 rCore-Tutorial-v3 根目录下使用 docker 环境
su
service docker status
service docker start
service docker status

# 建立基于 docker 的开发环境
make build_docker
# 进入到 docker 环境
make docker
# 此时位于 /mnt
# /mnt 目录下的内容和 rCore-Tutorial-v3 目录下的内容完全相同
cd os
make run
```

## 3. make build_docker 问题记录

遇到的所有问题都出自 make build_docker 这条指令。 

### 3.1. ERROR resolve image config for docker-image://docker.io/docker/dockerfile:1

#### 报错消息

```
Dockerfile:1
--------------------
   1 | >>> # syntax=docker/dockerfile:1
   2 |
   3 |     # Stage 1 Build QEMU
--------------------
ERROR: failed to solve: failed to resolve source metadata for docker.io/docker/dockerfile:1: failed to do request: Head "https://registry-1.docker.io/v2/docker/dockerfile/manifests/1": dial tcp 69.63.186.30:443: i/o timeout
```

#### 解决方法

```
# 创建代理配置文件
sudo mkdir -p /etc/default
# tee 指令像一个三通的 T 形水管。它从标准输入读取数据，然后同时将数据写入标准输出（屏幕）和一个或多个文件
# 注：下面 5 行是作为一条指令输入的
sudo tee /etc/default/docker <<EOF
# Docker 代理配置
export HTTP_PROXY="http://proxy.example.com:PORT"
export HTTPS_PROXY="http://proxy.example.com:PORT"
EOF
# 重启 docker 服务
sudo service docker restart
# 检查服务状态
sudo service docker status
```

### 3.2. wsl input/output error

#### 解决方法

wsl 的 .vdhx 文件所在磁盘空间不足。最好预留至少 5G 的空间。

### 3.3. 访问 https://download.qemu.org/qemu-7.0.0.tar.xz 下载 qemu 失败

#### 报错消息

```
 > [build_qemu 3/3] RUN wget https://download.qemu.org/qemu-7.0.0.tar.xz &&     tar xf qemu-7.0.0.tar.xz &&     cd qemu-7.0.0 &&     ./configure --target-list=riscv64-softmmu,riscv64-linux-user &&     make -j$(nproc) &&     make install:
0.274 --2025-10-15 15:32:12--  https://download.qemu.org/qemu-7.0.0.tar.xz
0.276 Resolving download.qemu.org (download.qemu.org)... 156.146.44.90, 84.17.57.26, 2a02:6ea0:d600::14, ...
0.584 Connecting to download.qemu.org (download.qemu.org)|156.146.44.90|:443... failed: Connection timed out.
134.1 Connecting to download.qemu.org (download.qemu.org)|84.17.57.26|:443... failed: Connection timed out.
269.3 Connecting to download.qemu.org (download.qemu.org)|2a02:6ea0:d600::14|:443... failed: Network is unreachable.
269.3 Connecting to download.qemu.org (download.qemu.org)|2a02:6ea0:d600::13|:443... failed: Network is unreachable.
```

#### 解决方法

```
vim ~/.docker/config.json

# 加入以下内容即可解决
{
	"proxies":
	{
		"default":
		{
			"httpProxy": "http://proxy.example.com:PORT",
			"httpsProxy": "http://proxy.example.com:PORT"
		}
	}
}
```

关于 docker 代理的配置可以参考[这篇文章](https://neucrack.com/p/286)。

### 3.4. client error (Connect): Connection reset by peer (os error 104)

#### 解决方法

多试几次。是的



# Space Acres 汉化版
本分叉使用了Space进行了中文翻译,只修改了前端,无其他修改,请放心使用

关于更新:跟随主版本


# 官方原版链接:


[![最新发布](https://img.shields.io/github/v/release/subspace/space-acres?display_name=tag&style=flat-square)](https://github.com/subspace/space-acres/releases)
[![最新下载](https://img.shields.io/github/downloads/subspace/space-acres/latest/total?style=flat-square)](https://github.com/subspace/space-acres/releases/latest)
[![Rust](https://img.shields.io/github/actions/workflow/status/subspace/space-acres/rust.yml?branch=main)](https://github.com/subspace/space-acres/actions/workflows/rust.yaml)

Space Acres是一个针对[Subspace Network](https://subspace.network/)农业的自定GUI应用程序。

## 当前状态

项目当前处于Alpha阶段。

这意味着虽然它通常应该工作，但期望有时候会出现问题，以意想不到的方式出错，错误处理可能会缺失。

当前版本仅支持Gemini 3h链，并且不允许选择其他任何选项。它支持从3g版本升级现有安装。

## 功能

当前功能：
* 配置（奖励地址，节点位置，多个农场，P2P端口）
* 节点同步与显示进度、速度和预计到达时间（ETA）
* 农民绘图/农业作物缓存/绘图/重新绘图进度显示和速度计算
* 农民审计/证明性能指标
* 农民扇区状态可视化

即将推出的功能/能力：请查看开放问题，如果缺少某些内容，也请考虑贡献！

## 安装

详细安装说明请见[docs/INSTALLATION.md](docs/INSTALLATION.md)

## 项目结构

从高层次上看，项目分为几个大模块：
* `backend` 处理所有后端功能
  * `config` 包含具有读取、写入和验证能力的配置数据结构
  * `farmer` 包含农民实现以及抽象其内部的封装数据结构
  * `networking` 包含`farmer`和`node`共享的网络堆栈，以及抽象其内部的封装数据结构
  * `node` 包含共识节点及其抽象内部的封装数据结构
  * `utils` 包含一些低级实用程序
* `docs` 包含文档文件
* `frontend` 处理大部分前端逻辑，每个模块对应一个主要的应用屏幕/视图
* `res` 包含应用操作和/或打包所需的各种非代码资源
  * `app.css` 包含一些小的、非关键的展示调整，未来可能需要随应用一起发布GTK4主题以确保一致的外观
  * `linux` 包含Linux特定资源
  * `windows` 包含Windows特定资源
* `main.rs` 处理高级UI和后端通信，将所有内容连接在一起

应用支持最少配置，并且目前还不支持操作员功能（至少目前还不支持）。

## 中文电报(非官方)

https://t.me/subspacecn


# Perseus Rust Support (v1.0.1)

**ID**: `perseus-rust`  
**描述**: 为 Cargo 项目提供创建、构建、运行一体化支持

---

## 支持功能

本扩展内置跨平台脚本，支持 Linux 系统下 Rust 项目的全流程开发操作。

## 可用命令列表

* **`rust.create.linux`** `[Linux]`
  创建 Linux 平台 Rust 项目，需传入参数：项目名称(`project_name`)

* **`rust.create.windows`** `[Windows]`
  创建 Windows 平台 Rust 项目，需传入参数：项目名称(`project_name`)

* **`rust.debug.build.linux`** `[Linux]`
  Linux 平台 Debug 模式构建 Rust 项目，无参数
  
* **`rust.release.build.linux`** `[Linux]`
  Linux 平台 Release 模式构建 Rust 项目，无参数

* **`rust.debug.buildAndRun.linux`** `[Linux]`
  Linux 平台 Debug 模式构建并直接运行项目，无参数

* **`rust.release.buildAndRun.linux`** `[Linux]`
  Linux 平台 Release 模式构建并直接运行项目，无参数

## 使用说明

1. 扩展依赖内置脚本文件（`./scripts/` 目录下所有 `.sh` / `.bat` 文件），请勿删除或修改脚本路径。
2. 创建项目命令必须填写 `project_name` 参数。
3. 构建/运行命令会自动在当前项目根目录执行。

---
*`版本：1.0.1 | 用途：Cargo 项目创建与构建支持`*

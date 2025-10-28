---
description: "创建设备管理和连接界面"
mode: "edit"
tools: ["file-system", "terminal"]
---

# 设备管理模块开发

创建一个完整的ADB设备管理系统，支持最多10台设备的连接、监控和任务分配。

## 功能需求

### 1. 设备发现和连接
- 自动扫描ADB可用设备
- 手动添加设备（IP地址连接）
- 设备连接状态实时监控
- 设备信息获取（型号、系统版本、电量等）

### 2. 设备列表界面
显示格式：
```
设备编号 | 设备名称    | 连接状态 | 电量 | 系统版本 | 操作
设备1   | iPhone13   | 已连接   | 85%  | iOS15   | [断开][测试]
设备2   | 小米11     | 已连接   | 92%  | MIUI12  | [断开][测试]
设备3   | 华为P40    | 未连接   | --   | --      | [连接]
...
设备10  | --        | 未配置   | --   | --      | [添加设备]
```

### 3. 设备健康监控
- 设备响应时间检测
- 电量状态监控（低电量警告）
- 网络连接状态
- 应用安装状态检查（小红书/抖音APP）
- 设备温度和性能监控

### 4. 任务分配可视化
- 实时显示每设备当前任务状态
- 任务队列长度显示
- 设备负载均衡指示
- 任务完成统计

## 技术实现

```python
import subprocess
import json
from qfluentwidgets import (
    VerticalScrollInterface, TableWidget,
    PrimaryPushButton, StateToolTip,
    ProgressBar, InfoBar, FluentIcon
)

class DeviceManager:
    def __init__(self):
        self.connected_devices = {}
        self.device_info = {}
        self.max_devices = 10

    def scan_devices(self):
        """扫描ADB可用设备"""
        result = subprocess.run(['adb', 'devices', '-l'],
                              capture_output=True, text=True)
        return self.parse_device_list(result.stdout)

    def connect_device(self, device_id):
        """连接设备"""
        pass

    def disconnect_device(self, device_id):
        """断开设备"""
        pass

    def get_device_info(self, device_id):
        """获取设备详细信息"""
        info = {}
        # 获取设备型号
        # 获取系统版本
        # 获取电量状态
        # 获取网络状态
        return info

    def test_device_connection(self, device_id):
        """测试设备连接"""
        pass

class DeviceManagementInterface(VerticalScrollInterface):
    def __init__(self):
        super().__init__(
            object_name="device_management",
            nav_text_cn="设备管理",
            nav_icon=FluentIcon.PHONE
        )
        self.device_manager = DeviceManager()
        self.setup_ui()

    def setup_ui(self):
        # 设备扫描区域
        # 设备列表表格
        # 连接控制按钮
        # 状态监控面板
```

## ADB命令集成

### 基础设备操作：
```bash
# 列出设备
adb devices -l

# 连接网络设备
adb connect 192.168.1.100:5555

# 获取设备信息
adb -s <device_id> shell getprop ro.product.model
adb -s <device_id> shell dumpsys battery | grep level

# 检查应用安装
adb -s <device_id> shell pm list packages | grep xiaohongshu
adb -s <device_id> shell pm list packages | grep douyin

# 设备截图测试
adb -s <device_id> shell screencap -p /sdcard/test.png
```

## 界面布局

```
┌─────────────────────────────────────────────────────┐
│ 📱 设备管理中心                                      │
│ [扫描设备] [刷新] [添加网络设备] 连接数: 3/10        │
├─────────────────────────────────────────────────────┤
│ 设备ID    │ 设备名称   │ 状态  │ 电量│系统  │ 操作    │
├─────────────────────────────────────────────────────┤
│ 🟢 设备1  │ iPhone13  │ 已连接│ 85% │iOS15 │[断开][测试]│
│ 🟢 设备2  │ 小米11    │ 已连接│ 92% │MIUI12│[断开][测试]│
│ 🟢 设备3  │ 华为P40   │ 已连接│ 78% │EMUI10│[断开][测试]│
│ 🔴 设备4  │ OPPO R15  │ 离线  │ --  │ --   │[连接]      │
│ ⚪ 设备5  │ --        │ 未配置│ --  │ --   │[添加设备]  │
│ ⚪ 设备6  │ --        │ 未配置│ --  │ --   │[添加设备]  │
│ ...      │ ...       │ ...   │ ... │ ...  │ ...       │
├─────────────────────────────────────────────────────┤
│ 📊 设备状态监控                                      │
│ 平均响应时间: 120ms | 总任务数: 1,250 | 完成率: 95%  │
│ 设备1: ████████████░░░░ 75% (375/500)               │
│ 设备2: ██████████████░░ 85% (425/500)               │
│ 设备3: ██████████░░░░░░ 60% (300/500)               │
├─────────────────────────────────────────────────────┤
│ ⚠️  设备4电量过低 (<20%)                             │
│ ✅ 所有设备APP版本检查完成                            │
│ 🔄 设备2任务执行中...                                │
└─────────────────────────────────────────────────────┘
```

## 错误处理和监控

1. **设备断线重连**: 自动检测并重新连接
2. **电量预警**: 电量<20%时暂停任务分配
3. **性能监控**: CPU/内存使用率过高时降低任务频率
4. **网络异常**: 检测网络状态，异常时切换连接方式
5. **应用崩溃恢复**: 监控目标应用状态，异常时自动重启

参考core-modules.instructions.md中的设备管理规范进行开发。

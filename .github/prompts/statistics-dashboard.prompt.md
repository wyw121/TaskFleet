---
description: "创建任务统计和数据展示界面"
mode: "edit"
tools: ["file-system", "chart"]
---

# 任务统计界面开发

创建一个完整的数据统计和可视化界面，显示关注任务的执行情况和成效分析。

## 核心统计功能

### 1. 总览统计卡片
- 总关注人数（累计）
- 今日新增关注数
- 本月新增关注数
- 成功率统计（成功关注/尝试关注）

### 2. 平台分布统计
- 小红书关注数量和占比
- 抖音关注数量和占比
- 未来平台扩展预留
- 平台效果对比分析

### 3. 设备效率统计
- 每台设备的关注数量
- 设备效率排行榜
- 设备故障率统计
- 负载分配均衡度

### 4. 时间趋势分析
- 每日关注数量趋势图
- 每小时活跃度分析
- 周期性任务完成情况
- 高峰期和低谷期识别

## 技术实现

```python
from qfluentwidgets import (
    VerticalScrollInterface, CardWidget,
    LineChart, PieChart, BarChart,
    TableWidget, StatisticsWidget,
    FluentIcon, InfoBar
)
import pyqtgraph as pg
from datetime import datetime, timedelta

class StatisticsInterface(VerticalScrollInterface):
    def __init__(self, data_manager):
        super().__init__(
            object_name="statistics_view",
            nav_text_cn="数据统计",
            nav_icon=FluentIcon.CHART
        )
        self.data_manager = data_manager
        self.setup_ui()

    def setup_ui(self):
        # 总览统计卡片区域
        self.create_overview_cards()

        # 图表展示区域
        self.create_charts_section()

        # 详细数据表格
        self.create_detailed_table()

        # 实时刷新定时器
        self.setup_refresh_timer()

    def create_overview_cards(self):
        """创建概览统计卡片"""
        cards_data = [
            {
                "title": "累计关注",
                "value": "12,456",
                "change": "+1,234 今日",
                "icon": FluentIcon.PEOPLE,
                "color": "#1890ff"
            },
            {
                "title": "成功率",
                "value": "94.2%",
                "change": "+2.1% 较昨日",
                "icon": FluentIcon.COMPLETED,
                "color": "#52c41a"
            },
            {
                "title": "活跃设备",
                "value": "8/10",
                "change": "正常运行",
                "icon": FluentIcon.PHONE,
                "color": "#722ed1"
            },
            {
                "title": "今日消费",
                "value": "¥1,245.60",
                "change": "余额充足",
                "icon": FluentIcon.MONEY,
                "color": "#fa8c16"
            }
        ]

        for card_data in cards_data:
            card = self.create_stat_card(card_data)
            self.addWidget(card)

    def create_charts_section(self):
        """创建图表区域"""
        # 关注数量趋势图
        trend_chart = self.create_trend_chart()

        # 平台分布饼图
        platform_chart = self.create_platform_pie_chart()

        # 设备效率柱状图
        device_chart = self.create_device_bar_chart()

        # 时间热力图
        heatmap_chart = self.create_time_heatmap()
```

## 界面布局设计

```
┌─────────────────────────────────────────────────────┐
│ 📊 数据统计中心                                      │
│ [今日] [本周] [本月] [自定义] 最后更新: 2分钟前      │
├─────────────────────────────────────────────────────┤
│ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐                │
│ │累计关注│ │成功率 │ │活跃设备│ │今日消费│               │
│ │12,456 │ │94.2% │ │ 8/10 │ │¥1,245│               │
│ │+1,234 │ │+2.1% │ │正常  │ │余额足│               │
│ └──────┘ └──────┘ └──────┘ └──────┘                │
├─────────────────────────────────────────────────────┤
│ 📈 关注数量趋势 (最近30天)                           │
│    关注数                                           │
│     500│    ●                                       │
│     400│  ● ● ●   ●                                 │
│     300│● ● ● ● ● ● ●                               │
│     200│● ● ● ● ● ● ● ●                             │
│     100└─────────────────────────────                │
│        1  5  10 15 20 25 30 日期                    │
├─────────────────────────────────────────────────────┤
│ 🥧 平台分布          📊 设备效率排行                 │
│   小红书: 65%        设备1: ████████░░ 80%          │
│   抖音: 35%          设备2: ██████████ 95%          │
│                     设备3: ██████░░░░ 60%          │
│                     设备4: ████████░░ 75%          │
├─────────────────────────────────────────────────────┤
│ 🕐 时间活跃度热力图                                  │
│    小时│ 0  2  4  6  8 10 12 14 16 18 20 22        │
│    周一│ ░  ░  ░  ▓  █  █  ▓  ▓  █  █  ▓  ░        │
│    周二│ ░  ░  ░  ▓  █  █  ▓  ▓  █  █  ▓  ░        │
│    周三│ ░  ░  ░  ▓  █  █  ▓  ▓  █  █  ▓  ░        │
│    ...│ ... ... ... ... ... ... ... ... ... ... ...│
├─────────────────────────────────────────────────────┤
│ 📋 详细统计数据                                      │
│ 时间    │平台  │设备│关注数│成功数│成功率│消费金额    │
│ 14:30  │小红书│设备1│ 50  │ 47  │94%  │¥47.00     │
│ 14:25  │抖音  │设备2│ 30  │ 29  │97%  │¥34.80     │
│ 14:20  │小红书│设备3│ 25  │ 24  │96%  │¥24.00     │
│ [导出Excel] [生成报告] [详细日志]                     │
└─────────────────────────────────────────────────────┘
```

## 数据分析功能

### 1. 效率分析算法
```python
def calculate_device_efficiency(self, device_id, time_range):
    """计算设备效率指标"""
    success_count = self.get_success_count(device_id, time_range)
    total_attempts = self.get_total_attempts(device_id, time_range)
    online_time = self.get_online_time(device_id, time_range)

    success_rate = success_count / total_attempts if total_attempts > 0 else 0
    speed = success_count / online_time if online_time > 0 else 0

    return {
        "success_rate": success_rate,
        "speed": speed,
        "stability": self.calculate_stability(device_id, time_range)
    }
```

### 2. 趋势预测
```python
def predict_daily_target(self, historical_data, target_days=7):
    """基于历史数据预测未来目标"""
    # 使用简单线性回归预测
    from sklearn.linear_model import LinearRegression

    X = np.array(range(len(historical_data))).reshape(-1, 1)
    y = np.array(historical_data)

    model = LinearRegression().fit(X, y)

    future_X = np.array(range(len(historical_data),
                             len(historical_data) + target_days)).reshape(-1, 1)
    predictions = model.predict(future_X)

    return predictions
```

### 3. 异常检测
- 突然的成功率下降
- 设备异常行为检测
- 平台政策变化影响
- 异常时间点标记

### 4. 性能优化建议
- 最佳任务分配建议
- 设备使用策略优化
- 时间段选择建议
- 成本效益分析

## 导出和报告功能

### Excel报告格式：
- 汇总统计页
- 每日明细页
- 设备效率页
- 平台对比页
- 成本分析页

### PDF报告内容：
- 执行摘要
- 可视化图表
- 关键指标分析
- 优化建议
- 未来规划

参考数据库schema和API接口设计进行数据获取和展示。

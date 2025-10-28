---
description: "创建计费系统和余额管理模块"
mode: "edit"
tools: ["file-system", "terminal"]
---

# 计费系统开发

开发一个完整的实时计费和余额管理系统，支持基于成功关注的扣费机制。

## 核心功能模块

### 1. 余额显示和管理
- 实时余额显示（精确到分）
- 余额变更历史记录
- 充值和消费明细
- 低余额预警提醒

### 2. 计费规则配置
- 服务器端规则同步
- 不同平台不同计费标准：
  - 小红书关注：¥1.00/次
  - 抖音关注：¥1.20/次
  - 批量操作折扣计算
- 实时费率更新

### 3. 任务前余额检查
- 提交任务前预算计算
- 余额充足性验证
- 不足余额的友好提示
- 任务暂停和恢复机制

### 4. 成功扣费逻辑
- 仅成功关注后扣费
- 实时与服务器同步
- 失败操作不扣费
- 退款和调整机制

## 技术实现架构

```python
from qfluentwidgets import (
    VerticalScrollInterface, InfoBar,
    PrimaryPushButton, LineEdit,
    TableWidget, ProgressBar, FluentIcon
)

class BillingManager:
    def __init__(self, api_client):
        self.api_client = api_client
        self.current_balance = 0.0
        self.billing_rules = {}
        self.transaction_history = []

    async def get_current_balance(self):
        """获取当前余额"""
        response = await self.api_client.get("/api/balance")
        self.current_balance = response["balance"]
        return self.current_balance

    async def calculate_task_cost(self, task_type, platform, quantity):
        """计算任务预估费用"""
        rate = self.billing_rules.get(f"{platform}_{task_type}", 1.0)
        return rate * quantity

    async def check_balance_sufficient(self, required_amount):
        """检查余额是否充足"""
        current = await self.get_current_balance()
        return current >= required_amount

    async def record_successful_follow(self, user_id, platform, cost):
        """记录成功关注并扣费"""
        try:
            response = await self.api_client.post("/api/billing/charge", {
                "user_id": user_id,
                "platform": platform,
                "operation": "follow",
                "cost": cost,
                "timestamp": datetime.now().isoformat()
            })
            if response["success"]:
                self.current_balance -= cost
                self.transaction_history.append({
                    "type": "charge",
                    "amount": -cost,
                    "description": f"{platform}关注用户{user_id}",
                    "timestamp": datetime.now()
                })
                return True
            return False
        except Exception as e:
            # 扣费失败，记录但不影响主流程
            logger.error(f"计费失败: {e}")
            return False

class BillingInterface(VerticalScrollInterface):
    def __init__(self, billing_manager):
        super().__init__(
            object_name="billing_management",
            nav_text_cn="余额管理",
            nav_icon=FluentIcon.MONEY
        )
        self.billing_manager = billing_manager
        self.setup_ui()

    def setup_ui(self):
        # 余额显示区域
        # 消费历史区域
        # 计费规则显示区域
        # 预算计算器
```

## 界面设计

```
┌─────────────────────────────────────────────────────┐
│ 💰 余额管理中心                                      │
├─────────────────────────────────────────────────────┤
│ 当前余额: ¥1,250.68                                 │
│ 📊 今日消费: ¥125.50 | 本月消费: ¥2,850.20         │
│ [充值] [消费明细] [导出账单]                        │
├─────────────────────────────────────────────────────┤
│ 📋 计费标准 (最后更新: 2025-01-15 10:30)            │
│ • 小红书关注: ¥1.00/次                              │
│ • 抖音关注: ¥1.20/次                                │
│ • 批量操作(>100): 9.5折优惠                         │
│ • 高级会员: 9折优惠                                  │
├─────────────────────────────────────────────────────┤
│ 🧮 费用计算器                                       │
│ 平台: [小红书 ▼] 操作类型: [关注 ▼]                │
│ 数量: [500____] 预计费用: ¥475.00 (含5%折扣)       │
│ 余额检查: ✅ 充足 (剩余: ¥775.68)                   │
├─────────────────────────────────────────────────────┤
│ 📈 消费历史 (最近10条)                              │
│ 时间       │类型    │平台  │数量│金额   │余额      │
│ 10:25:30  │关注    │小红书│ 50 │-¥50.00│¥1,250.68 │
│ 10:20:15  │关注    │抖音  │ 25 │-¥30.00│¥1,300.68 │
│ 09:45:20  │充值    │--   │ -- │+¥500.00│¥1,330.68│
│ [查看全部] [导出Excel]                              │
├─────────────────────────────────────────────────────┤
│ ⚠️  余额预警设置                                     │
│ 低于 [¥100___] 时提醒 ☑                            │
│ 任务暂停阈值: [¥50___] ☑                           │
│ 邮件通知: user@example.com ☑                       │
└─────────────────────────────────────────────────────┘
```

## 关键业务逻辑

### 1. 任务提交前检查流程
```python
async def submit_task(self, task_data):
    # 1. 计算任务预估费用
    estimated_cost = await self.calculate_task_cost(task_data)

    # 2. 检查余额充足性
    if not await self.check_balance_sufficient(estimated_cost):
        self.show_insufficient_balance_dialog(estimated_cost)
        return False

    # 3. 显示费用确认对话框
    if await self.confirm_task_cost(estimated_cost):
        return await self.execute_task(task_data)

    return False
```

### 2. 实时扣费流程
```python
async def process_successful_follow(self, user_id, platform):
    # 1. 获取当前费率
    rate = await self.get_current_rate(platform, "follow")

    # 2. 记录成功关注并扣费
    success = await self.record_successful_follow(user_id, platform, rate)

    # 3. 更新UI显示
    if success:
        self.update_balance_display()
        self.add_transaction_record(user_id, platform, rate)

    return success
```

### 3. 防重复扣费机制
- 基于唯一事务ID防重复
- 服务器端幂等性保证
- 客户端状态追踪
- 异常情况回滚机制

### 4. 数据同步策略
- 实时余额同步（每次操作后）
- 定时全量同步（每5分钟）
- 离线缓存和同步队列
- 冲突检测和解决

## 错误处理

1. **网络异常**: 本地缓存操作，网络恢复后同步
2. **服务器错误**: 重试机制，最多3次
3. **余额不足**: 友好提示，引导充值
4. **计费失败**: 记录日志，不影响主要功能
5. **数据不一致**: 强制同步，用户确认

参考server-backend API设计和数据库schema进行开发。

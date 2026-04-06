![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/yaofan.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/头部.png)

## mini 项目

本项目诞生之初是为了适配 Quantumult X 1.07 版本新增的机场订阅 img-url 特性，经过不断发展，包含了机场订阅图标，Task 图标，节点地区图标，策略组图标等

本项目可用于 QuantumultX 1.07 及以上版本，Loon 2.1.10(278)以上，和 Pharos Pro 1.3.3 及以上版本中

**注意：** 本项目图标可用于订阅，Task，策略组等位置的远程引用

### Quantumult X 使用方法：

#### 1、订阅图标

打开 QuanX 配置文件-编辑，找到［server_remote］字段，在想要增加图标的相应订阅中修改，在 enable ＝ true 之前加上
`img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Alpha/name.png`
注意此句和前后句都要用英文逗号隔开，并且逗号后先要空一格

**完整示例：**`https://raw.githubusercontent.com/crossutility/Quantumult-X/master/server-complete.txt, tag=Sample-02, as-policy=static, img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Alpha/name.png, enabled=false`

#### 2、策略图标

**2.1** UI 中使用

长按想要更改图标的策略组，弹出菜单选择编辑，在图标一栏填写

\*\*`https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png`

**2.2** 文本编辑：

打开 QuanX 配置文件-编辑，找到［policy］字段，在想要增加图标的相应策略段落中修改，在 enable ＝ true 之前加上
\*\*`img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png` 注意此句和前后句都要用英文逗号隔开，并且逗号后先要空一格

**完整示例：**`static=policy-name-1, Sample-A, Sample-B, Sample-C, img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png`

### Loon 使用方法：

**1** UI 中使用

点击下方“策略”选项卡，在策略界面点长按想要更改图标的订阅/策略组，弹出界面中，在图标一栏填写

\*\*`https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png`

**2** 文本编辑：
打开 Loon 配置选项卡，点击编辑-文本编辑，找到[Remote Proxy]/［Proxy Group］字段，在想要增加图标的相应订阅/策略段落中修改，加上 \*\*`img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png` 注意此句和前句要用英文逗号隔开

注：请将使用方法中的 name.png 替换成相应文件的文件名

### Pharos Pro 使用方法：

在 Pharos Pro 主页对应订阅上左滑，点击编辑，在弹出界面的图标一栏中填入 `https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Alpha/name.png`

### task 图标使用方法：

#### 1、文本编辑中使用

打开 QuanX 配置文件-编辑，找到［task_local］字段，在想要增加图标的相应签到脚本段落中修改，在 enable ＝ true 之前加上 `img-url=https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png` 注意此句和前后句都要用英文逗号隔开，并且逗号后先要空一格

#### 2、UI 中使用

主界面右下角点击风车开启菜单，然后找到调试一栏下的构造请求，点击进入构造请求界面，左滑相应 task，点击编辑，在图标一栏填写 `https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png`

🔘 彩色版本 `https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Color/name.png`

🔘 透明版本 `https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/Alpha/name.png`

**注：** task 图标的透明和彩色版本文件名完全一致，仅所在库不同

##### 图标索引，最上方为图标展示，下面第一行为对应机场的名称，第二行为文件名，请将使用方法中的 name.png 替换成相应文件的文件名

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/机场.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/模板1.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/模板2.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/模板3.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/模板4.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/模板5.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/策略.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/策略1.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/策略2.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/地区.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/地区1.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/Task.png)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-1.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-2.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-3.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-4.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-5.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-6.jpg)

![示例](https://fastly.jsdelivr.net/gh/iRoySwift/Profiles@main/Icon/none/-7.jpg)

## Filter 维护脚本

仓库内提供了一个可复用的 Rust 脚本，用于维护 `QuantumultX/Filter/*.list` 文件：

`scripts/filter_maintainer.rs`

### 编译命令

```bash
rustc --crate-name filter_maintainer scripts/filter_maintainer.rs -O -o /tmp/filter_maintainer
```

说明：

- 使用 `rustc` 直接编译，不依赖额外的 Cargo 工程。
- 输出文件放在 `/tmp/filter_maintainer`，适合临时执行。

### 检查冲突

```bash
/tmp/filter_maintainer check-conflicts
```

说明：

- 用于检查 `QuantumultX/Filter/*.list` 中是否存在“同一规则类型 + 同一目标值，但指向不同策略”的精确冲突。
- 默认只检查，不修改文件。

### 规范化列表

```bash
/tmp/filter_maintainer normalize --write
```

说明：

- 规范化规则类型大小写和逗号空格。
- 按规则类型排序。
- 去除完全重复的规则。
- 同步更新文件头部的统计值。
- 自动补全缺失的标准计数字段，并将缺失项写为 `0`。
- 头部统计不再单独展示 `DOMAIN` 系列字段。

### 清理精确冲突

```bash
/tmp/filter_maintainer resolve-exact-conflicts --write
```

说明：

- 只处理精确冲突，不处理语义上可能重叠但目标不完全相同的规则。
- 会按脚本内置的列表优先级保留更具体的规则，移除更宽泛列表中的冲突项。

### 清理可由后缀规则覆盖的精确 HOST/DOMAIN

```bash
/tmp/filter_maintainer resolve-redundant-exacts --write
```

说明：

- 用于清理同文件、同策略下，已经被 `HOST-SUFFIX` 或 `DOMAIN-SUFFIX` 覆盖的精确 `HOST` / `DOMAIN` 规则。
- 例如 `HOST,b-hls-10.sacdnssedge.com,GlobalMedia` 已经被 `HOST-SUFFIX,sacdnssedge.com,GlobalMedia` 覆盖时，就会删除前者。
- 适合做“列表压缩”和“规则简化”。

### 检查可由后缀规则覆盖的精确 HOST/DOMAIN

```bash
/tmp/filter_maintainer check-redundant-exacts
```

说明：

- 用于检查同文件、同策略下，哪些精确 `HOST` / `DOMAIN` 已经被现有 `HOST-SUFFIX` 或 `DOMAIN-SUFFIX` 覆盖。
- 默认只检查，不修改文件。

### 检查可能可由 HOST-WILDCARD 归并的 HOST-SUFFIX

```bash
/tmp/filter_maintainer check-wildcard-coverage
```

说明：

- 用于报告“可能可由现有 `HOST-WILDCARD` 归并”的 `HOST-SUFFIX` 候选项。
- 这是保守检查，不会自动删除规则。
- 之所以只做候选提示，是因为 `HOST-SUFFIX` 与 `HOST-WILDCARD` 对子域的真实匹配语义未必完全等价，直接自动改写有风险。

### 一次执行全部维护

```bash
/tmp/filter_maintainer all --write
```

说明：

- 依次执行规范化和精确冲突清理。
- 同时会清理已被后缀规则覆盖的精确 `HOST` / `DOMAIN`。
- 适合批量整理 `QuantumultX/Filter/*.list` 后统一收口。

### 仅预览，不写回文件

如果只是想看结果，不想修改文件，可以去掉 `--write`：

```bash
/tmp/filter_maintainer normalize
/tmp/filter_maintainer check-redundant-exacts
/tmp/filter_maintainer resolve-redundant-exacts
/tmp/filter_maintainer resolve-exact-conflicts
/tmp/filter_maintainer all
```

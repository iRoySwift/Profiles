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

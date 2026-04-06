// 这里引入后面会用到的标准库集合类型。
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
// 用于读取命令行参数。
use std::env;
// 用于读取和写入文件。
use std::fs;
// 用于返回标准 I/O 错误类型。
use std::io;
// Path / PathBuf 用来处理文件路径。
use std::path::{Path, PathBuf};

// 这是要维护的目标目录。
const FILTER_DIR: &str = "QuantumultX/Filter";

// 规则类型的固定排序顺序。
// 这样每次整理后文件顺序都稳定，不会来回抖动。
const RULE_ORDER: &[&str] = &[
    "HOST",
    "DOMAIN",
    "HOST-KEYWORD",
    "DOMAIN-KEYWORD",
    "HOST-SUFFIX",
    "DOMAIN-SUFFIX",
    "HOST-WILDCARD",
    "DOMAIN-WILDCARD",
    "IP-CIDR",
    "IP6-CIDR",
    "IP-CIDR6",
    "USER-AGENT",
];

// 这些规则类型会被统计到头部计数中。
// 头部统计只保留 Quantumult X 当前实际使用到的这些规则类型。
// DOMAIN 系列规则即使正文里仍存在，也不再在头部单独展示。
const COUNT_KEYS: &[&str] = &[
    "HOST",
    "HOST-KEYWORD",
    "HOST-SUFFIX",
    "HOST-WILDCARD",
    "IP-CIDR",
    "IP6-CIDR",
    "IP-CIDR6",
    "USER-AGENT",
];

// 头部注释的推荐输出顺序。
const PREFERRED_HEADER_ORDER: &[&str] = &[
    "NAME",
    "AUTHOR",
    "REPO",
    "UPDATED",
    "HOST",
    "HOST-KEYWORD",
    "HOST-SUFFIX",
    "HOST-WILDCARD",
    "IP-CIDR",
    "IP6-CIDR",
    "IP-CIDR6",
    "USER-AGENT",
    "TOTAL",
];

#[derive(Clone, Debug)]
struct Rule {
    // 规则类型，例如 HOST / HOST-SUFFIX / IP-CIDR。
    kind: String,
    // 规则目标值，例如域名、IP、UA 字符串。
    value: String,
    // 可选的策略名。
    // 有些列表是两段式规则，例如 HOST-SUFFIX,example.com，这种情况下 policy 为 None。
    policy: Option<String>,
    // 规则剩余字段，例如 no-resolve 之类的附加参数。
    extras: Vec<String>,
}

#[derive(Clone, Debug)]
struct Header {
    // 无法解析成 KEY: VALUE 的普通注释行。
    comments: Vec<String>,
    // 可结构化处理的头部键值。
    key_values: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
struct Document {
    // 文件路径。
    path: PathBuf,
    // 文件头。
    header: Header,
    // 文件中的全部规则。
    rules: Vec<Rule>,
}

#[derive(Clone, Debug)]
struct Conflict {
    // 冲突规则类型。
    kind: String,
    // 冲突目标值。
    value: String,
    // 这个目标值分别在什么文件里指向了什么策略。
    uses: Vec<(String, String)>,
}

#[derive(Clone, Debug)]
struct RedundantExactRule {
    // 所在文件名。
    file: String,
    // 被判定为冗余的精确规则类型。
    exact_kind: String,
    // 被判定为冗余的精确规则目标。
    exact_value: String,
    // 该规则的策略名。
    policy: String,
    // 覆盖它的后缀规则类型。
    covered_by_kind: String,
    // 覆盖它的后缀规则值。
    covered_by_value: String,
}

#[derive(Clone, Debug)]
struct WildcardCoverageCandidate {
    // 所在文件名。
    file: String,
    // 被认为“可能可由通配规则归并”的 HOST-SUFFIX 值。
    suffix_value: String,
    // 对应策略名。
    policy: String,
    // 触发这个候选判断的 HOST-WILDCARD。
    wildcard_value: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 跳过程序名本身，只保留用户传入的参数。
    let args: Vec<String> = env::args().skip(1).collect();
    // 没有参数时直接打印帮助。
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    // 是否真正写回文件。
    let write = args.iter().any(|arg| arg == "--write");
    // 第一个非 -- 开头的参数，视为子命令。
    let command = args
        .iter()
        .find(|arg| !arg.starts_with("--"))
        .map(|s| s.as_str())
        .unwrap_or("");

    // 根据不同子命令执行不同功能。
    match command {
        "normalize" => {
            // 读入全部 .list 文件。
            let mut docs = load_documents(FILTER_DIR)?;
            // 做规范化整理。
            let changed = normalize_documents(&mut docs);
            // 需要写回时再落盘。
            if write {
                write_documents(&docs)?;
            }
            // 输出被整理到的文件数。
            println!("normalized_files={}", changed);
        }
        "check-conflicts" => {
            // 先做一次内存中的规范化，避免因为格式差异导致误判。
            let mut docs = load_documents(FILTER_DIR)?;
            normalize_documents(&mut docs);
            // 收集精确冲突。
            let conflicts = collect_conflicts(&docs);
            // 打印冲突详情。
            print_conflicts(&conflicts);
            // 打印冲突总数。
            println!("total_conflicts={}", conflicts.len());
        }
        "check-redundant-exacts" => {
            // 检查已被同文件同策略后缀规则覆盖的精确 HOST / DOMAIN。
            let mut docs = load_documents(FILTER_DIR)?;
            normalize_documents(&mut docs);
            let redundant_rules = collect_redundant_exact_rules(&docs);
            print_redundant_exact_rules(&redundant_rules);
            println!("total_redundant_exacts={}", redundant_rules.len());
        }
        "check-wildcard-coverage" => {
            // 只报告“可能可以由 HOST-WILDCARD 归并”的 HOST-SUFFIX 候选项。
            // 这里故意只做检查，不自动删除，因为 HOST-SUFFIX 与 HOST-WILDCARD
            // 在不同客户端里的子域匹配语义可能并不完全等价。
            let mut docs = load_documents(FILTER_DIR)?;
            normalize_documents(&mut docs);
            let candidates = collect_wildcard_coverage_candidates(&docs);
            print_wildcard_coverage_candidates(&candidates);
            println!("total_wildcard_coverage_candidates={}", candidates.len());
        }
        "resolve-redundant-exacts" => {
            // 删除这类冗余精确规则。
            let mut docs = load_documents(FILTER_DIR)?;
            normalize_documents(&mut docs);
            let removed = resolve_redundant_exact_rules(&mut docs);
            // 删除后再规范化一次，顺便同步头部统计。
            normalize_documents(&mut docs);
            if write {
                write_documents(&docs)?;
            }
            println!("removed_redundant_exacts={}", removed);
        }
        "resolve-exact-conflicts" => {
            // 清理“同类型 + 同目标值 + 不同策略”的精确冲突。
            let mut docs = load_documents(FILTER_DIR)?;
            normalize_documents(&mut docs);
            let removed = resolve_exact_conflicts(&mut docs);
            normalize_documents(&mut docs);
            if write {
                write_documents(&docs)?;
            }
            let conflicts = collect_conflicts(&docs);
            println!("removed_rules={}", removed);
            println!("remaining_conflicts={}", conflicts.len());
        }
        "all" => {
            // all 模式会串起来执行一次完整维护流程。
            let mut docs = load_documents(FILTER_DIR)?;
            let normalized_before = normalize_documents(&mut docs);
            let removed_redundant = resolve_redundant_exact_rules(&mut docs);
            let removed = resolve_exact_conflicts(&mut docs);
            let normalized_after = normalize_documents(&mut docs);
            if write {
                write_documents(&docs)?;
            }
            let conflicts = collect_conflicts(&docs);
            println!("normalized_files_before={}", normalized_before);
            println!("removed_redundant_exacts={}", removed_redundant);
            println!("removed_rules={}", removed);
            println!("normalized_files_after={}", normalized_after);
            println!("remaining_conflicts={}", conflicts.len());
        }
        _ => {
            // 未知子命令时打印帮助。
            print_usage();
        }
    }

    // 程序正常结束。
    Ok(())
}

fn print_usage() {
    // 打印脚本支持的全部命令。
    eprintln!("Usage:");
    eprintln!("  rustc --crate-name filter_maintainer scripts/filter_maintainer.rs -O -o /tmp/filter_maintainer");
    eprintln!("  /tmp/filter_maintainer normalize [--write]");
    eprintln!("  /tmp/filter_maintainer check-conflicts");
    eprintln!("  /tmp/filter_maintainer check-redundant-exacts");
    eprintln!("  /tmp/filter_maintainer check-wildcard-coverage");
    eprintln!("  /tmp/filter_maintainer resolve-redundant-exacts [--write]");
    eprintln!("  /tmp/filter_maintainer resolve-exact-conflicts [--write]");
    eprintln!("  /tmp/filter_maintainer all [--write]");
}

fn load_documents(dir: &str) -> io::Result<Vec<Document>> {
    // 收集目录里的全部 .list 文件路径。
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("list") {
            paths.push(path);
        }
    }
    // 排序后再处理，保证遍历顺序稳定。
    paths.sort();

    // 逐个解析文件。
    let mut docs = Vec::new();
    for path in paths {
        docs.push(parse_document(&path)?);
    }
    Ok(docs)
}

fn parse_document(path: &Path) -> io::Result<Document> {
    // 读取整个文件文本。
    let content = fs::read_to_string(path)?;
    // 先暂存头部注释行。
    let mut header_lines = Vec::new();
    // 进入正文后就不再把注释当作头部处理。
    let mut body_started = false;
    // 存放解析出的规则。
    let mut rules = Vec::new();

    for raw in content.lines() {
        // 去掉首尾空白，统一解析。
        let line = raw.trim();
        if !body_started && (line.is_empty() || line.starts_with('#')) {
            header_lines.push(line.to_string());
            continue;
        }

        // 到这里说明已经进入正文区域。
        body_started = true;
        // 正文中的空行或注释直接忽略。
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // 能解析成规则的才加入结果。
        if let Some(rule) = parse_rule(line) {
            rules.push(rule);
        }
    }

    // 组装成完整文档对象。
    Ok(Document {
        path: path.to_path_buf(),
        header: parse_header(&header_lines),
        rules,
    })
}

fn parse_rule(line: &str) -> Option<Rule> {
    // 用逗号切分，并去掉每段两边的空格。
    let parts: Vec<String> = line.split(',').map(|part| part.trim().to_string()).collect();
    // 少于 2 段说明不是合法规则。
    if parts.len() < 2 {
        return None;
    }

    // 对 .list 文件做一个兼容处理：
    // 1. 两段式：HOST-SUFFIX,example.com
    // 2. 三段式带策略：HOST-SUFFIX,example.com,Global
    // 3. 三段式或更多带附加项：IP-CIDR,1.1.1.1/32,no-resolve
    let (policy, extras) = if parts.len() == 2 {
        (None, Vec::new())
    } else if parts[2].eq_ignore_ascii_case("no-resolve") {
        (None, parts[2..].to_vec())
    } else {
        (Some(parts[2].clone()), parts[3..].to_vec())
    };

    // 返回统一规范后的 Rule 结构。
    Some(Rule {
        kind: parts[0].to_ascii_uppercase(),
        value: parts[1].clone(),
        policy,
        extras,
    })
}

fn parse_header(lines: &[String]) -> Header {
    // comments 存普通注释，key_values 存结构化头。
    let mut comments = Vec::new();
    let mut key_values = BTreeMap::new();

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        // 能识别成 KEY: VALUE 的放进 key_values。
        if let Some((key, value)) = parse_header_key_value(trimmed) {
            key_values.insert(key, value);
        } else {
            // 否则保留原始注释。
            comments.push(trimmed.to_string());
        }
    }

    Header { comments, key_values }
}

fn parse_header_key_value(line: &str) -> Option<(String, String)> {
    // 头部键值必须从 # 开始。
    if !line.starts_with('#') {
        return None;
    }
    // 去掉开头的 # 再处理。
    let raw = line.trim_start_matches('#').trim();
    // 找到第一个冒号，左边是 key，右边是 value。
    let (key, value) = raw.split_once(':')?;
    // 把 key 统一成大写，便于后续比较。
    let normalized_key = key.trim().to_ascii_uppercase();
    if normalized_key.is_empty() {
        return None;
    }
    // 只接受常见的头部 key 格式。
    if !normalized_key
        .chars()
        .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '-')
    {
        return None;
    }
    Some((normalized_key, value.trim().to_string()))
}

fn normalize_documents(docs: &mut [Document]) -> usize {
    // changed 统计有多少文件在规范化后发生了变化。
    let mut changed = 0;
    for doc in docs {
        // 规范化前先渲染成字符串，用于比较前后差异。
        let before = render_document(doc);
        normalize_document(doc);
        let after = render_document(doc);
        if before != after {
            changed += 1;
        }
    }
    changed
}

fn normalize_document(doc: &mut Document) {
    // Normalize means:
    // 1. sort rules into a stable order
    // 2. remove exact duplicates after whitespace/case normalization
    // 3. sync header counters with the current body
    doc.rules.sort_by(|left, right| {
        let left_key = rule_sort_key(left);
        let right_key = rule_sort_key(right);
        left_key.cmp(&right_key)
    });

    let mut seen = HashSet::new();
    doc.rules.retain(|rule| seen.insert(rule_identity(rule)));

    update_header_counts(doc);
    normalize_header_format(doc);
}

fn rule_sort_key(rule: &Rule) -> (usize, String, String, String) {
    // 先按规则类型顺序排序。
    let order = RULE_ORDER
        .iter()
        .position(|kind| *kind == rule.kind)
        .unwrap_or(RULE_ORDER.len());
    // extras 也参与排序，避免附加参数顺序不稳定。
    let extras = if rule.extras.is_empty() {
        String::new()
    } else {
        rule.extras.join(",").to_ascii_lowercase()
    };
    // 最终排序键由 4 部分组成。
    (
        order,
        rule.value.to_ascii_lowercase(),
        extras,
        render_rule(rule).to_ascii_lowercase(),
    )
}

fn rule_identity(rule: &Rule) -> String {
    // 用渲染后的完整规则文本作为去重键。
    render_rule(rule)
}

fn render_rule(rule: &Rule) -> String {
    // 先放入固定的前三段。
    let mut parts = vec![rule.kind.clone(), rule.value.clone()];
    // 只有存在策略名时才输出策略段。
    if let Some(policy) = &rule.policy {
        parts.push(policy.clone());
    }
    // 再追加 extras。
    parts.extend(rule.extras.iter().cloned());
    // 最后重新拼回标准化文本。
    parts.join(",")
}

fn update_header_counts(doc: &mut Document) {
    // 先给每种计数字段初始化为 0。
    let mut counts = HashMap::new();
    for key in COUNT_KEYS {
        counts.insert(*key, 0usize);
    }

    // 遍历正文规则，累计每种类型的数量。
    for rule in &doc.rules {
        if let Some(value) = counts.get_mut(rule.kind.as_str()) {
            *value += 1;
        }
    }

    // 这里不再只更新“原本就存在”的字段。
    // 用户现在希望所有 .list 文件都补齐统一的计数头部，
    // 所以即使某个键原来缺失，也要主动写入 0 或实际数量。
    for key in COUNT_KEYS {
        doc.header
            .key_values
            .insert((*key).to_string(), counts[key].to_string());
    }

    // TOTAL 直接取正文规则总数。
    // 这样即使正文里还存在 DOMAIN 系列规则，TOTAL 也仍然准确。
    let total = doc.rules.len();
    doc.header
        .key_values
        .insert("TOTAL".to_string(), total.to_string());
}

fn normalize_header_format(doc: &mut Document) {
    // 先删除已经废弃的 DOMAIN 系列头部字段，避免它们在旧文件里继续残留。
    for obsolete_key in ["DOMAIN", "DOMAIN-KEYWORD", "DOMAIN-SUFFIX", "DOMAIN-WILDCARD"] {
        doc.header.key_values.remove(obsolete_key);
    }

    // 先确保推荐顺序中的关键头部都存在。
    // 这样后面渲染时就能稳定输出统一格式，而不会因为旧文件缺字段而跳过。
    for key in PREFERRED_HEADER_ORDER {
        if matches!(*key, "NAME" | "AUTHOR" | "REPO" | "UPDATED") {
            continue;
        }
        doc.header
            .key_values
            .entry((*key).to_string())
            .or_insert_with(|| "0".to_string());
    }

    // 先按推荐顺序重建一份有序头部。
    let mut ordered = BTreeMap::new();
    for key in PREFERRED_HEADER_ORDER {
        if let Some(value) = doc.header.key_values.get(*key) {
            ordered.insert((*key).to_string(), value.clone());
        }
    }

    // 推荐顺序之外的键也保留，追加到后面。
    for (key, value) in doc.header.key_values.clone() {
        if !ordered.contains_key(&key) {
            ordered.insert(key, value);
        }
    }

    // 回写整理后的头部键值。
    doc.header.key_values = ordered;
}

fn render_document(doc: &Document) -> String {
    // lines 用来逐行重建文件内容。
    let mut lines = Vec::new();

    // 先输出普通注释。
    for comment in &doc.header.comments {
        lines.push(comment.clone());
    }

    // 再按固定顺序输出结构化头部。
    for key in PREFERRED_HEADER_ORDER {
        if let Some(value) = doc.header.key_values.get(*key) {
            lines.push(format!("# {}: {}", key, value));
        }
    }

    // 如果有未收录到固定顺序里的键，再补到后面。
    for (key, value) in &doc.header.key_values {
        if !PREFERRED_HEADER_ORDER.contains(&key.as_str()) {
            lines.push(format!("# {}: {}", key, value));
        }
    }

    // 头部和正文之间插入一个空行。
    if !lines.is_empty() && !doc.rules.is_empty() {
        lines.push(String::new());
    }

    // 输出正文规则。
    for rule in &doc.rules {
        lines.push(render_rule(rule));
    }

    // 还原成完整文本，并保证文件末尾有换行。
    lines.join("\n") + "\n"
}

fn write_documents(docs: &[Document]) -> io::Result<()> {
    // 把全部文档逐个写回磁盘。
    for doc in docs {
        fs::write(&doc.path, render_document(doc))?;
    }
    Ok(())
}

fn collect_conflicts(docs: &[Document]) -> Vec<Conflict> {
    // 索引键是 (规则类型, 规则目标)。
    let mut index: BTreeMap<(String, String), Vec<(String, String)>> = BTreeMap::new();

    for doc in docs {
        // 只保留文件名，方便输出。
        let file = doc
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        for rule in &doc.rules {
            // 同一个目标可能在多个文件里出现。
            // 没有策略名的规则属于纯规则列表，不参与跨策略冲突判断。
            let Some(policy) = &rule.policy else {
                continue;
            };
            index
                .entry((rule.kind.clone(), rule.value.to_ascii_lowercase()))
                .or_default()
                .push((file.clone(), policy.clone()));
        }
    }

    // 找出同一个目标对应多个不同策略的情况。
    let mut conflicts = Vec::new();
    for ((kind, value), uses) in index {
        let policies: BTreeSet<_> = uses.iter().map(|(_, policy)| policy.clone()).collect();
        if policies.len() > 1 {
            conflicts.push(Conflict { kind, value, uses });
        }
    }

    conflicts
}

fn print_conflicts(conflicts: &[Conflict]) {
    // 把冲突打印成人能快速读懂的格式。
    for conflict in conflicts {
        let rendered = conflict
            .uses
            .iter()
            .map(|(file, policy)| format!("{}:{}", file, policy))
            .collect::<Vec<_>>()
            .join("; ");
        println!("{}|{} => {}", conflict.kind, conflict.value, rendered);
    }
}

fn collect_redundant_exact_rules(docs: &[Document]) -> Vec<RedundantExactRule> {
    // 存放所有“已被后缀规则覆盖的精确规则”。
    let mut redundant = Vec::new();

    for doc in docs {
        // 文件名仅用于输出结果。
        let file = doc
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        // 只取后缀规则，因为我们就是要检查精确 HOST / DOMAIN 是否被它们覆盖。
        let suffix_rules: Vec<&Rule> = doc
            .rules
            .iter()
            .filter(|rule| matches!(rule.kind.as_str(), "HOST-SUFFIX" | "DOMAIN-SUFFIX"))
            .collect();

        for rule in &doc.rules {
            // 只检查精确 HOST / DOMAIN。
            if !matches!(rule.kind.as_str(), "HOST" | "DOMAIN") {
                continue;
            }

            let exact_value = rule.value.to_ascii_lowercase();
            for suffix in &suffix_rules {
                // 必须同策略才允许认定为冗余。
                if suffix.policy != rule.policy {
                    continue;
                }

                let suffix_value = suffix.value.to_ascii_lowercase();
                // 值完全相同不算“由后缀覆盖的子域名”。
                if exact_value == suffix_value {
                    continue;
                }

                // 如果 exact_value 以 .suffix 结尾，说明它已经被 suffix 规则覆盖。
                if exact_value.ends_with(&format!(".{}", suffix_value)) {
                    redundant.push(RedundantExactRule {
                        file: file.clone(),
                        exact_kind: rule.kind.clone(),
                        exact_value: rule.value.clone(),
                        policy: rule.policy.clone().unwrap_or_default(),
                        covered_by_kind: suffix.kind.clone(),
                        covered_by_value: suffix.value.clone(),
                    });
                    // 一个精确规则被找到一个可覆盖它的后缀规则就够了。
                    break;
                }
            }
        }
    }

    redundant
}

fn print_redundant_exact_rules(rules: &[RedundantExactRule]) {
    // 输出可安全删除的精确规则，以及覆盖它的后缀规则。
    for rule in rules {
        println!(
            "{}: {}|{}|{} covered_by {}|{}",
            rule.file,
            rule.exact_kind,
            rule.exact_value,
            rule.policy,
            rule.covered_by_kind,
            rule.covered_by_value
        );
    }
}

fn resolve_redundant_exact_rules(docs: &mut [Document]) -> usize {
    // 先收集全部冗余精确规则。
    let redundant_rules = collect_redundant_exact_rules(docs);
    // removals 记录：某个文件里有哪些规则应该删除。
    let mut removals: HashMap<String, HashSet<String>> = HashMap::new();

    for rule in &redundant_rules {
        // 这里用 kind|value|policy 作为删除键，足够唯一。
        removals
            .entry(rule.file.clone())
            .or_default()
            .insert(format!("{}|{}|{}", rule.exact_kind, rule.exact_value, rule.policy));
    }

    // removed 统计实际删除数量。
    let mut removed = 0usize;
    for doc in docs {
        let file = doc
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        // 如果这个文件没有删除任务，就跳过。
        let Some(targets) = removals.get(&file) else {
            continue;
        };

        // retain 会保留返回 true 的规则。
        doc.rules.retain(|rule| {
            let key = format!(
                "{}|{}|{}",
                rule.kind,
                rule.value,
                rule.policy.clone().unwrap_or_default()
            );
            let keep = !targets.contains(&key);
            if !keep {
                removed += 1;
            }
            keep
        });
    }

    removed
}

fn collect_wildcard_coverage_candidates(docs: &[Document]) -> Vec<WildcardCoverageCandidate> {
    // 这里只做“候选提示”：
    // 如果同文件、同策略下，某条 HOST-SUFFIX 本身恰好能被现有 HOST-WILDCARD
    // 的字面模式匹配到，就把它列出来供人工判断是否值得进一步归并。
    // 之所以不直接自动删除，是因为 HOST-SUFFIX 通常还会覆盖子域，
    // 而 HOST-WILDCARD 是否覆盖对应子域，需要按客户端真实语义确认。
    let mut candidates = Vec::new();

    for doc in docs {
        let file = doc
            .path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        let wildcard_rules: Vec<&Rule> = doc
            .rules
            .iter()
            .filter(|rule| rule.kind == "HOST-WILDCARD")
            .collect();

        for rule in &doc.rules {
            // 这里只看 HOST-SUFFIX，因为用户当前关注的是 YouTube 这类域名后缀列表。
            if rule.kind != "HOST-SUFFIX" {
                continue;
            }

            let suffix_value = rule.value.to_ascii_lowercase();
            for wildcard in &wildcard_rules {
                // 必须同策略，避免跨策略误报。
                if wildcard.policy != rule.policy {
                    continue;
                }

                let wildcard_value = wildcard.value.to_ascii_lowercase();
                if wildcard_matches_host_by_label(&wildcard_value, &suffix_value) {
                    candidates.push(WildcardCoverageCandidate {
                        file: file.clone(),
                        suffix_value: rule.value.clone(),
                        policy: rule.policy.clone().unwrap_or_default(),
                        wildcard_value: wildcard.value.clone(),
                    });
                }
            }
        }
    }

    candidates
}

fn wildcard_matches_host_by_label(pattern: &str, host: &str) -> bool {
    // 这里使用“保守匹配”：
    // 1. 按 . 切成 label
    // 2. 只有当两边 label 数量完全一致时才认为可能匹配
    // 3. pattern 中的 * 仅代表“一个完整 label”
    //
    // 例如：
    // - youtube.* 可以匹配 youtube.ae
    // - youtube.*.* 可以匹配 youtube.co.uk
    // - youtube.com.* 可以匹配 youtube.com.tw
    // - 但不会把 youtube.* 视为覆盖 youtube.co.uk
    // - 也不会把 youtube.* 视为覆盖 www.youtube.ae
    let pattern_labels: Vec<&str> = pattern.split('.').collect();
    let host_labels: Vec<&str> = host.split('.').collect();

    if pattern_labels.len() != host_labels.len() {
        return false;
    }

    pattern_labels
        .iter()
        .zip(host_labels.iter())
        .all(|(pattern_label, host_label)| {
            if *pattern_label == "*" {
                !host_label.is_empty()
            } else {
                pattern_label.eq_ignore_ascii_case(host_label)
            }
        })
}

fn print_wildcard_coverage_candidates(candidates: &[WildcardCoverageCandidate]) {
    // 打印“可能可归并”的候选项，便于先人工审视。
    for candidate in candidates {
        println!(
            "{}: HOST-SUFFIX|{}|{} maybe_covered_by HOST-WILDCARD|{}",
            candidate.file, candidate.suffix_value, candidate.policy, candidate.wildcard_value
        );
    }
}

fn resolve_exact_conflicts(docs: &mut [Document]) -> usize {
    // 这里建立的索引会告诉我们：同一个目标值出现在哪些文件的哪条规则里。
    let mut index: BTreeMap<(String, String), Vec<(usize, usize, String)>> = BTreeMap::new();

    for (doc_index, doc) in docs.iter().enumerate() {
        for (rule_index, rule) in doc.rules.iter().enumerate() {
            // doc_index 和 rule_index 后面用于定位并删除规则。
            // 没有策略名的规则不做“跨策略精确冲突”处理。
            let Some(policy) = &rule.policy else {
                continue;
            };
            index.entry((rule.kind.clone(), rule.value.to_ascii_lowercase()))
                .or_default()
                .push((doc_index, rule_index, policy.clone()));
        }
    }

    // removals 记录每个文档里要删掉哪些 rule_index。
    let mut removals: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut removed = 0usize;

    for (_, uses) in index {
        // 先看这组相同目标是否真的指向了多个不同策略。
        let distinct_policies: BTreeSet<_> = uses.iter().map(|(_, _, policy)| policy.clone()).collect();
        if distinct_policies.len() <= 1 {
            continue;
        }

        // Exact conflicts are resolved by file priority so broader lists lose to
        // more specific lists. This mirrors the repository cleanup strategy.
        let winner = uses
            .iter()
            .max_by_key(|(doc_index, _, _)| {
                let file = docs[*doc_index]
                    .path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                (file_priority(file), file.to_string())
            })
            .cloned();

        if let Some((winner_doc, winner_rule, _)) = winner {
            for (doc_index, rule_index, _) in &uses {
                // 胜出的那条规则不删。
                if *doc_index == winner_doc && *rule_index == winner_rule {
                    continue;
                }
                // 其它文件中的同目标冲突规则都进删除列表。
                removals.entry(*doc_index).or_default().insert(*rule_index);
            }
        }
    }

    for (doc_index, rule_indexes) in removals {
        // 重新构建一份保留下来的规则列表。
        let mut keep = Vec::new();
        for (rule_index, rule) in docs[doc_index].rules.iter().cloned().enumerate() {
            if rule_indexes.contains(&rule_index) {
                removed += 1;
            } else {
                keep.push(rule);
            }
        }
        docs[doc_index].rules = keep;
    }

    removed
}

fn file_priority(file: &str) -> i32 {
    // Higher numbers win when the same exact rule target appears in multiple
    // files but points to different policies.
    match file {
        "ChinaIPs.list" => 100,
        "YouTube.list" => 95,
        "Netflix.list" => 94,
        "Media.list" => 93,
        "China.list" => 92,
        "Static.list" => 91,
        "Apple.list" => 90,
        "ChatGPT.list" => 90,
        "Spotify.list" => 90,
        "Solana.list" => 60,
        "Global.list" => 10,
        _ => 80,
    }
}

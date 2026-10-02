// First word of a model name, and who makes it.
// Not complete. Send a PR if yours is missing.
const VENDORS: &[(&str, &str)] = &[
    ("aya", "cohere"),
    ("chatgpt", "openai"),
    ("claude", "anthropic"),
    ("codestral", "mistral"),
    ("codex", "openai"),
    ("command", "cohere"),
    ("deepseek", "deepseek"),
    ("devstral", "mistral"),
    ("ernie", "baidu"),
    ("gemini", "google"),
    ("gemma", "google"),
    ("glm", "zhipu"),
    ("gpt", "openai"),
    ("granite", "ibm"),
    ("grok", "xai"),
    ("hunyuan", "tencent"),
    ("hy", "tencent"),
    ("jamba", "ai21"),
    ("kimi", "moonshot"),
    ("ling", "inclusionai"),
    ("llama", "meta"),
    ("longcat", "meituan"),
    ("magistral", "mistral"),
    ("mercury", "inception"),
    ("mimo", "xiaomi"),
    ("minimax", "minimax"),
    ("ministral", "mistral"),
    ("mistral", "mistral"),
    ("mixtral", "mistral"),
    ("muse", "meta"),
    ("nemotron", "nvidia"),
    ("nova", "amazon"),
    ("o1", "openai"),
    ("o3", "openai"),
    ("o4", "openai"),
    ("phi", "microsoft"),
    ("pixtral", "mistral"),
    ("qwen", "alibaba"),
    ("qwq", "alibaba"),
    ("r1", "deepseek"),
    ("ring", "inclusionai"),
    ("seed", "bytedance"),
    ("solar", "upstage"),
    ("sonar", "perplexity"),
    ("step", "stepfun"),
];

// OpenRouter and friends put the vendor in front, spelled their own way.
// Anything not listed here is used as is.
const PREFIXES: &[(&str, &str)] = &[
    ("ibm-granite", "ibm"),
    ("meta-llama", "meta"),
    ("mistralai", "mistral"),
    ("moonshotai", "moonshot"),
    ("qwen", "alibaba"),
    ("x-ai", "xai"),
    ("z-ai", "zhipu"),
];

fn find(table: &[(&str, &'static str)], key: &str) -> Option<&'static str> {
    table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

pub(crate) fn lookup(prefix: Option<&str>, first_word: Option<&str>) -> Option<String> {
    if let Some(p) = prefix {
        return Some(find(PREFIXES, p).unwrap_or(p).to_string());
    }
    find(VENDORS, first_word?).map(String::from)
}

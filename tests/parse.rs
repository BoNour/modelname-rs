use std::cmp::Ordering;

use modelname::{compare, latest, parse};

struct Case {
    input: &'static str,
    vendor: &'static str,
    family: &'static str,
    version: &'static str,
    size: &'static str,
    snapshot: &'static str,
    date: &'static str,
    tags: &'static [&'static str],
}

#[allow(clippy::too_many_arguments)]
fn row(
    input: &'static str,
    vendor: &'static str,
    family: &'static str,
    version: &'static str,
    size: &'static str,
    snapshot: &'static str,
    date: &'static str,
    tags: &'static [&'static str],
) -> Case {
    Case {
        input,
        vendor,
        family,
        version,
        size,
        snapshot,
        date,
        tags,
    }
}

#[test]
fn parses_names() {
    // (input, vendor, family, version, size, snapshot, date, tags)
    let cases = [
        // API IDs
        row(
            "claude-3-5-sonnet-20241022",
            "anthropic",
            "claude-sonnet",
            "3.5",
            "",
            "",
            "2024-10-22",
            &[],
        ),
        row(
            "claude-sonnet-4-5",
            "anthropic",
            "claude-sonnet",
            "4.5",
            "",
            "",
            "",
            &[],
        ),
        row(
            "claude-opus-4-1-20250805",
            "anthropic",
            "claude-opus",
            "4.1",
            "",
            "",
            "2025-08-05",
            &[],
        ),
        row(
            "anthropic/claude-haiku-4.5",
            "anthropic",
            "claude-haiku",
            "4.5",
            "",
            "",
            "",
            &[],
        ),
        row(
            "gpt-4o-2024-08-06",
            "openai",
            "gpt",
            "4o",
            "",
            "",
            "2024-08-06",
            &[],
        ),
        row("gpt-4.1-mini", "openai", "gpt-mini", "4.1", "", "", "", &[]),
        row("gpt-4-0613", "openai", "gpt", "4", "", "0613", "", &[]),
        row(
            "chatgpt-4o-latest",
            "openai",
            "chatgpt",
            "4o",
            "",
            "",
            "",
            &["latest"],
        ),
        row("o3-mini", "openai", "o3-mini", "", "", "", "", &[]),
        row(
            "gemini-2.0-flash-001",
            "google",
            "gemini-flash",
            "2.0",
            "",
            "001",
            "",
            &[],
        ),
        row(
            "gemini-2.5-pro-exp-03-25",
            "google",
            "gemini-pro",
            "2.5",
            "",
            "03-25",
            "",
            &["exp"],
        ),
        row("x-ai/grok-4", "xai", "grok", "4", "", "", "", &[]),
        row(
            "meta-llama/llama-3.3-70b-instruct:free",
            "meta",
            "llama",
            "3.3",
            "70b",
            "",
            "",
            &["free", "instruct"],
        ),
        row(
            "mistral-large-2411",
            "mistral",
            "mistral-large",
            "",
            "",
            "2411",
            "",
            &[],
        ),
        row(
            "mistral-7b-instruct-v0.2",
            "mistral",
            "mistral",
            "0.2",
            "7b",
            "",
            "",
            &["instruct"],
        ),
        row(
            "deepseek/deepseek-v3.2",
            "deepseek",
            "deepseek",
            "3.2",
            "",
            "",
            "",
            &[],
        ),
        row(
            "deepseek-chat-v3-1",
            "deepseek",
            "deepseek-chat",
            "3.1",
            "",
            "",
            "",
            &[],
        ),
        row(
            "qwen/qwen3-235b-a22b",
            "alibaba",
            "qwen",
            "3",
            "235b-a22b",
            "",
            "",
            &[],
        ),
        row(
            "moonshotai/kimi-k2",
            "moonshot",
            "kimi",
            "k2",
            "",
            "",
            "",
            &[],
        ),
        row("glm-4-6", "zhipu", "glm", "4.6", "", "", "", &[]),
        // Display names, from https://whizi.io/models/
        row(
            "Claude Opus 4.8",
            "anthropic",
            "claude-opus",
            "4.8",
            "",
            "",
            "",
            &[],
        ),
        row(
            "Claude Fable 5",
            "anthropic",
            "claude-fable",
            "5",
            "",
            "",
            "",
            &[],
        ),
        row("GPT-5.4 Nano", "openai", "gpt-nano", "5.4", "", "", "", &[]),
        row("GPT-6.1 Sol", "openai", "gpt-sol", "6.1", "", "", "", &[]),
        row(
            "GPT Chat Latest",
            "openai",
            "gpt-chat",
            "",
            "",
            "",
            "",
            &["latest"],
        ),
        row(
            "Gemini 3.1 Pro Preview",
            "google",
            "gemini-pro",
            "3.1",
            "",
            "",
            "",
            &["preview"],
        ),
        row(
            "Gemini 3.5 Flash Lite",
            "google",
            "gemini-flash-lite",
            "3.5",
            "",
            "",
            "",
            &[],
        ),
        row(
            "Grok Build 0.1",
            "xai",
            "grok-build",
            "0.1",
            "",
            "",
            "",
            &[],
        ),
        row(
            "DeepSeek V4 Pro 0813",
            "deepseek",
            "deepseek-pro",
            "4",
            "",
            "0813",
            "",
            &[],
        ),
        row(
            "DeepSeek V4.1 Flash",
            "deepseek",
            "deepseek-flash",
            "4.1",
            "",
            "",
            "",
            &[],
        ),
        row("R1", "deepseek", "r1", "", "", "", "", &[]),
        row(
            "Qwen3.5 Plus 2026-04-20",
            "alibaba",
            "qwen-plus",
            "3.5",
            "",
            "",
            "2026-04-20",
            &[],
        ),
        row(
            "Qwen3.6 35B A3B",
            "alibaba",
            "qwen",
            "3.6",
            "35b-a3b",
            "",
            "",
            &[],
        ),
        row(
            "Qwen3.8 2.4T A95B",
            "alibaba",
            "qwen",
            "3.8",
            "2.4t-a95b",
            "",
            "",
            &[],
        ),
        row(
            "Qwen3 Coder Next",
            "alibaba",
            "qwen-coder-next",
            "3",
            "",
            "",
            "",
            &[],
        ),
        row(
            "Kimi K2 0711",
            "moonshot",
            "kimi",
            "k2",
            "",
            "0711",
            "",
            &[],
        ),
        row(
            "Kimi K2.7 Code",
            "moonshot",
            "kimi-code",
            "k2.7",
            "",
            "",
            "",
            &[],
        ),
        row(
            "Kimi K2 Thinking",
            "moonshot",
            "kimi",
            "k2",
            "",
            "",
            "",
            &["thinking"],
        ),
        row("MiniMax M3", "minimax", "minimax", "m3", "", "", "", &[]),
        row(
            "MiMo-V2.6-Pro-UltraSpeed",
            "xiaomi",
            "mimo-pro-ultraspeed",
            "2.6",
            "",
            "",
            "",
            &[],
        ),
        row(
            "Mistral Large 3 2512",
            "mistral",
            "mistral-large",
            "3",
            "",
            "2512",
            "",
            &[],
        ),
        row(
            "Codestral 2508",
            "mistral",
            "codestral",
            "",
            "",
            "2508",
            "",
            &[],
        ),
        row(
            "Llama 3.3 70B Instruct",
            "meta",
            "llama",
            "3.3",
            "70b",
            "",
            "",
            &["instruct"],
        ),
        row(
            "Llama 4 Maverick",
            "meta",
            "llama-maverick",
            "4",
            "",
            "",
            "",
            &[],
        ),
        row("Granite 4.2 8B", "ibm", "granite", "4.2", "8b", "", "", &[]),
        row(
            "Nemotron 3 Ultra",
            "nvidia",
            "nemotron-ultra",
            "3",
            "",
            "",
            "",
            &[],
        ),
        row("Command A+", "cohere", "command-a+", "", "", "", "", &[]),
        row(
            "Hy4 preview",
            "tencent",
            "hy",
            "4",
            "",
            "",
            "",
            &["preview"],
        ),
        row(
            "Ring-2.6-1T",
            "inclusionai",
            "ring",
            "2.6",
            "1t",
            "",
            "",
            &[],
        ),
        row(
            "KAT-Coder-Pro V2.5",
            "",
            "kat-coder-pro",
            "2.5",
            "",
            "",
            "",
            &[],
        ),
        row("Nova Pro 1.0", "amazon", "nova-pro", "1.0", "", "", "", &[]),
        row("Phi 4", "microsoft", "phi", "4", "", "", "", &[]),
        row("Solar Pro 4", "upstage", "solar-pro", "4", "", "", "", &[]),
        row("Perceptron Mk1", "", "perceptron", "mk1", "", "", "", &[]),
        row("Inkling Small", "", "inkling-small", "", "", "", "", &[]),
    ];

    let mut failed = 0;
    for c in &cases {
        let m = parse(c.input);
        let mut tags: Vec<&str> = m.tags.iter().map(String::as_str).collect();
        tags.sort_unstable();
        let date = m.date.map(|d| d.to_string()).unwrap_or_default();
        let got = (
            m.vendor.as_deref().unwrap_or(""),
            m.family.as_str(),
            m.version.as_deref().unwrap_or(""),
            m.size.as_deref().unwrap_or(""),
            m.snapshot.as_deref().unwrap_or(""),
            date.as_str(),
            tags,
        );
        let want = (
            c.vendor,
            c.family,
            c.version,
            c.size,
            c.snapshot,
            c.date,
            c.tags.to_vec(),
        );
        if got != want {
            eprintln!(
                "parse({:?})
  got  {:?}
  want {:?}",
                c.input, got, want
            );
            failed += 1;
        }
    }
    assert_eq!(
        failed,
        0,
        "{} of {} names parsed wrong",
        failed,
        cases.len()
    );
}

#[test]
fn old_and_new_claude_names_share_a_family() {
    let a = parse("claude-3-5-sonnet-20241022");
    let b = parse("Claude Sonnet 4.5");
    assert_eq!(a.family, b.family);
    assert_eq!(compare(&a, &b), Ordering::Less);
}

#[test]
fn orders_versions_and_dates() {
    let pairs = [
        ("gemini-2.5-flash", "gemini-3.10-flash"),
        ("gemini-3.9-flash", "gemini-3.10-flash"),
        ("kimi-k2-0711", "kimi-k2-0905"),
        ("kimi-k2-0711", "kimi-k2.7-code"),
        ("kimi-k2.7-code", "kimi-k3"),
        ("claude-opus-4-1-20250805", "claude-opus-4-1-20250901"),
        ("claude-3-5-sonnet-20240620", "claude-3-5-sonnet-20241022"),
    ];
    for (older, newer) in pairs {
        let (a, b) = (parse(older), parse(newer));
        assert_eq!(compare(&a, &b), Ordering::Less, "{} < {}", older, newer);
        assert_eq!(compare(&b, &a), Ordering::Greater, "{} > {}", newer, older);
    }
    assert_eq!(compare(&parse("gpt-5"), &parse("GPT-5")), Ordering::Equal);
}

#[test]
fn picks_the_latest() {
    let ids = [
        "gpt-4o",
        "claude-sonnet-4-5",
        "claude-3-7-sonnet-20250219",
        "claude-sonnet-4-6",
        "claude-opus-5",
        "gemini-2.5-flash",
    ];
    assert_eq!(latest(ids, "claude-sonnet"), Some("claude-sonnet-4-6"));
    assert_eq!(latest(ids, "llama"), None);
}

#[test]
fn from_str_works() {
    let m: modelname::Model = "gpt-4.1-mini".parse().unwrap();
    assert_eq!(m.family, "gpt-mini");
}

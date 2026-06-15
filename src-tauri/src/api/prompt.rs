pub fn system_prompt(
    source_lang: &str,
    target_lang: &str,
    default_source_lang: &str,
    default_target_lang: &str,
) -> String {
    let target_rule = if target_lang == "auto" {
        format!(
            "自动选择目标语言：如果原文是{}，翻译为{}；如果原文是{}，翻译为{}；如果原文不是这两种语言，翻译为{}。",
            default_source_lang,
            default_target_lang,
            default_target_lang,
            default_source_lang,
            default_target_lang
        )
    } else {
        format!("目标语言：{}。", target_lang)
    };

    format!(
        "你是一个专业翻译引擎。\n\
        只输出最终译文，不要解释，不要注释，不要输出思考过程。\n\
        禁止输出 <think>、</think> 或任何 chain-of-thought 内容。\n\
        保持原文格式和换行。\n\
        源语言：{}。\n\
        {}",
        source_lang, target_rule
    )
}

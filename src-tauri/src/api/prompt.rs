pub fn system_prompt(source_lang: &str, target_lang: &str) -> String {
    let target_rule = if source_lang == "auto" && target_lang == "zh-CN" {
        "如果原文是中文，翻译为英文；如果原文不是中文，翻译为简体中文。".to_string()
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

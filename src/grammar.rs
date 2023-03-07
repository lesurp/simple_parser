#[macro_export]
macro_rules! grammar {
    ($($key:ident => { $($rule_set:tt)|* $(|)? }),* $(,)?) => {
        {
            let mut rule_map = std::collections::HashMap::<ParseKey, AlternativeRules>::new();
            $(
                let mut alternative_rules = Vec::new();
                $(
                    let rule_set = _rule_set!($rule_set);
                    alternative_rules.push(rule_set);
                )*
                rule_map.insert($crate::ParseKey(stringify!($key)), alternative_rules);
            )*
                rule_map
        }
    };
}

macro_rules! _rule_set {
    () => { Vec::new() };
    ([$($tokens:tt)*]) => {
        _rule_set!($($tokens)*)
    };
    ($t:literal, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend(_rule_set!($t));
        rule_set.extend(_rule_set!($($tail)*));
        rule_set
    }};
    ($t:ident, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend(_rule_set!($t));
        rule_set.extend(_rule_set!($($tail)*));
        rule_set
    }};
    ($t:expr, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend(_rule_set!($t));
        rule_set.extend(_rule_set!($($tail)*));
        rule_set
    }};
    ($rule:literal) => {
        vec![$crate::Rule::Terminal(Box::new(parse_token::<$rule>))]
    };
    ($rule:ident) => {
        vec![$crate::Rule::NonTerminal($crate::ParseKey(stringify!($rule)))]
    };
    ($rule:expr) => {
        vec![$rule]
    };
}

#[macro_export]
macro_rules! grammar {
    ($($key:ident => { $($rule_set:tt)|* $(|)? }),* $(,)?) => {
        {
            let mut rule_map = std::collections::HashMap::<ParseKey, AlternativeRules>::new();
            $(
                let mut alternative_rules = Vec::new();
                $(
                    let rule_set = $crate::grammar!(@ $rule_set);
                    alternative_rules.push(rule_set);
                )*
                rule_map.insert($crate::ParseKey(stringify!($key)), alternative_rules);
            )*
            rule_map
        }
    };
    (@) => { Vec::new() };
    (@ [$($tokens:tt)*]) => {
        $crate::grammar!(@ $($tokens)*)
    };
    (@ $t:literal, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend($crate::grammar!(@ $t));
        rule_set.extend($crate::grammar!(@ $($tail)*));
        rule_set
    }};
    (@ $t:ident, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend($crate::grammar!(@ $t));
        rule_set.extend($crate::grammar!(@ $($tail)*));
        rule_set
    }};
    (@ $t:expr, $($tail:tt)*) => {{
        let mut rule_set: $crate::RuleSet = Vec::new();
        rule_set.extend($crate::grammar!(@ $t));
        rule_set.extend($crate::grammar!(@ $($tail)*));
        rule_set
    }};
    (@ $rule:literal) => {
        vec![$crate::Rule::Terminal(Box::new(parse_token::<$rule>))]
    };
    (@ $rule:ident) => {
        vec![$crate::Rule::NonTerminal($crate::ParseKey(stringify!($rule)))]
    };
    (@ $rule:expr) => {
        vec![$rule]
    };
}

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use rustpython_parser::ast::{self, Expr, Ranged};

use ruff_diagnostics::{AlwaysAutofixableViolation, Diagnostic, Edit, Fix};
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;
use crate::registry::AsRule;

/// ## What it does
/// Checks for uses of deprecated methods from the `unittest` module.
///
/// ## Why is this bad?
/// The `unittest` module has deprecated aliases for some of its methods.
/// The aliases may be removed in future versions of Python. Instead,
/// use their non-deprecated counterparts.
///
/// ## Example
/// ```python
/// from unittest import TestCase
///
///
/// class SomeTest(TestCase):
///     def test_something(self):
///         self.assertEquals(1, 1)
/// ```
///
/// Use instead:
/// ```python
/// from unittest import TestCase
///
///
/// class SomeTest(TestCase):
///     def test_something(self):
///         self.assertEqual(1, 1)
/// ```
///
/// ## References
/// - [Python documentation: Deprecated aliases](https://docs.python.org/3/library/unittest.html#deprecated-aliases)
#[violation]
pub struct DeprecatedUnittestAlias {
    alias: String,
    target: String,
}

impl AlwaysAutofixableViolation for DeprecatedUnittestAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DeprecatedUnittestAlias { alias, target } = self;
        format!("`{alias}` is deprecated, use `{target}`")
    }

    fn autofix_title(&self) -> String {
        let DeprecatedUnittestAlias { alias, target } = self;
        format!("Replace `{target}` with `{alias}`")
    }
}

static DEPRECATED_ALIASES: Lazy<FxHashMap<&'static str, &'static str>> = Lazy::new(|| {
    FxHashMap::from_iter([
        ("failUnlessEqual", "assertEqual"),
        ("assertEquals", "assertEqual"),
        ("failIfEqual", "assertNotEqual"),
        ("assertNotEquals", "assertNotEqual"),
        ("failUnless", "assertTrue"),
        ("assert_", "assertTrue"),
        ("failIf", "assertFalse"),
        ("failUnlessRaises", "assertRaises"),
        ("failUnlessAlmostEqual", "assertAlmostEqual"),
        ("assertAlmostEquals", "assertAlmostEqual"),
        ("failIfAlmostEqual", "assertNotAlmostEqual"),
        ("assertNotAlmostEquals", "assertNotAlmostEqual"),
        ("assertRegexpMatches", "assertRegex"),
        ("assertNotRegexpMatches", "assertNotRegex"),
        ("assertRaisesRegexp", "assertRaisesRegex"),
    ])
});

/// UP005
pub(crate) fn deprecated_unittest_alias(checker: &mut Checker, expr: &Expr) {
    let Expr::Attribute(ast::ExprAttribute { value, attr, .. }) = expr else {
        return;
    };
    let Some(target) = DEPRECATED_ALIASES.get(attr.as_str()) else {
        return;
    };
    let Expr::Name(ast::ExprName { id, .. }) = value.as_ref() else {
        return;
    };
    if id != "self" {
        return;
    }
    let mut diagnostic = Diagnostic::new(
        DeprecatedUnittestAlias {
            alias: attr.to_string(),
            target: (*target).to_string(),
        },
        expr.range(),
    );
    if checker.patch(diagnostic.kind.rule()) {
        diagnostic.set_fix(Fix::suggested(Edit::range_replacement(
            format!("self.{target}"),
            expr.range(),
        )));
    }
    checker.diagnostics.push(diagnostic);
}

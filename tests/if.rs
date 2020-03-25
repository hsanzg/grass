#![cfg(test)]

#[macro_use]
mod macros;

test!(
    if_toplevel_true,
    "@if true {\n  a {\n    color: foo;\n}\n}\n",
    "a {\n  color: foo;\n}\n"
);
// test!(
//     if_inner_true,
//     "a {\n  @if true {\n    color: foo;\n}\n}\n",
//     "a {\n  color: foo;\n}\n"
// );
// test!(
//     if_toplevel_false,
//     "@if false {\n  a {\n    color: foo;\n}\n}\n",
//     ""
// );
// test!(
//     if_inner_false,
//     "a {\n  @if false {\n    color: foo;\n}\n}\n",
//     ""
// );
// test!(
//     if_else_toplevel_true,
//     "@if true {\n  a {\n    color: foo;\n}\n} @else {\n  b {\n    color: bar;\n}\n}\n",
//     "a {\n  color: foo;\n}\n"
// );
// test!(
//     if_else_inner_true,
//     "a {\n  @if true {\n    color: foo;\n} @else {\n    color: bar;\n}\n}\n",
//     "a {\n  color: foo;\n}\n"
// );
// test!(
//     if_else_toplevel_false,
//     "@if false {\n  a {\n    color: foo;\n}\n} @else {\n  a {\n    color: bar;\n}\n}\n",
//     "a {\n  color: bar;\n}\n"
// );
// test!(
//     if_else_inner_false,
//     "a {\n  @if false {\n    color: foo;\n} @else {\n    color: bar;\n}\n}\n",
//     "a {\n  color: bar;\n}\n"
// );
// error!(
//     no_brace_after_else,
//     "@if false {} @else -}", "Error: expected \"{\"."
// );
// test!(
//     if_else_if_no_else,
//     "a {\n  @if false {\n    color: red;\n} @else if true {\n    color: blue;\n}\n}\n",
//     "a {\n  color: blue;\n}\n"
// );
// test!(
//     if_false_else_if_false_else,
//     "a {\n  @if false {\n    color: red;\n} @else if false {\n    color: blue;\n} @else {\n    color: green;\n}\n}\n",
//     "a {\n  color: green;\n}\n"
// );
// test!(
//     if_false_else_if_true_else,
//     "a {\n  @if false {\n    color: red;\n} @else if true {\n    color: blue;\n} @else {\n    color: green;\n}\n}\n",
//     "a {\n  color: blue;\n}\n"
// );
// test!(
//     if_inner_style_missing_semicolon,
//     "a {\n  @if true {\n    color: red\n  }\n}\n",
//     "a {\n  color: red;\n}\n"
// );

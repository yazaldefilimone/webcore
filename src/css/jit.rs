use crate::css;

// Implement a JIT compiler for CSS baded on WebKit's CSS JIT compiler.
// https://www.webkit.org/blog/3271/webkit-css-selector-jit-compiler/
//
//

pub struct CssJitCompiler {
  pub stylesheet: css::StyleSheet,
}

impl CssJitCompiler {
  pub fn new(stylesheet: css::StyleSheet) -> Self {
    CssJitCompiler { stylesheet }
  }
}

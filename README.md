# Hermes
[![Windows](https://github.com/chadc1050/Hermes/actions/workflows/windows-build.yml/badge.svg)](https://github.com/chadc1050/Hermes/actions/workflows/windows-build.yml)
[![Linux Run](https://github.com/chadc1050/Hermes/actions/workflows/linux-build.yml/badge.svg)](https://github.com/chadc1050/Hermes/actions/workflows/linux-build.yml)

Hermes aims to be a drop in replacement for JS to allow for seamless opt in for WASM targeting. It will be a super set of JS, with additional opt in features to improve performance. With Hermes, you will be able to take any existing JS package and easily compile it with minimal overhead. The benefit of this is ultimatley that the developer 
will be able to access the existing massive ecosystem of JS packages, whilst being able to write JS interopable code. 

## Roadmap
### Emulated JS Compiler
1. AST Lowering
   - Reader
   - Lexer
   - Parser 
2. HLIR Lowering
   - Type inferencing and Checking
3. MLIR Lowering
4. LLVM Lowering
   - CodeGen

### Feature Enhanced Compiler
- Traits
- Explicit References

### StdLib
- JS Interpreter Runtime Standard Library Compatibility
  - NodeJS
  - BunJS
  - Deno
  - Just
  - Txiki
  - Napa
  - Elsa
  - Window
  - Additional Features

### Built-In Testing Library
- Syntactic similarity to Jest

### Package Manager
- API Compatibility with existing JS Package Managers (NPM, Yarn, ect...)
- API for accessing Hermes-first packages

### Toolchain Extensions
- Conversion tool (TS -> JS -> Hermes)

## Documentation
### Links
#### WASM
- [WASM Specification](https://webassembly.github.io/spec/core/)

#### Javascript
- [Javascript Specification](https://developer.mozilla.org/en-US/docs/Web/JavaScript)
- [ECMAScript Specification](https://262.ecma-international.org/#sec-ecmascript-language-lexical-grammar)

#### LLVM
- [LLVM Specification](https://llvm.org/docs/LangRef.html)
- [llvm-sys](https://crates.io/crates/llvm-sys)
- [llvmenv](https://crates.io/crates/llvmenv)
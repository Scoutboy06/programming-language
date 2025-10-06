use compiler::{compile, CompilerTarget};

fn main() {
    let code = r#"
    var a = 123;
    let b = 456;
    const c = 789;
    "#;

    compile(&code, CompilerTarget::Wasm, "out.wasm");
}

use compiler::{compile, CompilerTarget};

fn main() {
    let code = r#"
    function sum(num1: number, num2: number): number {
        return num1 + num2;
    }
    "#;

    let compiler_target = CompilerTarget::Wasm;

    compile(&code, compiler_target);
}

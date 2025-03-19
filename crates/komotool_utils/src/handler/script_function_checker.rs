use bevy_mod_scripting::lua::LuaScriptingPlugin;
use bevy_mod_scripting::rhai::rhai::{Engine, AST};
use bevy_mod_scripting::rhai::RhaiScriptingPlugin;
use full_moon::{
    ast::{self, Ast},
    parse,
};
use std::collections::HashSet;

pub trait ScriptFunctionChecker {
    /// Check if a script implementation contains a specific function
    fn has_function(script_bytes: &[u8], function_name: &str) -> bool;
    fn get_functions(script_bytes: &[u8]) -> HashSet<String>;
}

impl ScriptFunctionChecker for RhaiScriptingPlugin {
    fn has_function(script_bytes: &[u8], function_name: &str) -> bool {
        has_rhai_function(script_bytes, function_name)
    }

    fn get_functions(script_bytes: &[u8]) -> HashSet<String> {
        extract_rhai_functions(script_bytes)
    }
}

fn has_rhai_function(rhai_code: &[u8], function_name: &str) -> bool {
    // Convert bytes to a UTF-8 string
    let code_str = match std::str::from_utf8(rhai_code) {
        Ok(s) => s,
        Err(_) => return false, // Return false if the code isn't valid UTF-8
    };

    // Create a Rhai engine
    let engine = Engine::new();

    // Try to compile the code to AST
    match engine.compile(code_str) {
        Ok(ast) => contains_function(&ast, function_name),
        Err(_) => false, // Return false if compilation fails
    }
}

fn contains_function(ast: &AST, function_name: &str) -> bool {
    // Get all function definitions from the AST
    for fn_def in ast.iter_functions() {
        let name = fn_def.name;

        // Compare with the target function name
        if name == function_name {
            println!("Function name: {}", name);
            return true;
        }
    }
    false
}

fn extract_rhai_functions(script_bytes: &[u8]) -> HashSet<String> {
    let code_str = match std::str::from_utf8(script_bytes) {
        Ok(s) => s,
        Err(_) => return HashSet::new(),
    };
    let engine = Engine::new();
    match engine.compile(code_str) {
        Ok(ast) => ast
            .iter_functions()
            .map(|fn_def| fn_def.name.to_string())
            .collect(),
        Err(_) => HashSet::new(),
    }
}

impl ScriptFunctionChecker for LuaScriptingPlugin {
    fn has_function(script_bytes: &[u8], function_name: &str) -> bool {
        has_global_function(script_bytes, function_name)
    }

    fn get_functions(script_bytes: &[u8]) -> HashSet<String> {
        get_lua_functions(script_bytes)
    }
}

fn has_global_function(lua_code: &[u8], function_name: &str) -> bool {
    // Convert bytes to a UTF-8 string
    let code_str = match std::str::from_utf8(lua_code) {
        Ok(s) => s,
        Err(_) => return false, // Return false if the code isn't valid UTF-8
    };

    // Parse the Lua code to create an AST
    let ast = match parse(code_str) {
        Ok(ast) => ast,
        Err(_) => return false, // Return false if parsing fails
    };

    // Iterate through all function declarations in the AST
    contains_global_function(&ast, function_name)
}

fn contains_global_function(ast: &Ast, function_name: &str) -> bool {
    let statements = ast.nodes().stmts();

    for stmt in statements {
        if let ast::Stmt::FunctionDeclaration(func_decl) = stmt {
            // Check if this is a global function (not a method)
            let name_ref = func_decl.name().names().to_string();
            // Extract the function name from the AST
            //let name = name_ref.token().to_string();

            // Compare with the target function name
            if name_ref == function_name {
                println!("Function name: {}", name_ref);
                return true;
            }
        }
    }

    false
}

fn get_lua_functions(lua_code: &[u8]) -> HashSet<String> {
    let mut functions = HashSet::new();

    // Convert bytes to a UTF-8 string
    let code_str = match std::str::from_utf8(lua_code) {
        Ok(s) => s,
        Err(_) => return functions, // Return empty set if the code isn't valid UTF-8
    };

    // Parse the Lua code to create an AST
    let ast = match parse(code_str) {
        Ok(ast) => ast,
        Err(_) => return functions, // Return empty set if parsing fails
    };

    // Collect all function declarations in the AST
    collect_lua_functions(&ast, &mut functions);

    functions
}

fn collect_lua_functions(ast: &Ast, functions: &mut HashSet<String>) {
    let statements = ast.nodes().stmts();
    for stmt in statements {
        if let ast::Stmt::FunctionDeclaration(func_decl) = stmt {
            // Only collect global functions (not methods)
            let name_ref = func_decl.name().names().to_string();
            functions.insert(name_ref);
        }
    }
}

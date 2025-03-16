pub trait ScriptFunctionChecker {
    /// Check if a script implementation contains a specific function
    fn has_function(script_bytes: &[u8], function_name: &str) -> bool;
}

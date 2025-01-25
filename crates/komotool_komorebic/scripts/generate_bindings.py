import json
import re
import sys

def snake_to_camel(name: str) -> str:
    """
    Converts snake_case string to CamelCase
    Example: "operation_direction" -> "OperationDirection"
    """
    return re.sub(r'(?:^|_)([a-z])', lambda m: m.group(1).upper(), name)

def camel_to_snake(name):
    return re.sub(r'(?<!^)(?=[A-Z])', '_', name).lower()

def get_param_type(content: dict, schema: dict) -> str:
    """Resolve parameter types to primitives for registration functions"""
    # Handle array items that are lists (tuples)
    if isinstance(content, list):
        # Return tuple type string
        types = [get_param_type(item, schema) for item in content]
        return f"({', '.join(types)})"
    
    if '$ref' in content:
        type_name = content['$ref'].split('/')[-1]
        def_data = schema['definitions'].get(type_name, {})
        if 'enum' in def_data:
            return 'String'
        return type_name
    
    item_type = content.get('type', 'string')
    if isinstance(item_type, list):
        item_type = [t for t in item_type if t != 'null'][0]
    
    # Handle integer formats explicitly
    if item_type == 'integer':
        fmt = content.get('format', 'int32')
        return {
            'uint': 'usize',
            'uint8': 'u8',
            'uint16': 'u16',
            'uint32': 'u32',
            'uint64': 'u64',
            'int': 'isize',
            'int8': 'i8', 
            'int16': 'i16',
            'int32': 'i32',
            'int64': 'i64'
        }.get(fmt, 'i32')  # Default to i32 if format unknown
    
    return {
        'string': 'String',
        'boolean': 'bool',
        'array': 'Vec<String>'
    }.get(item_type, 'String')

def generate_param_list(content: dict, schema: dict) -> list:
    """Generate parameters with primitive types for registration functions"""
    if content.get('type') == 'array':
        items = content.get('items', {})
        
        # Handle tuple types (array items as list)
        if isinstance(items, list):
            params = []
            for i, item in enumerate(items):
                if '$ref' in item:
                    # Handle referenced types
                    type_name = item['$ref'].split('/')[-1]
                    def_data = schema['definitions'].get(type_name, {})
                    if 'enum' in def_data:
                        # Use snake_cased type name for enum parameters
                        param_name = camel_to_snake(type_name)
                    else:
                        # Fallback to numbered params for objects
                        param_name = f"param_{i}"
                else:
                    # Use numbered params for primitives
                    param_name = f"param_{i}"
                
                params.append((param_name, get_param_type(item, schema)))
            return params
        
        # Handle single-type arrays
        return [("params", get_param_type(items, schema))]
    
    if '$ref' in content:
        type_name = content['$ref'].split('/')[-1]
        def_data = schema['definitions'].get(type_name, {})
        if def_data.get('type') == 'object':
            return [
                (camel_to_snake(p), get_param_type(prop, schema))
                for p, prop in def_data.get('properties', {}).items()
            ]
        return [(camel_to_snake(type_name), get_param_type(content, schema))]
    
    return [("param", get_param_type(content, schema))]

def generate_conversion_code(def_name: str, def_data: dict, param_name: str) -> str:
    """Generate type conversion code for registration functions"""
    if 'enum' in def_data:
        variants = def_data['enum']
        code = f"let {param_name}: {def_name} = match {camel_to_snake(def_name)}.to_lowercase().as_str() {{\n"
        for v in variants:
            code += f'    "{v.lower()}" => {def_name}::{v},\n'
        code += f'    _ => {{\n'
        code += f'        log::error!("Invalid {def_name}: {{{param_name}}}");\n'
        code += f'        return false;\n'
        code += f'    }}\n}};\n'
        return code
    
    if def_data.get('type') == 'object':
        fields = [
            f"{camel_to_snake(p)}: {p}" 
            for p in def_data.get('properties', {}).keys()
        ]
        return f"let {param_name} = {def_name} {{\n    {',\n    '.join(fields)}\n}};\n"
    
    return ''

def generate_registrations(schema_file):
    with open(schema_file) as f:
        schema = json.load(f)
    
    print("NamespaceBuilder::<KomorebiMessageWrapper>::new(world)")
    
    for variant in schema['oneOf']:
        msg_type = variant['properties']['type']['enum'][0]
        fn_name = camel_to_snake(msg_type)
        registration = f'.register("{fn_name}", '
        
        if 'content' not in variant.get('required', []):
            registration += "|| {\n    let message = SocketMessage::" + msg_type + ";\n"
        else:
            content = variant['properties']['content']
            param_list = generate_param_list(content, schema)
            param_str = ', '.join([f"{name}: {typ}" for name, typ in param_list])
            
            registration += f"|{param_str}| {{\n"
            
            conversions = []
            converted_params = []
            
            # Process each parameter for potential conversion
            for param_name, original_type in param_list:
                if snake_to_camel(param_name) in schema['definitions']:
                    def_data = schema['definitions'][snake_to_camel(param_name)]
                    conversion = generate_conversion_code(
                        def_name=snake_to_camel(param_name),
                        def_data=def_data,
                        param_name=param_name
                    )
                    conversions.append(conversion)
                    converted_params.append(param_name)
                else:
                    converted_params.append(param_name)
            
            # Add all conversion blocks
            registration += '\n'.join(conversions)
            
            # Build message with converted params
            # Convert snake_case parameter names to CamelCase for enum variants
            camelized_params = [snake_to_camel(p) if p in schema['definitions'] else p for p in converted_params]
            registration += f"    let message = SocketMessage::{msg_type}({', '.join(camelized_params)});\n"
        
        registration += f"""    match send_message(&message) {{
        Ok(_) => true,
        Err(e) => {{
            log::error!("Failed to send {fn_name} message: {{}}", e);
            false
        }}
    }}
}})"""
        print(f"    {registration}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python generate_bindings.py <schema.json>")
        sys.exit(1)
    generate_registrations(sys.argv[1])

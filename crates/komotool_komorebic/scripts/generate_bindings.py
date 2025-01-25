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

def get_param_type(content: dict, schema: dict) -> tuple[str, bool]:
    """Resolve parameter types to primitives and check for optional (null) types"""
    # Handle array items that are lists (tuples)
    if isinstance(content, list):
        # Return tuple type string
        types = [get_param_type(item, schema)[0] for item in content]
        return (f"({', '.join(types)})", False)
    
    if '$ref' in content:
        type_name = content['$ref'].split('/')[-1]
        def_data = schema['definitions'].get(type_name, {})
        
        # Handle union enums like MoveBehaviour
        if 'oneOf' in def_data:
            if all('enum' in item for item in def_data['oneOf']):
                return ('String', False)
            # Handle enum struct variants
            if any('properties' in item for item in def_data['oneOf']):
                return ('String', False)
            
        if 'enum' in def_data:
            return ('String', False)
        return (type_name, False)

    is_optional = False
    item_type = content.get('type', 'string')
    
    if isinstance(item_type, list):
        if 'null' in item_type:
            is_optional = True
            # Get first non-null type
            item_type = next(t for t in item_type if t != 'null')
        else:
            item_type = item_type[0]
    
    # Handle integer formats explicitly
    if item_type == 'integer':
        fmt = content.get('format', 'int32')
        rust_type = {
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
    else:
        rust_type = {
            'string': 'String',
            'boolean': 'bool',
            'array': 'Vec<String>'
        }.get(item_type, 'String')
    
    return (rust_type, is_optional)

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
                
                params.append((param_name, *get_param_type(item, schema)))
            return params
        
        # Handle single-type arrays
        return [("params", *get_param_type(items, schema))]
    
    if '$ref' in content:
        type_name = content['$ref'].split('/')[-1]
        def_data = schema['definitions'].get(type_name, {})
        
        # Handle enum struct variants
        if 'oneOf' in def_data:
            params = []
            for variant in def_data['oneOf']:
                if 'properties' in variant:
                    # Add discriminant field (palette)
                    if 'palette' in variant['properties']:
                        params.append(("palette", "String", False))
                    # Add variant fields
                    for prop_name, prop in variant['properties'].items():
                        if prop_name != 'palette':
                            params.append((camel_to_snake(prop_name), *get_param_type(prop, schema)))
            return params

        if def_data.get('type') == 'object':
            return [
                (camel_to_snake(p), *get_param_type(prop, schema))
                for p, prop in def_data.get('properties', {}).items()
            ]
        return [(camel_to_snake(type_name), *get_param_type(content, schema))]
    
    return [("param", *get_param_type(content, schema))]

def generate_conversion_code(def_name: str, def_data: dict, param_name: str) -> str:
    """Generate type conversion code for registration functions"""
    # Handle enum struct variants
    if 'oneOf' in def_data:
        if any('properties' in item for item in def_data['oneOf']):
            code = f"let {param_name} = match {param_name}_palette.to_lowercase().as_str() {{\n"
            
            for variant in def_data['oneOf']:
                if 'properties' in variant:
                    # Get variant name from palette enum
                    palette = variant['properties']['palette']['enum'][0].lower()
                    variant_name = snake_to_camel(palette)
                    
                    code += f'    "{palette}" => {{\n'
                    code += f'        let name = {variant_name}::from_str(&{param_name}_name)?;\n'
                    
                    # Handle each field
                    for field, spec in variant['properties'].items():
                        if field not in ['name', 'palette']:
                            field_snake = camel_to_snake(field)
                            type_name = spec['anyOf'][0]['$ref'].split('/')[-1]
                            code += f"""
            let {field_snake} = if let Some({field_snake}) = {param_name}_{field_snake} {{
                Some({type_name}::from_str(&{field_snake})?)
            }} else {{
                None
            }};"""
                    
                    # Build variant
                    code += f"\n        {def_name}::{variant_name} {{\n"
                    code += f"            name,\n"
                    for field in variant['properties']:
                        if field not in ['name', 'palette']:
                            code += f"            {field}: {camel_to_snake(field)},\n"
                    code += "        }\n    }\n"
            
            code += f'    _ => {{\n'
            code += f'        log::error!("Invalid {def_name} palette: {{{param_name}_palette}}");\n'
            code += f'        return false;\n'
            code += f'    }}\n}};\n'
            
            return code

        # Handle regular enums
        variants = [
            v for item in def_data['oneOf'] 
            for v in item.get('enum', [])
        ]
        if variants:
            code = f"let {param_name}: {def_name} = match {param_name}.to_lowercase().as_str() {{\n"
            for v in variants:
                code += f'    "{v.lower()}" => {def_name}::{v},\n'
            code += f'    _ => {{\n'
            code += f'        log::error!("Invalid {def_name}: {{{param_name}}}");\n'
            code += f'        return false;\n'
            code += f'    }}\n}};\n'
            return code
    
    # Handle regular enums
    if 'enum' in def_data:
        variants = def_data['enum']
        code = f"let {param_name}: {def_name} = match {param_name}.to_lowercase().as_str() {{\n"
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
            param_str = ', '.join([f"{name}: {typ}" for name, typ, _ in param_list])
            
            registration += f"|{param_str}| {{\n"
            
            conversions = []
            converted_params = []
            
            # Process each parameter for potential conversion
            for param_name, original_type, is_optional in param_list:
                if snake_to_camel(param_name) in schema['definitions']:
                    def_data = schema['definitions'][snake_to_camel(param_name)]
                    conversion = generate_conversion_code(
                        def_name=snake_to_camel(param_name),
                        def_data=def_data,
                        param_name=param_name
                    )
                    conversions.append(conversion)
                    converted_params.append(param_name)
                elif is_optional:
                    conversion = f"""let {param_name} = match {param_name}.as_str() {{
        "" => None,
        _ => Some({param_name}),
    }};"""
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

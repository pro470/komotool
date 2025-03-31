#!/usr/bin/env python3
import argparse


def generate_rust_components_with_registration(base_word, limit):
    """
    Generate Rust component code with type registration.
    Args:
        base_word (str): The base word to use for the struct names
        limit (int): The upper limit for the numbering
    Returns:
        str: Generated Rust code
    """
    # Start with the imports
    rust_code = "use bevy_ecs::component::Component;\n"
    rust_code += "use bevy_reflect::Reflect;\n"
    rust_code += "use bevy_app::App;\n"
    rust_code += "use bevy_ecs::system::Commands;\n"
    rust_code += "use bevy_ecs::entity::Entity;\n"
    rust_code += "use crate::resources::ExtendedMarkerMap;\n"
    rust_code += "use bevy_mod_scripting::core::bindings::DynamicComponent;\n\n"

    # Generate components
    for i in range(1, limit + 1):
        component_name = f"{base_word}{i}"
        rust_code += f'#[derive(Component, Reflect)]\n'
        rust_code += f'pub struct {component_name};\n'
        if i < limit:  # Add newline between components except after the last one
            rust_code += '\n'

    # Add registration function
    rust_code += f"\n\npub fn register_{base_word.lower()}_types(app: &mut App) {{\n"
    rust_code += "    app"

    # Add each component to the registration chain
    for i in range(1, limit + 1):
        component_name = f"{base_word}{i}"
        rust_code += f"\n        .register_type::<{component_name}>()"

    # Close the function with semicolon
    rust_code += ";\n}\n"

    # Generate insert function
    rust_code += f"\n\npub fn insert_{base_word.lower()}_marker_component(index: usize, entity: Entity, mut commands: Commands, extended_marker_map: &ExtendedMarkerMap) {{\n"
    rust_code += "    match index {\n"
    for i in range(1, limit + 1):
        component_name = f"{base_word}{i}"
        rust_code += f"        {i} => {{ commands.entity(entity).insert({component_name}); }},\n"
    rust_code += f"        n if n > {limit} => unsafe {{\n"
    rust_code += "            if let Some(component_id) = extended_marker_map.makers.get(&n) {\n"
    rust_code += "                commands.entity(entity).insert_by_id(*component_id, DynamicComponent::default());\n"
    rust_code += "            }\n"
    rust_code += "        },\n"
    rust_code += "        _ => {},\n"  # Default case for 0 or unexpected values
    rust_code += "    };\n"
    rust_code += "}\n"

    # Generate despawn function
    rust_code += f"\n\npub fn despawn_{base_word.lower()}_marker_component(index: usize, entity: Entity, mut commands: Commands, extended_marker_map: &ExtendedMarkerMap) {{\n"
    rust_code += "    match index {\n"
    for i in range(1, limit + 1):
        component_name = f"{base_word}{i}"
        rust_code += f"        {i} => {{ commands.entity(entity).remove::<{component_name}>(); }},\n"
    rust_code += f"        n if n > {limit} => {{\n"
    rust_code += "            if let Some(component_id) = extended_marker_map.makers.get(&n) {\n"
    rust_code += "                commands.entity(entity).remove_by_id(*component_id);\n"
    rust_code += "            } else {\n"
    rust_code += f"                // Optional: Log warning if needed for index {{n}}\n"
    rust_code += "            }\n"
    rust_code += "        },\n"
    rust_code += "        _ => {},\n"  # Default case for 0 or unexpected values
    rust_code += "    };\n"
    rust_code += "}\n"

    return rust_code


def main():
    parser = argparse.ArgumentParser(description='Generate Rust component structs with type registration.')
    parser.add_argument('--word', type=str, help='Base word for component names (e.g., "Workspace")')
    parser.add_argument('--limit', type=int, help='Number of components to generate')
    parser.add_argument('--output', type=str, help='Output file (optional)')

    args = parser.parse_args()

    # If arguments are not provided, prompt interactively
    base_word = args.word if args.word else input("Enter the base word for your components: ")

    if args.limit:
        limit = args.limit
    else:
        try:
            limit = int(input("Enter the number of components to generate: "))
            if limit < 1:
                print("Limit must be a positive integer.")
                return
        except ValueError:
            print("Please enter a valid number for the limit.")
            return

    rust_code = generate_rust_components_with_registration(base_word, limit)

    # Print the generated code
    print("\nGenerated Rust Code:")
    print(rust_code)

    # Save to file if specified
    if args.output:
        with open(args.output, 'w') as f:
            f.write(rust_code)
        print(f"Code saved to {args.output}")
    else:
        # Optionally, prompt to save
        file_name = f"{base_word.lower()}_maker_components.rs"
        save_to_file = input(f"Save to {file_name}? (y/n): ")
        if save_to_file.lower() == 'y':
            with open(file_name, 'w') as f:
                f.write(rust_code)
            print(f"Code saved to {file_name}")


if __name__ == "__main__":
    main()

; Operations (named only)
(operation_definition
    (operation_type) @context
    (name) @name) @item

; Fragments
(fragment_definition
    "fragment" @context
    (fragment_name (name) @name)) @item

; Type system definitions
(object_type_definition
    "type" @context
    (name) @name) @item

(interface_type_definition
    "interface" @context
    (name) @name) @item

(enum_type_definition
    "enum" @context
    (name) @name) @item

(union_type_definition
    "union" @context
    (name) @name) @item

(input_object_type_definition
    "input" @context
    (name) @name) @item

(scalar_type_definition
    "scalar" @context
    (name) @name) @item

(directive_definition
    "directive" @context
    (name) @name) @item

; Type extensions
(object_type_extension
    "extend" @context
    "type" @context
    (name) @name) @item

(interface_type_extension
    "extend" @context
    "interface" @context
    (name) @name) @item

(enum_type_extension
    "extend" @context
    "enum" @context
    (name) @name) @item

(union_type_extension
    "extend" @context
    "union" @context
    (name) @name) @item

(input_object_type_extension
    "extend" @context
    "input" @context
    (name) @name) @item

(scalar_type_extension
    "extend" @context
    "scalar" @context
    (name) @name) @item

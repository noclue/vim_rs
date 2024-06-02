File {
    shebang: None,
    attrs: [],
    items: [
        Item::Struct {
            attrs: [
                Attribute {
                    pound_token: Pound,
                    style: AttrStyle::Outer,
                    bracket_token: Bracket,
                    meta: Meta::List {
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment {
                                    ident: Ident(
                                        derive,
                                    ),
                                    arguments: PathArguments::None,
                                },
                            ],
                        },
                        delimiter: MacroDelimiter::Paren(
                            Paren,
                        ),
                        tokens: TokenStream [
                            Ident {
                                sym: Debug,
                            },
                            Punct {
                                char: ',',
                                spacing: Alone,
                            },
                            Ident {
                                sym: serde,
                            },
                            Punct {
                                char: ':',
                                spacing: Joint,
                            },
                            Punct {
                                char: ':',
                                spacing: Alone,
                            },
                            Ident {
                                sym: Deserialize,
                            },
                        ],
                    },
                },
            ],
            vis: Visibility::Inherited,
            struct_token: Struct,
            ident: Ident(
                Device,
            ),
            generics: Generics {
                lt_token: None,
                params: [],
                gt_token: None,
                where_clause: None,
            },
            fields: Fields::Named {
                brace_token: Brace,
                named: [
                    Field {
                        attrs: [],
                        vis: Visibility::Inherited,
                        mutability: FieldMutability::None,
                        ident: Some(
                            Ident(
                                key,
                            ),
                        ),
                        colon_token: Some(
                            Colon,
                        ),
                        ty: Type::Path {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            i32,
                                        ),
                                        arguments: PathArguments::None,
                                    },
                                ],
                            },
                        },
                    },
                    Comma,
                    Field {
                        attrs: [],
                        vis: Visibility::Inherited,
                        mutability: FieldMutability::None,
                        ident: Some(
                            Ident(
                                controller_key,
                            ),
                        ),
                        colon_token: Some(
                            Colon,
                        ),
                        ty: Type::Path {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            Option,
                                        ),
                                        arguments: PathArguments::AngleBracketed {
                                            colon2_token: None,
                                            lt_token: Lt,
                                            args: [
                                                GenericArgument::Type(
                                                    Type::Path {
                                                        qself: None,
                                                        path: Path {
                                                            leading_colon: None,
                                                            segments: [
                                                                PathSegment {
                                                                    ident: Ident(
                                                                        i32,
                                                                    ),
                                                                    arguments: PathArguments::None,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                            ],
                                            gt_token: Gt,
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    Comma,
                    Field {
                        attrs: [],
                        vis: Visibility::Inherited,
                        mutability: FieldMutability::None,
                        ident: Some(
                            Ident(
                                unit_number,
                            ),
                        ),
                        colon_token: Some(
                            Colon,
                        ),
                        ty: Type::Path {
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: [
                                    PathSegment {
                                        ident: Ident(
                                            Option,
                                        ),
                                        arguments: PathArguments::AngleBracketed {
                                            colon2_token: None,
                                            lt_token: Lt,
                                            args: [
                                                GenericArgument::Type(
                                                    Type::Path {
                                                        qself: None,
                                                        path: Path {
                                                            leading_colon: None,
                                                            segments: [
                                                                PathSegment {
                                                                    ident: Ident(
                                                                        i32,
                                                                    ),
                                                                    arguments: PathArguments::None,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                            ],
                                            gt_token: Gt,
                                        },
                                    },
                                ],
                            },
                        },
                    },
                    Comma,
                ],
            },
            semi_token: None,
        },
    ],
}
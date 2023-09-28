// Block {
//     brace_token: Brace, 
//     stmts: [
//         Stmt::Macro {
//             attrs: [],
//             mac: Macro { 
//                 path: Path {
//                     leading_colon: None, 
//                     segments: [
//                         PathSegment { 
//                             ident: Ident { 
//                                 ident: "format", 
//                                 span: #0 bytes(678..684) 
//                             }, 
//                             arguments: PathArguments::None 
//                         }
//                     ]
//                 }, 
//                 bang_token: Not, 
//                 delimiter: MacroDelimiter::Paren(Paren), 
//                 tokens: TokenStream [
//                     Literal {
//                         kind: Str, 
//                         symbol: "Hello {}! id:{}", 
//                         suffix: None, 
//                         span: #0 bytes(686..703) 
//                     }, 
//                     Punct { 
//                         ch: ',', 
//                         spacing: Alone, 
//                         span: #0 bytes(703..704) 
//                     }, 
//                     Ident { 
//                         ident: "name", 
//                         span: #0 bytes(705..709) 
//                     }, 
//                     Punct { 
//                         ch: ',',
//                         spacing: Alone, 
//                         span: #0 bytes(709..710) 
//                     }, 
//                     Ident { 
//                         ident: "id", span: #0 bytes(711..713) 
//                     }
//                 ] 
//             },
//             semi_token: Some(Semi)
//         },
//         Stmt::Local { 
//             attrs: [], 
//             let_token: Let, 
//             pat: Pat::Ident { 
//                 attrs: [], 
//                 by_ref: None, 
//                 mutability: None, 
//                 ident: Ident { ident: "builder", span: #0 bytes(725..732) }, 
//                 subpat: None 
//             }, 
//             init: Some(
//                 LocalInit {
//                     eq_token: Eq, 
//                     expr: Expr::Call { 
//                         attrs: [], 
//                         func: Expr::Path { 
//                             attrs: [], 
//                             qself: None, 
//                             path: Path { 
//                                 leading_colon: None, 
//                                 segments: [
//                                     PathSegment { 
//                                         ident: Ident { 
//                                             ident: "Response", 
//                                             span: #0 bytes(735..743) 
//                                         },
//                                         arguments: PathArguments::None 
//                                     },
//                                     PathSep,
//                                     PathSegment{ 
//                                         ident: Ident { 
//                                             ident: "builder", 
//                                             span: #0 bytes(745..752) 
//                                         },
//                                         arguments: PathArguments::None 
//                                     }
//                                 ] 
//                             } 
//                         },
//                         paren_token: Paren, 
//                         args: [] 
//                     }, 
//                     diverge: None 
//                 }
//             ), 
//             semi_token: Semi 
//         }, 
//         Stmt::Local {
//             attrs: [], 
//             let_token: Let, 
//             pat: Pat::Ident { 
//                 attrs: [], 
//                 by_ref: None, 
//                 mutability: None, 
//                 ident: Ident { 
//                     ident: "ret", 
//                     span: #0 bytes(764..767) 
//                 }, 
//                 subpat: None 
//             }, 
//             init: Some(
//                 LocalInit { 
//                     eq_token: Eq,
//                     expr: Expr::Call {
//                         attrs: [], 
//                         func: Expr::Path { 
//                             attrs: [], 
//                             qself: None, 
//                             path: Path {
//                                 leading_colon: None, 
//                                 segments: [
//                                     PathSegment { 
//                                         ident: Ident { ident: "Bytes", span: #0 bytes(770..775) },
//                                         arguments: PathArguments::None
//                                     }, 
//                                     PathSep, 
//                                     PathSegment { 
//                                         ident: Ident { ident: "copy_from_slice", span: #0 bytes(777..792) }, 
//                                         arguments: PathArguments::None 
//                                     }
//                                 ]
//                             }
//                         }, 
//                         paren_token: Paren, 
//                         args: [
//                             Expr::Lit { attrs: [], lit: Lit::ByteStr { token: b"source" } }
//                         ]
//                     },
//                     diverge: None 
//                 }
//             ),
//             semi_token: Semi
//         },
//         Stmt::Expr(
//             Expr::Call { 
//                 attrs: [], 
//                 func: Expr::Path {
//                     attrs: [], 
//                     qself: None, 
//                     path: Path { 
//                         leading_colon: None, 
//                         segments: [
//                             PathSegment { 
//                                 ident: Ident { ident: "Ok", span: #0 bytes(809..811) }, 
//                             arguments: PathArguments::None
//                             }
//                         ]
//                     }
//                 }, 
//                 paren_token: Paren,
//                 args: [
//                     Expr::Tuple{ 
//                         attrs: [], 
//                         paren_token: Paren, 
//                         elems: [
//                             Expr::Path { 
//                                 attrs: [],
//                                 qself: None, 
//                                 path: Path { 
//                                     leading_colon: None, 
//                                     segments: [
//                                         PathSegment { 
//                                             ident: Ident { ident: "builder", span: #0 bytes(813..820) }, 
//                                             arguments: PathArguments::None
//                                         }
//                                     ] 
//                                 } 
//                             }, 
//                             Comma, 
//                             Expr::Path { 
//                                 attrs: [], 
//                                 qself: None, 
//                                 path: Path { 
//                                     leading_colon: None, 
//                                     segments: [
//                                         PathSegment { 
//                                             ident: Ident { ident: "ret", span: #0 bytes(822..825) }, 
//                                             arguments: PathArguments::None 
//                                         }
//                                     ] 
//                                 } 
//                             }
//                         ]
//                     }
//                 ] 
//             }, 
//             None
//         )
//     ]
// }
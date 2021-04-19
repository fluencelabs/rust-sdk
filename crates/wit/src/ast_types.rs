/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::parsed_type::ParsedType;

#[derive(Clone)]
pub(crate) struct AstFnArgument {
    pub name: String,
    pub ty: ParsedType,
}

#[derive(Clone)]
pub(crate) struct AstFnSignature {
    pub visibility: syn::Visibility,
    pub name: String,
    pub arguments: Vec<AstFnArgument>,
    // fce supports only one return value now,
    // waiting for adding multi-value support in Wasmer.
    pub output_type: Option<ParsedType>,
}

#[derive(Clone)]
pub(crate) struct AstRecordItem {
    pub name: String,
    pub fields: AstRecordFields,
    pub original: syn::ItemStruct,
}

#[allow(dead_code)] // at the moment tuple and unit structs aren't supported
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum AstRecordFields {
    Named(Vec<AstRecordField>),
    // named and unnamed variants have the same inner field types because of it's easy to handle it,
    // for additional info look at https://github.com/dtolnay/syn/issues/698
    Unnamed(Vec<AstRecordField>),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AstRecordField {
    // fields of tuple structs haven't got name
    pub name: Option<String>,
    pub ty: ParsedType,
}

#[derive(Clone)]
pub(crate) struct AstExternFnItem {
    pub link_name: Option<String>,
    // only imports are possible here
    pub signature: AstFnSignature,
}

#[derive(Clone)]
pub(crate) struct AstExternModItem {
    pub namespace: String,
    // only imports are possible here
    pub imports: Vec<AstExternFnItem>,
    pub original: syn::ItemForeignMod,
}

#[derive(Clone)]
pub(crate) struct AstFnItem {
    pub signature: AstFnSignature,
    pub original: syn::ItemFn,
}

#[derive(Clone)]
pub(crate) enum FCEAst {
    Function(AstFnItem),
    ExternMod(AstExternModItem),
    Record(AstRecordItem),
}
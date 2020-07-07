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

#![doc(html_root_url = "https://docs.rs/wit-support/0.2.0")]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_unsafe,
    unreachable_patterns
)]
#![warn(rust_2018_idioms)]
#![recursion_limit = "1024"]

mod fce_ast_types;
mod fce_macro_impl;
mod parsed_type;
mod parse_macro_input;
mod token_stream_generator;
mod wasm_type;

pub use fce_ast_types::*;
pub use fce_macro_impl::fce;
pub use parsed_type::ParsedType;
pub use token_stream_generator::GENERATED_SECTION_NAME;

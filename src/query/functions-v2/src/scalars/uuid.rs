// Copyright 2023 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_base::base::uuid::Uuid;
use common_expression::types::StringType;
use common_expression::FunctionDomain;
use common_expression::FunctionProperty;
use common_expression::FunctionRegistry;
use common_expression::Value;

pub fn register(registry: &mut FunctionRegistry) {
    registry.register_aliases("gen_random_uuid", &["uuid"]);

    registry.register_0_arg_core::<StringType, _, _>(
        "gen_random_uuid",
        FunctionProperty::default(),
        || FunctionDomain::Full,
        |_| {
            let value = UUIDv4::create();
            let c = format!("{:x}", value);
            Ok(Value::Scalar(c.as_bytes().to_vec()))
        },
    );
}

pub trait UUIDCreator {
    fn create() -> Uuid;
}

#[derive(Clone, Debug)]
pub struct UUIDv4;

impl UUIDCreator for UUIDv4 {
    fn create() -> Uuid {
        Uuid::new_v4()
    }
}
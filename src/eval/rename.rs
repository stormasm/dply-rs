// Copyright (C) 2023 Vince Vasta
// SPDX-License-Identifier: Apache-2.0
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
use anyhow::{anyhow, bail, Result};
use polars::prelude::*;

use crate::parser::{Expr, Operator};

use super::*;

/// Evaluates a rename call.
///
/// Parameters are checked before evaluation by the typing module.
pub fn eval(args: &[Expr], ctx: &mut Context) -> Result<()> {
    if let Some(df) = ctx.take_input() {
        let schema = df.schema().map_err(|e| anyhow!("Schema error: {e}"))?;
        // Store in a vec to preserve order.
        let mut schema_cols = schema.iter_names().map(|s| col(s)).collect::<Vec<_>>();

        for arg in args {
            if let Expr::BinaryOp(lhs, Operator::Assign, rhs) = arg {
                // rename(alias = column)
                let alias = args::identifier(lhs);
                let column = args::identifier(rhs);

                if let Some(idx) = schema_cols.iter().position(|c| c == &col(&column)) {
                    schema_cols[idx] = schema_cols[idx].clone().alias(&alias);
                } else {
                    bail!("unknown rename column {column}");
                }
            }
        }

        ctx.set_input(df.select(&schema_cols));
    } else {
        bail!("Missing input dataframe for rename.");
    }

    Ok(())
}

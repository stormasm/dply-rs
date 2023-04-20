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

//! Checks pipeline functions and arguments types.
use anyhow::{anyhow, Result};

use crate::parser::{Expr, Operator};
use matcher::*;

mod matcher;

/// Checks pipeline functions and arguments types.
pub fn pipeline(expr: &Expr) -> Result<()> {
    if let Expr::Pipeline(exprs) = expr {
        for expr in exprs {
            match_identifier.or(match_pipeline_fn).matches(expr)?;
        }

        Ok(())
    } else {
        Err(anyhow!("Not a pipeline"))
    }
}

/// Checks arguments for pipeline functions.
fn match_pipeline_fn(expr: &Expr) -> MatchResult {
    match_arrange
        .or(match_count)
        .or(match_csv)
        .or(match_distinct)
        .or(match_glimpse)
        .or(match_group_by)
        .or(match_parquet)
        .or(match_rename)
        .or(match_relocate)
        .or(match_select)
        .or(match_function("filter"))
        .or(match_function("mutate"))
        .or(match_function("summarize"))
        .matches(expr)
}

/// Checks arguments for arrange call.
fn match_arrange(expr: &Expr) -> MatchResult {
    // arrange(year, month, day)
    // arrange(year, desc(month), desc(day))
    let desc_fn = match_function("desc")
        .and(match_min_args(1))
        .and(match_max_args(1))
        .and(match_args(match_identifier));

    match_function("arrange")
        .and(match_min_args(1))
        .and(match_args(match_identifier.or(desc_fn)))
        .matches(expr)
}

/// Checks arguments for count call.
fn match_count(expr: &Expr) -> MatchResult {
    // count()
    // count(year, month, day)
    // count(year, sort = true)
    let sort_opt = match_assign(match_named("sort"), match_bool);

    match_function("count")
        .and(match_args(match_identifier.or(sort_opt)))
        .matches(expr)
}

/// Checks arguments for csv call.
fn match_csv(expr: &Expr) -> MatchResult {
    // csv("flights.csv")
    // csv("flights.csv", overwrite = true)
    let overwrite_opt = match_assign(match_named("overwrite"), match_bool);

    match_function("csv")
        .and(match_max_args(2))
        .and(match_arg(0, match_string))
        .and(match_opt_arg(1, overwrite_opt))
        .matches(expr)
}

/// Checks arguments for distinct call.
fn match_distinct(expr: &Expr) -> MatchResult {
    // distinct()
    // distinct(year, month)
    match_function("distinct")
        .and(match_args(match_identifier))
        .matches(expr)
}

/// Checks arguments for glimpse call.
fn match_glimpse(expr: &Expr) -> MatchResult {
    // glimpse()
    match_function("glimpse")
        .and(match_max_args(0))
        .matches(expr)
}

/// Checks arguments for group_by call.
fn match_group_by(expr: &Expr) -> MatchResult {
    // group_by(year, month)
    match_function("group_by")
        .and(match_min_args(1))
        .and(match_args(match_identifier))
        .matches(expr)
}

/// Checks arguments for parquet call.
fn match_parquet(expr: &Expr) -> MatchResult {
    // parquet("flights.parquet")
    // parquet("flights.parquet", overwrite = true)
    let overwrite_opt = match_assign(match_named("overwrite"), match_bool);

    match_function("parquet")
        .and(match_max_args(2))
        .and(match_arg(0, match_string))
        .and(match_opt_arg(1, overwrite_opt))
        .matches(expr)
}

/// Checks arguments for relocate call.
fn match_relocate(expr: &Expr) -> MatchResult {
    // relocate(gain, speed, before = 1)
    // relocate(gain, speed, before = day)
    let before_opt = match_assign(match_named("before"), match_number)
        .or(match_assign(match_named("before"), match_identifier));

    // relocate(gain, speed, after = 1)
    // relocate(gain, speed, after = day)
    let after_opt = match_assign(match_named("after"), match_number)
        .or(match_assign(match_named("after"), match_identifier));

    let args = match_identifier.or(before_opt).or(after_opt);

    match_function("relocate")
        .and(match_min_args(1))
        .and(match_args(args))
        .matches(expr)
}

/// Checks arguments for rename call.
fn match_rename(expr: &Expr) -> MatchResult {
    // rename(tail_num = tailnum, last_time = l_time)
    let rename_opt = match_assign(match_identifier, match_identifier);

    match_function("rename")
        .and(match_min_args(1))
        .and(match_args(rename_opt))
        .matches(expr)
}

/// Checks arguments for select call.
fn match_select(expr: &Expr) -> MatchResult {
    // select(year, contains("time"))
    let contains_fn = match_function("contains")
        .and(match_min_args(1))
        .and(match_max_args(1))
        .and(match_args(match_string));

    // select(year, starts_with("time"))
    let starts_with_fn = match_function("starts_with")
        .and(match_min_args(1))
        .and(match_max_args(1))
        .and(match_args(match_string));

    // select(year, ends_with("time"))
    let ends_with_fn = match_function("ends_with")
        .and(match_min_args(1))
        .and(match_max_args(1))
        .and(match_args(match_string));

    // select(tail_num = tailnum)
    let rename_opt = match_assign(match_identifier, match_identifier);

    let args = match_identifier
        .or(rename_opt)
        .or(contains_fn)
        .or(starts_with_fn)
        .or(ends_with_fn);

    match_function("select")
        .and(match_min_args(1))
        .and(match_args(args))
        .matches(expr)
}

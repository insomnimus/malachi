// SPDX-License-Identifier: Apache-2.0
// Copyright 2024 Taylan Gökkaya

// This file is licensed under the terms of Apache-2.0 License.

use super::IResult;
use crate::parser::prelude::*;

pub fn match_literal<'a>(lit: &'_ str, input: &'a str) -> IResult<&'a str, &'a str> {
	preceded(multispace0, tag(lit))(input)
}

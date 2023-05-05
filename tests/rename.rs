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
use anyhow::Result;
use indoc::indoc;

use dply::interpreter;

#[test]
fn rename() -> Result<()> {
    let input = indoc! {r#"
        parquet("tests/data/nyctaxi.parquet") |
            rename(
                vendor_id = VendorID,
                pickup_datetime = tpep_pickup_datetime,
                dropoff_datetime = tpep_dropoff_datetime,
                pu_location_id = PULocationID,
                do_location_id = DOLocationID
            ) |
            glimpse()
    "#};
    let output = interpreter::eval_to_string(input)?;

    assert_eq!(
        output,
        indoc!(
            r#"
            Rows: 250
            Columns: 19
            +-----------------------+--------------+-------------------------------------------------------------------------------+
            | vendor_id             | i64          | 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 1, 2,... |
            | pickup_datetime       | datetime[ns] | 2022-11-22 19:27:01, 2022-11-27 16:43:26, 2022-11-12 16:58:37, 2022-11-30...  |
            | dropoff_datetime      | datetime[ns] | 2022-11-22 19:45:53, 2022-11-27 16:50:06, 2022-11-12 17:12:31, 2022-11-30...  |
            | passenger_count       | i64          | 1, 2, 1, 1, 3, 1, 2, 1, 1, 2, 2, 1, 1, 1, 1, 5, 2, 5, 2, 1, 1, 1, 1, 1, 1,... |
            | trip_distance         | f64          | 3.14, 1.06, 2.36, 5.2, 0.0, 2.39, 1.52, 0.51, 0.98, 2.14, 0.85, 1.6, 3.1,...  |
            | rate_code             | str          | "Standard", "Standard", "Standard", "Standard", "Standard", "Standard",...    |
            | store_and_fwd_flag    | str          | "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N",... |
            | pu_location_id        | i64          | 234, 48, 142, 79, 237, 137, 107, 229, 162, 48, 143, 239, 263, 163, 237, 23... |
            | do_location_id        | i64          | 141, 142, 236, 75, 230, 140, 162, 161, 186, 239, 143, 43, 164, 138, 161, 2... |
            | payment_type          | str          | "Credit card", "Cash", "Credit card", "Credit card", "Credit card", "Cash"... |
            | fare_amount           | f64          | 14.5, 6.5, 11.5, 18.0, 12.5, 19.0, 8.5, 6.0, 12.0, 9.0, 5.5, 11.0, 11.5,...   |
            | extra                 | f64          | 1.0, 0.0, 0.0, 0.5, 3.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 3.5, 2.5, 0.0, 0.0,... |
            | mta_tax               | f64          | 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5,... |
            | tip_amount            | f64          | 3.76, 0.0, 2.96, 4.36, 3.25, 0.0, 0.0, 2.0, 3.26, 2.56, 1.76, 0.0, 3.7, 7.... |
            | tolls_amount          | f64          | 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 6.55, 0.0... |
            | improvement_surcharge | f64          | 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3,... |
            | total_amount          | f64          | 22.56, 9.8, 17.76, 26.16, 19.55, 22.3, 11.8, 11.3, 19.56, 15.36, 10.56, 15... |
            | congestion_surcharge  | f64          | 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5,... |
            | airport_fee           | f64          | 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,... |
            +-----------------------+--------------+-------------------------------------------------------------------------------+
        "#
        )
    );

    Ok(())
}

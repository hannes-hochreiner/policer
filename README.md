# Policer

A utility function to apply a policy to a list of dates.

## Context

The library was written as a utility for applying backup policies.
The idea is that backups (snapshots) would be made continuously, but not all snapshots should or can be kept.
Hence, a policy is drawn up, stating which snapshots should be kept.
For example, backups are made every 30 minutes.
One snapshot should be 30 minutes or more recent.
One snapshot should be 1 day or more recent.
One snapshot should be 1 month or more recent.
The final snapshot should be the most recent snapshot older than one month.
The policy in this case would be the intervals 30 minutes, 1 day, and 1 month.
The police function returns the snapshots not complying with the policy.

## Functions

### police

The police function takes the current date and a list of intervals from this date (i.e., the policy).
It then compares a list of dates (actually tuples of dates and objects) to the policy intervals and applies the following logic:
* every space between two intervals is interpreted as a bucket
* the space before the first and after the last interval is also counted as a bucket (i.e., the number of buckets will be equal to the number of intervals plus one)
* for every bucket the best fitting date is chosen
* if there are several dates in a bucket
  * only the oldest date is kept, if the bucket is not the last bucket (i.e., the one furthers in the past)
  * if the bucket is the oldest, the newest date is kept
* if the number of retained dates is smaller than the number of buckets, dates from the list of dates to be deleted are added until
  * either there are no more dates to be deleted
  * or the number of retained dates equals the number of buckets

## License

This work is licensed under the MIT or Apache 2.0 license.

`SPDX-License-Identifier: MIT OR Apache-2.0`
use std::cmp;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
#[derive(Eq)]
enum EventType
{
    Begins,
    Asleep,
    Wakes
}

impl PartialEq for EventType {
    fn eq(&self, _other: &EventType) -> bool {
        true
    }
}

#[derive(Debug)]
#[derive(Eq)]
pub struct Event
{
    id : i32,
    month: i32,
    day: i32,
    hour: i32,
    time: i32,
    ev: EventType,
}

impl PartialOrd for Event
{
    fn partial_cmp(&self, other: &Event) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for Event
{ 
    fn cmp(&self, other: &Event) -> cmp::Ordering {
        other.month.cmp(&self.month).then(other.day.cmp(&self.day).then(other.hour.cmp(&self.hour).then(other.time.cmp(&self.time))))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.time == other.time
    }
}

fn main() {
    let input = "[1518-09-22 23:50] Guard #2309 begins shift
[1518-06-26 00:42] falls asleep
[1518-07-10 00:00] Guard #761 begins shift
[1518-10-09 00:43] wakes up
[1518-10-12 00:35] falls asleep
[1518-07-03 00:40] falls asleep
[1518-10-09 00:34] wakes up
[1518-07-18 00:48] wakes up
[1518-09-21 00:12] wakes up
[1518-10-08 00:19] falls asleep
[1518-03-27 00:39] falls asleep
[1518-09-19 00:46] wakes up
[1518-08-15 00:37] wakes up
[1518-05-15 00:46] falls asleep
[1518-08-17 00:29] falls asleep
[1518-04-28 00:00] Guard #1861 begins shift
[1518-03-25 00:49] falls asleep
[1518-10-19 00:02] Guard #947 begins shift
[1518-10-08 00:36] falls asleep
[1518-07-19 00:03] Guard #443 begins shift
[1518-09-30 00:00] Guard #2003 begins shift
[1518-10-25 00:29] wakes up
[1518-08-29 00:47] falls asleep
[1518-05-01 00:20] falls asleep
[1518-03-18 23:50] Guard #2591 begins shift
[1518-10-30 00:48] falls asleep
[1518-07-14 00:40] wakes up
[1518-11-11 00:30] falls asleep
[1518-08-25 00:11] wakes up
[1518-09-14 23:58] Guard #1931 begins shift
[1518-08-11 00:58] wakes up
[1518-05-13 00:21] falls asleep
[1518-10-25 00:53] wakes up
[1518-09-12 00:18] falls asleep
[1518-05-31 00:05] falls asleep
[1518-06-11 00:02] Guard #1861 begins shift
[1518-07-03 00:53] wakes up
[1518-09-12 23:58] Guard #2309 begins shift
[1518-07-15 00:56] wakes up
[1518-07-18 00:02] Guard #2591 begins shift
[1518-07-14 00:49] falls asleep
[1518-11-05 00:29] falls asleep
[1518-04-27 00:18] falls asleep
[1518-09-15 00:30] falls asleep
[1518-06-24 00:25] falls asleep
[1518-05-04 00:54] wakes up
[1518-03-31 00:59] wakes up
[1518-04-14 00:02] Guard #2281 begins shift
[1518-08-08 00:03] falls asleep
[1518-03-14 00:50] wakes up
[1518-08-21 00:03] Guard #1783 begins shift
[1518-07-14 00:35] falls asleep
[1518-08-02 00:15] wakes up
[1518-11-19 23:57] Guard #2141 begins shift
[1518-03-19 00:00] falls asleep
[1518-06-18 00:53] falls asleep
[1518-07-26 00:50] falls asleep
[1518-11-01 00:39] wakes up
[1518-10-05 00:48] wakes up
[1518-03-16 23:56] Guard #1889 begins shift
[1518-09-26 00:19] falls asleep
[1518-05-05 00:20] falls asleep
[1518-07-05 00:26] falls asleep
[1518-08-10 00:57] wakes up
[1518-06-08 00:17] falls asleep
[1518-08-07 00:58] wakes up
[1518-06-18 00:58] wakes up
[1518-07-24 00:50] wakes up
[1518-05-20 00:00] Guard #467 begins shift
[1518-10-24 23:50] Guard #2141 begins shift
[1518-11-08 00:42] falls asleep
[1518-09-05 00:52] wakes up
[1518-06-18 00:26] falls asleep
[1518-08-31 00:51] wakes up
[1518-09-04 00:50] wakes up
[1518-06-10 00:21] wakes up
[1518-11-09 00:46] wakes up
[1518-05-09 00:34] wakes up
[1518-10-22 00:31] falls asleep
[1518-08-01 00:50] wakes up
[1518-07-24 00:10] falls asleep
[1518-04-24 00:22] falls asleep
[1518-09-29 00:26] falls asleep
[1518-04-26 00:53] wakes up
[1518-05-03 00:50] falls asleep
[1518-10-17 23:46] Guard #2789 begins shift
[1518-10-25 00:50] falls asleep
[1518-07-22 00:03] falls asleep
[1518-09-24 23:58] Guard #1297 begins shift
[1518-06-18 00:36] wakes up
[1518-11-20 00:21] falls asleep
[1518-06-14 00:30] falls asleep
[1518-05-05 00:31] falls asleep
[1518-04-12 00:46] falls asleep
[1518-03-13 23:54] Guard #3541 begins shift
[1518-05-29 00:56] wakes up
[1518-10-17 00:36] falls asleep
[1518-10-01 00:58] wakes up
[1518-04-27 00:53] wakes up
[1518-05-02 00:51] wakes up
[1518-11-02 00:49] wakes up
[1518-05-11 00:02] Guard #947 begins shift
[1518-11-13 00:49] wakes up
[1518-10-21 00:05] falls asleep
[1518-05-09 00:01] Guard #743 begins shift
[1518-06-03 00:00] falls asleep
[1518-09-06 00:40] falls asleep
[1518-07-27 00:53] wakes up
[1518-03-11 00:22] falls asleep
[1518-06-15 23:50] Guard #761 begins shift
[1518-10-10 00:40] falls asleep
[1518-04-30 00:18] falls asleep
[1518-06-04 00:32] wakes up
[1518-03-23 00:51] falls asleep
[1518-04-09 00:49] wakes up
[1518-07-08 23:56] Guard #3541 begins shift
[1518-10-24 00:00] Guard #2789 begins shift
[1518-11-09 00:04] Guard #2003 begins shift
[1518-04-05 00:57] falls asleep
[1518-10-11 00:50] falls asleep
[1518-11-23 00:57] wakes up
[1518-05-26 00:10] falls asleep
[1518-05-22 00:23] falls asleep
[1518-09-09 00:00] falls asleep
[1518-04-15 00:25] falls asleep
[1518-06-16 00:46] wakes up
[1518-09-21 00:39] wakes up
[1518-10-22 00:03] Guard #743 begins shift
[1518-09-11 00:58] wakes up
[1518-09-23 23:58] Guard #1297 begins shift
[1518-07-12 00:49] wakes up
[1518-10-11 00:57] falls asleep
[1518-10-22 00:59] wakes up
[1518-05-22 00:32] wakes up
[1518-03-25 00:58] wakes up
[1518-05-25 00:03] Guard #233 begins shift
[1518-06-04 00:01] falls asleep
[1518-03-26 00:07] falls asleep
[1518-11-04 00:51] falls asleep
[1518-07-25 00:31] wakes up
[1518-07-25 00:01] Guard #467 begins shift
[1518-10-05 00:16] falls asleep
[1518-07-06 00:31] wakes up
[1518-11-12 00:54] wakes up
[1518-06-20 00:03] Guard #3541 begins shift
[1518-05-22 00:55] falls asleep
[1518-04-02 00:21] wakes up
[1518-04-23 00:31] falls asleep
[1518-11-15 00:55] wakes up
[1518-08-10 00:03] Guard #1931 begins shift
[1518-11-22 23:53] Guard #2003 begins shift
[1518-03-08 23:46] Guard #2591 begins shift
[1518-05-06 00:14] falls asleep
[1518-09-23 00:16] wakes up
[1518-11-04 00:55] wakes up
[1518-08-20 00:03] Guard #3529 begins shift
[1518-06-29 00:58] wakes up
[1518-04-20 00:13] falls asleep
[1518-03-13 00:01] Guard #2141 begins shift
[1518-06-12 00:35] falls asleep
[1518-03-28 00:00] Guard #2003 begins shift
[1518-03-25 00:45] wakes up
[1518-10-13 00:26] wakes up
[1518-06-16 00:05] falls asleep
[1518-11-12 00:10] falls asleep
[1518-03-30 00:28] falls asleep
[1518-07-28 00:20] falls asleep
[1518-07-01 00:54] wakes up
[1518-03-11 00:56] wakes up
[1518-07-25 00:17] falls asleep
[1518-10-06 00:58] wakes up
[1518-08-11 00:22] falls asleep
[1518-10-02 00:27] falls asleep
[1518-09-10 00:50] wakes up
[1518-04-27 00:00] Guard #2591 begins shift
[1518-11-18 00:20] wakes up
[1518-09-09 00:16] wakes up
[1518-06-07 00:58] wakes up
[1518-08-13 00:28] falls asleep
[1518-11-19 00:35] wakes up
[1518-03-23 00:48] wakes up
[1518-10-16 00:06] wakes up
[1518-07-04 00:56] wakes up
[1518-08-03 00:02] Guard #1931 begins shift
[1518-09-29 00:42] wakes up
[1518-09-08 00:59] wakes up
[1518-09-23 00:46] wakes up
[1518-04-06 00:00] Guard #2281 begins shift
[1518-09-13 00:59] wakes up
[1518-10-01 23:59] Guard #2789 begins shift
[1518-08-19 00:19] falls asleep
[1518-06-07 00:48] wakes up
[1518-08-30 23:56] Guard #1931 begins shift
[1518-11-08 00:58] wakes up
[1518-11-01 00:47] falls asleep
[1518-05-01 00:40] wakes up
[1518-10-15 00:40] falls asleep
[1518-07-05 23:57] Guard #1297 begins shift
[1518-03-09 23:54] Guard #947 begins shift
[1518-04-05 00:34] wakes up
[1518-09-22 00:53] wakes up
[1518-03-20 00:06] falls asleep
[1518-04-09 00:48] falls asleep
[1518-09-10 00:45] falls asleep
[1518-08-27 00:11] falls asleep
[1518-06-01 00:00] Guard #443 begins shift
[1518-08-02 00:36] wakes up
[1518-05-12 00:59] wakes up
[1518-10-25 00:38] falls asleep
[1518-04-18 00:22] wakes up
[1518-04-01 00:50] falls asleep
[1518-06-28 00:31] falls asleep
[1518-11-10 00:16] falls asleep
[1518-09-22 00:48] falls asleep
[1518-08-20 00:22] falls asleep
[1518-08-16 00:51] wakes up
[1518-07-29 00:03] Guard #467 begins shift
[1518-06-09 23:57] Guard #467 begins shift
[1518-08-15 00:42] falls asleep
[1518-10-23 00:23] falls asleep
[1518-10-17 00:02] Guard #2141 begins shift
[1518-10-17 00:54] wakes up
[1518-03-23 00:32] wakes up
[1518-09-30 00:59] wakes up
[1518-08-23 23:46] Guard #2789 begins shift
[1518-05-22 00:56] wakes up
[1518-05-25 00:29] falls asleep
[1518-07-22 00:53] wakes up
[1518-10-22 00:36] wakes up
[1518-03-17 00:49] wakes up
[1518-06-11 00:20] falls asleep
[1518-07-23 00:30] falls asleep
[1518-04-08 00:39] wakes up
[1518-08-15 23:59] Guard #743 begins shift
[1518-05-28 23:56] Guard #467 begins shift
[1518-08-01 00:00] falls asleep
[1518-03-15 00:31] falls asleep
[1518-07-12 00:31] falls asleep
[1518-10-27 00:54] wakes up
[1518-09-11 00:34] wakes up
[1518-04-09 00:28] falls asleep
[1518-06-30 00:00] Guard #2003 begins shift
[1518-09-21 00:19] falls asleep
[1518-03-14 00:32] falls asleep
[1518-05-11 00:41] falls asleep
[1518-09-13 00:32] falls asleep
[1518-04-20 00:38] wakes up
[1518-04-01 00:58] wakes up
[1518-11-03 00:43] falls asleep
[1518-11-15 00:08] falls asleep
[1518-11-02 00:31] falls asleep
[1518-09-08 00:03] Guard #3541 begins shift
[1518-11-18 00:16] falls asleep
[1518-06-13 00:09] falls asleep
[1518-09-14 00:33] falls asleep
[1518-06-06 23:57] Guard #2141 begins shift
[1518-09-19 00:18] falls asleep
[1518-08-25 00:07] falls asleep
[1518-09-28 00:02] Guard #1931 begins shift
[1518-07-07 00:48] wakes up
[1518-11-10 00:55] wakes up
[1518-03-25 00:30] falls asleep
[1518-10-26 23:57] Guard #2003 begins shift
[1518-05-05 00:04] Guard #1889 begins shift
[1518-07-20 00:51] wakes up
[1518-08-28 00:38] wakes up
[1518-07-01 00:35] wakes up
[1518-05-17 00:50] wakes up
[1518-08-17 00:03] Guard #2141 begins shift
[1518-10-02 00:59] wakes up
[1518-05-21 00:39] wakes up
[1518-04-23 00:53] wakes up
[1518-07-21 23:52] Guard #2789 begins shift
[1518-08-24 00:26] falls asleep
[1518-05-20 00:46] wakes up
[1518-10-21 00:42] wakes up
[1518-11-14 00:46] wakes up
[1518-04-09 00:44] wakes up
[1518-05-23 23:58] Guard #2309 begins shift
[1518-10-18 00:01] falls asleep
[1518-08-15 00:51] wakes up
[1518-08-17 23:51] Guard #1861 begins shift
[1518-06-10 00:19] falls asleep
[1518-08-08 23:56] Guard #2521 begins shift
[1518-10-15 23:46] Guard #443 begins shift
[1518-07-06 00:57] falls asleep
[1518-05-06 00:40] falls asleep
[1518-09-02 00:56] wakes up
[1518-06-22 00:49] wakes up
[1518-04-02 00:26] falls asleep
[1518-11-02 23:49] Guard #233 begins shift
[1518-09-03 23:57] Guard #3329 begins shift
[1518-05-28 00:01] Guard #2281 begins shift
[1518-10-14 00:25] wakes up
[1518-08-19 00:55] wakes up
[1518-10-05 23:53] Guard #1861 begins shift
[1518-10-11 00:32] wakes up
[1518-06-26 00:27] wakes up
[1518-07-30 00:53] falls asleep
[1518-10-10 00:54] falls asleep
[1518-09-07 00:24] falls asleep
[1518-09-29 00:00] Guard #1229 begins shift
[1518-10-13 00:25] falls asleep
[1518-08-26 00:37] wakes up
[1518-04-21 00:50] wakes up
[1518-06-03 00:52] falls asleep
[1518-06-22 00:00] Guard #2141 begins shift
[1518-08-31 00:36] falls asleep
[1518-05-26 23:58] Guard #761 begins shift
[1518-06-16 00:45] falls asleep
[1518-05-26 00:48] wakes up
[1518-06-11 00:44] falls asleep
[1518-09-23 00:23] falls asleep
[1518-08-08 00:50] wakes up
[1518-07-07 00:39] wakes up
[1518-07-18 00:19] falls asleep
[1518-05-05 23:59] Guard #1861 begins shift
[1518-03-25 23:57] Guard #3329 begins shift
[1518-10-09 00:15] falls asleep
[1518-11-11 23:56] Guard #761 begins shift
[1518-05-31 00:47] wakes up
[1518-07-11 00:31] falls asleep
[1518-04-17 00:51] wakes up
[1518-06-29 00:31] wakes up
[1518-04-15 00:01] Guard #233 begins shift
[1518-07-10 23:57] Guard #443 begins shift
[1518-07-23 23:56] Guard #2789 begins shift
[1518-04-08 00:23] falls asleep
[1518-06-12 00:29] falls asleep
[1518-11-01 00:03] Guard #2141 begins shift
[1518-08-22 00:39] wakes up
[1518-05-13 00:01] Guard #233 begins shift
[1518-06-14 00:49] wakes up
[1518-05-19 00:53] wakes up
[1518-03-22 23:58] Guard #761 begins shift
[1518-09-29 00:11] falls asleep
[1518-05-30 00:54] wakes up
[1518-05-19 00:18] falls asleep
[1518-09-13 00:56] falls asleep
[1518-08-17 00:56] wakes up
[1518-07-01 00:00] Guard #743 begins shift
[1518-06-18 00:47] wakes up
[1518-08-14 00:59] wakes up
[1518-05-10 00:10] falls asleep
[1518-07-12 00:54] falls asleep
[1518-07-11 00:47] wakes up
[1518-05-11 00:36] wakes up
[1518-05-24 00:36] wakes up
[1518-10-29 00:38] falls asleep
[1518-10-03 00:40] wakes up
[1518-11-18 00:42] wakes up
[1518-07-03 00:47] falls asleep
[1518-04-24 23:50] Guard #3541 begins shift
[1518-05-23 00:36] wakes up
[1518-05-11 23:57] Guard #2003 begins shift
[1518-03-31 00:45] falls asleep
[1518-10-13 00:44] falls asleep
[1518-03-31 00:07] falls asleep
[1518-11-16 00:10] falls asleep
[1518-04-20 00:09] wakes up
[1518-06-24 00:04] Guard #1229 begins shift
[1518-05-18 00:49] falls asleep
[1518-06-16 00:55] falls asleep
[1518-04-10 23:56] Guard #2281 begins shift
[1518-03-31 00:03] Guard #1889 begins shift
[1518-10-25 00:47] wakes up
[1518-11-14 00:04] Guard #2141 begins shift
[1518-05-04 00:00] Guard #443 begins shift
[1518-11-05 00:47] wakes up
[1518-08-13 23:58] Guard #1783 begins shift
[1518-09-29 00:55] falls asleep
[1518-05-04 00:27] falls asleep
[1518-04-29 00:35] wakes up
[1518-05-07 00:15] falls asleep
[1518-05-21 00:49] wakes up
[1518-05-18 23:58] Guard #743 begins shift
[1518-04-01 23:49] Guard #467 begins shift
[1518-08-18 00:04] falls asleep
[1518-09-17 00:42] wakes up
[1518-11-04 00:00] Guard #3529 begins shift
[1518-06-16 00:41] wakes up
[1518-10-16 00:01] falls asleep
[1518-04-07 00:06] falls asleep
[1518-05-21 00:37] falls asleep
[1518-04-08 23:56] Guard #743 begins shift
[1518-05-23 00:21] falls asleep
[1518-03-21 00:04] Guard #3529 begins shift
[1518-05-05 00:48] wakes up
[1518-04-08 00:42] falls asleep
[1518-06-21 00:25] wakes up
[1518-08-03 00:19] falls asleep
[1518-07-29 00:20] falls asleep
[1518-09-27 00:19] wakes up
[1518-04-01 00:00] Guard #3529 begins shift
[1518-07-02 00:56] wakes up
[1518-03-13 00:44] falls asleep
[1518-03-27 00:59] wakes up
[1518-11-05 23:46] Guard #1229 begins shift
[1518-03-23 00:54] wakes up
[1518-07-17 00:27] falls asleep
[1518-11-16 00:49] falls asleep
[1518-08-02 00:04] falls asleep
[1518-09-05 00:47] falls asleep
[1518-06-11 23:59] Guard #443 begins shift
[1518-07-16 00:06] falls asleep
[1518-07-03 00:03] Guard #3541 begins shift
[1518-10-06 00:41] falls asleep
[1518-09-09 00:36] falls asleep
[1518-05-01 00:44] falls asleep
[1518-03-25 00:01] Guard #761 begins shift
[1518-03-26 00:57] wakes up
[1518-11-01 00:10] falls asleep
[1518-04-12 00:38] wakes up
[1518-03-22 00:44] wakes up
[1518-07-07 00:37] falls asleep
[1518-03-13 00:36] wakes up
[1518-09-14 00:02] Guard #761 begins shift
[1518-10-10 23:57] Guard #1889 begins shift
[1518-08-11 23:50] Guard #2309 begins shift
[1518-08-13 00:50] falls asleep
[1518-07-16 00:58] wakes up
[1518-05-11 00:32] wakes up
[1518-04-22 23:57] Guard #2003 begins shift
[1518-06-12 23:57] Guard #947 begins shift
[1518-10-29 00:46] falls asleep
[1518-10-11 00:59] wakes up
[1518-10-07 00:16] falls asleep
[1518-11-19 00:24] falls asleep
[1518-03-15 00:48] wakes up
[1518-04-15 23:59] Guard #1783 begins shift
[1518-05-10 00:59] wakes up
[1518-06-15 00:55] falls asleep
[1518-04-12 23:56] Guard #2963 begins shift
[1518-06-01 00:33] wakes up
[1518-06-12 00:32] wakes up
[1518-03-19 00:43] wakes up
[1518-08-24 00:01] falls asleep
[1518-09-07 00:03] Guard #467 begins shift
[1518-11-11 00:02] Guard #3541 begins shift
[1518-11-17 00:41] wakes up
[1518-04-03 00:32] wakes up
[1518-05-22 00:12] falls asleep
[1518-10-25 00:01] falls asleep
[1518-03-27 00:02] Guard #2789 begins shift
[1518-10-28 00:59] wakes up
[1518-05-06 00:51] wakes up
[1518-03-11 00:35] falls asleep
[1518-03-09 00:55] wakes up
[1518-07-03 00:42] wakes up
[1518-03-22 00:59] wakes up
[1518-05-14 00:51] falls asleep
[1518-05-07 00:56] wakes up
[1518-06-23 00:14] falls asleep
[1518-05-22 00:01] Guard #443 begins shift
[1518-08-07 00:26] falls asleep
[1518-08-10 00:53] falls asleep
[1518-09-01 00:03] Guard #2789 begins shift
[1518-09-13 00:45] wakes up
[1518-09-25 23:59] Guard #743 begins shift
[1518-07-28 00:42] wakes up
[1518-04-02 23:59] Guard #233 begins shift
[1518-04-04 00:53] wakes up
[1518-07-20 00:57] wakes up
[1518-05-14 00:00] Guard #1861 begins shift
[1518-10-23 00:46] wakes up
[1518-04-24 00:04] Guard #761 begins shift
[1518-05-11 00:31] falls asleep
[1518-03-25 00:24] wakes up
[1518-10-16 00:10] falls asleep
[1518-11-17 00:21] falls asleep
[1518-10-02 00:57] falls asleep
[1518-03-29 00:06] falls asleep
[1518-08-06 00:17] falls asleep
[1518-05-31 00:58] wakes up
[1518-10-09 23:56] Guard #2591 begins shift
[1518-06-18 23:57] Guard #2141 begins shift
[1518-06-20 00:58] wakes up
[1518-09-27 00:16] falls asleep
[1518-09-01 23:59] Guard #3541 begins shift
[1518-11-06 00:04] falls asleep
[1518-03-21 23:56] Guard #233 begins shift
[1518-07-06 00:48] wakes up
[1518-11-08 00:12] wakes up
[1518-05-27 00:25] falls asleep
[1518-10-02 23:58] Guard #3529 begins shift
[1518-10-12 00:56] wakes up
[1518-04-25 00:05] falls asleep
[1518-06-01 00:58] wakes up
[1518-08-27 23:56] Guard #743 begins shift
[1518-07-31 00:41] wakes up
[1518-05-08 00:47] falls asleep
[1518-04-15 00:54] falls asleep
[1518-04-10 00:56] wakes up
[1518-07-08 00:01] Guard #1861 begins shift
[1518-10-04 00:55] falls asleep
[1518-10-15 00:00] Guard #1931 begins shift
[1518-09-30 00:28] falls asleep
[1518-06-05 00:33] wakes up
[1518-06-03 00:54] wakes up
[1518-10-04 00:51] wakes up
[1518-08-04 23:58] Guard #1889 begins shift
[1518-07-14 00:03] Guard #1783 begins shift
[1518-05-16 00:59] wakes up
[1518-05-22 00:16] wakes up
[1518-11-09 00:43] falls asleep
[1518-10-07 00:00] Guard #2141 begins shift
[1518-07-19 00:59] wakes up
[1518-04-10 00:04] Guard #2309 begins shift
[1518-04-08 00:20] wakes up
[1518-04-25 00:36] wakes up
[1518-08-21 23:58] Guard #443 begins shift
[1518-06-26 00:58] wakes up
[1518-03-29 00:00] Guard #1861 begins shift
[1518-08-29 00:51] wakes up
[1518-06-03 00:25] falls asleep
[1518-06-13 00:29] wakes up
[1518-09-20 00:36] falls asleep
[1518-04-19 00:18] wakes up
[1518-10-30 00:35] wakes up
[1518-04-08 00:00] Guard #2591 begins shift
[1518-04-30 23:59] Guard #2141 begins shift
[1518-09-23 00:53] wakes up
[1518-10-30 00:02] Guard #2309 begins shift
[1518-08-25 00:00] Guard #947 begins shift
[1518-05-21 00:45] falls asleep
[1518-08-27 00:37] wakes up
[1518-04-08 00:43] wakes up
[1518-07-20 23:56] Guard #2963 begins shift
[1518-08-11 00:52] falls asleep
[1518-05-08 00:48] wakes up
[1518-08-18 00:33] falls asleep
[1518-04-30 00:04] Guard #2309 begins shift
[1518-08-21 00:40] falls asleep
[1518-07-28 00:04] Guard #2309 begins shift
[1518-11-19 00:47] wakes up
[1518-03-21 00:59] wakes up
[1518-05-17 00:02] Guard #1297 begins shift
[1518-08-06 00:48] falls asleep
[1518-06-03 23:47] Guard #443 begins shift
[1518-07-19 23:58] Guard #467 begins shift
[1518-08-01 23:54] Guard #233 begins shift
[1518-07-04 00:00] Guard #1861 begins shift
[1518-06-19 00:38] falls asleep
[1518-09-28 00:28] wakes up
[1518-05-25 00:50] wakes up
[1518-05-03 00:54] wakes up
[1518-09-25 00:55] wakes up
[1518-06-05 00:12] falls asleep
[1518-06-15 00:59] wakes up
[1518-06-12 00:41] wakes up
[1518-06-16 00:29] wakes up
[1518-06-18 00:39] falls asleep
[1518-07-29 23:58] Guard #2591 begins shift
[1518-07-25 00:48] wakes up
[1518-07-13 00:00] Guard #2003 begins shift
[1518-10-24 00:44] wakes up
[1518-08-18 00:27] wakes up
[1518-09-04 00:27] wakes up
[1518-05-20 00:33] falls asleep
[1518-06-17 23:56] Guard #2309 begins shift
[1518-09-14 00:51] wakes up
[1518-10-18 00:09] wakes up
[1518-03-26 00:47] wakes up
[1518-07-20 00:55] falls asleep
[1518-03-16 00:28] falls asleep
[1518-05-17 00:45] falls asleep
[1518-07-01 00:30] falls asleep
[1518-09-06 00:19] falls asleep
[1518-04-21 23:52] Guard #743 begins shift
[1518-07-30 00:59] wakes up
[1518-08-04 00:20] falls asleep
[1518-05-03 00:57] falls asleep
[1518-11-07 23:57] Guard #1297 begins shift
[1518-05-24 00:59] wakes up
[1518-10-06 00:02] falls asleep
[1518-05-18 00:59] wakes up
[1518-11-09 00:49] falls asleep
[1518-09-23 00:05] falls asleep
[1518-07-03 00:30] wakes up
[1518-09-09 00:49] wakes up
[1518-08-04 00:25] wakes up
[1518-08-12 00:04] falls asleep
[1518-10-25 23:56] Guard #2003 begins shift
[1518-06-22 00:53] falls asleep
[1518-07-22 23:58] Guard #1229 begins shift
[1518-05-12 00:42] falls asleep
[1518-04-30 00:55] wakes up
[1518-10-20 00:26] wakes up
[1518-07-06 00:43] falls asleep
[1518-04-10 00:32] falls asleep
[1518-10-30 23:56] Guard #1931 begins shift
[1518-03-11 00:27] wakes up
[1518-03-24 00:39] falls asleep
[1518-09-07 00:47] wakes up
[1518-08-24 00:14] wakes up
[1518-11-01 00:38] falls asleep
[1518-03-22 00:57] falls asleep
[1518-03-09 00:02] falls asleep
[1518-10-24 00:49] falls asleep
[1518-10-04 00:56] wakes up
[1518-05-29 23:57] Guard #1229 begins shift
[1518-09-01 00:57] wakes up
[1518-04-08 00:15] falls asleep
[1518-04-05 00:12] falls asleep
[1518-03-29 00:58] wakes up
[1518-10-21 00:23] wakes up
[1518-03-23 00:44] falls asleep
[1518-04-03 00:12] falls asleep
[1518-07-25 00:34] falls asleep
[1518-07-17 00:50] wakes up
[1518-08-29 00:03] Guard #3541 begins shift
[1518-05-16 00:19] falls asleep
[1518-10-11 00:51] wakes up
[1518-03-28 00:52] wakes up
[1518-08-02 00:32] falls asleep
[1518-03-31 00:42] wakes up
[1518-07-08 00:57] falls asleep
[1518-07-06 00:17] falls asleep
[1518-06-28 00:00] Guard #467 begins shift
[1518-11-16 00:25] wakes up
[1518-05-29 00:39] falls asleep
[1518-04-02 00:52] wakes up
[1518-05-09 23:56] Guard #2309 begins shift
[1518-03-20 00:02] Guard #3541 begins shift
[1518-05-05 00:26] wakes up
[1518-09-29 00:19] wakes up
[1518-07-15 00:04] Guard #3529 begins shift
[1518-08-20 00:57] wakes up
[1518-06-09 00:18] falls asleep
[1518-04-22 00:05] falls asleep
[1518-05-09 00:54] wakes up
[1518-09-04 00:23] falls asleep
[1518-10-20 00:02] falls asleep
[1518-07-08 00:58] wakes up
[1518-07-19 00:28] falls asleep
[1518-07-23 00:45] wakes up
[1518-06-29 00:06] falls asleep
[1518-05-10 00:39] falls asleep
[1518-09-11 00:18] falls asleep
[1518-07-29 00:22] wakes up
[1518-10-03 23:56] Guard #1889 begins shift
[1518-09-26 00:35] wakes up
[1518-05-14 00:55] wakes up
[1518-04-29 00:20] falls asleep
[1518-11-17 23:58] Guard #1889 begins shift
[1518-08-04 00:04] falls asleep
[1518-10-19 00:10] falls asleep
[1518-06-29 00:01] Guard #1861 begins shift
[1518-08-21 00:58] wakes up
[1518-11-20 00:43] wakes up
[1518-07-05 00:30] wakes up
[1518-11-09 00:55] wakes up
[1518-07-26 00:53] wakes up
[1518-04-26 00:47] falls asleep
[1518-06-01 23:46] Guard #947 begins shift
[1518-06-08 00:45] wakes up
[1518-10-30 00:56] wakes up
[1518-10-31 00:22] falls asleep
[1518-04-17 00:21] falls asleep
[1518-04-07 00:52] wakes up
[1518-06-07 00:35] falls asleep
[1518-06-25 00:00] Guard #2963 begins shift
[1518-08-16 00:17] falls asleep
[1518-11-07 00:17] falls asleep
[1518-10-26 00:47] falls asleep
[1518-07-01 00:48] falls asleep
[1518-11-07 00:01] Guard #2003 begins shift
[1518-05-27 00:50] wakes up
[1518-03-30 00:42] wakes up
[1518-04-25 00:50] falls asleep
[1518-05-07 00:22] wakes up
[1518-10-12 00:00] Guard #1889 begins shift
[1518-08-14 00:29] falls asleep
[1518-08-11 00:39] wakes up
[1518-08-08 00:39] falls asleep
[1518-05-26 00:45] falls asleep
[1518-08-23 00:18] falls asleep
[1518-06-16 00:39] falls asleep
[1518-04-18 00:19] falls asleep
[1518-03-16 00:04] Guard #2789 begins shift
[1518-06-09 00:20] wakes up
[1518-11-19 00:42] falls asleep
[1518-09-16 00:00] Guard #2521 begins shift
[1518-09-08 00:29] falls asleep
[1518-11-04 23:59] Guard #467 begins shift
[1518-10-13 23:58] Guard #1931 begins shift
[1518-09-02 23:54] Guard #443 begins shift
[1518-10-27 00:29] falls asleep
[1518-11-01 00:54] wakes up
[1518-04-21 00:01] Guard #1861 begins shift
[1518-06-14 00:03] Guard #443 begins shift
[1518-10-24 00:55] wakes up
[1518-06-24 00:38] wakes up
[1518-08-24 00:53] wakes up
[1518-03-24 00:50] wakes up
[1518-08-26 00:00] Guard #743 begins shift
[1518-10-04 00:30] falls asleep
[1518-06-02 00:05] falls asleep
[1518-04-25 00:52] wakes up
[1518-09-06 00:58] wakes up
[1518-09-04 00:33] falls asleep
[1518-05-06 00:37] wakes up
[1518-07-31 23:52] Guard #743 begins shift
[1518-05-26 00:22] wakes up
[1518-04-20 00:07] falls asleep
[1518-06-15 00:01] Guard #3529 begins shift
[1518-06-22 00:25] falls asleep
[1518-07-06 00:58] wakes up
[1518-05-31 00:57] falls asleep
[1518-03-22 00:21] falls asleep
[1518-11-09 00:40] wakes up
[1518-04-19 00:04] Guard #2591 begins shift
[1518-08-18 00:36] wakes up
[1518-03-15 00:04] Guard #1931 begins shift
[1518-06-28 00:44] wakes up
[1518-11-21 23:58] Guard #2003 begins shift
[1518-05-30 23:50] Guard #761 begins shift
[1518-10-13 00:53] wakes up
[1518-03-20 00:55] wakes up
[1518-11-01 00:52] falls asleep
[1518-08-13 00:03] Guard #467 begins shift
[1518-11-06 00:29] wakes up
[1518-11-23 00:05] falls asleep
[1518-10-06 00:19] wakes up
[1518-04-22 00:41] wakes up
[1518-09-22 00:03] Guard #1297 begins shift
[1518-08-23 00:02] Guard #2309 begins shift
[1518-06-30 00:24] falls asleep
[1518-11-01 23:57] Guard #1229 begins shift
[1518-09-18 00:26] falls asleep
[1518-06-26 23:59] Guard #1889 begins shift
[1518-09-29 00:58] wakes up
[1518-04-04 00:36] falls asleep
[1518-06-27 00:25] falls asleep
[1518-10-06 00:29] falls asleep
[1518-06-11 00:53] wakes up
[1518-11-01 00:30] wakes up
[1518-11-03 00:34] wakes up
[1518-04-09 00:37] falls asleep
[1518-10-28 00:34] falls asleep
[1518-09-24 00:23] falls asleep
[1518-03-28 00:23] falls asleep
[1518-09-24 00:50] falls asleep
[1518-06-06 00:02] Guard #467 begins shift
[1518-05-01 23:56] Guard #3541 begins shift
[1518-07-09 00:33] wakes up
[1518-11-17 00:04] Guard #233 begins shift
[1518-07-11 23:59] Guard #1931 begins shift
[1518-09-05 00:04] Guard #1931 begins shift
[1518-10-09 00:38] falls asleep
[1518-09-18 00:04] Guard #1931 begins shift
[1518-09-27 00:03] Guard #2309 begins shift
[1518-10-14 00:11] falls asleep
[1518-06-26 00:00] Guard #1229 begins shift
[1518-11-21 00:03] Guard #2521 begins shift
[1518-05-10 00:18] wakes up
[1518-03-24 00:04] Guard #3329 begins shift
[1518-09-12 00:58] wakes up
[1518-04-16 23:57] Guard #3329 begins shift
[1518-07-14 00:56] wakes up
[1518-07-27 00:31] falls asleep
[1518-06-21 00:49] falls asleep
[1518-07-26 23:57] Guard #233 begins shift
[1518-08-13 00:44] wakes up
[1518-03-12 00:03] Guard #2521 begins shift
[1518-10-30 00:06] falls asleep
[1518-09-01 00:08] falls asleep
[1518-05-25 23:58] Guard #2141 begins shift
[1518-10-02 00:47] wakes up
[1518-10-11 00:26] falls asleep
[1518-09-02 00:42] falls asleep
[1518-10-29 00:53] wakes up
[1518-07-12 00:56] wakes up
[1518-06-26 00:08] falls asleep
[1518-03-14 00:04] falls asleep
[1518-04-04 00:52] falls asleep
[1518-09-20 23:54] Guard #761 begins shift
[1518-11-10 00:53] falls asleep
[1518-06-07 00:54] falls asleep
[1518-09-11 00:00] Guard #3329 begins shift
[1518-05-09 00:29] falls asleep
[1518-03-23 00:10] falls asleep
[1518-09-12 00:00] Guard #467 begins shift
[1518-09-17 00:04] Guard #743 begins shift
[1518-04-12 00:20] falls asleep
[1518-09-24 00:30] wakes up
[1518-11-18 00:24] falls asleep
[1518-08-23 00:58] wakes up
[1518-04-18 00:45] wakes up
[1518-11-09 00:33] falls asleep
[1518-05-14 00:37] falls asleep
[1518-08-26 00:32] falls asleep
[1518-11-14 00:34] falls asleep
[1518-03-29 23:59] Guard #3529 begins shift
[1518-08-05 00:52] wakes up
[1518-08-26 23:57] Guard #1783 begins shift
[1518-04-05 00:59] wakes up
[1518-06-23 00:42] wakes up
[1518-04-15 00:58] wakes up
[1518-03-10 23:56] Guard #443 begins shift
[1518-06-09 00:04] Guard #1297 begins shift
[1518-08-07 00:52] falls asleep
[1518-09-23 00:52] falls asleep
[1518-08-03 23:49] Guard #743 begins shift
[1518-10-21 00:33] falls asleep
[1518-07-10 00:23] falls asleep
[1518-05-07 00:00] Guard #1297 begins shift
[1518-07-10 00:50] wakes up
[1518-09-06 00:34] wakes up
[1518-10-26 00:29] wakes up
[1518-09-11 00:37] falls asleep
[1518-05-24 00:15] falls asleep
[1518-08-06 00:51] wakes up
[1518-06-20 00:46] falls asleep
[1518-04-09 00:33] wakes up
[1518-08-22 00:17] falls asleep
[1518-03-25 00:17] falls asleep
[1518-11-22 00:38] falls asleep
[1518-03-16 00:43] wakes up
[1518-10-08 00:01] Guard #2003 begins shift
[1518-06-21 00:00] Guard #3329 begins shift
[1518-07-13 00:58] wakes up
[1518-07-03 00:21] falls asleep
[1518-05-02 00:50] falls asleep
[1518-03-10 00:38] wakes up
[1518-11-13 00:03] Guard #1783 begins shift
[1518-09-18 00:34] wakes up
[1518-08-04 00:32] falls asleep
[1518-10-01 00:01] Guard #3529 begins shift
[1518-05-09 00:37] falls asleep
[1518-11-01 00:49] wakes up
[1518-04-05 00:04] Guard #761 begins shift
[1518-08-30 00:21] falls asleep
[1518-10-19 00:56] wakes up
[1518-06-16 00:56] wakes up
[1518-05-13 00:40] wakes up
[1518-06-03 00:12] wakes up
[1518-06-02 23:46] Guard #2789 begins shift
[1518-08-11 00:00] Guard #761 begins shift
[1518-08-05 00:42] falls asleep
[1518-10-27 23:56] Guard #2003 begins shift
[1518-05-15 23:57] Guard #761 begins shift
[1518-06-21 00:17] falls asleep
[1518-07-17 00:00] Guard #1889 begins shift
[1518-04-03 23:59] Guard #1783 begins shift
[1518-06-01 00:24] falls asleep
[1518-10-04 23:56] Guard #2141 begins shift
[1518-04-20 00:03] Guard #947 begins shift
[1518-10-20 23:53] Guard #1297 begins shift
[1518-07-12 00:45] falls asleep
[1518-05-03 00:58] wakes up
[1518-08-06 23:56] Guard #3541 begins shift
[1518-09-24 00:57] wakes up
[1518-06-21 00:59] wakes up
[1518-06-01 00:54] falls asleep
[1518-09-25 00:42] falls asleep
[1518-09-19 00:04] Guard #1889 begins shift
[1518-10-01 00:57] falls asleep
[1518-08-08 00:22] wakes up
[1518-07-12 00:35] wakes up
[1518-05-03 00:02] Guard #1783 begins shift
[1518-10-29 00:03] Guard #947 begins shift
[1518-04-18 00:31] falls asleep
[1518-10-08 00:20] wakes up
[1518-04-04 00:47] wakes up
[1518-09-10 00:03] Guard #1297 begins shift
[1518-04-16 00:30] falls asleep
[1518-06-11 00:37] wakes up
[1518-07-07 00:00] Guard #1229 begins shift
[1518-10-26 00:28] falls asleep
[1518-11-10 00:01] Guard #947 begins shift
[1518-10-26 00:57] wakes up
[1518-07-31 00:01] Guard #2309 begins shift
[1518-03-10 00:05] falls asleep
[1518-11-18 23:58] Guard #2003 begins shift
[1518-07-05 00:02] Guard #3541 begins shift
[1518-06-02 00:13] wakes up
[1518-03-17 23:56] Guard #2963 begins shift
[1518-07-13 00:27] falls asleep
[1518-11-08 00:07] falls asleep
[1518-04-29 00:04] Guard #1931 begins shift
[1518-11-15 23:59] Guard #1889 begins shift
[1518-09-19 00:25] falls asleep
[1518-11-22 00:54] wakes up
[1518-09-08 23:53] Guard #1889 begins shift
[1518-11-11 00:54] wakes up
[1518-10-02 00:45] falls asleep
[1518-08-28 00:31] falls asleep
[1518-03-14 00:28] wakes up
[1518-06-05 00:00] Guard #743 begins shift
[1518-06-23 00:02] Guard #467 begins shift
[1518-08-26 00:57] wakes up
[1518-08-26 00:45] falls asleep
[1518-05-21 00:00] Guard #233 begins shift
[1518-08-06 00:40] wakes up
[1518-10-09 00:00] Guard #743 begins shift
[1518-05-11 00:35] falls asleep
[1518-04-24 00:57] wakes up
[1518-07-09 00:52] wakes up
[1518-09-28 00:06] falls asleep
[1518-11-03 00:54] wakes up
[1518-08-12 00:28] wakes up
[1518-08-04 00:55] wakes up
[1518-09-03 00:35] wakes up
[1518-04-02 00:00] falls asleep
[1518-09-19 23:57] Guard #3541 begins shift
[1518-10-02 00:37] wakes up
[1518-11-07 00:58] wakes up
[1518-04-12 00:51] wakes up
[1518-09-20 00:59] wakes up
[1518-04-12 00:02] Guard #1931 begins shift
[1518-07-04 00:47] falls asleep
[1518-05-07 00:39] falls asleep
[1518-05-16 00:52] falls asleep
[1518-05-30 00:22] falls asleep
[1518-09-19 00:20] wakes up
[1518-05-08 00:01] Guard #3329 begins shift
[1518-04-28 00:31] falls asleep
[1518-07-25 23:56] Guard #443 begins shift
[1518-06-16 23:57] Guard #2281 begins shift
[1518-11-03 00:01] falls asleep
[1518-09-17 00:15] falls asleep
[1518-03-21 00:31] falls asleep
[1518-10-24 00:41] falls asleep
[1518-05-01 00:57] wakes up
[1518-04-16 00:45] wakes up
[1518-03-26 00:51] falls asleep
[1518-05-15 00:58] wakes up
[1518-06-06 00:35] falls asleep
[1518-08-04 00:11] wakes up
[1518-10-10 00:51] wakes up
[1518-06-06 00:46] wakes up
[1518-10-13 00:00] Guard #761 begins shift
[1518-10-03 00:35] falls asleep
[1518-08-19 00:00] Guard #761 begins shift
[1518-10-08 00:53] wakes up
[1518-11-16 00:58] wakes up
[1518-03-13 00:32] falls asleep
[1518-07-02 00:54] falls asleep
[1518-07-20 00:42] falls asleep
[1518-05-11 00:54] wakes up
[1518-05-14 23:56] Guard #1297 begins shift
[1518-09-05 23:56] Guard #1931 begins shift
[1518-06-07 23:57] Guard #947 begins shift
[1518-05-16 00:21] wakes up
[1518-04-21 00:35] falls asleep
[1518-08-14 23:58] Guard #2591 begins shift
[1518-10-23 00:31] wakes up
[1518-06-27 00:56] wakes up
[1518-06-30 00:27] wakes up
[1518-06-29 00:50] falls asleep
[1518-08-13 00:51] wakes up
[1518-08-06 00:03] Guard #2141 begins shift
[1518-08-15 00:06] falls asleep
[1518-10-23 00:39] falls asleep
[1518-10-31 00:42] wakes up
[1518-10-15 00:46] wakes up
[1518-05-23 00:03] Guard #3529 begins shift
[1518-11-14 23:58] Guard #2309 begins shift
[1518-10-10 00:59] wakes up
[1518-06-07 00:18] falls asleep
[1518-04-28 00:56] wakes up
[1518-10-14 00:30] falls asleep
[1518-09-15 00:37] wakes up
[1518-06-03 00:44] wakes up
[1518-04-07 00:04] Guard #1297 begins shift
[1518-10-16 00:39] wakes up
[1518-04-26 00:04] Guard #1229 begins shift
[1518-08-07 23:46] Guard #2003 begins shift
[1518-03-13 00:58] wakes up
[1518-04-15 00:51] wakes up
[1518-07-15 00:53] falls asleep
[1518-10-07 00:57] wakes up
[1518-05-14 00:44] wakes up
[1518-11-10 00:50] wakes up
[1518-08-30 00:37] wakes up
[1518-08-29 23:59] Guard #2591 begins shift
[1518-09-21 00:04] falls asleep
[1518-06-19 00:44] wakes up
[1518-07-09 00:46] falls asleep
[1518-10-22 00:54] falls asleep
[1518-03-17 00:28] falls asleep
[1518-08-03 00:27] wakes up
[1518-11-13 00:17] falls asleep
[1518-10-29 00:39] wakes up
[1518-07-16 00:02] Guard #2591 begins shift
[1518-07-02 00:03] Guard #3529 begins shift
[1518-06-22 00:57] wakes up
[1518-05-18 00:00] Guard #761 begins shift
[1518-07-07 00:46] falls asleep
[1518-05-24 00:50] falls asleep
[1518-07-31 00:06] falls asleep
[1518-04-19 00:09] falls asleep
[1518-07-09 00:26] falls asleep
[1518-10-22 23:57] Guard #3541 begins shift
[1518-10-06 00:37] wakes up
[1518-04-17 23:59] Guard #3541 begins shift
[1518-10-14 00:53] wakes up
[1518-06-07 00:30] wakes up
[1518-10-19 23:52] Guard #1861 begins shift
[1518-09-03 00:01] falls asleep
[1518-08-07 00:43] wakes up";

    let s: Vec<Vec<&str>>= input.split('\n').map(|x| x.split(|c| c==']' || c==' ' || c == ':').filter(|x| !x.is_empty()).collect()).collect();
    let mut events : Vec<Event> = Vec::new();
    for ss in s
    {
        match ss[3]
        {
            "Guard" => events.push(Event{id : ss[4][1..].parse::<i32>().unwrap(), month: ss[0][6..8].parse::<i32>().unwrap(), day:ss[0][9..].parse::<i32>().unwrap(),hour: ss[1].parse::<i32>().unwrap(), time: ss[2].parse::<i32>().unwrap(), ev: EventType::Begins}),
            "falls" => events.push(Event{id : -1, month: ss[0][6..8].parse::<i32>().unwrap(),day:ss[0][9..].parse::<i32>().unwrap(), hour: ss[1].parse::<i32>().unwrap(), time: ss[2].parse::<i32>().unwrap(), ev: EventType::Asleep}),
            "wakes" => events.push(Event{id : -1, month: ss[0][6..8].parse::<i32>().unwrap(), day:ss[0][9..].parse::<i32>().unwrap(), hour: ss[1].parse::<i32>().unwrap(), time: ss[2].parse::<i32>().unwrap(), ev: EventType::Wakes}),
            _ => println!("Error"),
        }
    }
    events.sort();
    let mut last_time_event : i32 = 0;
    let mut m : HashMap<i32,i32> = HashMap::new();
    let mut times_asleep : HashMap<(i32,i32), i32> = HashMap::new();
    let mut last_idx : i32 = -1;
    for e in events
    {
        match  e.ev
        {
            EventType::Begins => last_idx = e.id,
            EventType::Asleep => last_time_event = e.time,
            EventType::Wakes => { 
                let cur_time_span = e.time-last_time_event;
                m.entry(last_idx)
                .and_modify(|v| *v += cur_time_span)
                .or_insert(cur_time_span);
                for i in last_time_event..e.time
                {
                times_asleep.entry((last_idx,i))
                .and_modify(|v| *v+=1)
                .or_insert(1);
                }
                last_time_event = e.time; 
            },
        }
    }
    let max_k = m.iter().max_by_key(|(_,&v)| v).unwrap().0;
    let s  =times_asleep.iter().filter(|((k1,_),_)| k1 == max_k).map(|((_,k2),v)| (k2,v)).max_by_key(|(_,&v)| v).unwrap().0;
    println!("Part 1: {}", s*max_k);
    let s2  =times_asleep.iter().max_by_key(|((_,_),&v)| v).unwrap();
    println!("Part 2: {}",(s2.0).0*(s2.0).1);

}

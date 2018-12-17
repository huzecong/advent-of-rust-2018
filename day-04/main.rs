extern crate chrono;

use std::collections::HashMap;
use std::fs;

use chrono::{NaiveDateTime, Timelike};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum EventType {
    BeginShift(i32),
    WakeUp,
    FallAsleep,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Event {
    time: NaiveDateTime,
    typ: EventType,
}

fn parse(s: &str) -> Event {
    let time_str = &s[1..17];
    let time = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M").ok().unwrap();
    let type_str = &s[19..];
    let typ = match type_str {
        "wakes up" => EventType::WakeUp,
        "falls asleep" => EventType::FallAsleep,
        s => {
            let guard_id = s.split_whitespace().skip(1).next().unwrap()[1..]
                    .parse::<i32>().ok().unwrap();
            EventType::BeginShift(guard_id)
        }
    };
    Event { time, typ }
}

trait Updateable<K, V> {
    fn update(&mut self, key: K, delta: V);
}

impl<K, V> Updateable<K, V> for HashMap<K, V> where
        K: std::cmp::Eq + std::hash::Hash,
        V: std::default::Default + std::ops::AddAssign {
    fn update(&mut self, key: K, delta: V) {
        *self.entry(key).or_default() += delta;
    }
}

trait ArgMax<T> {
    fn argmax(self) -> Option<usize>;
}

impl<T, Iter> ArgMax<T> for Iter where
        T: std::cmp::Ord,
        Iter: Iterator<Item=T> {
    fn argmax(self) -> Option<usize> {
        // select_fold1 from iterator.rs
        let mut it = self.enumerate();
        it.next().map(|first| {
            it.fold(first, |(i_sel, x_sel), (i_val, x_val)| {
                if x_val >= x_sel { (i_val, x_val) } else { (i_sel, x_sel) }
            })
        }).map(|(i, _)| i)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();  // temporary bound to the scope
    let mut events: Vec<Event> = input.split_terminator("\n").map(parse).collect::<Vec<_>>();
    events.sort();

    // Part 1
    let mut asleep_count = HashMap::<i32, u32>::new();
    let mut asleep_minute_count = HashMap::<i32, [u32; 60]>::new();
    let mut guard_id: Option<i32> = None;
    let mut sleep_time: Option<u32> = None;
    for event in events.iter() {
        match event.typ {
            EventType::BeginShift(id) => {
                guard_id = Some(id);
                sleep_time = None;
            }
            EventType::FallAsleep => {
                debug_assert!(guard_id.is_some());
                debug_assert!(sleep_time.is_none());
                sleep_time = Some(event.time.minute());
            }
            EventType::WakeUp => {
                debug_assert!(guard_id.is_some());
                debug_assert!(sleep_time.is_some());
                asleep_count.update(guard_id.unwrap(), event.time.minute() - sleep_time.unwrap());
                let minute_count = asleep_minute_count.entry(guard_id.unwrap())
                        .or_insert_with(|| [0u32; 60]);
                for minute in sleep_time.unwrap()..event.time.minute() {
                    minute_count[minute as usize] += 1;
                }
                sleep_time = None;
            }
        }
    }

    let (&asleep_most_guard, _) = asleep_count.iter().max_by_key(|(_, &value)| value).unwrap();
    let asleep_most_minute = asleep_minute_count.get(&asleep_most_guard).unwrap()
            .iter().argmax().unwrap();
    println!("{}", asleep_most_guard * asleep_most_minute as i32);

    // Part 2
    let (&asleep_freq_guard, asleep_freq_minute) = asleep_minute_count.iter().map(|(id, vec)| {
        let minute = vec.iter().argmax().unwrap();
        (id, minute, vec[minute])
    }).max_by_key(|(_, _, ref v)| *v).map(|(i, m, _)| (i, m)).unwrap();
    println!("{}", asleep_freq_guard * asleep_freq_minute as i32);
}

//! Iterator implementations for SMC data

use crate::types::*;

macro_rules! iter_impl {
    ( $(#[$meta:meta])*
    $struct:ident($range:tt) = $max:ident : $get:ident -> $out:tt) => {
        $(#[$meta])*
        /// Advancing this iterator by calling `nth` is a O(1) operation
        /// and will not query all intermediate keys.
        #[derive(Debug)]
        pub struct $struct<'a> {
            smc: &'a mut $crate::Smc,
            next: $range,
            max: $range,
        }

        impl<'a> $struct<'a> {
            pub(crate) fn new(smc: &'a mut $crate::Smc) -> $crate::Result<Self> {
                let max = $range::from(smc.$max()?);
                Ok(Self { smc, next: 0, max })
            }
        }

        impl<'a> Iterator for $struct<'a> {
            type Item = $crate::Result<$out>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.next >= self.max {
                    return None;
                }
                let value = match self.smc.$get(self.next) {
                    Ok(value) => value,
                    Err(e) => return Some(Err(e)),
                };
                self.next += 1;
                Some(Ok(value))
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let items_left = (self.max - self.next) as usize;
                (items_left, Some(items_left))
            }

            fn count(self) -> usize {
                (self.max - self.next) as usize
            }

            fn last(mut self) -> Option<Self::Item> {
                self.next = self.next.max(self.max.saturating_sub(1));
                self.next()
            }

            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.next = (self.next as usize).saturating_add(n) as $range;
                self.next()
            }
        }

        impl<'a> DoubleEndedIterator for $struct<'a> {
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.max <= self.next {
                    return None;
                }
                let value = match self.smc.$get(self.max) {
                    Ok(value) => value,
                    Err(e) => return Some(Err(e)),
                };
                self.max = self.max.saturating_sub(1);
                Some(Ok(value))
            }

            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                self.max = (self.max as usize).saturating_sub(n) as $range;
                self.next_back()
            }
        }
    };
}

iter_impl! {
    /// Iterator for [`FanSpeed`]s.
    FanIter(u8) = number_of_fans: fan_speed -> FanSpeed
}

iter_impl! {
    /// Iterator for [`BatteryDetail`]s.
    BatteryIter(u8) = number_of_batteries: battery_detail -> BatteryDetail
}

#[cfg(any(doc, target_os = "macos"))]
iter_impl! {
    /// Iterator for the [`Celsius`] temperatures of all cpu cores.
    CpuIter(u8) = number_of_cpus: cpu_core_temperature -> Celsius
}

iter_impl! {
    /// Iterator for all [`DbgKeyInfo`]s.
    KeysIter(u32) = number_of_keys: key_info_by_index -> DbgKeyInfo
}

iter_impl! {
    /// Iterator for all [`Dbg`]s.
    DataIter(u32) = number_of_keys: key_data_by_index -> Dbg
}
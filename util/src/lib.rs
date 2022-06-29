pub struct Lookup<T> {
    items: Vec<Item<T>>,
    free: Vec<usize>,
}

struct Item<T> {
    version: usize,
    val: Option<T>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct LookupId {
    index: usize,
    version: usize,
}

impl<T> Lookup<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Lookup {
            items: Vec::new(),
            free: Vec::new(),
        }
    }
    // TODO: make inserting cleaner.
    #[inline]
    pub fn insert(&mut self, val: T) -> LookupId {
        if let Some(index) = self.free.pop() {
            let item = self.items.get_mut(index).unwrap();
            item.val = Some(val);
            LookupId {
                index,
                version: item.version,
            }
        } else {
            self.items.push(Item {
                version: 1,
                val: Some(val),
            });
            LookupId {
                version: 1,
                index: self.items.len() - 1,
            }
        }
    }

    #[inline]
    pub fn get(&self, id: &LookupId) -> Option<&T> {
        self.items
            .get(id.index)
            .and_then(|item| (item.version == id.version).then(|| item.val.as_ref())?)
    }

    #[inline]
    pub fn remove(&mut self, id: &LookupId) -> Option<T> {
        self.items.get_mut(id.index).and_then(|item| {
            (item.version == id.version).then(|| {
                item.version += 1;
                self.free.push(id.index);
                item.val.take()
            })?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Lookup, LookupId};

    #[test]
    fn inserting() {
        let mut lookup: Lookup<char> = Lookup::new();
        let a: LookupId = lookup.insert('A');
        let b: LookupId = lookup.insert('B');
        let c: LookupId = lookup.insert('C');

        assert_eq!('A', *lookup.get(&a).unwrap());
        assert_eq!('B', *lookup.get(&b).unwrap());
        assert_eq!('C', *lookup.get(&c).unwrap());
    }

    #[test]
    fn insert_uses_slots() {
        let mut lookup: Lookup<char> = Lookup::new();
        let _: LookupId = lookup.insert('A');
        let b: LookupId = lookup.insert('B');
        let _: LookupId = lookup.insert('C');

        let _ = lookup.remove(&b);
        let d: LookupId = lookup.insert('D');
        assert!(b.index == d.index && b.version == 1 && d.version == 2)
    }
}

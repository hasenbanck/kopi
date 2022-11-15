use super::{Seal, Unseal, Value, ValueScope};
use crate::value::Array;

/// A hash map.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Map<'scope>(pub(crate) v8::Local<'scope, v8::Map>);

impl<'scope> Seal<Map<'scope>> for v8::Local<'scope, v8::Map> {
    #[inline(always)]
    fn seal(self) -> Map<'scope> {
        Map(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Map>> for Map<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Map> {
        self.0
    }
}

impl<'scope> From<Map<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Map<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Map<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Map>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Map<'scope> {
    /// Creates a new map.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>) -> Map<'scope> {
        v8::Map::new(scope.unseal()).seal()
    }

    /// Returns the number of elements in the map.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.size()
    }

    /// Returns `true` if the map is empty.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.size() == 0
    }

    /// Clears the map.
    #[inline(always)]
    pub fn clear(&self) {
        self.0.clear()
    }

    /// Returns the value at the given key.
    #[inline(always)]
    pub fn get(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> Option<Value<'scope>> {
        self.0.get(scope.unseal(), key.unseal()).map(|v| v.seal())
    }

    /// Either updates or inserts the value at the given key.
    #[inline(always)]
    pub fn set(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>, value: Value<'scope>) {
        let _ = self.0.set(scope.unseal(), key.unseal(), value.unseal());
    }

    /// Returns `true` if the map contains an entry with the given key.
    #[inline(always)]
    pub fn contains_key(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> bool {
        self.0.has(scope.unseal(), key.unseal()).unwrap_or(false)
    }

    /// Remove the entry with the given key. Returns `true` there was something to remove.
    #[inline(always)]
    pub fn remove(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> bool {
        self.0.delete(scope.unseal(), key.unseal()).unwrap_or(false)
    }

    /// Returns an array of the map.
    ///
    /// The array contains all key and value pairs as follows:
    ///
    /// ```ignore
    /// key0, value0, key1, value1...
    /// ```
    #[inline(always)]
    pub fn to_array(&self, scope: &mut ValueScope<'scope>) -> Array<'scope> {
        self.0.as_array(scope.unseal()).seal()
    }
}

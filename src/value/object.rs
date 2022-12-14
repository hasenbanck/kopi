use std::num::NonZeroI32;

pub use v8::{GetPropertyNamesArgs, IntegrityLevel, PropertyAttribute};

use super::{Array, Name, Seal, Unseal, Value, ValueScope};

/// An object.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Object<'scope>(pub(crate) v8::Local<'scope, v8::Object>);

impl<'scope> Seal<Object<'scope>> for v8::Local<'scope, v8::Object> {
    #[inline(always)]
    fn seal(self) -> Object<'scope> {
        Object(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::Object>> for Object<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::Object> {
        self.0
    }
}

impl<'scope> From<Object<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: Object<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for Object<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::Object>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> Object<'scope> {
    /// Creates a new object.
    #[inline(always)]
    pub fn new(scope: &mut ValueScope<'scope>) -> Object<'scope> {
        v8::Object::new(scope.unseal()).seal()
    }

    /// Creates a ECMAScript object with the given properties of `prototype_or_null`.
    ///
    /// It can be any value, and if it's `null`, the newly created object won't have a
    /// prototype at all.
    ///
    /// All properties will be created as enumerable, configurable and writable properties.
    #[inline(always)]
    pub fn with_prototype_and_properties<N, V>(
        scope: &mut ValueScope<'scope>,
        prototype_or_null: Value<'scope>,
        names: N,
        values: V,
    ) -> Object<'scope>
    where
        N: AsRef<[Name<'scope>]>,
        V: AsRef<[Value<'scope>]>,
    {
        let names = names.as_ref();
        let values = values.as_ref();

        // SAFETY: This is safe, since Name wraps a v8::Local<v8::Name> transparently.
        let names = unsafe {
            std::slice::from_raw_parts(
                names.as_ptr() as *const v8::Local<'scope, v8::Name>,
                names.len(),
            )
        };

        // SAFETY: This is safe, since Value wraps a v8::Local<v8::Value> transparently.
        let values = unsafe {
            std::slice::from_raw_parts(
                values.as_ptr() as *const v8::Local<'scope, v8::Value>,
                values.len(),
            )
        };

        v8::Object::with_prototype_and_properties(
            scope.unseal(),
            prototype_or_null.unseal(),
            names,
            values,
        )
        .seal()
    }

    // TODO return error in case it fails.
    /// Sets the value at the given key.
    #[inline(always)]
    pub fn set(
        &self,
        scope: &mut ValueScope<'scope>,
        key: Value<'scope>,
        value: Value<'scope>,
    ) -> bool {
        self.0
            .set(scope.unseal(), key.unseal(), value.unseal())
            .expect("TODO")
    }

    // TODO return error in case it fails.
    /// Sets the value at the given index.
    #[inline(always)]
    pub fn set_index(
        &self,
        scope: &mut ValueScope<'scope>,
        index: u32,
        value: Value<'scope>,
    ) -> bool {
        self.0
            .set_index(scope.unseal(), index, value.unseal())
            .expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Set the prototype object.
    #[inline(always)]
    pub fn set_prototype(&self, scope: &mut ValueScope<'scope>, prototype: Value<'scope>) -> bool {
        self.0
            .set_prototype(scope.unseal(), prototype.unseal())
            .expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Implements `CreateDataProperty` (ECMA-262, 7.3.5).
    ///
    /// Defines a configurable, writable, enumerable property with the given value on the object
    /// unless the property already exists and is not configurable or the object is not extensible.
    #[inline(always)]
    pub fn create_data_property(
        &self,
        scope: &mut ValueScope<'scope>,
        key: Name<'scope>,
        value: Value<'scope>,
    ) -> bool {
        self.0
            .create_data_property(scope.unseal(), key.unseal(), value.unseal())
            .expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Implements `DefineOwnProperty` (ECMA-262, 10.1.6).
    ///
    /// In general, [`Object::create_data_property()`] will be faster, however, does not allow for
    /// specifying attributes.
    #[inline(always)]
    pub fn define_own_property(
        &self,
        scope: &mut ValueScope<'scope>,
        key: Name<'scope>,
        value: Value<'scope>,
        attr: PropertyAttribute,
    ) -> bool {
        self.0
            .define_own_property(scope.unseal(), key.unseal(), value.unseal(), attr)
            .expect("TODO")
    }

    /// Returns the value at the given key if present.
    #[inline(always)]
    pub fn get(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> Option<Value<'scope>> {
        self.0.get(scope.unseal(), key.unseal()).map(|v| v.seal())
    }

    /// Returns the value at the given index if present.
    #[inline(always)]
    pub fn get_index(&self, scope: &mut ValueScope<'scope>, index: u32) -> Option<Value<'scope>> {
        self.0.get_index(scope.unseal(), index).map(|v| v.seal())
    }

    /// Returns the prototype object if present.
    #[inline(always)]
    pub fn prototype(&self, scope: &mut ValueScope<'scope>) -> Option<Value<'scope>> {
        self.0.get_prototype(scope.unseal()).map(|v| v.seal())
    }

    /// Returns the V8 hash value for this value. The current implementation
    /// uses a hidden property to store the identity hash.
    ///
    /// The hash is not guaranteed to be unique.
    #[inline(always)]
    pub fn identity_hash(&self) -> NonZeroI32 {
        self.0.get_identity_hash()
    }

    /// This function has the same functionality as [`Object::property_names()`] but the
    /// returned array doesn't contain the names of properties from prototype objects.
    #[inline(always)]
    pub fn own_property_names(
        &self,
        scope: &mut ValueScope<'scope>,
        args: GetPropertyNamesArgs,
    ) -> Option<Array<'scope>> {
        self.0
            .get_own_property_names(scope.unseal(), args)
            .map(|v| v.seal())
    }

    /// Returns an array containing the names of the filtered properties of this
    /// object, including properties from prototype objects.
    #[inline(always)]
    pub fn property_names(
        &self,
        scope: &mut ValueScope<'scope>,
        args: GetPropertyNamesArgs,
    ) -> Option<Array<'scope>> {
        self.0
            .get_property_names(scope.unseal(), args)
            .map(|v| v.seal())
    }

    // TODO return error in case it fails. What is the error case?
    /// Calls the abstract operation HasProperty(O, P) (ECMA-262, 7.3.12).
    ///
    /// Returns `true` if the object has the property.
    #[inline(always)]
    pub fn has(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> bool {
        self.0.has(scope.unseal(), key.unseal()).expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Returns `true` if there is a value at the given index.
    #[inline(always)]
    pub fn has_index(&self, scope: &mut ValueScope<'scope>, index: u32) -> bool {
        self.0.has_index(scope.unseal(), index).expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Calls the abstract operation HasOwnProperty(O, P) (ECMA-262, 7.3.13).
    ///
    /// Returns `true` if the object has the property.
    #[inline(always)]
    pub fn has_own_property(&self, scope: &mut ValueScope<'scope>, key: Name<'scope>) -> bool {
        self.0
            .has_own_property(scope.unseal(), key.unseal())
            .expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Deletes the value at the given key.
    ///
    /// Returns `true` if the value could be deleted.
    #[inline(always)]
    pub fn delete(&self, scope: &mut ValueScope<'scope>, key: Value<'scope>) -> bool {
        self.0.delete(scope.unseal(), key.unseal()).expect("TODO")
    }

    // TODO return error in case it fails. What is the error case?
    /// Deletes the value at the given index.
    ///
    /// Returns `true` if the value could be deleted.
    #[inline(always)]
    pub fn delete_index(&self, scope: &mut ValueScope<'scope>, index: u32) -> bool {
        self.0.delete_index(scope.unseal(), index).expect("TODO")
    }

    /// Returns the number of internal fields for this object.
    #[inline(always)]
    pub fn internal_field_count(&self) -> usize {
        self.0.internal_field_count()
    }

    /// Returns the value from a internal field if present.
    #[inline(always)]
    pub fn internal_field(
        &self,
        scope: &mut ValueScope<'scope>,
        index: usize,
    ) -> Option<Value<'scope>> {
        self.0
            .get_internal_field(scope.unseal(), index)
            .map(|v| v.seal())
    }

    // TODO return error in case it fails. What is the error case?
    /// Sets the integrity level of the object.
    ///
    /// Returns `true` if the integrity level could be set.
    #[inline(always)]
    pub fn set_integrity_level(
        &self,
        scope: &mut ValueScope<'scope>,
        level: IntegrityLevel,
    ) -> bool {
        self.0
            .set_integrity_level(scope.unseal(), level)
            .expect("TODO")
    }

    /// Sets the value in an internal field.
    ///
    /// Returns `false` when the index is out of bounds and the value could not be set.
    #[inline(always)]
    pub fn set_internal_field(&self, index: usize, value: Value<'scope>) -> bool {
        self.0.set_internal_field(index, value.unseal())
    }
}

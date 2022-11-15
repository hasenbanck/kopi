use super::{Primitive, Seal, Unseal, Value, ValueScope};

/// A big integer.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BigInt<'scope>(pub(crate) v8::Local<'scope, v8::BigInt>);

impl<'scope> Seal<BigInt<'scope>> for v8::Local<'scope, v8::BigInt> {
    #[inline(always)]
    fn seal(self) -> BigInt<'scope> {
        BigInt(self)
    }
}

impl<'scope> Unseal<v8::Local<'scope, v8::BigInt>> for BigInt<'scope> {
    #[inline(always)]
    fn unseal(self) -> v8::Local<'scope, v8::BigInt> {
        self.0
    }
}

impl<'scope> From<BigInt<'scope>> for Value<'scope> {
    #[inline(always)]
    fn from(value: BigInt<'scope>) -> Self {
        Value(value.0.into())
    }
}

impl<'scope> TryFrom<Value<'scope>> for BigInt<'scope> {
    type Error = v8::DataError;

    #[inline(always)]
    fn try_from(value: Value<'scope>) -> Result<Self, Self::Error> {
        let inner = v8::Local::<v8::BigInt>::try_from(value.0)?;
        Ok(Self(inner))
    }
}

impl<'scope> From<BigInt<'scope>> for Primitive<'scope> {
    #[inline(always)]
    fn from(value: BigInt<'scope>) -> Self {
        Primitive(value.0.into())
    }
}

impl<'scope> BigInt<'scope> {
    /// Creates a new big int from the given i64 value.
    #[inline(always)]
    pub fn new_from_i64(scope: &mut ValueScope<'scope>, value: i64) -> BigInt<'scope> {
        v8::BigInt::new_from_i64(scope.unseal(), value).seal()
    }

    /// Creates a new big int from the given u64 value.
    #[inline(always)]
    pub fn new_from_u64(scope: &mut ValueScope<'scope>, value: u64) -> BigInt<'scope> {
        v8::BigInt::new_from_u64(scope.unseal(), value).seal()
    }

    /// Creates a new big int using the sign bit and the given of words.
    ///
    /// The resulting big int is calculated as:
    ///
    /// ```ignore
    /// (-1)^sign_bit * (words[0] * (2^64)^0 + words[1] * (2^64)^1 + ...)
    /// ```
    ///
    /// Returns `None` if the value could not be created.
    #[inline(always)]
    pub fn new_from_words<W>(
        scope: &mut ValueScope<'scope>,
        sign_bit: bool,
        words: &W,
    ) -> Option<BigInt<'scope>>
    where
        W: AsRef<[u64]>,
    {
        v8::BigInt::new_from_words(scope.unseal(), sign_bit, words.as_ref()).map(|v| v.seal())
    }

    /// Returns the u64 value of the big int. The second return value signals if the conversion
    /// was lossless (`true`) or the value had to be truncated `false`.
    #[inline(always)]
    pub fn value_u64(&self) -> (u64, bool) {
        self.0.u64_value()
    }

    /// Returns the i64 value of the big int. The second return value signals if the conversion
    /// was lossless (`true`) or the value had to be truncated `false`.
    #[inline(always)]
    pub fn value_i64(&self) -> (i64, bool) {
        self.0.i64_value()
    }

    /// Returns the number of words needed to store the big int.
    #[inline(always)]
    pub fn word_count(&self) -> usize {
        self.0.word_count()
    }

    /// Writes the words of the big int into the given slice. Returns the sign bit, which will be
    /// `true` if this big int is negative. The number will be truncated, if the `words` slice is to
    /// small to hold the full big int.
    ///
    /// Use [`word_count`()] to get the required size.
    #[inline]
    pub fn value_words<W>(&self, mut words: W) -> bool
    where
        W: AsMut<[u64]>,
    {
        let (sign, _) = self.0.to_words_array(words.as_mut());
        sign
    }
}

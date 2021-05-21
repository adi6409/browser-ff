//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsICryptoHash.idl
//


/// `interface nsICryptoHash : nsISupports`
///

/// ```text
/// /**
///  * nsICryptoHash
///  * This interface provides crytographic hashing algorithms.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICryptoHash {
    vtable: *const nsICryptoHashVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICryptoHash.
unsafe impl XpCom for nsICryptoHash {
    const IID: nsIID = nsID(0x1e5b7c43, 0x4688, 0x45ce,
        [0x92, 0xe1, 0x77, 0xed, 0x93, 0x1e, 0x3b, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICryptoHash {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICryptoHash.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICryptoHashCoerce {
    /// Cheaply cast a value of this type from a `nsICryptoHash`.
    fn coerce_from(v: &nsICryptoHash) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICryptoHashCoerce for nsICryptoHash {
    #[inline]
    fn coerce_from(v: &nsICryptoHash) -> &Self {
        v
    }
}

impl nsICryptoHash {
    /// Cast this `nsICryptoHash` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICryptoHashCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICryptoHash {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsICryptoHashCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICryptoHash) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICryptoHash
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICryptoHashVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in unsigned long aAlgorithm); */
    pub Init: unsafe extern "system" fn (this: *const nsICryptoHash, aAlgorithm: u32) -> ::nserror::nsresult,

    /* [must_use] void initWithString (in ACString aAlgorithm); */
    pub InitWithString: unsafe extern "system" fn (this: *const nsICryptoHash, aAlgorithm: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen); */
    pub Update: unsafe extern "system" fn (this: *const nsICryptoHash, aData: *const u8, aLen: u32) -> ::nserror::nsresult,

    /* [must_use] void updateFromStream (in nsIInputStream aStream, in unsigned long aLen); */
    pub UpdateFromStream: unsafe extern "system" fn (this: *const nsICryptoHash, aStream: *const nsIInputStream, aLen: u32) -> ::nserror::nsresult,

    /* ACString finish (in boolean aASCII); */
    pub Finish: unsafe extern "system" fn (this: *const nsICryptoHash, aASCII: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICryptoHash {
    /// ```text
    /// /**
    ///      * Hashing Algorithms.  These values are to be used by the
    ///      * |init| method to indicate which hashing function to
    ///      * use.  These values must be identical to the values defined
    ///      * in security/nss/lib/util/hasht.h in type HASH_HashType.
    ///      * This allows us to use NSS mapping functions like
    ///      * HASH_GetHashOidTagByHashType with these values.
    ///      */
    /// ```
    ///

    pub const MD5: i64 = 2;


    pub const SHA1: i64 = 3;


    pub const SHA256: i64 = 4;


    pub const SHA384: i64 = 5;


    pub const SHA512: i64 = 6;

    /// ```text
    /// /**
    ///      * Initialize the hashing object. This method may be
    ///      * called multiple times with different algorithm types.
    ///      *
    ///      * @param aAlgorithm the algorithm type to be used.
    ///      *        This value must be one of the above valid
    ///      *        algorithm types.
    ///      *
    ///      * @throws NS_ERROR_INVALID_ARG if an unsupported algorithm
    ///      *         type is passed.
    ///      *
    ///      * NOTE: This method or initWithString must be called
    ///      *       before any other method on this interface is called.
    ///      */
    /// ```
    ///

    /// `void init (in unsigned long aAlgorithm);`
    #[inline]
    pub unsafe fn Init(&self, aAlgorithm: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aAlgorithm)
    }


    /// ```text
    /// /**
    ///      * Initialize the hashing object. This method may be
    ///      * called multiple times with different algorithm types.
    ///      *
    ///      * @param aAlgorithm the algorithm type to be used.
    ///      *
    ///      * @throws NS_ERROR_INVALID_ARG if an unsupported algorithm
    ///      *         type is passed.
    ///      *
    ///      * NOTE: This method or init must be called before any
    ///      *       other method on this interface is called.
    ///      */
    /// ```
    ///

    /// `[must_use] void initWithString (in ACString aAlgorithm);`
    #[inline]
    pub unsafe fn InitWithString(&self, aAlgorithm: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).InitWithString)(self, aAlgorithm)
    }


    /// ```text
    /// /**
    ///      * @param aData a buffer to calculate the hash over
    ///      *
    ///      * @param aLen the length of the buffer |aData|
    ///      *
    ///      * @throws NS_ERROR_NOT_INITIALIZED If |init| has not been called.
    ///      */
    /// ```
    ///

    /// `void update ([array, size_is (aLen), const] in octet aData, in unsigned long aLen);`
    #[inline]
    pub unsafe fn Update(&self, aData: *const u8, aLen: u32) -> ::nserror::nsresult {
        ((*self.vtable).Update)(self, aData, aLen)
    }


    /// ```text
    /// /**
    ///      * Calculates and updates a new hash based on a given data stream.
    ///      *
    ///      * @param aStream an input stream to read from.
    ///      *
    ///      * @param aLen How much to read from the given |aStream|. Passing UINT32_MAX
    ///      *        indicates that all data available will be used to update the hash.
    ///      *
    ///      * @throws NS_ERROR_NOT_INITIALIZED If |init| has not been called.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE If the requested amount of
    ///      *         data to be calculated into the hash is not available.
    ///      *
    ///      */
    /// ```
    ///

    /// `[must_use] void updateFromStream (in nsIInputStream aStream, in unsigned long aLen);`
    #[inline]
    pub unsafe fn UpdateFromStream(&self, aStream: *const nsIInputStream, aLen: u32) -> ::nserror::nsresult {
        ((*self.vtable).UpdateFromStream)(self, aStream, aLen)
    }


    /// ```text
    /// /**
    ///      * Completes this hash object and produces the actual hash data.
    ///      *
    ///      * @param aASCII If true then the returned value is a base64 encoded string.
    ///      *        If false, then the returned value is binary data.
    ///      *
    ///      * @return a hash of the data that was read by this object.  This can
    ///      *         be either binary data or base 64 encoded.
    ///      *
    ///      * @throws NS_ERROR_NOT_INITIALIZED If |init| has not been called.
    ///      *
    ///      * NOTE: This method may be called any time after |init|
    ///      *       is called.  This call resets the object to its
    ///      *       pre-init state.
    ///      */
    /// ```
    ///

    /// `ACString finish (in boolean aASCII);`
    #[inline]
    pub unsafe fn Finish(&self, aASCII: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, aASCII, _retval)
    }


}



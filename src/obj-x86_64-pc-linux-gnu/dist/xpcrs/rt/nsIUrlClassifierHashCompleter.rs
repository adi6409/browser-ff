//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierHashCompleter.idl
//


/// `interface nsIFullHashMatch : nsISupports`
///

/// ```text
/// /**
///  * This interface contains feilds in Matches object of FullHashResponse(V4).
///  * Reference from:
///  * https://developers.google.com/safe-browsing/v4/update-api#http-post-response_2
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFullHashMatch {
    vtable: *const nsIFullHashMatchVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFullHashMatch.
unsafe impl XpCom for nsIFullHashMatch {
    const IID: nsIID = nsID(0xaabeb50e, 0xd9f7, 0x418e,
        [0x94, 0x69, 0x2c, 0xd9, 0x60, 0x89, 0x58, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFullHashMatch {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFullHashMatch.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFullHashMatchCoerce {
    /// Cheaply cast a value of this type from a `nsIFullHashMatch`.
    fn coerce_from(v: &nsIFullHashMatch) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFullHashMatchCoerce for nsIFullHashMatch {
    #[inline]
    fn coerce_from(v: &nsIFullHashMatch) -> &Self {
        v
    }
}

impl nsIFullHashMatch {
    /// Cast this `nsIFullHashMatch` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFullHashMatchCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFullHashMatch {
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
impl<T: nsISupportsCoerce> nsIFullHashMatchCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFullHashMatch) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFullHashMatch
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFullHashMatchVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString tableName; */
    pub GetTableName: unsafe extern "system" fn (this: *const nsIFullHashMatch, aTableName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString fullHash; */
    pub GetFullHash: unsafe extern "system" fn (this: *const nsIFullHashMatch, aFullHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t cacheDuration; */
    pub GetCacheDuration: unsafe extern "system" fn (this: *const nsIFullHashMatch, aCacheDuration: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFullHashMatch {


    /// `readonly attribute ACString tableName;`
    #[inline]
    pub unsafe fn GetTableName(&self, aTableName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTableName)(self, aTableName)
    }



    /// `readonly attribute ACString fullHash;`
    #[inline]
    pub unsafe fn GetFullHash(&self, aFullHash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFullHash)(self, aFullHash)
    }



    /// `readonly attribute uint32_t cacheDuration;`
    #[inline]
    pub unsafe fn GetCacheDuration(&self, aCacheDuration: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheDuration)(self, aCacheDuration)
    }


}


/// `interface nsIUrlClassifierHashCompleterCallback : nsISupports`
///

/// ```text
/// /**
///  * This interface is implemented by nsIUrlClassifierHashCompleter clients.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierHashCompleterCallback {
    vtable: *const nsIUrlClassifierHashCompleterCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierHashCompleterCallback.
unsafe impl XpCom for nsIUrlClassifierHashCompleterCallback {
    const IID: nsIID = nsID(0xda16de40, 0xdf26, 0x414d,
        [0xbd, 0xe7, 0xc4, 0xfa, 0xf4, 0x50, 0x48, 0x68]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierHashCompleterCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierHashCompleterCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierHashCompleterCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierHashCompleterCallback`.
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierHashCompleterCallbackCoerce for nsIUrlClassifierHashCompleterCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierHashCompleterCallback {
    /// Cast this `nsIUrlClassifierHashCompleterCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierHashCompleterCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierHashCompleterCallback {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierHashCompleterCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleterCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierHashCompleterCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierHashCompleterCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId); */
    pub CompletionV2: unsafe extern "system" fn (this: *const nsIUrlClassifierHashCompleterCallback, hash: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString, chunkId: uint32_t) -> ::nserror::nsresult,

    /* void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes); */
    pub CompletionV4: unsafe extern "system" fn (this: *const nsIUrlClassifierHashCompleterCallback, partialHash: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString, negativeCacheDuration: uint32_t, fullHashes: *const nsIArray) -> ::nserror::nsresult,

    /* void completionFinished (in nsresult status); */
    pub CompletionFinished: unsafe extern "system" fn (this: *const nsIUrlClassifierHashCompleterCallback, status: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierHashCompleterCallback {

    /// ```text
    /// /**
    ///    * A complete hash has been found that matches the partial hash.
    ///    * This method may be called 0-n times for a given
    ///    * nsIUrlClassifierCompleter::complete() call.
    ///    *
    ///    * @param hash
    ///    *        The 256-bit hash that was discovered.
    ///    * @param table
    ///    *        The name of the table that this hash belongs to.
    ///    * @param chunkId
    ///    *        The database chunk that this hash belongs to.
    ///    */
    /// ```
    ///

    /// `void completionV2 (in ACString hash, in ACString table, in uint32_t chunkId);`
    #[inline]
    pub unsafe fn CompletionV2(&self, hash: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString, chunkId: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).CompletionV2)(self, hash, table, chunkId)
    }


    /// ```text
    /// /**
    ///    * This will be called when a fullhash response is received and parsed
    ///    * no matter if any full hash has been found.
    ///    *
    ///    * @param partialHash
    ///    *        The hash that was sent for completion.
    ///    * @param table
    ///    *        The name of the table that this hash belongs to.
    ///    * @param negativeCacheDuration
    ///    *        The negative cache duration in millisecond.
    ///    * @param fullHashes
    ///    *        Array of fullhashes that match the prefix.
    ///    */
    /// ```
    ///

    /// `void completionV4 (in ACString partialHash, in ACString table, in uint32_t negativeCacheDuration, in nsIArray fullHashes);`
    #[inline]
    pub unsafe fn CompletionV4(&self, partialHash: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString, negativeCacheDuration: uint32_t, fullHashes: *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).CompletionV4)(self, partialHash, table, negativeCacheDuration, fullHashes)
    }


    /// ```text
    /// /**
    ///    * The completion is complete.  This method is called once per
    ///    * nsIUrlClassifierCompleter::complete() call, after all completion()
    ///    * calls are finished.
    ///    *
    ///    * @param status
    ///    *        NS_OK if the request completed successfully, or an error code.
    ///    */
    /// ```
    ///

    /// `void completionFinished (in nsresult status);`
    #[inline]
    pub unsafe fn CompletionFinished(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).CompletionFinished)(self, status)
    }


}


/// `interface nsIUrlClassifierHashCompleter : nsISupports`
///

/// ```text
/// /**
///  * Clients updating the url-classifier database have the option of sending
///  * partial (32-bit) hashes of URL fragments to be blocklisted.  If the
///  * url-classifier encounters one of these truncated hashes, it will ask an
///  * nsIUrlClassifierCompleter instance to asynchronously provide the complete
///  * hash, along with some associated metadata.
///  * This is only ever used for testing and should absolutely be deleted (I
    ///  * think).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierHashCompleter {
    vtable: *const nsIUrlClassifierHashCompleterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierHashCompleter.
unsafe impl XpCom for nsIUrlClassifierHashCompleter {
    const IID: nsIID = nsID(0x231fb2ad, 0xea8a, 0x4e63,
        [0xa3, 0x31, 0xea, 0xfc, 0x3b, 0x43, 0x48, 0x11]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierHashCompleter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierHashCompleter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierHashCompleterCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierHashCompleter`.
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierHashCompleterCoerce for nsIUrlClassifierHashCompleter {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self {
        v
    }
}

impl nsIUrlClassifierHashCompleter {
    /// Cast this `nsIUrlClassifierHashCompleter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierHashCompleterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierHashCompleter {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierHashCompleterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierHashCompleter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierHashCompleter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierHashCompleterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback); */
    pub Complete: unsafe extern "system" fn (this: *const nsIUrlClassifierHashCompleter, partialHash: *const ::nsstring::nsACString, gethashUrl: *const ::nsstring::nsACString, tableName: *const ::nsstring::nsACString, callback: *const nsIUrlClassifierHashCompleterCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierHashCompleter {

    /// ```text
    /// /**
    ///    * Request a completed hash from the given gethash url.
    ///    *
    ///    * @param partialHash
    ///    *        The 32-bit hash encountered by the url-classifier.
    ///    * @param gethashUrl
    ///    *        The gethash url to use.
    ///    * @param tableName
    ///    *        The table where we matched the partial hash.
    ///    * @param callback
    ///    *        An nsIUrlClassifierCompleterCallback instance.
    ///    */
    /// ```
    ///

    /// `void complete (in ACString partialHash, in ACString gethashUrl, in ACString tableName, in nsIUrlClassifierHashCompleterCallback callback);`
    #[inline]
    pub unsafe fn Complete(&self, partialHash: *const ::nsstring::nsACString, gethashUrl: *const ::nsstring::nsACString, tableName: *const ::nsstring::nsACString, callback: *const nsIUrlClassifierHashCompleterCallback) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, partialHash, gethashUrl, tableName, callback)
    }


}



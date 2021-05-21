//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierInfo.idl
//


/// `interface nsIUrlClassifierPositiveCacheEntry : nsISupports`
///

/// ```text
/// /**
///  * nsIUrlClassifierPositiveCacheEntry Represents a positive cache entry.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierPositiveCacheEntry {
    vtable: *const nsIUrlClassifierPositiveCacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierPositiveCacheEntry.
unsafe impl XpCom for nsIUrlClassifierPositiveCacheEntry {
    const IID: nsIID = nsID(0xb3c27f8c, 0x7db8, 0x4f3f,
        [0x97, 0xa5, 0x5a, 0x94, 0xd7, 0x81, 0xe5, 0x65]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierPositiveCacheEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierPositiveCacheEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierPositiveCacheEntryCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierPositiveCacheEntry`.
    fn coerce_from(v: &nsIUrlClassifierPositiveCacheEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierPositiveCacheEntryCoerce for nsIUrlClassifierPositiveCacheEntry {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPositiveCacheEntry) -> &Self {
        v
    }
}

impl nsIUrlClassifierPositiveCacheEntry {
    /// Cast this `nsIUrlClassifierPositiveCacheEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierPositiveCacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierPositiveCacheEntry {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierPositiveCacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPositiveCacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierPositiveCacheEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierPositiveCacheEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString fullhash; */
    pub GetFullhash: unsafe extern "system" fn (this: *const nsIUrlClassifierPositiveCacheEntry, aFullhash: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long long expiry; */
    pub GetExpiry: unsafe extern "system" fn (this: *const nsIUrlClassifierPositiveCacheEntry, aExpiry: *mut i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierPositiveCacheEntry {

    /// ```text
    /// /**
    ///    * Fullhash for the positive cache entry.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString fullhash;`
    #[inline]
    pub unsafe fn GetFullhash(&self, aFullhash: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFullhash)(self, aFullhash)
    }


    /// ```text
    /// /**
    ///    * Positive cache expiry.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long expiry;`
    #[inline]
    pub unsafe fn GetExpiry(&self, aExpiry: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetExpiry)(self, aExpiry)
    }


}


/// `interface nsIUrlClassifierCacheEntry : nsISupports`
///

/// ```text
/// /**
///  * nsIUrlClassifierCacheEntry contains cache information for
///  * a given prefix.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierCacheEntry {
    vtable: *const nsIUrlClassifierCacheEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierCacheEntry.
unsafe impl XpCom for nsIUrlClassifierCacheEntry {
    const IID: nsIID = nsID(0xd6297907, 0x8236, 0x4126,
        [0xad, 0xaf, 0xc3, 0xaa, 0x23, 0x9a, 0x0d, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierCacheEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierCacheEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierCacheEntryCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierCacheEntry`.
    fn coerce_from(v: &nsIUrlClassifierCacheEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierCacheEntryCoerce for nsIUrlClassifierCacheEntry {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCacheEntry) -> &Self {
        v
    }
}

impl nsIUrlClassifierCacheEntry {
    /// Cast this `nsIUrlClassifierCacheEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierCacheEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierCacheEntry {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierCacheEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCacheEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierCacheEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierCacheEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString prefix; */
    pub GetPrefix: unsafe extern "system" fn (this: *const nsIUrlClassifierCacheEntry, aPrefix: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long long expiry; */
    pub GetExpiry: unsafe extern "system" fn (this: *const nsIUrlClassifierCacheEntry, aExpiry: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute nsIArray matches; */
    pub GetMatches: unsafe extern "system" fn (this: *const nsIUrlClassifierCacheEntry, aMatches: *mut *const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierCacheEntry {

    /// ```text
    /// /**
    ///    * Prefix for this cache entry.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString prefix;`
    #[inline]
    pub unsafe fn GetPrefix(&self, aPrefix: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrefix)(self, aPrefix)
    }


    /// ```text
    /// /**
    ///    * Negative cache expiry.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long expiry;`
    #[inline]
    pub unsafe fn GetExpiry(&self, aExpiry: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetExpiry)(self, aExpiry)
    }


    /// ```text
    /// /**
    ///    * An array of nsIUrlClassifierPositiveCacheEntry, each item represents
    ///    * a positive cache entry with its fullhash and expiry.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray matches;`
    #[inline]
    pub unsafe fn GetMatches(&self, aMatches: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetMatches)(self, aMatches)
    }


}


/// `interface nsIUrlClassifierCacheInfo : nsISupports`
///

/// ```text
/// /**
///  * Cache information for a given table.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierCacheInfo {
    vtable: *const nsIUrlClassifierCacheInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierCacheInfo.
unsafe impl XpCom for nsIUrlClassifierCacheInfo {
    const IID: nsIID = nsID(0x69384f24, 0xd9c5, 0x4462,
        [0xb2, 0x4e, 0x35, 0x1c, 0x69, 0xe3, 0xb4, 0x6a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierCacheInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierCacheInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierCacheInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierCacheInfo`.
    fn coerce_from(v: &nsIUrlClassifierCacheInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierCacheInfoCoerce for nsIUrlClassifierCacheInfo {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCacheInfo) -> &Self {
        v
    }
}

impl nsIUrlClassifierCacheInfo {
    /// Cast this `nsIUrlClassifierCacheInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierCacheInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierCacheInfo {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierCacheInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCacheInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierCacheInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierCacheInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString table; */
    pub GetTable: unsafe extern "system" fn (this: *const nsIUrlClassifierCacheInfo, aTable: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray entries; */
    pub GetEntries: unsafe extern "system" fn (this: *const nsIUrlClassifierCacheInfo, aEntries: *mut *const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierCacheInfo {

    /// ```text
    /// /**
    ///    * Table name.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString table;`
    #[inline]
    pub unsafe fn GetTable(&self, aTable: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTable)(self, aTable)
    }



    /// `readonly attribute nsIArray entries;`
    #[inline]
    pub unsafe fn GetEntries(&self, aEntries: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetEntries)(self, aEntries)
    }


}


/// `interface nsIUrlClassifierGetCacheCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierGetCacheCallback {
    vtable: *const nsIUrlClassifierGetCacheCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierGetCacheCallback.
unsafe impl XpCom for nsIUrlClassifierGetCacheCallback {
    const IID: nsIID = nsID(0x26e12ea4, 0x14ff, 0x4c77,
        [0x85, 0x8f, 0x67, 0x45, 0x99, 0x8b, 0x76, 0x59]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierGetCacheCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierGetCacheCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierGetCacheCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierGetCacheCallback`.
    fn coerce_from(v: &nsIUrlClassifierGetCacheCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierGetCacheCallbackCoerce for nsIUrlClassifierGetCacheCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierGetCacheCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierGetCacheCallback {
    /// Cast this `nsIUrlClassifierGetCacheCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierGetCacheCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierGetCacheCallback {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierGetCacheCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierGetCacheCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierGetCacheCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierGetCacheCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onGetCacheComplete (in nsIUrlClassifierCacheInfo info); */
    pub OnGetCacheComplete: unsafe extern "system" fn (this: *const nsIUrlClassifierGetCacheCallback, info: *const nsIUrlClassifierCacheInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierGetCacheCallback {


    /// `void onGetCacheComplete (in nsIUrlClassifierCacheInfo info);`
    #[inline]
    pub unsafe fn OnGetCacheComplete(&self, info: *const nsIUrlClassifierCacheInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnGetCacheComplete)(self, info)
    }


}


/// `interface nsIUrlClassifierInfo : nsISupports`
///

/// ```text
/// /**
///  * Interface to query url-classifier information.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierInfo {
    vtable: *const nsIUrlClassifierInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierInfo.
unsafe impl XpCom for nsIUrlClassifierInfo {
    const IID: nsIID = nsID(0x411bbff4, 0x1b88, 0x4687,
        [0xaa, 0x36, 0xe2, 0xbb, 0xdd, 0x93, 0xf6, 0xe8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierInfo`.
    fn coerce_from(v: &nsIUrlClassifierInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierInfoCoerce for nsIUrlClassifierInfo {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierInfo) -> &Self {
        v
    }
}

impl nsIUrlClassifierInfo {
    /// Cast this `nsIUrlClassifierInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierInfo {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getCacheInfo (in ACString table, in nsIUrlClassifierGetCacheCallback callback); */
    pub GetCacheInfo: unsafe extern "system" fn (this: *const nsIUrlClassifierInfo, table: *const ::nsstring::nsACString, callback: *const nsIUrlClassifierGetCacheCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierInfo {

    /// ```text
    /// /**
    ///    * An asynchronous call to return cache information for the table.
    ///    */
    /// ```
    ///

    /// `void getCacheInfo (in ACString table, in nsIUrlClassifierGetCacheCallback callback);`
    #[inline]
    pub unsafe fn GetCacheInfo(&self, table: *const ::nsstring::nsACString, callback: *const nsIUrlClassifierGetCacheCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheInfo)(self, table, callback)
    }


}



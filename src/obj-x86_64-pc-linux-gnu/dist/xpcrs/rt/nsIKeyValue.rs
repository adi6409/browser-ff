//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/kvstore/nsIKeyValue.idl
//


/// `interface nsIKeyValueService : nsISupports`
///

/// ```text
/// /**
///  * The nsIKeyValue* interfaces provide a simple, asynchronous API to a key/value
///  * storage engine.  Basic put/get/has/delete operations are supported, as is
///  * enumeration of key/value pairs and the use of multiple named databases within
///  * a single storage file.  Operations have ACID semantics.
///  *
///  * This API does not (yet) support transactions, so it will not be appropriate
///  * for all use cases.  Extension of this API to support transactions is tracked
///  * by bug 1499238.
///  *
///  * The kvstore.jsm module wraps this API in a more idiomatic, Promise-based
///  * JS API that supports async/await.  In most cases, you're better off using
///  * that API from JS rather than using this one directly.  Bug 1512319 tracks
///  * native support for Promise in Rust-implemented XPCOM methods.
///  */
/// /**
///  * The key/value service.  Enables retrieval of handles to key/value databases.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueService {
    vtable: *const nsIKeyValueServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueService.
unsafe impl XpCom for nsIKeyValueService {
    const IID: nsIID = nsID(0x46c893dd, 0x4c14, 0x4de0,
        [0xb3, 0x3d, 0xa1, 0xbe, 0x18, 0xc6, 0xd0, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueService`.
    fn coerce_from(v: &nsIKeyValueService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueServiceCoerce for nsIKeyValueService {
    #[inline]
    fn coerce_from(v: &nsIKeyValueService) -> &Self {
        v
    }
}

impl nsIKeyValueService {
    /// Cast this `nsIKeyValueService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueService {
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
impl<T: nsISupportsCoerce> nsIKeyValueServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getOrCreate (in nsIKeyValueDatabaseCallback callback, in AUTF8String path, in AUTF8String name); */
    pub GetOrCreate: unsafe extern "system" fn (this: *const nsIKeyValueService, callback: *const nsIKeyValueDatabaseCallback, path: *const ::nsstring::nsACString, name: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueService {

    /// ```text
    /// /**
    ///      * Get a handle to an existing database or a newly-created one
    ///      * at the specified path and with the given name.
    ///      *
    ///      * The service supports multiple named databases at the same path
    ///      * (i.e. within the same storage file), so you can call this method
    ///      * multiple times with the same path and different names to retrieve
    ///      * multiple databases stored in the same location on disk.
    ///      */
    /// ```
    ///

    /// `void getOrCreate (in nsIKeyValueDatabaseCallback callback, in AUTF8String path, in AUTF8String name);`
    #[inline]
    pub unsafe fn GetOrCreate(&self, callback: *const nsIKeyValueDatabaseCallback, path: *const ::nsstring::nsACString, name: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrCreate)(self, callback, path, name)
    }


}


/// `interface nsIKeyValueDatabase : nsISupports`
///

/// ```text
/// /**
///  * A key/value database.
///  *
///  * All methods are asynchronous and take a callback as their first argument.
///  * The types of the callbacks vary, but they can all be implemented in JS
///  * via an object literal with the relevant methods.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueDatabase {
    vtable: *const nsIKeyValueDatabaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueDatabase.
unsafe impl XpCom for nsIKeyValueDatabase {
    const IID: nsIID = nsID(0xc449398e, 0x174c, 0x425b,
        [0x81, 0x95, 0xda, 0x6a, 0xa0, 0xcc, 0xd9, 0xa5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueDatabase {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueDatabase.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueDatabaseCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueDatabase`.
    fn coerce_from(v: &nsIKeyValueDatabase) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueDatabaseCoerce for nsIKeyValueDatabase {
    #[inline]
    fn coerce_from(v: &nsIKeyValueDatabase) -> &Self {
        v
    }
}

impl nsIKeyValueDatabase {
    /// Cast this `nsIKeyValueDatabase` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueDatabaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueDatabase {
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
impl<T: nsISupportsCoerce> nsIKeyValueDatabaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueDatabase) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueDatabase
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueDatabaseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void put (in nsIKeyValueVoidCallback callback, in AUTF8String key, in nsIVariant value); */
    pub Put: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVoidCallback, key: *const ::nsstring::nsACString, value: *const nsIVariant) -> ::nserror::nsresult,

    /* void writeMany (in nsIKeyValueVoidCallback callback, in Array<nsIKeyValuePair> pairs); */
    pub WriteMany: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVoidCallback, pairs: *const thin_vec::ThinVec<RefPtr<nsIKeyValuePair>>) -> ::nserror::nsresult,

    /* void get (in nsIKeyValueVariantCallback callback, in AUTF8String key, [optional] in nsIVariant defaultValue); */
    pub Get: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVariantCallback, key: *const ::nsstring::nsACString, defaultValue: *const nsIVariant) -> ::nserror::nsresult,

    /* void has (in nsIKeyValueVariantCallback callback, in AUTF8String key); */
    pub Has: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVariantCallback, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void delete (in nsIKeyValueVoidCallback callback, in AUTF8String key); */
    pub Delete: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVoidCallback, key: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void clear (in nsIKeyValueVoidCallback callback); */
    pub Clear: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueVoidCallback) -> ::nserror::nsresult,

    /* void enumerate (in nsIKeyValueEnumeratorCallback callback, [optional] in AUTF8String fromKey, [optional] in AUTF8String toKey); */
    pub Enumerate: unsafe extern "system" fn (this: *const nsIKeyValueDatabase, callback: *const nsIKeyValueEnumeratorCallback, fromKey: *const ::nsstring::nsACString, toKey: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueDatabase {

    /// ```text
    /// /**
    ///      * Write the specified key/value pair to the database.
    ///      */
    /// ```
    ///

    /// `void put (in nsIKeyValueVoidCallback callback, in AUTF8String key, in nsIVariant value);`
    #[inline]
    pub unsafe fn Put(&self, callback: *const nsIKeyValueVoidCallback, key: *const ::nsstring::nsACString, value: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).Put)(self, callback, key, value)
    }


    /// ```text
    /// /**
    ///      * Write multiple key/value pairs to the database.
    ///      *
    ///      * It supports two types of write:
    ///      *   * Put a key/value pair into the database. It takes a nsIKeyValuePair
    ///      *     where its key and value follow the same types as the put() method.
    ///      *   * Delete a key/value pair from database. It takes a nsIkeyValuePair
    ///      *     where its value property must be null or undefined.
    ///      *
    ///      * This features the "all-or-nothing" semantics, i.e. if any error occurs
    ///      * during the call, it will rollback the previous writes and terminate the
    ///      * call. In addition, writeMany should be more efficient than calling "put"
    ///      * or "delete" for every single key/value pair since it does all the writes
    ///      * in a single transaction.
    ///      *
    ///      * Note:
    ///      *   * If there are multiple values with the same key in the specified
    ///      *     pairs, only the last value will be stored in the database.
    ///      *   * Deleting a key that is not in the database will be silently ignored.
    ///      *   * If the same key gets put and deleted for multiple times, the final
    ///      *     state of that key is subject to the ordering of the put(s) and delete(s).
    ///      */
    /// ```
    ///

    /// `void writeMany (in nsIKeyValueVoidCallback callback, in Array<nsIKeyValuePair> pairs);`
    #[inline]
    pub unsafe fn WriteMany(&self, callback: *const nsIKeyValueVoidCallback, pairs: *const thin_vec::ThinVec<RefPtr<nsIKeyValuePair>>) -> ::nserror::nsresult {
        ((*self.vtable).WriteMany)(self, callback, pairs)
    }


    /// ```text
    /// /**
    ///      * Retrieve the value of the specified key from the database.
    ///      *
    ///      * If the key/value pair doesn't exist in the database, and you specify
    ///      * a default value, then the default value will be returned.  Otherwise,
    ///      * the callback's resolve() method will be called with a variant
    ///      * of type VTYPE_EMPTY, which translates to the JS `null` value.
    ///      */
    /// ```
    ///

    /// `void get (in nsIKeyValueVariantCallback callback, in AUTF8String key, [optional] in nsIVariant defaultValue);`
    #[inline]
    pub unsafe fn Get(&self, callback: *const nsIKeyValueVariantCallback, key: *const ::nsstring::nsACString, defaultValue: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, callback, key, defaultValue)
    }


    /// ```text
    /// /**
    ///      * Determine whether or not the key exists in the database.
    ///      */
    /// ```
    ///

    /// `void has (in nsIKeyValueVariantCallback callback, in AUTF8String key);`
    #[inline]
    pub unsafe fn Has(&self, callback: *const nsIKeyValueVariantCallback, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Has)(self, callback, key)
    }


    /// ```text
    /// /**
    ///      * Remove the key/value pair with the given key from the database.
    ///      *
    ///      * If the given key doesn't exist in the database, this operation doesn't
    ///      * fail; or rather, it fails silently, calling the resolve() method
    ///      * of its callback rather than reject().  If you want to know whether
    ///      * or not a key exists when deleting it, call the has() method first.
    ///      */
    /// ```
    ///

    /// `void delete (in nsIKeyValueVoidCallback callback, in AUTF8String key);`
    #[inline]
    pub unsafe fn Delete(&self, callback: *const nsIKeyValueVoidCallback, key: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Delete)(self, callback, key)
    }


    /// ```text
    /// /**
    ///      * Clear all the key/value pairs from the database.
    ///      */
    /// ```
    ///

    /// `void clear (in nsIKeyValueVoidCallback callback);`
    #[inline]
    pub unsafe fn Clear(&self, callback: *const nsIKeyValueVoidCallback) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, callback)
    }


    /// ```text
    /// /**
    ///      * Enumerate key/value pairs, starting with the first key equal to
    ///      * or greater than the "from" key (inclusive) and ending with the last key
    ///      * less than the "to" key (exclusive) sorted lexicographically.
    ///      *
    ///      * If either key is omitted, the range extends to the first and/or last key
    ///      * in the database.
    ///      */
    /// ```
    ///

    /// `void enumerate (in nsIKeyValueEnumeratorCallback callback, [optional] in AUTF8String fromKey, [optional] in AUTF8String toKey);`
    #[inline]
    pub unsafe fn Enumerate(&self, callback: *const nsIKeyValueEnumeratorCallback, fromKey: *const ::nsstring::nsACString, toKey: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Enumerate)(self, callback, fromKey, toKey)
    }


}


/// `interface nsIKeyValuePair : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValuePair {
    vtable: *const nsIKeyValuePairVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValuePair.
unsafe impl XpCom for nsIKeyValuePair {
    const IID: nsIID = nsID(0xbc37b06a, 0x23b5, 0x4b32,
        [0x82, 0x81, 0x4b, 0x84, 0x79, 0x60, 0x1c, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValuePair {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValuePair.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValuePairCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValuePair`.
    fn coerce_from(v: &nsIKeyValuePair) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValuePairCoerce for nsIKeyValuePair {
    #[inline]
    fn coerce_from(v: &nsIKeyValuePair) -> &Self {
        v
    }
}

impl nsIKeyValuePair {
    /// Cast this `nsIKeyValuePair` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValuePairCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValuePair {
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
impl<T: nsISupportsCoerce> nsIKeyValuePairCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValuePair) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValuePair
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValuePairVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String key; */
    pub GetKey: unsafe extern "system" fn (this: *const nsIKeyValuePair, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsIVariant value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsIKeyValuePair, aValue: *mut *const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValuePair {

    /// ```text
    /// /**
    ///  * A key/value pair.  Returned by nsIKeyValueEnumerator.getNext().
    ///  */
    /// ```
    ///

    /// `readonly attribute AUTF8String key;`
    #[inline]
    pub unsafe fn GetKey(&self, aKey: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetKey)(self, aKey)
    }



    /// `readonly attribute nsIVariant value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


}


/// `interface nsIKeyValueEnumerator : nsISupports`
///

/// ```text
/// /**
///  * An enumerator of key/value pairs.  Although its methods are similar
///  * to those of nsISimpleEnumerator, this interface's getNext() method returns
///  * an nsIKeyValuePair rather than an nsISupports, so consumers don't need
///  * to QI it to that interface; but this interface doesn't implement the JS
///  * iteration protocol (because the Rust-XPCOM bindings don't yet support it),
///  * which is another reason why you should use the kvstore.jsm module from JS
///  * instead of accessing this API directly.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueEnumerator {
    vtable: *const nsIKeyValueEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueEnumerator.
unsafe impl XpCom for nsIKeyValueEnumerator {
    const IID: nsIID = nsID(0xb9ba7116, 0xb7ff, 0x4717,
        [0x9a, 0x28, 0xa0, 0x8e, 0x68, 0x79, 0xb1, 0x99]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueEnumerator`.
    fn coerce_from(v: &nsIKeyValueEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueEnumeratorCoerce for nsIKeyValueEnumerator {
    #[inline]
    fn coerce_from(v: &nsIKeyValueEnumerator) -> &Self {
        v
    }
}

impl nsIKeyValueEnumerator {
    /// Cast this `nsIKeyValueEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueEnumerator {
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
impl<T: nsISupportsCoerce> nsIKeyValueEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* bool hasMoreElements (); */
    pub HasMoreElements: unsafe extern "system" fn (this: *const nsIKeyValueEnumerator, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIKeyValuePair getNext (); */
    pub GetNext: unsafe extern "system" fn (this: *const nsIKeyValueEnumerator, _retval: *mut *const nsIKeyValuePair) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueEnumerator {


    /// `bool hasMoreElements ();`
    #[inline]
    pub unsafe fn HasMoreElements(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMoreElements)(self, _retval)
    }



    /// `nsIKeyValuePair getNext ();`
    #[inline]
    pub unsafe fn GetNext(&self, _retval: *mut *const nsIKeyValuePair) -> ::nserror::nsresult {
        ((*self.vtable).GetNext)(self, _retval)
    }


}


/// `interface nsIKeyValueDatabaseCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueDatabaseCallback {
    vtable: *const nsIKeyValueDatabaseCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueDatabaseCallback.
unsafe impl XpCom for nsIKeyValueDatabaseCallback {
    const IID: nsIID = nsID(0x2becc1f8, 0x2d80, 0x4b63,
        [0x92, 0xa8, 0x24, 0xee, 0x8f, 0x79, 0xee, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueDatabaseCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueDatabaseCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueDatabaseCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueDatabaseCallback`.
    fn coerce_from(v: &nsIKeyValueDatabaseCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueDatabaseCallbackCoerce for nsIKeyValueDatabaseCallback {
    #[inline]
    fn coerce_from(v: &nsIKeyValueDatabaseCallback) -> &Self {
        v
    }
}

impl nsIKeyValueDatabaseCallback {
    /// Cast this `nsIKeyValueDatabaseCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueDatabaseCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueDatabaseCallback {
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
impl<T: nsISupportsCoerce> nsIKeyValueDatabaseCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueDatabaseCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueDatabaseCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueDatabaseCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resolve (in nsIKeyValueDatabase database); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIKeyValueDatabaseCallback, database: *const nsIKeyValueDatabase) -> ::nserror::nsresult,

    /* void reject (in AUTF8String message); */
    pub Reject: unsafe extern "system" fn (this: *const nsIKeyValueDatabaseCallback, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueDatabaseCallback {

    /// ```text
    /// /**
    ///  * A callback for the nsIKeyValueService.getOrCreate() method.
    ///  *
    ///  * The result is an nsIKeyValueDatabase.
    ///  */
    /// ```
    ///

    /// `void resolve (in nsIKeyValueDatabase database);`
    #[inline]
    pub unsafe fn Resolve(&self, database: *const nsIKeyValueDatabase) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, database)
    }



    /// `void reject (in AUTF8String message);`
    #[inline]
    pub unsafe fn Reject(&self, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Reject)(self, message)
    }


}


/// `interface nsIKeyValueEnumeratorCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueEnumeratorCallback {
    vtable: *const nsIKeyValueEnumeratorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueEnumeratorCallback.
unsafe impl XpCom for nsIKeyValueEnumeratorCallback {
    const IID: nsIID = nsID(0xb7ea2183, 0x880b, 0x4424,
        [0xab, 0x24, 0x5a, 0xa1, 0x55, 0x5b, 0x77, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueEnumeratorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueEnumeratorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueEnumeratorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueEnumeratorCallback`.
    fn coerce_from(v: &nsIKeyValueEnumeratorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueEnumeratorCallbackCoerce for nsIKeyValueEnumeratorCallback {
    #[inline]
    fn coerce_from(v: &nsIKeyValueEnumeratorCallback) -> &Self {
        v
    }
}

impl nsIKeyValueEnumeratorCallback {
    /// Cast this `nsIKeyValueEnumeratorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueEnumeratorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueEnumeratorCallback {
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
impl<T: nsISupportsCoerce> nsIKeyValueEnumeratorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueEnumeratorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueEnumeratorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueEnumeratorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resolve (in nsIKeyValueEnumerator enumerator); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIKeyValueEnumeratorCallback, enumerator: *const nsIKeyValueEnumerator) -> ::nserror::nsresult,

    /* void reject (in AUTF8String message); */
    pub Reject: unsafe extern "system" fn (this: *const nsIKeyValueEnumeratorCallback, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueEnumeratorCallback {

    /// ```text
    /// /**
    ///  * A callback for the nsIKeyValueDatabase.enumerate() method.
    ///  *
    ///  * The result is an nsIKeyValueEnumerator.
    ///  */
    /// ```
    ///

    /// `void resolve (in nsIKeyValueEnumerator enumerator);`
    #[inline]
    pub unsafe fn Resolve(&self, enumerator: *const nsIKeyValueEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, enumerator)
    }



    /// `void reject (in AUTF8String message);`
    #[inline]
    pub unsafe fn Reject(&self, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Reject)(self, message)
    }


}


/// `interface nsIKeyValuePairCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValuePairCallback {
    vtable: *const nsIKeyValuePairCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValuePairCallback.
unsafe impl XpCom for nsIKeyValuePairCallback {
    const IID: nsIID = nsID(0x50f65485, 0xec1e, 0x4307,
        [0x81, 0x2b, 0xb8, 0xa1, 0x5e, 0x1f, 0x38, 0x2e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValuePairCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValuePairCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValuePairCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValuePairCallback`.
    fn coerce_from(v: &nsIKeyValuePairCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValuePairCallbackCoerce for nsIKeyValuePairCallback {
    #[inline]
    fn coerce_from(v: &nsIKeyValuePairCallback) -> &Self {
        v
    }
}

impl nsIKeyValuePairCallback {
    /// Cast this `nsIKeyValuePairCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValuePairCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValuePairCallback {
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
impl<T: nsISupportsCoerce> nsIKeyValuePairCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValuePairCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValuePairCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValuePairCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resolve (in nsIKeyValuePair pair); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIKeyValuePairCallback, pair: *const nsIKeyValuePair) -> ::nserror::nsresult,

    /* void reject (in AUTF8String message); */
    pub Reject: unsafe extern "system" fn (this: *const nsIKeyValuePairCallback, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValuePairCallback {

    /// ```text
    /// /**
    ///  * A callback for the nsIKeyValueEnumerator.getNext() method.
    ///  *
    ///  * The result is the next key/value pair, expressed as separate key and value
    ///  * parameters.
    ///  */
    /// ```
    ///

    /// `void resolve (in nsIKeyValuePair pair);`
    #[inline]
    pub unsafe fn Resolve(&self, pair: *const nsIKeyValuePair) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, pair)
    }



    /// `void reject (in AUTF8String message);`
    #[inline]
    pub unsafe fn Reject(&self, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Reject)(self, message)
    }


}


/// `interface nsIKeyValueVariantCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueVariantCallback {
    vtable: *const nsIKeyValueVariantCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueVariantCallback.
unsafe impl XpCom for nsIKeyValueVariantCallback {
    const IID: nsIID = nsID(0x174ebfa1, 0x74ea, 0x42a7,
        [0xaa, 0x90, 0x85, 0xbb, 0xaf, 0x1d, 0xa4, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueVariantCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueVariantCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueVariantCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueVariantCallback`.
    fn coerce_from(v: &nsIKeyValueVariantCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueVariantCallbackCoerce for nsIKeyValueVariantCallback {
    #[inline]
    fn coerce_from(v: &nsIKeyValueVariantCallback) -> &Self {
        v
    }
}

impl nsIKeyValueVariantCallback {
    /// Cast this `nsIKeyValueVariantCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueVariantCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueVariantCallback {
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
impl<T: nsISupportsCoerce> nsIKeyValueVariantCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueVariantCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueVariantCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueVariantCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resolve (in nsIVariant result); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIKeyValueVariantCallback, result: *const nsIVariant) -> ::nserror::nsresult,

    /* void reject (in AUTF8String message); */
    pub Reject: unsafe extern "system" fn (this: *const nsIKeyValueVariantCallback, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueVariantCallback {

    /// ```text
    /// /**
    ///  * A callback for the nsIKeyValueDatabase.has() and .get() methods.
    ///  *
    ///  * The result is an nsIVariant, which is always a boolean for the has() method
    ///  * and can be any supported data type for the get() method.
    ///  */
    /// ```
    ///

    /// `void resolve (in nsIVariant result);`
    #[inline]
    pub unsafe fn Resolve(&self, result: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, result)
    }



    /// `void reject (in AUTF8String message);`
    #[inline]
    pub unsafe fn Reject(&self, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Reject)(self, message)
    }


}


/// `interface nsIKeyValueVoidCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyValueVoidCallback {
    vtable: *const nsIKeyValueVoidCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyValueVoidCallback.
unsafe impl XpCom for nsIKeyValueVoidCallback {
    const IID: nsIID = nsID(0x0c17497a, 0xccf8, 0x451a,
        [0x83, 0x8d, 0x9d, 0xfa, 0x7f, 0x84, 0x63, 0x79]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyValueVoidCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyValueVoidCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyValueVoidCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyValueVoidCallback`.
    fn coerce_from(v: &nsIKeyValueVoidCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyValueVoidCallbackCoerce for nsIKeyValueVoidCallback {
    #[inline]
    fn coerce_from(v: &nsIKeyValueVoidCallback) -> &Self {
        v
    }
}

impl nsIKeyValueVoidCallback {
    /// Cast this `nsIKeyValueVoidCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyValueVoidCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyValueVoidCallback {
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
impl<T: nsISupportsCoerce> nsIKeyValueVoidCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyValueVoidCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyValueVoidCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyValueVoidCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void resolve (); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIKeyValueVoidCallback) -> ::nserror::nsresult,

    /* void reject (in AUTF8String message); */
    pub Reject: unsafe extern "system" fn (this: *const nsIKeyValueVoidCallback, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyValueVoidCallback {

    /// ```text
    /// /**
    ///  * A callback for the nsIKeyValueDatabase.put() and .delete() methods.
    ///  *
    ///  * There is no result, but the resolve() method is still called when those
    ///  * async operations complete, to notify consumers of completion.
    ///  */
    /// ```
    ///

    /// `void resolve ();`
    #[inline]
    pub unsafe fn Resolve(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, )
    }



    /// `void reject (in AUTF8String message);`
    #[inline]
    pub unsafe fn Reject(&self, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Reject)(self, message)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/extensions/storage/mozIExtensionStorageArea.idl
//


/// `interface mozIExtensionStorageArea : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIExtensionStorageArea {
    vtable: *const mozIExtensionStorageAreaVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIExtensionStorageArea.
unsafe impl XpCom for mozIExtensionStorageArea {
    const IID: nsIID = nsID(0xd8eb3ff1, 0x9b4b, 0x435a,
        [0x99, 0xca, 0x5b, 0x8c, 0xba, 0xba, 0x24, 0x20]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIExtensionStorageArea {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIExtensionStorageArea.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIExtensionStorageAreaCoerce {
    /// Cheaply cast a value of this type from a `mozIExtensionStorageArea`.
    fn coerce_from(v: &mozIExtensionStorageArea) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIExtensionStorageAreaCoerce for mozIExtensionStorageArea {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageArea) -> &Self {
        v
    }
}

impl mozIExtensionStorageArea {
    /// Cast this `mozIExtensionStorageArea` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIExtensionStorageAreaCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIExtensionStorageArea {
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
impl<T: nsISupportsCoerce> mozIExtensionStorageAreaCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageArea) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIExtensionStorageArea
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIExtensionStorageAreaVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void set (in AUTF8String extensionId, in AUTF8String json, in mozIExtensionStorageCallback callback); */
    pub Set: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, extensionId: *const ::nsstring::nsACString, json: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,

    /* void get (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
    pub Get: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, extensionId: *const ::nsstring::nsACString, key: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,

    /* void remove (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback); */
    pub Remove: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, extensionId: *const ::nsstring::nsACString, key: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,

    /* void clear (in AUTF8String extensionId, in mozIExtensionStorageCallback callback); */
    pub Clear: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, extensionId: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,

    /* void getBytesInUse (in AUTF8String extensionId, in AUTF8String keys, in mozIExtensionStorageCallback callback); */
    pub GetBytesInUse: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, extensionId: *const ::nsstring::nsACString, keys: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,

    /* void takeMigrationInfo (in mozIExtensionStorageCallback callback); */
    pub TakeMigrationInfo: unsafe extern "system" fn (this: *const mozIExtensionStorageArea, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIExtensionStorageArea {

    pub const SYNC_QUOTA_BYTES: i64 = 102400;


    pub const SYNC_QUOTA_BYTES_PER_ITEM: i64 = 8192;


    pub const SYNC_MAX_ITEMS: i64 = 512;


    /// `void set (in AUTF8String extensionId, in AUTF8String json, in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn Set(&self, extensionId: *const ::nsstring::nsACString, json: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Set)(self, extensionId, json, callback)
    }



    /// `void get (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn Get(&self, extensionId: *const ::nsstring::nsACString, key: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, extensionId, key, callback)
    }



    /// `void remove (in AUTF8String extensionId, in AUTF8String key, in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn Remove(&self, extensionId: *const ::nsstring::nsACString, key: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, extensionId, key, callback)
    }



    /// `void clear (in AUTF8String extensionId, in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn Clear(&self, extensionId: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, extensionId, callback)
    }



    /// `void getBytesInUse (in AUTF8String extensionId, in AUTF8String keys, in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn GetBytesInUse(&self, extensionId: *const ::nsstring::nsACString, keys: *const ::nsstring::nsACString, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetBytesInUse)(self, extensionId, keys, callback)
    }



    /// `void takeMigrationInfo (in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn TakeMigrationInfo(&self, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).TakeMigrationInfo)(self, callback)
    }


}


/// `interface mozIConfigurableExtensionStorageArea : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIConfigurableExtensionStorageArea {
    vtable: *const mozIConfigurableExtensionStorageAreaVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIConfigurableExtensionStorageArea.
unsafe impl XpCom for mozIConfigurableExtensionStorageArea {
    const IID: nsIID = nsID(0x2b008295, 0x1bcc, 0x4610,
        [0x84, 0xf1, 0xad, 0x4c, 0xab, 0x2f, 0xa9, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIConfigurableExtensionStorageArea {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIConfigurableExtensionStorageArea.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIConfigurableExtensionStorageAreaCoerce {
    /// Cheaply cast a value of this type from a `mozIConfigurableExtensionStorageArea`.
    fn coerce_from(v: &mozIConfigurableExtensionStorageArea) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIConfigurableExtensionStorageAreaCoerce for mozIConfigurableExtensionStorageArea {
    #[inline]
    fn coerce_from(v: &mozIConfigurableExtensionStorageArea) -> &Self {
        v
    }
}

impl mozIConfigurableExtensionStorageArea {
    /// Cast this `mozIConfigurableExtensionStorageArea` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIConfigurableExtensionStorageAreaCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIConfigurableExtensionStorageArea {
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
impl<T: nsISupportsCoerce> mozIConfigurableExtensionStorageAreaCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIConfigurableExtensionStorageArea) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIConfigurableExtensionStorageArea
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIConfigurableExtensionStorageAreaVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void configure (in nsIFile databaseFile, in nsIFile kintoFile); */
    pub Configure: unsafe extern "system" fn (this: *const mozIConfigurableExtensionStorageArea, databaseFile: *const nsIFile, kintoFile: *const nsIFile) -> ::nserror::nsresult,

    /* void teardown (in mozIExtensionStorageCallback callback); */
    pub Teardown: unsafe extern "system" fn (this: *const mozIConfigurableExtensionStorageArea, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIConfigurableExtensionStorageArea {


    /// `void configure (in nsIFile databaseFile, in nsIFile kintoFile);`
    #[inline]
    pub unsafe fn Configure(&self, databaseFile: *const nsIFile, kintoFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Configure)(self, databaseFile, kintoFile)
    }



    /// `void teardown (in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn Teardown(&self, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Teardown)(self, callback)
    }


}


/// `interface mozISyncedExtensionStorageArea : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISyncedExtensionStorageArea {
    vtable: *const mozISyncedExtensionStorageAreaVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISyncedExtensionStorageArea.
unsafe impl XpCom for mozISyncedExtensionStorageArea {
    const IID: nsIID = nsID(0x6dac82c9, 0x1d8a, 0x4893,
        [0x8c, 0x0f, 0x6e, 0x62, 0x6a, 0xef, 0x80, 0x2c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISyncedExtensionStorageArea {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISyncedExtensionStorageArea.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISyncedExtensionStorageAreaCoerce {
    /// Cheaply cast a value of this type from a `mozISyncedExtensionStorageArea`.
    fn coerce_from(v: &mozISyncedExtensionStorageArea) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISyncedExtensionStorageAreaCoerce for mozISyncedExtensionStorageArea {
    #[inline]
    fn coerce_from(v: &mozISyncedExtensionStorageArea) -> &Self {
        v
    }
}

impl mozISyncedExtensionStorageArea {
    /// Cast this `mozISyncedExtensionStorageArea` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISyncedExtensionStorageAreaCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISyncedExtensionStorageArea {
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
impl<T: nsISupportsCoerce> mozISyncedExtensionStorageAreaCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISyncedExtensionStorageArea) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISyncedExtensionStorageArea
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISyncedExtensionStorageAreaVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void fetchPendingSyncChanges (in mozIExtensionStorageCallback callback); */
    pub FetchPendingSyncChanges: unsafe extern "system" fn (this: *const mozISyncedExtensionStorageArea, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISyncedExtensionStorageArea {


    /// `void fetchPendingSyncChanges (in mozIExtensionStorageCallback callback);`
    #[inline]
    pub unsafe fn FetchPendingSyncChanges(&self, callback: *const mozIExtensionStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).FetchPendingSyncChanges)(self, callback)
    }


}


/// `interface mozIExtensionStorageListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIExtensionStorageListener {
    vtable: *const mozIExtensionStorageListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIExtensionStorageListener.
unsafe impl XpCom for mozIExtensionStorageListener {
    const IID: nsIID = nsID(0x8cb3c7e4, 0xd0ca, 0x4353,
        [0xbc, 0xcd, 0x26, 0x73, 0xb4, 0xe1, 0x15, 0x10]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIExtensionStorageListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIExtensionStorageListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIExtensionStorageListenerCoerce {
    /// Cheaply cast a value of this type from a `mozIExtensionStorageListener`.
    fn coerce_from(v: &mozIExtensionStorageListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIExtensionStorageListenerCoerce for mozIExtensionStorageListener {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageListener) -> &Self {
        v
    }
}

impl mozIExtensionStorageListener {
    /// Cast this `mozIExtensionStorageListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIExtensionStorageListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIExtensionStorageListener {
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
impl<T: nsISupportsCoerce> mozIExtensionStorageListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIExtensionStorageListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIExtensionStorageListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onChanged (in AUTF8String extensionId, in AUTF8String json); */
    pub OnChanged: unsafe extern "system" fn (this: *const mozIExtensionStorageListener, extensionId: *const ::nsstring::nsACString, json: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIExtensionStorageListener {


    /// `void onChanged (in AUTF8String extensionId, in AUTF8String json);`
    #[inline]
    pub unsafe fn OnChanged(&self, extensionId: *const ::nsstring::nsACString, json: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnChanged)(self, extensionId, json)
    }


}


/// `interface mozIExtensionStorageCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIExtensionStorageCallback {
    vtable: *const mozIExtensionStorageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIExtensionStorageCallback.
unsafe impl XpCom for mozIExtensionStorageCallback {
    const IID: nsIID = nsID(0x870dca40, 0x6602, 0x4748,
        [0x84, 0x93, 0xc4, 0x25, 0x3e, 0xb7, 0xf3, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIExtensionStorageCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIExtensionStorageCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIExtensionStorageCallbackCoerce {
    /// Cheaply cast a value of this type from a `mozIExtensionStorageCallback`.
    fn coerce_from(v: &mozIExtensionStorageCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIExtensionStorageCallbackCoerce for mozIExtensionStorageCallback {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageCallback) -> &Self {
        v
    }
}

impl mozIExtensionStorageCallback {
    /// Cast this `mozIExtensionStorageCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIExtensionStorageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIExtensionStorageCallback {
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
impl<T: nsISupportsCoerce> mozIExtensionStorageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIExtensionStorageCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIExtensionStorageCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIExtensionStorageCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSuccess (in nsIVariant result); */
    pub HandleSuccess: unsafe extern "system" fn (this: *const mozIExtensionStorageCallback, result: *const nsIVariant) -> ::nserror::nsresult,

    /* void handleError (in nsresult code, in AUTF8String message); */
    pub HandleError: unsafe extern "system" fn (this: *const mozIExtensionStorageCallback, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIExtensionStorageCallback {


    /// `void handleSuccess (in nsIVariant result);`
    #[inline]
    pub unsafe fn HandleSuccess(&self, result: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).HandleSuccess)(self, result)
    }



    /// `void handleError (in nsresult code, in AUTF8String message);`
    #[inline]
    pub unsafe fn HandleError(&self, code: ::nserror::nsresult, message: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).HandleError)(self, code, message)
    }


}



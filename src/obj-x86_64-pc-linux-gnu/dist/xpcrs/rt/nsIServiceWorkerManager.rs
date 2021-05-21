//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIServiceWorkerManager.idl
//


/// `interface nsIServiceWorkerUnregisterCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerUnregisterCallback {
    vtable: *const nsIServiceWorkerUnregisterCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerUnregisterCallback.
unsafe impl XpCom for nsIServiceWorkerUnregisterCallback {
    const IID: nsIID = nsID(0x52ee2c9d, 0xee87, 0x4caf,
        [0x95, 0x88, 0x23, 0xae, 0x77, 0xff, 0x87, 0x98]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerUnregisterCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerUnregisterCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerUnregisterCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerUnregisterCallback`.
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerUnregisterCallbackCoerce for nsIServiceWorkerUnregisterCallback {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self {
        v
    }
}

impl nsIServiceWorkerUnregisterCallback {
    /// Cast this `nsIServiceWorkerUnregisterCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerUnregisterCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerUnregisterCallback {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerUnregisterCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerUnregisterCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerUnregisterCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerUnregisterCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void unregisterSucceeded (in bool aState); */
    pub UnregisterSucceeded: unsafe extern "system" fn (this: *const nsIServiceWorkerUnregisterCallback, aState: bool) -> ::nserror::nsresult,

    /* void unregisterFailed (); */
    pub UnregisterFailed: unsafe extern "system" fn (this: *const nsIServiceWorkerUnregisterCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerUnregisterCallback {


    /// `void unregisterSucceeded (in bool aState);`
    #[inline]
    pub unsafe fn UnregisterSucceeded(&self, aState: bool) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterSucceeded)(self, aState)
    }



    /// `void unregisterFailed ();`
    #[inline]
    pub unsafe fn UnregisterFailed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterFailed)(self, )
    }


}


/// `interface nsIServiceWorkerInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerInfo {
    vtable: *const nsIServiceWorkerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerInfo.
unsafe impl XpCom for nsIServiceWorkerInfo {
    const IID: nsIID = nsID(0x76e357ed, 0x208d, 0x4e4c,
        [0x91, 0x65, 0x1c, 0x40, 0x59, 0x70, 0x78, 0x79]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerInfo`.
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerInfoCoerce for nsIServiceWorkerInfo {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self {
        v
    }
}

impl nsIServiceWorkerInfo {
    /// Cast this `nsIServiceWorkerInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerInfo {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString scriptSpec; */
    pub GetScriptSpec: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aScriptSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString cacheName; */
    pub GetCacheName: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aCacheName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short state; */
    pub GetState: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aState: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute nsIWorkerDebugger debugger; */
    pub GetDebugger: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aDebugger: *mut*const nsIWorkerDebugger) -> ::nserror::nsresult,

    /* readonly attribute bool handlesFetchEvents; */
    pub GetHandlesFetchEvents: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aHandlesFetchEvents: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute PRTime installedTime; */
    pub GetInstalledTime: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aInstalledTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime activatedTime; */
    pub GetActivatedTime: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aActivatedTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime redundantTime; */
    pub GetRedundantTime: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo, aRedundantTime: *mut PRTime) -> ::nserror::nsresult,

    /* void attachDebugger (); */
    pub AttachDebugger: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* void detachDebugger (); */
    pub DetachDebugger: unsafe extern "system" fn (this: *const nsIServiceWorkerInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerInfo {

    pub const STATE_PARSED: i64 = 0;


    pub const STATE_INSTALLING: i64 = 1;


    pub const STATE_INSTALLED: i64 = 2;


    pub const STATE_ACTIVATING: i64 = 3;


    pub const STATE_ACTIVATED: i64 = 4;


    pub const STATE_REDUNDANT: i64 = 5;


    pub const STATE_UNKNOWN: i64 = 6;


    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AString scriptSpec;`
    #[inline]
    pub unsafe fn GetScriptSpec(&self, aScriptSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptSpec)(self, aScriptSpec)
    }



    /// `readonly attribute AString cacheName;`
    #[inline]
    pub unsafe fn GetCacheName(&self, aCacheName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCacheName)(self, aCacheName)
    }



    /// `readonly attribute unsigned short state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }



    /// `readonly attribute nsIWorkerDebugger debugger;`
    #[inline]
    pub unsafe fn GetDebugger(&self, aDebugger: *mut*const nsIWorkerDebugger) -> ::nserror::nsresult {
        ((*self.vtable).GetDebugger)(self, aDebugger)
    }



    /// `readonly attribute bool handlesFetchEvents;`
    #[inline]
    pub unsafe fn GetHandlesFetchEvents(&self, aHandlesFetchEvents: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHandlesFetchEvents)(self, aHandlesFetchEvents)
    }



    /// `readonly attribute PRTime installedTime;`
    #[inline]
    pub unsafe fn GetInstalledTime(&self, aInstalledTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetInstalledTime)(self, aInstalledTime)
    }



    /// `readonly attribute PRTime activatedTime;`
    #[inline]
    pub unsafe fn GetActivatedTime(&self, aActivatedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetActivatedTime)(self, aActivatedTime)
    }



    /// `readonly attribute PRTime redundantTime;`
    #[inline]
    pub unsafe fn GetRedundantTime(&self, aRedundantTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetRedundantTime)(self, aRedundantTime)
    }



    /// `void attachDebugger ();`
    #[inline]
    pub unsafe fn AttachDebugger(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AttachDebugger)(self, )
    }



    /// `void detachDebugger ();`
    #[inline]
    pub unsafe fn DetachDebugger(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DetachDebugger)(self, )
    }


}


/// `interface nsIServiceWorkerRegistrationInfoListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoListener {
    vtable: *const nsIServiceWorkerRegistrationInfoListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerRegistrationInfoListener.
unsafe impl XpCom for nsIServiceWorkerRegistrationInfoListener {
    const IID: nsIID = nsID(0x87e63548, 0xd440, 0x4b8a,
        [0xb1, 0x58, 0x65, 0xad, 0x1d, 0xe0, 0x21, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerRegistrationInfoListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerRegistrationInfoListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerRegistrationInfoListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerRegistrationInfoListener`.
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerRegistrationInfoListenerCoerce for nsIServiceWorkerRegistrationInfoListener {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self {
        v
    }
}

impl nsIServiceWorkerRegistrationInfoListener {
    /// Cast this `nsIServiceWorkerRegistrationInfoListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerRegistrationInfoListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerRegistrationInfoListener {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerRegistrationInfoListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfoListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerRegistrationInfoListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onChange (); */
    pub OnChange: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfoListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerRegistrationInfoListener {


    /// `void onChange ();`
    #[inline]
    pub unsafe fn OnChange(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnChange)(self, )
    }


}


/// `interface nsIServiceWorkerRegistrationInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfo {
    vtable: *const nsIServiceWorkerRegistrationInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerRegistrationInfo.
unsafe impl XpCom for nsIServiceWorkerRegistrationInfo {
    const IID: nsIID = nsID(0xddbc1fd4, 0x2f2e, 0x4fca,
        [0xa3, 0x95, 0x6e, 0x01, 0x0b, 0xbe, 0xdf, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerRegistrationInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerRegistrationInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerRegistrationInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerRegistrationInfo`.
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerRegistrationInfoCoerce for nsIServiceWorkerRegistrationInfo {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self {
        v
    }
}

impl nsIServiceWorkerRegistrationInfo {
    /// Cast this `nsIServiceWorkerRegistrationInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerRegistrationInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerRegistrationInfo {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerRegistrationInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerRegistrationInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerRegistrationInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerRegistrationInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute AString scope; */
    pub GetScope: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aScope: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString scriptSpec; */
    pub GetScriptSpec: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aScriptSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short updateViaCache; */
    pub GetUpdateViaCache: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aUpdateViaCache: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute PRTime lastUpdateTime; */
    pub GetLastUpdateTime: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aLastUpdateTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute nsIServiceWorkerInfo evaluatingWorker; */
    pub GetEvaluatingWorker: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aEvaluatingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* readonly attribute nsIServiceWorkerInfo installingWorker; */
    pub GetInstallingWorker: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aInstallingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* readonly attribute nsIServiceWorkerInfo waitingWorker; */
    pub GetWaitingWorker: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aWaitingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* readonly attribute nsIServiceWorkerInfo activeWorker; */
    pub GetActiveWorker: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aActiveWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID); */
    pub GetWorkerByID: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, aID: u64, _retval: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult,

    /* void addListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, listener: *const nsIServiceWorkerRegistrationInfoListener) -> ::nserror::nsresult,

    /* void removeListener (in nsIServiceWorkerRegistrationInfoListener listener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo, listener: *const nsIServiceWorkerRegistrationInfoListener) -> ::nserror::nsresult,

    /* void forceShutdown (); */
    pub ForceShutdown: unsafe extern "system" fn (this: *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerRegistrationInfo {

    pub const UPDATE_VIA_CACHE_IMPORTS: i64 = 0;


    pub const UPDATE_VIA_CACHE_ALL: i64 = 1;


    pub const UPDATE_VIA_CACHE_NONE: i64 = 2;


    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `readonly attribute AString scope;`
    #[inline]
    pub unsafe fn GetScope(&self, aScope: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetScope)(self, aScope)
    }



    /// `readonly attribute AString scriptSpec;`
    #[inline]
    pub unsafe fn GetScriptSpec(&self, aScriptSpec: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptSpec)(self, aScriptSpec)
    }



    /// `readonly attribute unsigned short updateViaCache;`
    #[inline]
    pub unsafe fn GetUpdateViaCache(&self, aUpdateViaCache: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateViaCache)(self, aUpdateViaCache)
    }



    /// `readonly attribute PRTime lastUpdateTime;`
    #[inline]
    pub unsafe fn GetLastUpdateTime(&self, aLastUpdateTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastUpdateTime)(self, aLastUpdateTime)
    }



    /// `readonly attribute nsIServiceWorkerInfo evaluatingWorker;`
    #[inline]
    pub unsafe fn GetEvaluatingWorker(&self, aEvaluatingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetEvaluatingWorker)(self, aEvaluatingWorker)
    }



    /// `readonly attribute nsIServiceWorkerInfo installingWorker;`
    #[inline]
    pub unsafe fn GetInstallingWorker(&self, aInstallingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetInstallingWorker)(self, aInstallingWorker)
    }



    /// `readonly attribute nsIServiceWorkerInfo waitingWorker;`
    #[inline]
    pub unsafe fn GetWaitingWorker(&self, aWaitingWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetWaitingWorker)(self, aWaitingWorker)
    }



    /// `readonly attribute nsIServiceWorkerInfo activeWorker;`
    #[inline]
    pub unsafe fn GetActiveWorker(&self, aActiveWorker: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveWorker)(self, aActiveWorker)
    }



    /// `nsIServiceWorkerInfo getWorkerByID (in unsigned long long aID);`
    #[inline]
    pub unsafe fn GetWorkerByID(&self, aID: u64, _retval: *mut *const nsIServiceWorkerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetWorkerByID)(self, aID, _retval)
    }



    /// `void addListener (in nsIServiceWorkerRegistrationInfoListener listener);`
    #[inline]
    pub unsafe fn AddListener(&self, listener: *const nsIServiceWorkerRegistrationInfoListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, listener)
    }



    /// `void removeListener (in nsIServiceWorkerRegistrationInfoListener listener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, listener: *const nsIServiceWorkerRegistrationInfoListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, listener)
    }



    /// `void forceShutdown ();`
    #[inline]
    pub unsafe fn ForceShutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ForceShutdown)(self, )
    }


}


/// `interface nsIServiceWorkerManagerListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerManagerListener {
    vtable: *const nsIServiceWorkerManagerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerManagerListener.
unsafe impl XpCom for nsIServiceWorkerManagerListener {
    const IID: nsIID = nsID(0x9e523e7c, 0xad6f, 0x4df0,
        [0x80, 0x77, 0xc7, 0x4a, 0xeb, 0xbc, 0x67, 0x9d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerManagerListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerManagerListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerManagerListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerManagerListener`.
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerManagerListenerCoerce for nsIServiceWorkerManagerListener {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self {
        v
    }
}

impl nsIServiceWorkerManagerListener {
    /// Cast this `nsIServiceWorkerManagerListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerManagerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerManagerListener {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerManagerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManagerListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerManagerListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerManagerListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onRegister (in nsIServiceWorkerRegistrationInfo aInfo); */
    pub OnRegister: unsafe extern "system" fn (this: *const nsIServiceWorkerManagerListener, aInfo: *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult,

    /* void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo); */
    pub OnUnregister: unsafe extern "system" fn (this: *const nsIServiceWorkerManagerListener, aInfo: *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerManagerListener {


    /// `void onRegister (in nsIServiceWorkerRegistrationInfo aInfo);`
    #[inline]
    pub unsafe fn OnRegister(&self, aInfo: *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnRegister)(self, aInfo)
    }



    /// `void onUnregister (in nsIServiceWorkerRegistrationInfo aInfo);`
    #[inline]
    pub unsafe fn OnUnregister(&self, aInfo: *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult {
        ((*self.vtable).OnUnregister)(self, aInfo)
    }


}


/// `interface nsIServiceWorkerManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceWorkerManager {
    vtable: *const nsIServiceWorkerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceWorkerManager.
unsafe impl XpCom for nsIServiceWorkerManager {
    const IID: nsIID = nsID(0x7404c8e8, 0x4d47, 0x4449,
        [0x8e, 0xd1, 0x47, 0xd1, 0x26, 0x1d, 0x4e, 0x33]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceWorkerManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceWorkerManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceWorkerManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceWorkerManager`.
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceWorkerManagerCoerce for nsIServiceWorkerManager {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self {
        v
    }
}

impl nsIServiceWorkerManager {
    /// Cast this `nsIServiceWorkerManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceWorkerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceWorkerManager {
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
impl<T: nsISupportsCoerce> nsIServiceWorkerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceWorkerManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceWorkerManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceWorkerManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] Promise registerForTest (in nsIPrincipal aPrincipal, in AString aScope, in AString aScriptURL); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub RegisterForTest: *const ::libc::c_void,

    /* [implicit_jscontext] Promise registerForAddonPrincipal (in nsIPrincipal aPrincipal); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub RegisterForAddonPrincipal: *const ::libc::c_void,

    /* void unregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope); */
    pub Unregister: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIServiceWorkerRegistrationInfo getRegistrationByPrincipal (in nsIPrincipal aPrincipal, in AString aScope); */
    pub GetRegistrationByPrincipal: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aScope: *const ::nsstring::nsAString, _retval: *mut *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] bool StartControlling (in const_ClientInfoRef aClientInfo, in const_ServiceWorkerDescriptorRef aServiceWorker); */
    /// Unable to generate binding because `native type const mozilla::dom::ClientInfo unsupported`
    pub StartControlling: *const ::libc::c_void,

    /* AString getScopeForUrl (in nsIPrincipal aPrincipal, in AString aPath); */
    pub GetScopeForUrl: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aPath: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsIArray getAllRegistrations (); */
    pub GetAllRegistrations: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, _retval: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void removeRegistrationsByOriginAttributes (in AString aOriginAttributes); */
    pub RemoveRegistrationsByOriginAttributes: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aOriginAttributes: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] void propagateSoftUpdate (in jsval aOriginAttributes, in AString aScope); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub PropagateSoftUpdate: *const ::libc::c_void,

    /* void propagateUnregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope); */
    pub PropagateUnregister: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void sendNotificationClickEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    pub SendNotificationClickEvent: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aOriginSuffix: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString, aID: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aBody: *const ::nsstring::nsAString, aTag: *const ::nsstring::nsAString, aIcon: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aBehavior: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void sendNotificationCloseEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior); */
    pub SendNotificationCloseEvent: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aOriginSuffix: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString, aID: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aBody: *const ::nsstring::nsAString, aTag: *const ::nsstring::nsAString, aIcon: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aBehavior: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [optional_argc] void sendPushEvent (in ACString aOriginAttributes, in ACString aScope, [optional] in Array<uint8_t> aDataBytes); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub SendPushEvent: *const ::libc::c_void,

    /* void sendPushSubscriptionChangeEvent (in ACString aOriginAttributes, in ACString scope); */
    pub SendPushSubscriptionChangeEvent: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aOriginAttributes: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void addListener (in nsIServiceWorkerManagerListener aListener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aListener: *const nsIServiceWorkerManagerListener) -> ::nserror::nsresult,

    /* void removeListener (in nsIServiceWorkerManagerListener aListener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, aListener: *const nsIServiceWorkerManagerListener) -> ::nserror::nsresult,

    /* bool isParentInterceptEnabled (); */
    pub IsParentInterceptEnabled: unsafe extern "system" fn (this: *const nsIServiceWorkerManager, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceWorkerManager {

    /// ```text
    /// /**
    ///    * A testing helper that registers a service worker for testing purpose (e.g. used to test
        ///    * a remote worker that has to spawn a new process to be launched).
    ///    * This method can only be used when "dom.serviceWorkers.testing.enabled" is true and
    ///    * it doesn't support all the registration options (e.g. updateViaCache is set automatically
        ///    * to "imports").
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise registerForTest (in nsIPrincipal aPrincipal, in AString aScope, in AString aScriptURL);`
    const _RegisterForTest: () = ();

    /// ```text
    /// /**
    ///    * Register an extension background service worker for a given
    ///    * extension principal and return a promise that resolves to the
    ///    * nsIServiceWorkerRegistrationInfo (or rejects if there was one
        ///    * already registered).
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise registerForAddonPrincipal (in nsIPrincipal aPrincipal);`
    const _RegisterForAddonPrincipal: () = ();

    /// ```text
    /// /**
    ///    * Unregister an existing ServiceWorker registration for `aScope`.
    ///    * It keeps aCallback alive until the operation is concluded.
    ///    */
    /// ```
    ///

    /// `void unregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope);`
    #[inline]
    pub unsafe fn Unregister(&self, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Unregister)(self, aPrincipal, aCallback, aScope)
    }



    /// `nsIServiceWorkerRegistrationInfo getRegistrationByPrincipal (in nsIPrincipal aPrincipal, in AString aScope);`
    #[inline]
    pub unsafe fn GetRegistrationByPrincipal(&self, aPrincipal: *const nsIPrincipal, aScope: *const ::nsstring::nsAString, _retval: *mut *const nsIServiceWorkerRegistrationInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetRegistrationByPrincipal)(self, aPrincipal, aScope, _retval)
    }



    /// `[nostdcall,notxpcom] bool StartControlling (in const_ClientInfoRef aClientInfo, in const_ServiceWorkerDescriptorRef aServiceWorker);`
    const _StartControlling: () = ();


    /// `AString getScopeForUrl (in nsIPrincipal aPrincipal, in AString aPath);`
    #[inline]
    pub unsafe fn GetScopeForUrl(&self, aPrincipal: *const nsIPrincipal, aPath: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetScopeForUrl)(self, aPrincipal, aPath, _retval)
    }



    /// `nsIArray getAllRegistrations ();`
    #[inline]
    pub unsafe fn GetAllRegistrations(&self, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetAllRegistrations)(self, _retval)
    }



    /// `void removeRegistrationsByOriginAttributes (in AString aOriginAttributes);`
    #[inline]
    pub unsafe fn RemoveRegistrationsByOriginAttributes(&self, aOriginAttributes: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveRegistrationsByOriginAttributes)(self, aOriginAttributes)
    }



    /// `[implicit_jscontext] void propagateSoftUpdate (in jsval aOriginAttributes, in AString aScope);`
    const _PropagateSoftUpdate: () = ();


    /// `void propagateUnregister (in nsIPrincipal aPrincipal, in nsIServiceWorkerUnregisterCallback aCallback, in AString aScope);`
    #[inline]
    pub unsafe fn PropagateUnregister(&self, aPrincipal: *const nsIPrincipal, aCallback: *const nsIServiceWorkerUnregisterCallback, aScope: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).PropagateUnregister)(self, aPrincipal, aCallback, aScope)
    }



    /// `void sendNotificationClickEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior);`
    #[inline]
    pub unsafe fn SendNotificationClickEvent(&self, aOriginSuffix: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString, aID: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aBody: *const ::nsstring::nsAString, aTag: *const ::nsstring::nsAString, aIcon: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aBehavior: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendNotificationClickEvent)(self, aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior)
    }



    /// `void sendNotificationCloseEvent (in ACString aOriginSuffix, in ACString scope, in AString aID, in AString aTitle, in AString aDir, in AString aLang, in AString aBody, in AString aTag, in AString aIcon, in AString aData, in AString aBehavior);`
    #[inline]
    pub unsafe fn SendNotificationCloseEvent(&self, aOriginSuffix: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString, aID: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aBody: *const ::nsstring::nsAString, aTag: *const ::nsstring::nsAString, aIcon: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aBehavior: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SendNotificationCloseEvent)(self, aOriginSuffix, scope, aID, aTitle, aDir, aLang, aBody, aTag, aIcon, aData, aBehavior)
    }



    /// `[optional_argc] void sendPushEvent (in ACString aOriginAttributes, in ACString aScope, [optional] in Array<uint8_t> aDataBytes);`
    const _SendPushEvent: () = ();


    /// `void sendPushSubscriptionChangeEvent (in ACString aOriginAttributes, in ACString scope);`
    #[inline]
    pub unsafe fn SendPushSubscriptionChangeEvent(&self, aOriginAttributes: *const ::nsstring::nsACString, scope: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendPushSubscriptionChangeEvent)(self, aOriginAttributes, scope)
    }



    /// `void addListener (in nsIServiceWorkerManagerListener aListener);`
    #[inline]
    pub unsafe fn AddListener(&self, aListener: *const nsIServiceWorkerManagerListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, aListener)
    }



    /// `void removeListener (in nsIServiceWorkerManagerListener aListener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, aListener: *const nsIServiceWorkerManagerListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, aListener)
    }



    /// `bool isParentInterceptEnabled ();`
    #[inline]
    pub unsafe fn IsParentInterceptEnabled(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsParentInterceptEnabled)(self, _retval)
    }


}



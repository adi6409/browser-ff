//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushService.idl
//


/// `interface nsIPushSubscription : nsISupports`
///

/// ```text
/// /**
///  * A push subscription, passed as an argument to a subscription callback.
///  * Similar to the `PushSubscription` WebIDL interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushSubscription {
    vtable: *const nsIPushSubscriptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushSubscription.
unsafe impl XpCom for nsIPushSubscription {
    const IID: nsIID = nsID(0x1de32d5c, 0xea88, 0x4c9e,
        [0x96, 0x26, 0xb0, 0x32, 0xbd, 0x87, 0xf4, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushSubscription {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushSubscription.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushSubscriptionCoerce {
    /// Cheaply cast a value of this type from a `nsIPushSubscription`.
    fn coerce_from(v: &nsIPushSubscription) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushSubscriptionCoerce for nsIPushSubscription {
    #[inline]
    fn coerce_from(v: &nsIPushSubscription) -> &Self {
        v
    }
}

impl nsIPushSubscription {
    /// Cast this `nsIPushSubscription` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushSubscriptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushSubscription {
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
impl<T: nsISupportsCoerce> nsIPushSubscriptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushSubscription) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushSubscription
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushSubscriptionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString endpoint; */
    pub GetEndpoint: unsafe extern "system" fn (this: *const nsIPushSubscription, aEndpoint: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute long long pushCount; */
    pub GetPushCount: unsafe extern "system" fn (this: *const nsIPushSubscription, aPushCount: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long lastPush; */
    pub GetLastPush: unsafe extern "system" fn (this: *const nsIPushSubscription, aLastPush: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long quota; */
    pub GetQuota: unsafe extern "system" fn (this: *const nsIPushSubscription, aQuota: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute bool isSystemSubscription; */
    pub GetIsSystemSubscription: unsafe extern "system" fn (this: *const nsIPushSubscription, aIsSystemSubscription: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute jsval p256dhPrivateKey; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetP256dhPrivateKey: *const ::libc::c_void,

    /* bool quotaApplies (); */
    pub QuotaApplies: unsafe extern "system" fn (this: *const nsIPushSubscription, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool isExpired (); */
    pub IsExpired: unsafe extern "system" fn (this: *const nsIPushSubscription, _retval: *mut bool) -> ::nserror::nsresult,

    /* Array<uint8_t> getKey (in AString name); */
    pub GetKey: unsafe extern "system" fn (this: *const nsIPushSubscription, name: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushSubscription {


    /// `readonly attribute AString endpoint;`
    #[inline]
    pub unsafe fn GetEndpoint(&self, aEndpoint: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetEndpoint)(self, aEndpoint)
    }



    /// `readonly attribute long long pushCount;`
    #[inline]
    pub unsafe fn GetPushCount(&self, aPushCount: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetPushCount)(self, aPushCount)
    }



    /// `readonly attribute long long lastPush;`
    #[inline]
    pub unsafe fn GetLastPush(&self, aLastPush: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastPush)(self, aLastPush)
    }



    /// `readonly attribute long quota;`
    #[inline]
    pub unsafe fn GetQuota(&self, aQuota: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetQuota)(self, aQuota)
    }



    /// `readonly attribute bool isSystemSubscription;`
    #[inline]
    pub unsafe fn GetIsSystemSubscription(&self, aIsSystemSubscription: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSystemSubscription)(self, aIsSystemSubscription)
    }



    /// `readonly attribute jsval p256dhPrivateKey;`
    const _GetP256dhPrivateKey: () = ();


    /// `bool quotaApplies ();`
    #[inline]
    pub unsafe fn QuotaApplies(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).QuotaApplies)(self, _retval)
    }



    /// `bool isExpired ();`
    #[inline]
    pub unsafe fn IsExpired(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsExpired)(self, _retval)
    }



    /// `Array<uint8_t> getKey (in AString name);`
    #[inline]
    pub unsafe fn GetKey(&self, name: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetKey)(self, name, _retval)
    }


}


/// `interface nsIPushSubscriptionCallback : nsISupports`
///

/// ```text
/// /**
///  * Called by methods that return a push subscription. A non-success
///  * |status| indicates that there was a problem returning the
///  * subscription, and the |subscription| argument should be ignored. Otherwise,
///  * |subscription| will point to a valid push subscription, or |null| if the
///  * subscription does not exist.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushSubscriptionCallback {
    vtable: *const nsIPushSubscriptionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushSubscriptionCallback.
unsafe impl XpCom for nsIPushSubscriptionCallback {
    const IID: nsIID = nsID(0x1799c074, 0x9d52, 0x46b0,
        [0xab, 0x3c, 0xc0, 0x97, 0x90, 0x73, 0x2f, 0x6f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushSubscriptionCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushSubscriptionCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushSubscriptionCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIPushSubscriptionCallback`.
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushSubscriptionCallbackCoerce for nsIPushSubscriptionCallback {
    #[inline]
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self {
        v
    }
}

impl nsIPushSubscriptionCallback {
    /// Cast this `nsIPushSubscriptionCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushSubscriptionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushSubscriptionCallback {
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
impl<T: nsISupportsCoerce> nsIPushSubscriptionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushSubscriptionCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushSubscriptionCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushSubscriptionCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPushSubscription (in nsresult status, in nsIPushSubscription subscription); */
    pub OnPushSubscription: unsafe extern "system" fn (this: *const nsIPushSubscriptionCallback, status: ::nserror::nsresult, subscription: *const nsIPushSubscription) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushSubscriptionCallback {


    /// `void onPushSubscription (in nsresult status, in nsIPushSubscription subscription);`
    #[inline]
    pub unsafe fn OnPushSubscription(&self, status: ::nserror::nsresult, subscription: *const nsIPushSubscription) -> ::nserror::nsresult {
        ((*self.vtable).OnPushSubscription)(self, status, subscription)
    }


}


/// `interface nsIUnsubscribeResultCallback : nsISupports`
///

/// ```text
/// /**
///  * Called by |unsubscribe|. A non-success |status| indicates that there was
///  * a problem unsubscribing, and the |success| argument should be ignored.
///  * Otherwise, |success| is true if unsubscribing was successful, and false if
///  * the subscription does not exist.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUnsubscribeResultCallback {
    vtable: *const nsIUnsubscribeResultCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUnsubscribeResultCallback.
unsafe impl XpCom for nsIUnsubscribeResultCallback {
    const IID: nsIID = nsID(0xd574118f, 0x61a9, 0x4270,
        [0xb1, 0xf6, 0x44, 0x61, 0xaa, 0x85, 0xc4, 0xf5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUnsubscribeResultCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUnsubscribeResultCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUnsubscribeResultCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUnsubscribeResultCallback`.
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUnsubscribeResultCallbackCoerce for nsIUnsubscribeResultCallback {
    #[inline]
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self {
        v
    }
}

impl nsIUnsubscribeResultCallback {
    /// Cast this `nsIUnsubscribeResultCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUnsubscribeResultCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUnsubscribeResultCallback {
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
impl<T: nsISupportsCoerce> nsIUnsubscribeResultCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnsubscribeResultCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUnsubscribeResultCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUnsubscribeResultCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onUnsubscribe (in nsresult status, in bool success); */
    pub OnUnsubscribe: unsafe extern "system" fn (this: *const nsIUnsubscribeResultCallback, status: ::nserror::nsresult, success: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUnsubscribeResultCallback {


    /// `void onUnsubscribe (in nsresult status, in bool success);`
    #[inline]
    pub unsafe fn OnUnsubscribe(&self, status: ::nserror::nsresult, success: bool) -> ::nserror::nsresult {
        ((*self.vtable).OnUnsubscribe)(self, status, success)
    }


}


/// `interface nsIPushClearResultCallback : nsISupports`
///

/// ```text
/// /**
///  * Called by |clearForDomain|. A non-success |status| indicates that there was
///  * a problem clearing subscriptions for the given domain.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushClearResultCallback {
    vtable: *const nsIPushClearResultCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushClearResultCallback.
unsafe impl XpCom for nsIPushClearResultCallback {
    const IID: nsIID = nsID(0xbd47b38e, 0x8bfa, 0x4f92,
        [0x83, 0x4e, 0x83, 0x2a, 0x44, 0x31, 0xe0, 0x5e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushClearResultCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushClearResultCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushClearResultCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIPushClearResultCallback`.
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushClearResultCallbackCoerce for nsIPushClearResultCallback {
    #[inline]
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self {
        v
    }
}

impl nsIPushClearResultCallback {
    /// Cast this `nsIPushClearResultCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushClearResultCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushClearResultCallback {
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
impl<T: nsISupportsCoerce> nsIPushClearResultCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushClearResultCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushClearResultCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushClearResultCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onClear (in nsresult status); */
    pub OnClear: unsafe extern "system" fn (this: *const nsIPushClearResultCallback, status: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushClearResultCallback {


    /// `void onClear (in nsresult status);`
    #[inline]
    pub unsafe fn OnClear(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnClear)(self, status)
    }


}


/// `interface nsIPushService : nsISupports`
///

/// ```text
/// /**
///  * A service for components to subscribe and receive push messages from web
///  * services. This functionality is exposed to content via the Push DOM API,
///  * which uses service workers. This interface exists to support the DOM API,
///  * and allows privileged code to receive messages without migrating to service
///  * workers.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushService {
    vtable: *const nsIPushServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushService.
unsafe impl XpCom for nsIPushService {
    const IID: nsIID = nsID(0x678ef584, 0xbf25, 0x47aa,
        [0xac, 0x84, 0x03, 0xef, 0xc0, 0x86, 0x5b, 0x68]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPushService`.
    fn coerce_from(v: &nsIPushService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushServiceCoerce for nsIPushService {
    #[inline]
    fn coerce_from(v: &nsIPushService) -> &Self {
        v
    }
}

impl nsIPushService {
    /// Cast this `nsIPushService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushService {
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
impl<T: nsISupportsCoerce> nsIPushServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString pushTopic; */
    pub GetPushTopic: unsafe extern "system" fn (this: *const nsIPushService, aPushTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString subscriptionChangeTopic; */
    pub GetSubscriptionChangeTopic: unsafe extern "system" fn (this: *const nsIPushService, aSubscriptionChangeTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString subscriptionModifiedTopic; */
    pub GetSubscriptionModifiedTopic: unsafe extern "system" fn (this: *const nsIPushService, aSubscriptionModifiedTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void subscribe (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    pub Subscribe: unsafe extern "system" fn (this: *const nsIPushService, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult,

    /* void subscribeWithKey (in AString scope, in nsIPrincipal principal, in Array<uint8_t> key, in nsIPushSubscriptionCallback callback); */
    pub SubscribeWithKey: unsafe extern "system" fn (this: *const nsIPushService, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, key: *const thin_vec::ThinVec<uint8_t>, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult,

    /* void unsubscribe (in AString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback); */
    pub Unsubscribe: unsafe extern "system" fn (this: *const nsIPushService, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIUnsubscribeResultCallback) -> ::nserror::nsresult,

    /* void getSubscription (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback); */
    pub GetSubscription: unsafe extern "system" fn (this: *const nsIPushService, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult,

    /* void clearForDomain (in AString domain, in nsIPushClearResultCallback callback); */
    pub ClearForDomain: unsafe extern "system" fn (this: *const nsIPushService, domain: *const ::nsstring::nsAString, callback: *const nsIPushClearResultCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushService {

    /// ```text
    /// /** Observer topic names, exported for convenience. */
    /// ```
    ///

    /// `readonly attribute AString pushTopic;`
    #[inline]
    pub unsafe fn GetPushTopic(&self, aPushTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPushTopic)(self, aPushTopic)
    }



    /// `readonly attribute AString subscriptionChangeTopic;`
    #[inline]
    pub unsafe fn GetSubscriptionChangeTopic(&self, aSubscriptionChangeTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubscriptionChangeTopic)(self, aSubscriptionChangeTopic)
    }



    /// `readonly attribute AString subscriptionModifiedTopic;`
    #[inline]
    pub unsafe fn GetSubscriptionModifiedTopic(&self, aSubscriptionModifiedTopic: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSubscriptionModifiedTopic)(self, aSubscriptionModifiedTopic)
    }


    /// ```text
    /// /**
    ///    * Creates a push subscription for the given |scope| URL and |principal|.
    ///    * If a subscription already exists for this |(scope, principal)| pair,
    ///    * the callback will receive the existing record as the second argument.
    ///    *
    ///    * The |endpoint| property of the subscription record is a URL string
    ///    * that can be used to send push messages to subscribers.
    ///    *
    ///    * Each incoming message fires a `push-message` observer notification, with
    ///    * an `nsIPushMessage` as the subject and the |scope| as the data.
    ///    *
    ///    * If the server drops a subscription, a `push-subscription-change` observer
    ///    * will be fired, with the subject set to |principal| and the data set to
    ///    * |scope|. Servers may drop subscriptions at any time, so callers should
    ///    * recreate subscriptions if desired.
    ///    */
    /// ```
    ///

    /// `void subscribe (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback);`
    #[inline]
    pub unsafe fn Subscribe(&self, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult {
        ((*self.vtable).Subscribe)(self, scope, principal, callback)
    }


    /// ```text
    /// /**
    ///    * Creates a restricted push subscription with the given public |key|. The
    ///    * application server must use the corresponding private key to authenticate
    ///    * message delivery requests, as described in draft-thomson-webpush-vapid.
    ///    */
    /// ```
    ///

    /// `void subscribeWithKey (in AString scope, in nsIPrincipal principal, in Array<uint8_t> key, in nsIPushSubscriptionCallback callback);`
    #[inline]
    pub unsafe fn SubscribeWithKey(&self, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, key: *const thin_vec::ThinVec<uint8_t>, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult {
        ((*self.vtable).SubscribeWithKey)(self, scope, principal, key, callback)
    }


    /// ```text
    /// /**
    ///    * Removes a push subscription for the given |scope|.
    ///    */
    /// ```
    ///

    /// `void unsubscribe (in AString scope, in nsIPrincipal principal, in nsIUnsubscribeResultCallback callback);`
    #[inline]
    pub unsafe fn Unsubscribe(&self, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIUnsubscribeResultCallback) -> ::nserror::nsresult {
        ((*self.vtable).Unsubscribe)(self, scope, principal, callback)
    }


    /// ```text
    /// /**
    ///    * Retrieves the subscription record associated with the given
    ///    * |(scope, principal)| pair. If the subscription does not exist, the
    ///    * callback will receive |null| as the second argument.
    ///    */
    /// ```
    ///

    /// `void getSubscription (in AString scope, in nsIPrincipal principal, in nsIPushSubscriptionCallback callback);`
    #[inline]
    pub unsafe fn GetSubscription(&self, scope: *const ::nsstring::nsAString, principal: *const nsIPrincipal, callback: *const nsIPushSubscriptionCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetSubscription)(self, scope, principal, callback)
    }


    /// ```text
    /// /**
    ///    * Drops every subscription for the given |domain|, or all domains if
    ///    * |domain| is "*".
    ///    */
    /// ```
    ///

    /// `void clearForDomain (in AString domain, in nsIPushClearResultCallback callback);`
    #[inline]
    pub unsafe fn ClearForDomain(&self, domain: *const ::nsstring::nsAString, callback: *const nsIPushClearResultCallback) -> ::nserror::nsresult {
        ((*self.vtable).ClearForDomain)(self, domain, callback)
    }


}


/// `interface nsIPushQuotaManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushQuotaManager {
    vtable: *const nsIPushQuotaManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushQuotaManager.
unsafe impl XpCom for nsIPushQuotaManager {
    const IID: nsIID = nsID(0xa2555e70, 0x46f8, 0x4b52,
        [0xbf, 0x02, 0xd9, 0x78, 0xb9, 0x79, 0xd1, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushQuotaManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushQuotaManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushQuotaManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIPushQuotaManager`.
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushQuotaManagerCoerce for nsIPushQuotaManager {
    #[inline]
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self {
        v
    }
}

impl nsIPushQuotaManager {
    /// Cast this `nsIPushQuotaManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushQuotaManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushQuotaManager {
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
impl<T: nsISupportsCoerce> nsIPushQuotaManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushQuotaManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushQuotaManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushQuotaManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notificationForOriginShown (in string origin); */
    pub NotificationForOriginShown: unsafe extern "system" fn (this: *const nsIPushQuotaManager, origin: *const libc::c_char) -> ::nserror::nsresult,

    /* void notificationForOriginClosed (in string origin); */
    pub NotificationForOriginClosed: unsafe extern "system" fn (this: *const nsIPushQuotaManager, origin: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushQuotaManager {

    /// ```text
    /// /**
    ///    * Informs the quota manager that a notification
    ///    * for the given origin has been shown. Used to
    ///    * determine if push quota should be relaxed.
    ///    */
    /// ```
    ///

    /// `void notificationForOriginShown (in string origin);`
    #[inline]
    pub unsafe fn NotificationForOriginShown(&self, origin: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).NotificationForOriginShown)(self, origin)
    }


    /// ```text
    /// /**
    ///    * Informs the quota manager that a notification
    ///    * for the given origin has been closed. Used to
    ///    * determine if push quota should be relaxed.
    ///    */
    /// ```
    ///

    /// `void notificationForOriginClosed (in string origin);`
    #[inline]
    pub unsafe fn NotificationForOriginClosed(&self, origin: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).NotificationForOriginClosed)(self, origin)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsILocalCertService.idl
//


/// `interface nsILocalCertService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalCertService {
    vtable: *const nsILocalCertServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalCertService.
unsafe impl XpCom for nsILocalCertService {
    const IID: nsIID = nsID(0x9702fdd4, 0x4c2c, 0x439c,
        [0xba, 0x2e, 0x19, 0xcd, 0xa0, 0x18, 0xeb, 0x99]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalCertService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalCertService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalCertServiceCoerce {
    /// Cheaply cast a value of this type from a `nsILocalCertService`.
    fn coerce_from(v: &nsILocalCertService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalCertServiceCoerce for nsILocalCertService {
    #[inline]
    fn coerce_from(v: &nsILocalCertService) -> &Self {
        v
    }
}

impl nsILocalCertService {
    /// Cast this `nsILocalCertService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalCertServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalCertService {
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
impl<T: nsISupportsCoerce> nsILocalCertServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalCertService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalCertServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb); */
    pub GetOrCreateCert: unsafe extern "system" fn (this: *const nsILocalCertService, nickname: *const ::nsstring::nsACString, cb: *const nsILocalCertGetCallback) -> ::nserror::nsresult,

    /* [must_use] void removeCert (in ACString nickname, in nsILocalCertCallback cb); */
    pub RemoveCert: unsafe extern "system" fn (this: *const nsILocalCertService, nickname: *const ::nsstring::nsACString, cb: *const nsILocalCertCallback) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean loginPromptRequired; */
    pub GetLoginPromptRequired: unsafe extern "system" fn (this: *const nsILocalCertService, aLoginPromptRequired: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalCertService {

    /// ```text
    /// /**
    ///    * Get or create a new self-signed X.509 cert to represent this device over a
    ///    * secure transport, like TLS.
    ///    *
    ///    * The cert is stored permanently in the profile's key store after first use,
    ///    * and is valid for 1 year.  If an expired or otherwise invalid cert is found
    ///    * with the nickname supplied here, it is removed and a new one is made.
    ///    *
    ///    * @param nickname Nickname that identifies the cert
    ///    * @param cb       Callback to be notified with the result
    ///    */
    /// ```
    ///

    /// `[must_use] void getOrCreateCert (in ACString nickname, in nsILocalCertGetCallback cb);`
    #[inline]
    pub unsafe fn GetOrCreateCert(&self, nickname: *const ::nsstring::nsACString, cb: *const nsILocalCertGetCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetOrCreateCert)(self, nickname, cb)
    }


    /// ```text
    /// /**
    ///    * Remove a X.509 cert with the given nickname.
    ///    *
    ///    * @param nickname Nickname that identifies the cert
    ///    * @param cb       Callback to be notified with the result
    ///    */
    /// ```
    ///

    /// `[must_use] void removeCert (in ACString nickname, in nsILocalCertCallback cb);`
    #[inline]
    pub unsafe fn RemoveCert(&self, nickname: *const ::nsstring::nsACString, cb: *const nsILocalCertCallback) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCert)(self, nickname, cb)
    }


    /// ```text
    /// /**
    ///    * Whether calling |getOrCreateCert| or |removeCert| will trigger a login
    ///    * prompt to be displayed.  Generally this happens if the user has set a
    ///    * master password, but has not yet logged in.
    ///    */
    /// ```
    ///

    /// `[must_use] readonly attribute boolean loginPromptRequired;`
    #[inline]
    pub unsafe fn GetLoginPromptRequired(&self, aLoginPromptRequired: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetLoginPromptRequired)(self, aLoginPromptRequired)
    }


}


/// `interface nsILocalCertGetCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalCertGetCallback {
    vtable: *const nsILocalCertGetCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalCertGetCallback.
unsafe impl XpCom for nsILocalCertGetCallback {
    const IID: nsIID = nsID(0xcc09633e, 0x7c70, 0x4093,
        [0xa9, 0xcf, 0x79, 0xab, 0x67, 0x6c, 0xa8, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalCertGetCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalCertGetCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalCertGetCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsILocalCertGetCallback`.
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalCertGetCallbackCoerce for nsILocalCertGetCallback {
    #[inline]
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self {
        v
    }
}

impl nsILocalCertGetCallback {
    /// Cast this `nsILocalCertGetCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalCertGetCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalCertGetCallback {
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
impl<T: nsISupportsCoerce> nsILocalCertGetCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertGetCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalCertGetCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalCertGetCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleCert (in nsIX509Cert cert, in nsresult result); */
    pub HandleCert: unsafe extern "system" fn (this: *const nsILocalCertGetCallback, cert: *const nsIX509Cert, result: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalCertGetCallback {

    /// ```text
    /// /**
    ///    * Called with the result of the getOrCreateCert operation above.
    ///    *
    ///    * @param cert   Requested cert, or null if some error
    ///    * @param result Result code from the get operation
    ///    */
    /// ```
    ///

    /// `void handleCert (in nsIX509Cert cert, in nsresult result);`
    #[inline]
    pub unsafe fn HandleCert(&self, cert: *const nsIX509Cert, result: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).HandleCert)(self, cert, result)
    }


}


/// `interface nsILocalCertCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalCertCallback {
    vtable: *const nsILocalCertCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalCertCallback.
unsafe impl XpCom for nsILocalCertCallback {
    const IID: nsIID = nsID(0x518124e9, 0x55e6, 0x4e23,
        [0x97, 0xc0, 0x49, 0x95, 0xb3, 0xa1, 0xbe, 0xc6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalCertCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalCertCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalCertCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsILocalCertCallback`.
    fn coerce_from(v: &nsILocalCertCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalCertCallbackCoerce for nsILocalCertCallback {
    #[inline]
    fn coerce_from(v: &nsILocalCertCallback) -> &Self {
        v
    }
}

impl nsILocalCertCallback {
    /// Cast this `nsILocalCertCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalCertCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalCertCallback {
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
impl<T: nsISupportsCoerce> nsILocalCertCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalCertCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalCertCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalCertCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleResult (in nsresult result); */
    pub HandleResult: unsafe extern "system" fn (this: *const nsILocalCertCallback, result: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalCertCallback {

    /// ```text
    /// /**
    ///    * Called with the result of the removeCert operation above.
    ///    *
    ///    * @param result Result code from the operation
    ///    */
    /// ```
    ///

    /// `void handleResult (in nsresult result);`
    #[inline]
    pub unsafe fn HandleResult(&self, result: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).HandleResult)(self, result)
    }


}



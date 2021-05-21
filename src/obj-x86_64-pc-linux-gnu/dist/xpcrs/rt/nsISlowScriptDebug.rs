//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsISlowScriptDebug.idl
//


/// `interface nsISlowScriptDebugCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISlowScriptDebugCallback {
    vtable: *const nsISlowScriptDebugCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISlowScriptDebugCallback.
unsafe impl XpCom for nsISlowScriptDebugCallback {
    const IID: nsIID = nsID(0xf7dbb80c, 0x5d1e, 0x4fd9,
        [0xb5, 0x5c, 0xa9, 0xff, 0xda, 0x4a, 0x75, 0xb1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISlowScriptDebugCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISlowScriptDebugCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISlowScriptDebugCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISlowScriptDebugCallback`.
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISlowScriptDebugCallbackCoerce for nsISlowScriptDebugCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebugCallback {
    /// Cast this `nsISlowScriptDebugCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISlowScriptDebugCallback {
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
impl<T: nsISupportsCoerce> nsISlowScriptDebugCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISlowScriptDebugCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISlowScriptDebugCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSlowScriptDebug (in nsIDOMWindow aWindow); */
    pub HandleSlowScriptDebug: unsafe extern "system" fn (this: *const nsISlowScriptDebugCallback, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISlowScriptDebugCallback {


    /// `void handleSlowScriptDebug (in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn HandleSlowScriptDebug(&self, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).HandleSlowScriptDebug)(self, aWindow)
    }


}


/// `interface nsISlowScriptDebuggerStartupCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISlowScriptDebuggerStartupCallback {
    vtable: *const nsISlowScriptDebuggerStartupCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISlowScriptDebuggerStartupCallback.
unsafe impl XpCom for nsISlowScriptDebuggerStartupCallback {
    const IID: nsIID = nsID(0xb1c6ecd0, 0x8fa4, 0x11e4,
        [0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISlowScriptDebuggerStartupCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISlowScriptDebuggerStartupCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISlowScriptDebuggerStartupCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISlowScriptDebuggerStartupCallback`.
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISlowScriptDebuggerStartupCallbackCoerce for nsISlowScriptDebuggerStartupCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebuggerStartupCallback {
    /// Cast this `nsISlowScriptDebuggerStartupCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISlowScriptDebuggerStartupCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISlowScriptDebuggerStartupCallback {
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
impl<T: nsISupportsCoerce> nsISlowScriptDebuggerStartupCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebuggerStartupCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISlowScriptDebuggerStartupCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISlowScriptDebuggerStartupCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void finishDebuggerStartup (); */
    pub FinishDebuggerStartup: unsafe extern "system" fn (this: *const nsISlowScriptDebuggerStartupCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISlowScriptDebuggerStartupCallback {


    /// `void finishDebuggerStartup ();`
    #[inline]
    pub unsafe fn FinishDebuggerStartup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FinishDebuggerStartup)(self, )
    }


}


/// `interface nsISlowScriptDebugRemoteCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISlowScriptDebugRemoteCallback {
    vtable: *const nsISlowScriptDebugRemoteCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISlowScriptDebugRemoteCallback.
unsafe impl XpCom for nsISlowScriptDebugRemoteCallback {
    const IID: nsIID = nsID(0xdbee14b0, 0x8fa0, 0x11e4,
        [0xb4, 0xa9, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISlowScriptDebugRemoteCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISlowScriptDebugRemoteCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISlowScriptDebugRemoteCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISlowScriptDebugRemoteCallback`.
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISlowScriptDebugRemoteCallbackCoerce for nsISlowScriptDebugRemoteCallback {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self {
        v
    }
}

impl nsISlowScriptDebugRemoteCallback {
    /// Cast this `nsISlowScriptDebugRemoteCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugRemoteCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISlowScriptDebugRemoteCallback {
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
impl<T: nsISupportsCoerce> nsISlowScriptDebugRemoteCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebugRemoteCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISlowScriptDebugRemoteCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISlowScriptDebugRemoteCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleSlowScriptDebug (in EventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback); */
    pub HandleSlowScriptDebug: unsafe extern "system" fn (this: *const nsISlowScriptDebugRemoteCallback, aBrowser: *const libc::c_void, aCallback: *const nsISlowScriptDebuggerStartupCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISlowScriptDebugRemoteCallback {


    /// `void handleSlowScriptDebug (in EventTarget aBrowser, in nsISlowScriptDebuggerStartupCallback aCallback);`
    #[inline]
    pub unsafe fn HandleSlowScriptDebug(&self, aBrowser: *const libc::c_void, aCallback: *const nsISlowScriptDebuggerStartupCallback) -> ::nserror::nsresult {
        ((*self.vtable).HandleSlowScriptDebug)(self, aBrowser, aCallback)
    }


}


/// `interface nsISlowScriptDebug : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISlowScriptDebug {
    vtable: *const nsISlowScriptDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISlowScriptDebug.
unsafe impl XpCom for nsISlowScriptDebug {
    const IID: nsIID = nsID(0xf75d4164, 0x3aa7, 0x4395,
        [0xba, 0x44, 0xa5, 0xf9, 0x5b, 0x2e, 0x84, 0x27]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISlowScriptDebug {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISlowScriptDebug.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISlowScriptDebugCoerce {
    /// Cheaply cast a value of this type from a `nsISlowScriptDebug`.
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISlowScriptDebugCoerce for nsISlowScriptDebug {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self {
        v
    }
}

impl nsISlowScriptDebug {
    /// Cast this `nsISlowScriptDebug` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISlowScriptDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISlowScriptDebug {
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
impl<T: nsISupportsCoerce> nsISlowScriptDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISlowScriptDebug) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISlowScriptDebug
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISlowScriptDebugVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsISlowScriptDebugCallback activationHandler; */
    pub GetActivationHandler: unsafe extern "system" fn (this: *const nsISlowScriptDebug, aActivationHandler: *mut *const nsISlowScriptDebugCallback) -> ::nserror::nsresult,

    /* attribute nsISlowScriptDebugCallback activationHandler; */
    pub SetActivationHandler: unsafe extern "system" fn (this: *const nsISlowScriptDebug, aActivationHandler: *const nsISlowScriptDebugCallback) -> ::nserror::nsresult,

    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
    pub GetRemoteActivationHandler: unsafe extern "system" fn (this: *const nsISlowScriptDebug, aRemoteActivationHandler: *mut *const nsISlowScriptDebugRemoteCallback) -> ::nserror::nsresult,

    /* attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler; */
    pub SetRemoteActivationHandler: unsafe extern "system" fn (this: *const nsISlowScriptDebug, aRemoteActivationHandler: *const nsISlowScriptDebugRemoteCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISlowScriptDebug {


    /// `attribute nsISlowScriptDebugCallback activationHandler;`
    #[inline]
    pub unsafe fn GetActivationHandler(&self, aActivationHandler: *mut *const nsISlowScriptDebugCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetActivationHandler)(self, aActivationHandler)
    }



    /// `attribute nsISlowScriptDebugCallback activationHandler;`
    #[inline]
    pub unsafe fn SetActivationHandler(&self, aActivationHandler: *const nsISlowScriptDebugCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetActivationHandler)(self, aActivationHandler)
    }



    /// `attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler;`
    #[inline]
    pub unsafe fn GetRemoteActivationHandler(&self, aRemoteActivationHandler: *mut *const nsISlowScriptDebugRemoteCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteActivationHandler)(self, aRemoteActivationHandler)
    }



    /// `attribute nsISlowScriptDebugRemoteCallback remoteActivationHandler;`
    #[inline]
    pub unsafe fn SetRemoteActivationHandler(&self, aRemoteActivationHandler: *const nsISlowScriptDebugRemoteCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetRemoteActivationHandler)(self, aRemoteActivationHandler)
    }


}



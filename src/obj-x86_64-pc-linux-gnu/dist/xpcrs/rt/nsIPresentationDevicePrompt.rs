//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationDevicePrompt.idl
//


/// `interface nsIPresentationDeviceRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDeviceRequest {
    vtable: *const nsIPresentationDeviceRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDeviceRequest.
unsafe impl XpCom for nsIPresentationDeviceRequest {
    const IID: nsIID = nsID(0xb2aa7f6a, 0x9448, 0x446a,
        [0xbb, 0xa4, 0x9c, 0x29, 0x63, 0x8b, 0x0e, 0xd4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDeviceRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDeviceRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDeviceRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDeviceRequest`.
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDeviceRequestCoerce for nsIPresentationDeviceRequest {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self {
        v
    }
}

impl nsIPresentationDeviceRequest {
    /// Cast this `nsIPresentationDeviceRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDeviceRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDeviceRequest {
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
impl<T: nsISupportsCoerce> nsIPresentationDeviceRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDeviceRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDeviceRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDeviceRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString origin; */
    pub GetOrigin: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, aOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray requestURLs; */
    pub GetRequestURLs: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, aRequestURLs: *mut*const nsIArray) -> ::nserror::nsresult,

    /* readonly attribute EventTarget chromeEventHandler; */
    pub GetChromeEventHandler: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, aChromeEventHandler: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* void select (in nsIPresentationDevice device); */
    pub Select: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, device: *const nsIPresentationDevice) -> ::nserror::nsresult,

    /* void cancel (in nsresult reason); */
    pub Cancel: unsafe extern "system" fn (this: *const nsIPresentationDeviceRequest, reason: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDeviceRequest {


    /// `readonly attribute AString origin;`
    #[inline]
    pub unsafe fn GetOrigin(&self, aOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrigin)(self, aOrigin)
    }



    /// `readonly attribute nsIArray requestURLs;`
    #[inline]
    pub unsafe fn GetRequestURLs(&self, aRequestURLs: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestURLs)(self, aRequestURLs)
    }



    /// `readonly attribute EventTarget chromeEventHandler;`
    #[inline]
    pub unsafe fn GetChromeEventHandler(&self, aChromeEventHandler: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeEventHandler)(self, aChromeEventHandler)
    }



    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `void select (in nsIPresentationDevice device);`
    #[inline]
    pub unsafe fn Select(&self, device: *const nsIPresentationDevice) -> ::nserror::nsresult {
        ((*self.vtable).Select)(self, device)
    }



    /// `void cancel (in nsresult reason);`
    #[inline]
    pub unsafe fn Cancel(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, reason)
    }


}


/// `interface nsIPresentationDevicePrompt : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationDevicePrompt {
    vtable: *const nsIPresentationDevicePromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationDevicePrompt.
unsafe impl XpCom for nsIPresentationDevicePrompt {
    const IID: nsIID = nsID(0xac1a7e44, 0xde86, 0x454f,
        [0xa9, 0xf1, 0x27, 0x6d, 0xe2, 0x53, 0x98, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationDevicePrompt {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationDevicePrompt.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationDevicePromptCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationDevicePrompt`.
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationDevicePromptCoerce for nsIPresentationDevicePrompt {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self {
        v
    }
}

impl nsIPresentationDevicePrompt {
    /// Cast this `nsIPresentationDevicePrompt` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationDevicePromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationDevicePrompt {
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
impl<T: nsISupportsCoerce> nsIPresentationDevicePromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationDevicePrompt) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationDevicePrompt
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationDevicePromptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void promptDeviceSelection (in nsIPresentationDeviceRequest request); */
    pub PromptDeviceSelection: unsafe extern "system" fn (this: *const nsIPresentationDevicePrompt, request: *const nsIPresentationDeviceRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationDevicePrompt {


    /// `void promptDeviceSelection (in nsIPresentationDeviceRequest request);`
    #[inline]
    pub unsafe fn PromptDeviceSelection(&self, request: *const nsIPresentationDeviceRequest) -> ::nserror::nsresult {
        ((*self.vtable).PromptDeviceSelection)(self, request)
    }


}



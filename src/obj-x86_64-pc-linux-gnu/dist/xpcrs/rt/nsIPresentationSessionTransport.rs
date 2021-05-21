//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/presentation/interfaces/nsIPresentationSessionTransport.idl
//


/// `interface nsIPresentationSessionTransportCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionTransportCallback {
    vtable: *const nsIPresentationSessionTransportCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionTransportCallback.
unsafe impl XpCom for nsIPresentationSessionTransportCallback {
    const IID: nsIID = nsID(0x9f158786, 0x41a6, 0x4a10,
        [0xb2, 0x9b, 0x94, 0x97, 0xf2, 0x5d, 0x4b, 0x67]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionTransportCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionTransportCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionTransportCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionTransportCallback`.
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionTransportCallbackCoerce for nsIPresentationSessionTransportCallback {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransportCallback {
    /// Cast this `nsIPresentationSessionTransportCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionTransportCallback {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransportCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionTransportCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionTransportCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifyTransportReady (); */
    pub NotifyTransportReady: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportCallback) -> ::nserror::nsresult,

    /* void notifyTransportClosed (in nsresult reason); */
    pub NotifyTransportClosed: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportCallback, reason: ::nserror::nsresult) -> ::nserror::nsresult,

    /* void notifyData (in ACString data, in boolean isBinary); */
    pub NotifyData: unsafe extern "system" fn (this: *const nsIPresentationSessionTransportCallback, data: *const ::nsstring::nsACString, isBinary: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionTransportCallback {


    /// `void notifyTransportReady ();`
    #[inline]
    pub unsafe fn NotifyTransportReady(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyTransportReady)(self, )
    }



    /// `void notifyTransportClosed (in nsresult reason);`
    #[inline]
    pub unsafe fn NotifyTransportClosed(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).NotifyTransportClosed)(self, reason)
    }



    /// `void notifyData (in ACString data, in boolean isBinary);`
    #[inline]
    pub unsafe fn NotifyData(&self, data: *const ::nsstring::nsACString, isBinary: bool) -> ::nserror::nsresult {
        ((*self.vtable).NotifyData)(self, data, isBinary)
    }


}


/// `interface nsIPresentationSessionTransport : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPresentationSessionTransport {
    vtable: *const nsIPresentationSessionTransportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPresentationSessionTransport.
unsafe impl XpCom for nsIPresentationSessionTransport {
    const IID: nsIID = nsID(0x670b7e1b, 0x65be, 0x42b6,
        [0xa5, 0x96, 0xbe, 0x57, 0x19, 0x07, 0xfa, 0x18]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPresentationSessionTransport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPresentationSessionTransport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPresentationSessionTransportCoerce {
    /// Cheaply cast a value of this type from a `nsIPresentationSessionTransport`.
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPresentationSessionTransportCoerce for nsIPresentationSessionTransport {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self {
        v
    }
}

impl nsIPresentationSessionTransport {
    /// Cast this `nsIPresentationSessionTransport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPresentationSessionTransportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPresentationSessionTransport {
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
impl<T: nsISupportsCoerce> nsIPresentationSessionTransportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPresentationSessionTransport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPresentationSessionTransport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPresentationSessionTransportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIPresentationSessionTransportCallback callback; */
    pub GetCallback: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, aCallback: *mut *const nsIPresentationSessionTransportCallback) -> ::nserror::nsresult,

    /* attribute nsIPresentationSessionTransportCallback callback; */
    pub SetCallback: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, aCallback: *const nsIPresentationSessionTransportCallback) -> ::nserror::nsresult,

    /* readonly attribute nsINetAddr selfAddress; */
    pub GetSelfAddress: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, aSelfAddress: *mut*const nsINetAddr) -> ::nserror::nsresult,

    /* void enableDataNotification (); */
    pub EnableDataNotification: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport) -> ::nserror::nsresult,

    /* void send (in AString data); */
    pub Send: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, data: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void sendBinaryMsg (in ACString data); */
    pub SendBinaryMsg: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void sendBlob (in Blob blob); */
    pub SendBlob: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, blob: *const libc::c_void) -> ::nserror::nsresult,

    /* void close (in nsresult reason); */
    pub Close: unsafe extern "system" fn (this: *const nsIPresentationSessionTransport, reason: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPresentationSessionTransport {


    /// `attribute nsIPresentationSessionTransportCallback callback;`
    #[inline]
    pub unsafe fn GetCallback(&self, aCallback: *mut *const nsIPresentationSessionTransportCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCallback)(self, aCallback)
    }



    /// `attribute nsIPresentationSessionTransportCallback callback;`
    #[inline]
    pub unsafe fn SetCallback(&self, aCallback: *const nsIPresentationSessionTransportCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCallback)(self, aCallback)
    }



    /// `readonly attribute nsINetAddr selfAddress;`
    #[inline]
    pub unsafe fn GetSelfAddress(&self, aSelfAddress: *mut*const nsINetAddr) -> ::nserror::nsresult {
        ((*self.vtable).GetSelfAddress)(self, aSelfAddress)
    }



    /// `void enableDataNotification ();`
    #[inline]
    pub unsafe fn EnableDataNotification(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EnableDataNotification)(self, )
    }



    /// `void send (in AString data);`
    #[inline]
    pub unsafe fn Send(&self, data: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Send)(self, data)
    }



    /// `void sendBinaryMsg (in ACString data);`
    #[inline]
    pub unsafe fn SendBinaryMsg(&self, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SendBinaryMsg)(self, data)
    }



    /// `void sendBlob (in Blob blob);`
    #[inline]
    pub unsafe fn SendBlob(&self, blob: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SendBlob)(self, blob)
    }



    /// `void close (in nsresult reason);`
    #[inline]
    pub unsafe fn Close(&self, reason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, reason)
    }


}



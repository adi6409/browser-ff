//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/network/interfaces/nsITCPSocketCallback.idl
//


/// `interface nsITCPSocketCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITCPSocketCallback {
    vtable: *const nsITCPSocketCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITCPSocketCallback.
unsafe impl XpCom for nsITCPSocketCallback {
    const IID: nsIID = nsID(0xac2c4b69, 0xcb79, 0x4767,
        [0xb1, 0xce, 0xbc, 0xf6, 0x29, 0x45, 0xcd, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITCPSocketCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITCPSocketCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITCPSocketCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsITCPSocketCallback`.
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITCPSocketCallbackCoerce for nsITCPSocketCallback {
    #[inline]
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self {
        v
    }
}

impl nsITCPSocketCallback {
    /// Cast this `nsITCPSocketCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITCPSocketCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITCPSocketCallback {
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
impl<T: nsISupportsCoerce> nsITCPSocketCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITCPSocketCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITCPSocketCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITCPSocketCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void fireErrorEvent (in AString name, in AString type); */
    pub FireErrorEvent: unsafe extern "system" fn (this: *const nsITCPSocketCallback, name: *const ::nsstring::nsAString, type_: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void fireDataStringEvent (in AString type, in ACString data); */
    pub FireDataStringEvent: unsafe extern "system" fn (this: *const nsITCPSocketCallback, type_: *const ::nsstring::nsAString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void fireDataArrayEvent (in AString type, [const] in nsUint8TArrayRef data); */
    /// Unable to generate binding because `native type nsTArray<uint8_t> unsupported`
    pub FireDataArrayEvent: *const ::libc::c_void,

    /* void fireEvent (in AString type); */
    pub FireEvent: unsafe extern "system" fn (this: *const nsITCPSocketCallback, type_: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void updateReadyState (in unsigned long readystate); */
    pub UpdateReadyState: unsafe extern "system" fn (this: *const nsITCPSocketCallback, readystate: u32) -> ::nserror::nsresult,

    /* void updateBufferedAmount (in uint32_t bufferedAmount, in uint32_t trackingNumber); */
    pub UpdateBufferedAmount: unsafe extern "system" fn (this: *const nsITCPSocketCallback, bufferedAmount: uint32_t, trackingNumber: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITCPSocketCallback {

    pub const BUFFER_SIZE: i64 = 65536;


    /// `void fireErrorEvent (in AString name, in AString type);`
    #[inline]
    pub unsafe fn FireErrorEvent(&self, name: *const ::nsstring::nsAString, type_: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FireErrorEvent)(self, name, type_)
    }



    /// `void fireDataStringEvent (in AString type, in ACString data);`
    #[inline]
    pub unsafe fn FireDataStringEvent(&self, type_: *const ::nsstring::nsAString, data: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).FireDataStringEvent)(self, type_, data)
    }



    /// `void fireDataArrayEvent (in AString type, [const] in nsUint8TArrayRef data);`
    const _FireDataArrayEvent: () = ();


    /// `void fireEvent (in AString type);`
    #[inline]
    pub unsafe fn FireEvent(&self, type_: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FireEvent)(self, type_)
    }



    /// `void updateReadyState (in unsigned long readystate);`
    #[inline]
    pub unsafe fn UpdateReadyState(&self, readystate: u32) -> ::nserror::nsresult {
        ((*self.vtable).UpdateReadyState)(self, readystate)
    }



    /// `void updateBufferedAmount (in uint32_t bufferedAmount, in uint32_t trackingNumber);`
    #[inline]
    pub unsafe fn UpdateBufferedAmount(&self, bufferedAmount: uint32_t, trackingNumber: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).UpdateBufferedAmount)(self, bufferedAmount, trackingNumber)
    }


}



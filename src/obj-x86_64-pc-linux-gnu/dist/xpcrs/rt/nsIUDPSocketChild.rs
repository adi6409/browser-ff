//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/network/interfaces/nsIUDPSocketChild.idl
//


/// `interface nsIUDPSocketInternal : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUDPSocketInternal {
    vtable: *const nsIUDPSocketInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUDPSocketInternal.
unsafe impl XpCom for nsIUDPSocketInternal {
    const IID: nsIID = nsID(0x613dd3ad, 0x598b, 0x4da9,
        [0xad, 0x63, 0xbb, 0xda, 0x50, 0xc2, 0x00, 0x98]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUDPSocketInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUDPSocketInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUDPSocketInternalCoerce {
    /// Cheaply cast a value of this type from a `nsIUDPSocketInternal`.
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUDPSocketInternalCoerce for nsIUDPSocketInternal {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self {
        v
    }
}

impl nsIUDPSocketInternal {
    /// Cast this `nsIUDPSocketInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUDPSocketInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUDPSocketInternal {
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
impl<T: nsISupportsCoerce> nsIUDPSocketInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUDPSocketInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUDPSocketInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUDPSocketInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callListenerOpened (); */
    pub CallListenerOpened: unsafe extern "system" fn (this: *const nsIUDPSocketInternal) -> ::nserror::nsresult,

    /* void callListenerConnected (); */
    pub CallListenerConnected: unsafe extern "system" fn (this: *const nsIUDPSocketInternal) -> ::nserror::nsresult,

    /* void callListenerClosed (); */
    pub CallListenerClosed: unsafe extern "system" fn (this: *const nsIUDPSocketInternal) -> ::nserror::nsresult,

    /* void callListenerReceivedData (in AUTF8String host, in unsigned short port, in Array<uint8_t> data); */
    pub CallListenerReceivedData: unsafe extern "system" fn (this: *const nsIUDPSocketInternal, host: *const ::nsstring::nsACString, port: u16, data: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,

    /* void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber); */
    pub CallListenerError: unsafe extern "system" fn (this: *const nsIUDPSocketInternal, message: *const ::nsstring::nsACString, filename: *const ::nsstring::nsACString, lineNumber: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUDPSocketInternal {


    /// `void callListenerOpened ();`
    #[inline]
    pub unsafe fn CallListenerOpened(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CallListenerOpened)(self, )
    }



    /// `void callListenerConnected ();`
    #[inline]
    pub unsafe fn CallListenerConnected(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CallListenerConnected)(self, )
    }



    /// `void callListenerClosed ();`
    #[inline]
    pub unsafe fn CallListenerClosed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CallListenerClosed)(self, )
    }



    /// `void callListenerReceivedData (in AUTF8String host, in unsigned short port, in Array<uint8_t> data);`
    #[inline]
    pub unsafe fn CallListenerReceivedData(&self, host: *const ::nsstring::nsACString, port: u16, data: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).CallListenerReceivedData)(self, host, port, data)
    }



    /// `void callListenerError (in AUTF8String message, in AUTF8String filename, in uint32_t lineNumber);`
    #[inline]
    pub unsafe fn CallListenerError(&self, message: *const ::nsstring::nsACString, filename: *const ::nsstring::nsACString, lineNumber: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).CallListenerError)(self, message, filename, lineNumber)
    }


}



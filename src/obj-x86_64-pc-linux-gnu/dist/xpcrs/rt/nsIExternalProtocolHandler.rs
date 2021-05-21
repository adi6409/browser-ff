//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIExternalProtocolHandler.idl
//


/// `interface nsIExternalProtocolHandler : nsIProtocolHandler`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExternalProtocolHandler {
    vtable: *const nsIExternalProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExternalProtocolHandler.
unsafe impl XpCom for nsIExternalProtocolHandler {
    const IID: nsIID = nsID(0x0e61f3b2, 0x34d7, 0x4c79,
        [0xbf, 0xdc, 0x48, 0x60, 0xbc, 0x73, 0x41, 0xb7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExternalProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExternalProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExternalProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIExternalProtocolHandler`.
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExternalProtocolHandlerCoerce for nsIExternalProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self {
        v
    }
}

impl nsIExternalProtocolHandler {
    /// Cast this `nsIExternalProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExternalProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExternalProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProtocolHandlerCoerce> nsIExternalProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExternalProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExternalProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProtocolHandlerVTable,

    /* boolean externalAppExistsForScheme (in ACString scheme); */
    pub ExternalAppExistsForScheme: unsafe extern "system" fn (this: *const nsIExternalProtocolHandler, scheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExternalProtocolHandler {

    /// ```text
    /// /**
    ///      * This method checks if the external handler exists for a given scheme.
    ///      *
    ///      * @param scheme external scheme.
    ///      * @return TRUE if the external handler exists for the input scheme, FALSE otherwise.
    ///      */
    /// ```
    ///

    /// `boolean externalAppExistsForScheme (in ACString scheme);`
    #[inline]
    pub unsafe fn ExternalAppExistsForScheme(&self, scheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExternalAppExistsForScheme)(self, scheme, _retval)
    }


}



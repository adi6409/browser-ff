//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolProxyService2.idl
//


/// `interface nsIProtocolProxyService2 : nsIProtocolProxyService`
///

/// ```text
/// /**
///  * An extension of nsIProtocolProxyService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolProxyService2 {
    vtable: *const nsIProtocolProxyService2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolProxyService2.
unsafe impl XpCom for nsIProtocolProxyService2 {
    const IID: nsIID = nsID(0xb2e5b2c0, 0xe21e, 0x4845,
        [0xb3, 0x36, 0xbe, 0x6d, 0x60, 0xa3, 0x89, 0x51]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolProxyService2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolProxyService2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolProxyService2Coerce {
    /// Cheaply cast a value of this type from a `nsIProtocolProxyService2`.
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolProxyService2Coerce for nsIProtocolProxyService2 {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self {
        v
    }
}

impl nsIProtocolProxyService2 {
    /// Cast this `nsIProtocolProxyService2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolProxyService2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolProxyService2 {
    type Target = nsIProtocolProxyService;
    #[inline]
    fn deref(&self) -> &nsIProtocolProxyService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProtocolProxyServiceCoerce> nsIProtocolProxyService2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolProxyService2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolProxyService2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolProxyService2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProtocolProxyServiceVTable,

    /* void reloadPAC (); */
    pub ReloadPAC: unsafe extern "system" fn (this: *const nsIProtocolProxyService2) -> ::nserror::nsresult,

    /* nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget); */
    pub AsyncResolve2: unsafe extern "system" fn (this: *const nsIProtocolProxyService2, aChannel: *const nsIChannel, aFlags: u32, aCallback: *const nsIProtocolProxyCallback, aMainThreadTarget: *const nsISerialEventTarget, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolProxyService2 {

    /// ```text
    /// /**
    ///    * Call this method to cause the PAC file (if any is configured) to be
    ///    * reloaded.  The PAC file is loaded asynchronously.
    ///    */
    /// ```
    ///

    /// `void reloadPAC ();`
    #[inline]
    pub unsafe fn ReloadPAC(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ReloadPAC)(self, )
    }


    /// ```text
    /// /**
    ///      * This method is identical to asyncResolve() except:
    ///      *  - it only accepts an nsIChannel, not an nsIURI;
    ///      *  - it may execute the callback function immediately (i.e from the stack
        ///      *    of asyncResolve2()) if it is immediately ready to run.
    ///      *    The nsICancelable return value will be null in that case.
    ///      */
    /// ```
    ///

    /// `nsICancelable asyncResolve2 (in nsIChannel aChannel, in unsigned long aFlags, in nsIProtocolProxyCallback aCallback, [optional] in nsISerialEventTarget aMainThreadTarget);`
    #[inline]
    pub unsafe fn AsyncResolve2(&self, aChannel: *const nsIChannel, aFlags: u32, aCallback: *const nsIProtocolProxyCallback, aMainThreadTarget: *const nsISerialEventTarget, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).AsyncResolve2)(self, aChannel, aFlags, aCallback, aMainThreadTarget, _retval)
    }


}



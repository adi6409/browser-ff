//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsIDNSListener.idl
//


/// `interface nsIDNSListener : nsISupports`
///

/// ```text
/// /**
///  * nsIDNSListener
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDNSListener {
    vtable: *const nsIDNSListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDNSListener.
unsafe impl XpCom for nsIDNSListener {
    const IID: nsIID = nsID(0x27d49bfe, 0x280c, 0x49e0,
        [0xbb, 0xaa, 0xf6, 0x20, 0x0c, 0x23, 0x2c, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDNSListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDNSListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDNSListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDNSListener`.
    fn coerce_from(v: &nsIDNSListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDNSListenerCoerce for nsIDNSListener {
    #[inline]
    fn coerce_from(v: &nsIDNSListener) -> &Self {
        v
    }
}

impl nsIDNSListener {
    /// Cast this `nsIDNSListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDNSListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDNSListener {
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
impl<T: nsISupportsCoerce> nsIDNSListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDNSListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDNSListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDNSListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus); */
    pub OnLookupComplete: unsafe extern "system" fn (this: *const nsIDNSListener, aRequest: *const nsICancelable, aRecord: *const nsIDNSRecord, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDNSListener {

    /// ```text
    /// /**
    ///      * called when an asynchronous host lookup completes.
    ///      *
    ///      * @param aRequest
    ///      *        the value returned from asyncResolve.
    ///      * @param aRecord
    ///      *        the DNS record corresponding to the hostname that was resolved.
    ///      *        this parameter is null if there was an error.
    ///      *        depending on the type parameter passed to asyncResolve() the
    ///      *        caller should QueryInterface to either nsIDNSAddrRecord or
    ///      *        nsIDNSByTypeRecord.
    ///      * @param aStatus
    ///      *        if the lookup failed, this parameter gives the reason.
    ///      */
    /// ```
    ///

    /// `void onLookupComplete (in nsICancelable aRequest, in nsIDNSRecord aRecord, in nsresult aStatus);`
    #[inline]
    pub unsafe fn OnLookupComplete(&self, aRequest: *const nsICancelable, aRecord: *const nsIDNSRecord, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnLookupComplete)(self, aRequest, aRecord, aStatus)
    }


}



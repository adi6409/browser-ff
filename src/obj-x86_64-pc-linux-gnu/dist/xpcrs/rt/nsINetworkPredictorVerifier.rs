//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkPredictorVerifier.idl
//


/// `interface nsINetworkPredictorVerifier : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetworkPredictorVerifier {
    vtable: *const nsINetworkPredictorVerifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkPredictorVerifier.
unsafe impl XpCom for nsINetworkPredictorVerifier {
    const IID: nsIID = nsID(0x2e43bb32, 0xdabf, 0x4494,
        [0x9f, 0x90, 0x2b, 0x31, 0x95, 0xb1, 0xc7, 0x3d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkPredictorVerifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkPredictorVerifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkPredictorVerifierCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkPredictorVerifier`.
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkPredictorVerifierCoerce for nsINetworkPredictorVerifier {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self {
        v
    }
}

impl nsINetworkPredictorVerifier {
    /// Cast this `nsINetworkPredictorVerifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkPredictorVerifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkPredictorVerifier {
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
impl<T: nsISupportsCoerce> nsINetworkPredictorVerifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictorVerifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkPredictorVerifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkPredictorVerifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPredictPrefetch (in nsIURI uri, in uint32_t status); */
    pub OnPredictPrefetch: unsafe extern "system" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI, status: uint32_t) -> ::nserror::nsresult,

    /* void onPredictPreconnect (in nsIURI uri); */
    pub OnPredictPreconnect: unsafe extern "system" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI) -> ::nserror::nsresult,

    /* void onPredictDNS (in nsIURI uri); */
    pub OnPredictDNS: unsafe extern "system" fn (this: *const nsINetworkPredictorVerifier, uri: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkPredictorVerifier {

    /// ```text
    /// /**
    ///      * Callback for when we do a predictive prefetch
    ///      *
    ///      * @param uri - The URI that was prefetched
    ///      * @param status - The request status code returned by the
    ///      *   prefetch attempt e.g. 200 (OK):w
    ///      */
    /// ```
    ///

    /// `void onPredictPrefetch (in nsIURI uri, in uint32_t status);`
    #[inline]
    pub unsafe fn OnPredictPrefetch(&self, uri: *const nsIURI, status: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).OnPredictPrefetch)(self, uri, status)
    }


    /// ```text
    /// /**
    ///      * Callback for when we do a predictive preconnect
    ///      *
    ///      * @param uri - The URI that was preconnected to
    ///      */
    /// ```
    ///

    /// `void onPredictPreconnect (in nsIURI uri);`
    #[inline]
    pub unsafe fn OnPredictPreconnect(&self, uri: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).OnPredictPreconnect)(self, uri)
    }


    /// ```text
    /// /**
    ///      * Callback for when we do a predictive DNS lookup
    ///      *
    ///      * @param uri - The URI that was looked up
    ///      */
    /// ```
    ///

    /// `void onPredictDNS (in nsIURI uri);`
    #[inline]
    pub unsafe fn OnPredictDNS(&self, uri: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).OnPredictDNS)(self, uri)
    }


}



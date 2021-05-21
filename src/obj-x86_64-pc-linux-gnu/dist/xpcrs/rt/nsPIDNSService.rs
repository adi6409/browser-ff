//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsPIDNSService.idl
//


/// `interface nsPIDNSService : nsIDNSService`
///

/// ```text
/// /**
///  * This is a private interface used by the internals of the networking library.
///  * It will never be frozen.  Do not use it in external code.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsPIDNSService {
    vtable: *const nsPIDNSServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsPIDNSService.
unsafe impl XpCom for nsPIDNSService {
    const IID: nsIID = nsID(0x24e598fd, 0x7b1a, 0x436c,
        [0x91, 0x54, 0x14, 0xd8, 0xb3, 0x8d, 0xf8, 0xa5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsPIDNSService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsPIDNSService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsPIDNSServiceCoerce {
    /// Cheaply cast a value of this type from a `nsPIDNSService`.
    fn coerce_from(v: &nsPIDNSService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsPIDNSServiceCoerce for nsPIDNSService {
    #[inline]
    fn coerce_from(v: &nsPIDNSService) -> &Self {
        v
    }
}

impl nsPIDNSService {
    /// Cast this `nsPIDNSService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsPIDNSServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsPIDNSService {
    type Target = nsIDNSService;
    #[inline]
    fn deref(&self) -> &nsIDNSService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDNSServiceCoerce> nsPIDNSServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsPIDNSService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsPIDNSService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsPIDNSServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDNSServiceVTable,

    /* void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsPIDNSService) -> ::nserror::nsresult,

    /* void shutdown (); */
    pub Shutdown: unsafe extern "system" fn (this: *const nsPIDNSService) -> ::nserror::nsresult,

    /* attribute boolean prefetchEnabled; */
    pub GetPrefetchEnabled: unsafe extern "system" fn (this: *const nsPIDNSService, aPrefetchEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean prefetchEnabled; */
    pub SetPrefetchEnabled: unsafe extern "system" fn (this: *const nsPIDNSService, aPrefetchEnabled: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsPIDNSService {

    /// ```text
    /// /**
    ///      * called to initialize the DNS service.
    ///      */
    /// ```
    ///

    /// `void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }


    /// ```text
    /// /**
    ///      * called to shutdown the DNS service.  any pending asynchronous
    ///      * requests will be canceled, and the local cache of DNS records
    ///      * will be cleared.  NOTE: the operating system may still have
    ///      * its own cache of DNS records, which would be unaffected by
    ///      * this method.
    ///      */
    /// ```
    ///

    /// `void shutdown ();`
    #[inline]
    pub unsafe fn Shutdown(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Shutdown)(self, )
    }


    /// ```text
    /// /**
    ///      * Whether or not DNS prefetching (aka RESOLVE_SPECULATE) is enabled
    ///      */
    /// ```
    ///

    /// `attribute boolean prefetchEnabled;`
    #[inline]
    pub unsafe fn GetPrefetchEnabled(&self, aPrefetchEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPrefetchEnabled)(self, aPrefetchEnabled)
    }


    /// ```text
    /// /**
    ///      * Whether or not DNS prefetching (aka RESOLVE_SPECULATE) is enabled
    ///      */
    /// ```
    ///

    /// `attribute boolean prefetchEnabled;`
    #[inline]
    pub unsafe fn SetPrefetchEnabled(&self, aPrefetchEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrefetchEnabled)(self, aPrefetchEnabled)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/prefetch/nsIPrefetchService.idl
//


/// `interface nsIPrefetchService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrefetchService {
    vtable: *const nsIPrefetchServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrefetchService.
unsafe impl XpCom for nsIPrefetchService {
    const IID: nsIID = nsID(0x422a1807, 0x4e7f, 0x463d,
        [0xb8, 0xd7, 0xca, 0x2c, 0xeb, 0x9b, 0x5d, 0x53]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrefetchService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrefetchService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrefetchServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPrefetchService`.
    fn coerce_from(v: &nsIPrefetchService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrefetchServiceCoerce for nsIPrefetchService {
    #[inline]
    fn coerce_from(v: &nsIPrefetchService) -> &Self {
        v
    }
}

impl nsIPrefetchService {
    /// Cast this `nsIPrefetchService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrefetchServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrefetchService {
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
impl<T: nsISupportsCoerce> nsIPrefetchServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrefetchService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrefetchService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrefetchServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void prefetchURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in boolean aExplicit); */
    pub PrefetchURI: unsafe extern "system" fn (this: *const nsIPrefetchService, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aSource: *const libc::c_void, aExplicit: bool) -> ::nserror::nsresult,

    /* void preloadURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in nsContentPolicyType aPolicyType); */
    pub PreloadURI: unsafe extern "system" fn (this: *const nsIPrefetchService, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aSource: *const libc::c_void, aPolicyType: nsContentPolicyType) -> ::nserror::nsresult,

    /* boolean hasMoreElements (); */
    pub HasMoreElements: unsafe extern "system" fn (this: *const nsIPrefetchService, _retval: *mut bool) -> ::nserror::nsresult,

    /* void cancelPrefetchPreloadURI (in nsIURI aURI, in Node aSource); */
    pub CancelPrefetchPreloadURI: unsafe extern "system" fn (this: *const nsIPrefetchService, aURI: *const nsIURI, aSource: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrefetchService {

    /// ```text
    /// /**
    ///      * Enqueue a request to prefetch the specified URI.
    ///      *
    ///      * @param aURI the URI of the document to prefetch
    ///      * @param aReferrerInfo the referrerInfo of the request
    ///      * @param aSource the DOM node (such as a <link> tag) that requested this
    ///      *        fetch, or null if the prefetch was not requested by a DOM node.
    ///      * @param aExplicit the link element has an explicit prefetch link type
    ///      */
    /// ```
    ///

    /// `void prefetchURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in boolean aExplicit);`
    #[inline]
    pub unsafe fn PrefetchURI(&self, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aSource: *const libc::c_void, aExplicit: bool) -> ::nserror::nsresult {
        ((*self.vtable).PrefetchURI)(self, aURI, aReferrerInfo, aSource, aExplicit)
    }


    /// ```text
    /// /**
    ///      * Start a preload of the specified URI.
    ///      *
    ///      * @param aURI the URI of the document to preload
    ///      * @param aReferrerInfo the referrerInfo of the request
    ///      * @param aSource the DOM node (such as a <link> tag) that requested this
    ///      *        fetch, or null if the prefetch was not requested by a DOM node.
    ///      * @param aPolicyType content policy to be used for this load.
    ///      */
    /// ```
    ///

    /// `void preloadURI (in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in Node aSource, in nsContentPolicyType aPolicyType);`
    #[inline]
    pub unsafe fn PreloadURI(&self, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aSource: *const libc::c_void, aPolicyType: nsContentPolicyType) -> ::nserror::nsresult {
        ((*self.vtable).PreloadURI)(self, aURI, aReferrerInfo, aSource, aPolicyType)
    }


    /// ```text
    /// /**
    ///      * Find out if there are any prefetches running or queued
    ///      */
    /// ```
    ///

    /// `boolean hasMoreElements ();`
    #[inline]
    pub unsafe fn HasMoreElements(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMoreElements)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Cancel prefetch or preload for a Node.
    ///      */
    /// ```
    ///

    /// `void cancelPrefetchPreloadURI (in nsIURI aURI, in Node aSource);`
    #[inline]
    pub unsafe fn CancelPrefetchPreloadURI(&self, aURI: *const nsIURI, aSource: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CancelPrefetchPreloadURI)(self, aURI, aSource)
    }


}



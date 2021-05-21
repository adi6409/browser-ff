//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIRefreshURI.idl
//


/// `interface nsIRefreshURI : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRefreshURI {
    vtable: *const nsIRefreshURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRefreshURI.
unsafe impl XpCom for nsIRefreshURI {
    const IID: nsIID = nsID(0xa5e61a3c, 0x51bd, 0x45be,
        [0xac, 0x0c, 0xe8, 0x7b, 0x71, 0x86, 0x06, 0x56]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRefreshURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRefreshURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRefreshURICoerce {
    /// Cheaply cast a value of this type from a `nsIRefreshURI`.
    fn coerce_from(v: &nsIRefreshURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRefreshURICoerce for nsIRefreshURI {
    #[inline]
    fn coerce_from(v: &nsIRefreshURI) -> &Self {
        v
    }
}

impl nsIRefreshURI {
    /// Cast this `nsIRefreshURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRefreshURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRefreshURI {
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
impl<T: nsISupportsCoerce> nsIRefreshURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRefreshURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRefreshURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRefreshURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void refreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh); */
    pub RefreshURI: unsafe extern "system" fn (this: *const nsIRefreshURI, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aMillis: i32, aRepeat: bool, aMetaRefresh: bool) -> ::nserror::nsresult,

    /* void forceRefreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aMetaRefresh); */
    pub ForceRefreshURI: unsafe extern "system" fn (this: *const nsIRefreshURI, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aMillis: i32, aMetaRefresh: bool) -> ::nserror::nsresult,

    /* void setupRefreshURI (in nsIChannel aChannel); */
    pub SetupRefreshURI: unsafe extern "system" fn (this: *const nsIRefreshURI, aChannel: *const nsIChannel) -> ::nserror::nsresult,

    /* void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in unsigned long long aInnerWindowID, in ACString aHeader); */
    pub SetupRefreshURIFromHeader: unsafe extern "system" fn (this: *const nsIRefreshURI, aBaseURI: *const nsIURI, principal: *const nsIPrincipal, aInnerWindowID: u64, aHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void cancelRefreshURITimers (); */
    pub CancelRefreshURITimers: unsafe extern "system" fn (this: *const nsIRefreshURI) -> ::nserror::nsresult,

    /* readonly attribute boolean refreshPending; */
    pub GetRefreshPending: unsafe extern "system" fn (this: *const nsIRefreshURI, aRefreshPending: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRefreshURI {

    /// ```text
    /// /**
    ///       * Load a uri after waiting for aMillis milliseconds. If the docshell
    ///       * is busy loading a page currently, the refresh request will be
    ///       * queued and executed when the current load finishes.
    ///       *
    ///       * @param aUri The uri to refresh.
    ///       * @param aPrincipal The triggeringPrincipal for the refresh load
    ///       *   May be null, in which case the principal of current document will be
    ///       *   applied.
    ///       * @param aMillis The number of milliseconds to wait.
    ///       * @param aRepeat Flag to indicate if the uri is to be
    ///       *                repeatedly refreshed every aMillis milliseconds.
    ///       * @param aMetaRefresh Flag to indicate if this is a Meta refresh.
    ///       */
    /// ```
    ///

    /// `void refreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aRepeat, in boolean aMetaRefresh);`
    #[inline]
    pub unsafe fn RefreshURI(&self, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aMillis: i32, aRepeat: bool, aMetaRefresh: bool) -> ::nserror::nsresult {
        ((*self.vtable).RefreshURI)(self, aURI, aPrincipal, aMillis, aRepeat, aMetaRefresh)
    }


    /// ```text
    /// /**
    ///       * Loads a URI immediately as if it were a refresh.
    ///       *
    ///       * @param aURI The URI to refresh.
    ///       * @param aPrincipal The triggeringPrincipal for the refresh load
    ///       *   May be null, in which case the principal of current document will be
    ///       *   applied.
    ///       * @param aMillis The number of milliseconds by which this refresh would
    ///       *                be delayed if it were not being forced.
    ///       * @param aMetaRefresh Flag to indicate if this is a meta refresh.
    ///       */
    /// ```
    ///

    /// `void forceRefreshURI (in nsIURI aURI, in nsIPrincipal aPrincipal, in long aMillis, in boolean aMetaRefresh);`
    #[inline]
    pub unsafe fn ForceRefreshURI(&self, aURI: *const nsIURI, aPrincipal: *const nsIPrincipal, aMillis: i32, aMetaRefresh: bool) -> ::nserror::nsresult {
        ((*self.vtable).ForceRefreshURI)(self, aURI, aPrincipal, aMillis, aMetaRefresh)
    }


    /// ```text
    /// /**
    ///       * Checks the passed in channel to see if there is a refresh header,
    ///       * if there is, will setup a timer to refresh the uri found
    ///       * in the header. If docshell is busy loading a page currently, the
    ///       * request will be queued and executed when the current page
    ///       * finishes loading.
    ///       *
    ///       * Returns the NS_REFRESHURI_HEADER_FOUND success code if a refresh
    ///       * header was found and successfully setup.
    ///       *
    ///       * @param aChannel The channel to be parsed.
    ///       */
    /// ```
    ///

    /// `void setupRefreshURI (in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn SetupRefreshURI(&self, aChannel: *const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).SetupRefreshURI)(self, aChannel)
    }


    /// ```text
    /// /**
    ///       * Parses the passed in header string and sets up a refreshURI if
    ///       * a "refresh" header is found. If docshell is busy loading a page
    ///       * currently, the request will be queued and executed when
    ///       * the current page finishes loading.
    ///       *
    ///       * @param aBaseURI base URI to resolve refresh uri with.
    ///       * @param aPrincipal The triggeringPrincipal for the refresh load
    ///       *   May be null, in which case the principal of current document will be
    ///       *   applied.
    ///       * @param aInnerWindowID The window id to use for error reporting.
    ///       * @param aHeader  The meta refresh header string.
    ///       */
    /// ```
    ///

    /// `void setupRefreshURIFromHeader (in nsIURI aBaseURI, in nsIPrincipal principal, in unsigned long long aInnerWindowID, in ACString aHeader);`
    #[inline]
    pub unsafe fn SetupRefreshURIFromHeader(&self, aBaseURI: *const nsIURI, principal: *const nsIPrincipal, aInnerWindowID: u64, aHeader: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetupRefreshURIFromHeader)(self, aBaseURI, principal, aInnerWindowID, aHeader)
    }


    /// ```text
    /// /**
    ///       * Cancels all timer loads.
    ///       */
    /// ```
    ///

    /// `void cancelRefreshURITimers ();`
    #[inline]
    pub unsafe fn CancelRefreshURITimers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CancelRefreshURITimers)(self, )
    }


    /// ```text
    /// /**
    ///      * True when there are pending refreshes, false otherwise.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean refreshPending;`
    #[inline]
    pub unsafe fn GetRefreshPending(&self, aRefreshPending: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRefreshPending)(self, aRefreshPending)
    }


}



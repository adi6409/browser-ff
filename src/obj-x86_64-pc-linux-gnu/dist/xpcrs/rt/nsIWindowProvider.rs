//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowcreator/nsIWindowProvider.idl
//


/// `interface nsIWindowProvider : nsISupports`
///

/// ```text
/// /**
///  * The nsIWindowProvider interface exists so that the window watcher's default
///  * behavior of opening a new window can be easly modified.  When the window
///  * watcher needs to open a new window, it will first check with the
///  * nsIWindowProvider it gets from the parent window.  If there is no provider
///  * or the provider does not provide a window, the window watcher will proceed
///  * to actually open a new window.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowProvider {
    vtable: *const nsIWindowProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowProvider.
unsafe impl XpCom for nsIWindowProvider {
    const IID: nsIID = nsID(0xe97a3830, 0x15ef, 0x499b,
        [0x83, 0x72, 0xc2, 0x2d, 0x12, 0x80, 0x91, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowProvider`.
    fn coerce_from(v: &nsIWindowProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowProviderCoerce for nsIWindowProvider {
    #[inline]
    fn coerce_from(v: &nsIWindowProvider) -> &Self {
        v
    }
}

impl nsIWindowProvider {
    /// Cast this `nsIWindowProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowProvider {
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
impl<T: nsISupportsCoerce> nsIWindowProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] BrowsingContext provideWindow (in nsIOpenWindowInfo aOpenWindowInfo, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aWidthSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsDocShellLoadStatePtr aLoadState, out boolean aWindowIsNew); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub ProvideWindow: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowProvider {

    /// ```text
    /// /**
    ///    * A method to request that this provider provide a window.  The window
    ///    * returned need not to have the right name or parent set on it; setting
    ///    * those is the caller's responsibility.  The provider can always return null
    ///    * to have the caller create a brand-new window.
    ///    *
    ///    * @param aOpenWindowInfo  Must not be null.  This is the information the
    ///    *                         caller wants to be used to construct the new window.
    ///    *
    ///    * @param aChromeFlags The chrome flags the caller will use to create a new
    ///    *        window if this provider returns null.  See nsIWebBrowserChrome for
    ///    *        the possible values of this field.
    ///    *
    ///    * @param aWidthSpecified Whether the attempt to create a window is trying
    ///    *        to specify the width for the new window.
    ///    *
    ///    * @param aURI The URI to be loaded in the new window (may be NULL).  The
    ///    *        nsIWindowProvider implementation must not load this URI into the
    ///    *        window it returns.  This URI is provided solely to help the
    ///    *        nsIWindowProvider implementation make decisions; the caller will
    ///    *        handle loading the URI in the window returned if provideWindow
    ///    *        returns a window.
    ///    *
    ///    *        When making decisions based on aURI, note that even when it's not
    ///    *        null, aURI may not represent all relevant information about the
    ///    *        load.  For example, the load may have extra load flags, POST data,
    ///    *        etc.
    ///    *
    ///    * @param aName The name of the window being opened.  Setting the name on the
    ///    *        return value of provideWindow will be handled by the caller; aName
    ///    *        is provided solely to help the nsIWindowProvider implementation
    ///    *        make decisions.
    ///    *
    ///    * @param aFeatures The feature string for the window being opened.  This may
    ///    *        be empty.  The nsIWindowProvider implementation is allowed to apply
    ///    *        the feature string to the window it returns in any way it sees fit.
    ///    *        See the nsIWindowWatcher interface for details on feature strings.
    ///    *
    ///    * @param aLoadState Specify setup information of the load in the new window
    ///    *
    ///    * @param aWindowIsNew [out] Whether the window being returned was just
    ///    *        created by the window provider implementation.  This can be used by
    ///    *        callers to keep track of which windows were opened by the user as
    ///    *        opposed to being opened programmatically.  This should be set to
    ///    *        false if the window being returned existed before the
    ///    *        provideWindow() call.  The value of this out parameter is
    ///    *        meaningless if provideWindow() returns null.
    ///    *
    ///    * @return A window the caller should use or null if the caller should just
    ///    *         create a new window.  The returned window may be newly opened by
    ///    *         the nsIWindowProvider implementation or may be a window that
    ///    *         already existed.
    ///    *
    ///    * @throw NS_ERROR_ABORT if the caller should cease its attempt to open a new
    ///    *                       window.
    ///    *
    ///    * @see nsIWindowWatcher for more information on aFeatures.
    ///    * @see nsIWebBrowserChrome for more information on aChromeFlags.
    ///    */
    /// ```
    ///

    /// `[noscript] BrowsingContext provideWindow (in nsIOpenWindowInfo aOpenWindowInfo, in unsigned long aChromeFlags, in boolean aCalledFromJS, in boolean aWidthSpecified, in nsIURI aURI, in AString aName, in AUTF8String aFeatures, in boolean aForceNoOpener, in boolean aForceNoReferrer, in nsDocShellLoadStatePtr aLoadState, out boolean aWindowIsNew);`
    const _ProvideWindow: () = ();

}



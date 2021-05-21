//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChrome3.idl
//


/// `interface nsIWebBrowserChrome3 : nsIWebBrowserChrome`
///

/// ```text
/// /**
///  * nsIWebBrowserChrome3 is an extension to nsIWebBrowserChrome.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserChrome3 {
    vtable: *const nsIWebBrowserChrome3VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserChrome3.
unsafe impl XpCom for nsIWebBrowserChrome3 {
    const IID: nsIID = nsID(0x542b6625, 0x35a9, 0x426a,
        [0x82, 0x57, 0xc1, 0x2a, 0x34, 0x53, 0x83, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserChrome3 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserChrome3.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserChrome3Coerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserChrome3`.
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserChrome3Coerce for nsIWebBrowserChrome3 {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self {
        v
    }
}

impl nsIWebBrowserChrome3 {
    /// Cast this `nsIWebBrowserChrome3` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserChrome3Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserChrome3 {
    type Target = nsIWebBrowserChrome;
    #[inline]
    fn deref(&self) -> &nsIWebBrowserChrome {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIWebBrowserChromeCoerce> nsIWebBrowserChrome3Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChrome3) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserChrome3
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserChrome3VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIWebBrowserChromeVTable,

    /* AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab); */
    pub OnBeforeLinkTraversal: unsafe extern "system" fn (this: *const nsIWebBrowserChrome3, originalTarget: *const ::nsstring::nsAString, linkURI: *const nsIURI, linkNode: *const libc::c_void, isAppTab: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal, in nsIContentSecurityPolicy aCsp); */
    pub ShouldLoadURI: unsafe extern "system" fn (this: *const nsIWebBrowserChrome3, aDocShell: *const nsIDocShell, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aHasPostData: bool, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool shouldLoadURIInThisProcess (in nsIURI aURI); */
    pub ShouldLoadURIInThisProcess: unsafe extern "system" fn (this: *const nsIWebBrowserChrome3, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserChrome3 {

    /// ```text
    /// /**
    ///    * Determines the appropriate target for a link.
    ///    *
    ///    * @param originalTarget
    ///    *        The original link target.
    ///    * @param linkURI
    ///    *        Link destination URI.
    ///    * @param aDOMNode
    ///    *        Link DOM node.
    ///    * @param isAppTab
    ///    *        Whether or not the link is in an app tab.
    ///    * @returns A new link target, if appropriate.
    ///    *          Otherwise returns originalTarget.
    ///    */
    /// ```
    ///

    /// `AString onBeforeLinkTraversal (in AString originalTarget, in nsIURI linkURI, in Node linkNode, in boolean isAppTab);`
    #[inline]
    pub unsafe fn OnBeforeLinkTraversal(&self, originalTarget: *const ::nsstring::nsAString, linkURI: *const nsIURI, linkNode: *const libc::c_void, isAppTab: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnBeforeLinkTraversal)(self, originalTarget, linkURI, linkNode, isAppTab, _retval)
    }


    /// ```text
    /// /**
    ///    * Determines whether a load should continue.
    ///    *
    ///    * @param aDocShell
    ///    *        The docshell performing the load.
    ///    * @param aURI
    ///    *        The URI being loaded.
    ///    * @param aReferrerInfo
    ///    *        The referrerInfo of the load.
    ///    * @param aHasPostData
    ///    *        True if the load which is being asked about has associated post data
    ///    *        which would be discarded if the load was redirected across process
    ///    *        boundaries.
    ///    * @param aTriggeringPrincipal
    ///    *        The principal that initiated the load of aURI.
    ///    * @param aCsp
    ///    *        The CSP to be used for that load. That is the CSP that e.g. upgrades
    ///    *        the load to HTTPS in case upgrade-insecure-requests is set.
    ///    */
    /// ```
    ///

    /// `bool shouldLoadURI (in nsIDocShell aDocShell, in nsIURI aURI, in nsIReferrerInfo aReferrerInfo, in boolean aHasPostData, in nsIPrincipal aTriggeringPrincipal, in nsIContentSecurityPolicy aCsp);`
    #[inline]
    pub unsafe fn ShouldLoadURI(&self, aDocShell: *const nsIDocShell, aURI: *const nsIURI, aReferrerInfo: *const nsIReferrerInfo, aHasPostData: bool, aTriggeringPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ShouldLoadURI)(self, aDocShell, aURI, aReferrerInfo, aHasPostData, aTriggeringPrincipal, aCsp, _retval)
    }



    /// `bool shouldLoadURIInThisProcess (in nsIURI aURI);`
    #[inline]
    pub unsafe fn ShouldLoadURIInThisProcess(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ShouldLoadURIInThisProcess)(self, aURI, _retval)
    }


}



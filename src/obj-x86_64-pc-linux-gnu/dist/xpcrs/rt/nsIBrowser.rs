//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowser.idl
//


/// `interface nsIBrowser : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowser {
    vtable: *const nsIBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowser.
unsafe impl XpCom for nsIBrowser {
    const IID: nsIID = nsID(0x14e5a0cb, 0xe223, 0x4202,
        [0x95, 0xe8, 0xfe, 0x53, 0x27, 0x51, 0x93, 0xea]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowser`.
    fn coerce_from(v: &nsIBrowser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserCoerce for nsIBrowser {
    #[inline]
    fn coerce_from(v: &nsIBrowser) -> &Self {
        v
    }
}

impl nsIBrowser {
    /// Cast this `nsIBrowser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowser {
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
impl<T: nsISupportsCoerce> nsIBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void dropLinks (in Array<AString> links, in nsIPrincipal triggeringPrincipal); */
    pub DropLinks: unsafe extern "system" fn (this: *const nsIBrowser, links: *const thin_vec::ThinVec<::nsstring::nsString>, triggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* void swapBrowsers (in nsIBrowser aOtherBrowser); */
    pub SwapBrowsers: unsafe extern "system" fn (this: *const nsIBrowser, aOtherBrowser: *const nsIBrowser) -> ::nserror::nsresult,

    /* void closeBrowser (); */
    pub CloseBrowser: unsafe extern "system" fn (this: *const nsIBrowser) -> ::nserror::nsresult,

    /* readonly attribute boolean isRemoteBrowser; */
    pub GetIsRemoteBrowser: unsafe extern "system" fn (this: *const nsIBrowser, aIsRemoteBrowser: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal contentPrincipal; */
    pub GetContentPrincipal: unsafe extern "system" fn (this: *const nsIBrowser, aContentPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal contentPartitionedPrincipal; */
    pub GetContentPartitionedPrincipal: unsafe extern "system" fn (this: *const nsIBrowser, aContentPartitionedPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIContentSecurityPolicy csp; */
    pub GetCsp: unsafe extern "system" fn (this: *const nsIBrowser, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* readonly attribute nsIReferrerInfo referrerInfo; */
    pub GetReferrerInfo: unsafe extern "system" fn (this: *const nsIBrowser, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult,

    /* attribute boolean isNavigating; */
    pub GetIsNavigating: unsafe extern "system" fn (this: *const nsIBrowser, aIsNavigating: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isNavigating; */
    pub SetIsNavigating: unsafe extern "system" fn (this: *const nsIBrowser, aIsNavigating: bool) -> ::nserror::nsresult,

    /* attribute boolean mayEnableCharacterEncodingMenu; */
    pub GetMayEnableCharacterEncodingMenu: unsafe extern "system" fn (this: *const nsIBrowser, aMayEnableCharacterEncodingMenu: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean mayEnableCharacterEncodingMenu; */
    pub SetMayEnableCharacterEncodingMenu: unsafe extern "system" fn (this: *const nsIBrowser, aMayEnableCharacterEncodingMenu: bool) -> ::nserror::nsresult,

    /* attribute boolean charsetAutodetected; */
    pub GetCharsetAutodetected: unsafe extern "system" fn (this: *const nsIBrowser, aCharsetAutodetected: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean charsetAutodetected; */
    pub SetCharsetAutodetected: unsafe extern "system" fn (this: *const nsIBrowser, aCharsetAutodetected: bool) -> ::nserror::nsresult,

    /* void updateForStateChange (in AString aCharset, in nsIURI aDocumentURI, in AString aContentType); */
    pub UpdateForStateChange: unsafe extern "system" fn (this: *const nsIBrowser, aCharset: *const ::nsstring::nsAString, aDocumentURI: *const nsIURI, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void updateWebNavigationForLocationChange (in boolean aCanGoBack, in boolean aCanGoForward); */
    pub UpdateWebNavigationForLocationChange: unsafe extern "system" fn (this: *const nsIBrowser, aCanGoBack: bool, aCanGoForward: bool) -> ::nserror::nsresult,

    /* void updateForLocationChange (in nsIURI aLocation, in AString aCharset, in boolean aMayEnableCharacterEncodingMenu, in boolean aCharsetAutodetected, in nsIURI aDocumentURI, in AString aTitle, in nsIPrincipal aContentPrincipal, in nsIPrincipal aContentPartitionedPrincipal, in nsIContentSecurityPolicy aCSP, in nsIReferrerInfo aReferrerInfo, in boolean aIsSynthetic, in boolean aHasRequestContextID, in uint64_t aRequestContextID, in AString aContentType); */
    pub UpdateForLocationChange: unsafe extern "system" fn (this: *const nsIBrowser, aLocation: *const nsIURI, aCharset: *const ::nsstring::nsAString, aMayEnableCharacterEncodingMenu: bool, aCharsetAutodetected: bool, aDocumentURI: *const nsIURI, aTitle: *const ::nsstring::nsAString, aContentPrincipal: *const nsIPrincipal, aContentPartitionedPrincipal: *const nsIPrincipal, aCSP: *const nsIContentSecurityPolicy, aReferrerInfo: *const nsIReferrerInfo, aIsSynthetic: bool, aHasRequestContextID: bool, aRequestContextID: uint64_t, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIBrowser_ProcessBehavior processSwitchBehavior; */
    pub GetProcessSwitchBehavior: unsafe extern "system" fn (this: *const nsIBrowser, aProcessSwitchBehavior: *mut u8) -> ::nserror::nsresult,

    /* Promise prepareToChangeRemoteness (); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub PrepareToChangeRemoteness: *const ::libc::c_void,

    /* void beforeChangeRemoteness (); */
    pub BeforeChangeRemoteness: unsafe extern "system" fn (this: *const nsIBrowser) -> ::nserror::nsresult,

    /* bool finishChangeRemoteness (in uint64_t aPendingSwitchId); */
    pub FinishChangeRemoteness: unsafe extern "system" fn (this: *const nsIBrowser, aPendingSwitchId: uint64_t, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowser {


    /// `void dropLinks (in Array<AString> links, in nsIPrincipal triggeringPrincipal);`
    #[inline]
    pub unsafe fn DropLinks(&self, links: *const thin_vec::ThinVec<::nsstring::nsString>, triggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).DropLinks)(self, links, triggeringPrincipal)
    }


    /// ```text
    /// /**
    ///    * Swapping of frameloaders are usually initiated from a frameloader owner
    ///    * or other components operating on frameloader owners. This is done by calling
    ///    * swapFrameLoaders at MozFrameLoaderOwner webidl interface.
    ///    *
    ///    * This function aimed to provide the other way around -
    ///    * if the swapping is initiated from frameloader itself or other platform level
    ///    * components, it uses this interface to delegate the swapping request to
    ///    * frameloader owners and ask them to re-initiate frameloader swapping, so that
    ///    * frameloader owners such as <xul:browser> can setup their properties and /
    ///    * or listeners properly on swapping.
    ///    */
    /// ```
    ///

    /// `void swapBrowsers (in nsIBrowser aOtherBrowser);`
    #[inline]
    pub unsafe fn SwapBrowsers(&self, aOtherBrowser: *const nsIBrowser) -> ::nserror::nsresult {
        ((*self.vtable).SwapBrowsers)(self, aOtherBrowser)
    }


    /// ```text
    /// /**
    ///    * Close the browser (usually means to remove a tab).
    ///    */
    /// ```
    ///

    /// `void closeBrowser ();`
    #[inline]
    pub unsafe fn CloseBrowser(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CloseBrowser)(self, )
    }


    /// ```text
    /// /**
    ///    * A browser can change from remote to non-remote and vice versa.
    ///    * For example, when navigating from an in-process chrome page to
    ///    * a web page, this value would change from false to true.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isRemoteBrowser;`
    #[inline]
    pub unsafe fn GetIsRemoteBrowser(&self, aIsRemoteBrowser: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsRemoteBrowser)(self, aIsRemoteBrowser)
    }



    /// `readonly attribute nsIPrincipal contentPrincipal;`
    #[inline]
    pub unsafe fn GetContentPrincipal(&self, aContentPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetContentPrincipal)(self, aContentPrincipal)
    }



    /// `readonly attribute nsIPrincipal contentPartitionedPrincipal;`
    #[inline]
    pub unsafe fn GetContentPartitionedPrincipal(&self, aContentPartitionedPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetContentPartitionedPrincipal)(self, aContentPartitionedPrincipal)
    }



    /// `readonly attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn GetCsp(&self, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCsp)(self, aCsp)
    }



    /// `readonly attribute nsIReferrerInfo referrerInfo;`
    #[inline]
    pub unsafe fn GetReferrerInfo(&self, aReferrerInfo: *mut*const nsIReferrerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetReferrerInfo)(self, aReferrerInfo)
    }


    /// ```text
    /// /**
    ///    * Whether or not the browser is in the process of an nsIWebNavigation
    ///    * navigation method.
    ///    */
    /// ```
    ///

    /// `attribute boolean isNavigating;`
    #[inline]
    pub unsafe fn GetIsNavigating(&self, aIsNavigating: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsNavigating)(self, aIsNavigating)
    }


    /// ```text
    /// /**
    ///    * Whether or not the browser is in the process of an nsIWebNavigation
    ///    * navigation method.
    ///    */
    /// ```
    ///

    /// `attribute boolean isNavigating;`
    #[inline]
    pub unsafe fn SetIsNavigating(&self, aIsNavigating: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsNavigating)(self, aIsNavigating)
    }


    /// ```text
    /// /**
    ///    * Whether or not the character encoding menu may be enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean mayEnableCharacterEncodingMenu;`
    #[inline]
    pub unsafe fn GetMayEnableCharacterEncodingMenu(&self, aMayEnableCharacterEncodingMenu: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetMayEnableCharacterEncodingMenu)(self, aMayEnableCharacterEncodingMenu)
    }


    /// ```text
    /// /**
    ///    * Whether or not the character encoding menu may be enabled.
    ///    */
    /// ```
    ///

    /// `attribute boolean mayEnableCharacterEncodingMenu;`
    #[inline]
    pub unsafe fn SetMayEnableCharacterEncodingMenu(&self, aMayEnableCharacterEncodingMenu: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMayEnableCharacterEncodingMenu)(self, aMayEnableCharacterEncodingMenu)
    }


    /// ```text
    /// /**
    ///    * Whether or not the character encoding was detected by analyzing
    ///    * content (as opposed to reading a protocol label).
    ///    */
    /// ```
    ///

    /// `attribute boolean charsetAutodetected;`
    #[inline]
    pub unsafe fn GetCharsetAutodetected(&self, aCharsetAutodetected: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCharsetAutodetected)(self, aCharsetAutodetected)
    }


    /// ```text
    /// /**
    ///    * Whether or not the character encoding was detected by analyzing
    ///    * content (as opposed to reading a protocol label).
    ///    */
    /// ```
    ///

    /// `attribute boolean charsetAutodetected;`
    #[inline]
    pub unsafe fn SetCharsetAutodetected(&self, aCharsetAutodetected: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCharsetAutodetected)(self, aCharsetAutodetected)
    }


    /// ```text
    /// /**
    ///    * Called by Gecko to update the browser when its state changes.
    ///    *
    ///    * @param aCharset the new character set of the document
    ///    * @param aDocumentURI the URI of the current document
    ///    * @param aContentType the content type of the document
    ///    */
    /// ```
    ///

    /// `void updateForStateChange (in AString aCharset, in nsIURI aDocumentURI, in AString aContentType);`
    #[inline]
    pub unsafe fn UpdateForStateChange(&self, aCharset: *const ::nsstring::nsAString, aDocumentURI: *const nsIURI, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UpdateForStateChange)(self, aCharset, aDocumentURI, aContentType)
    }


    /// ```text
    /// /**
    ///    * Called by Gecko to update the nsIWebNavigation when a location change occurs.
    ///    *
    ///    * @param aCanGoBack whether or not the nsIWebNavigation can go backwards in
    ///    *                   history
    ///    * @param aCanGoForward whether or not the nsIWebNavigation can go
    ///    *                      forward in history
    ///    */
    /// ```
    ///

    /// `void updateWebNavigationForLocationChange (in boolean aCanGoBack, in boolean aCanGoForward);`
    #[inline]
    pub unsafe fn UpdateWebNavigationForLocationChange(&self, aCanGoBack: bool, aCanGoForward: bool) -> ::nserror::nsresult {
        ((*self.vtable).UpdateWebNavigationForLocationChange)(self, aCanGoBack, aCanGoForward)
    }


    /// ```text
    /// /**
    ///    * Called by Gecko to update the browser when a location change occurs.
    ///    *
    ///    * @param aLocation the new location
    ///    * @param aCharset the character set of the document
    ///    * @param aMayEnableCharacterEncodingMenu whether or not the content encoding
    ///    *                                        menu may be enabled
    ///    * @param aCharsetAutodetected whether or not the given character set was
    ///    *                             autodetected
    ///    * @param aDocumentURI the URI of the new document
    ///    * @param aTitle the title of the new doucment
    ///    * @param aContentPrincipal the security principal of the new document
    ///    * @param aContentPartitionedPrincipal the security principal for the new
    ///    *                                     document's storage
    ///    * @param aCSP the content security policy of the new document
    ///    * @param aReferrerInfo the referrer info of the new document
    ///    * @param aIsSynthetic whether or not the document is synthetic
    ///    * @param aHasRequestContextID whether or not the the request context has a
    ///    *                             value (true) or null should be used (false)
    ///    * @param aRequestContextID the request context ID
    ///    * @param aContentType the content type of the document
    ///    */
    /// ```
    ///

    /// `void updateForLocationChange (in nsIURI aLocation, in AString aCharset, in boolean aMayEnableCharacterEncodingMenu, in boolean aCharsetAutodetected, in nsIURI aDocumentURI, in AString aTitle, in nsIPrincipal aContentPrincipal, in nsIPrincipal aContentPartitionedPrincipal, in nsIContentSecurityPolicy aCSP, in nsIReferrerInfo aReferrerInfo, in boolean aIsSynthetic, in boolean aHasRequestContextID, in uint64_t aRequestContextID, in AString aContentType);`
    #[inline]
    pub unsafe fn UpdateForLocationChange(&self, aLocation: *const nsIURI, aCharset: *const ::nsstring::nsAString, aMayEnableCharacterEncodingMenu: bool, aCharsetAutodetected: bool, aDocumentURI: *const nsIURI, aTitle: *const ::nsstring::nsAString, aContentPrincipal: *const nsIPrincipal, aContentPartitionedPrincipal: *const nsIPrincipal, aCSP: *const nsIContentSecurityPolicy, aReferrerInfo: *const nsIReferrerInfo, aIsSynthetic: bool, aHasRequestContextID: bool, aRequestContextID: uint64_t, aContentType: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).UpdateForLocationChange)(self, aLocation, aCharset, aMayEnableCharacterEncodingMenu, aCharsetAutodetected, aDocumentURI, aTitle, aContentPrincipal, aContentPartitionedPrincipal, aCSP, aReferrerInfo, aIsSynthetic, aHasRequestContextID, aRequestContextID, aContentType)
    }



    /// `readonly attribute nsIBrowser_ProcessBehavior processSwitchBehavior;`
    #[inline]
    pub unsafe fn GetProcessSwitchBehavior(&self, aProcessSwitchBehavior: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessSwitchBehavior)(self, aProcessSwitchBehavior)
    }


    /// ```text
    /// /**
    ///    * Called to perform any async tasks which must be completed before changing
    ///    * remoteness. Gecko will wait for the returned promise to resolve before
    ///    * performing the process switch.
    ///    */
    /// ```
    ///

    /// `Promise prepareToChangeRemoteness ();`
    const _PrepareToChangeRemoteness: () = ();

    /// ```text
    /// /** Called immediately before changing remoteness */
    /// ```
    ///

    /// `void beforeChangeRemoteness ();`
    #[inline]
    pub unsafe fn BeforeChangeRemoteness(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeforeChangeRemoteness)(self, )
    }


    /// ```text
    /// /**
    ///    * Called immediately after changing remoteness.
    ///    *
    ///    * If this method returns `true`, Gecko will assume frontend handled resuming
    ///    * the load, and will not attempt to resume the load itself.
    ///    */
    /// ```
    ///

    /// `bool finishChangeRemoteness (in uint64_t aPendingSwitchId);`
    #[inline]
    pub unsafe fn FinishChangeRemoteness(&self, aPendingSwitchId: uint64_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).FinishChangeRemoteness)(self, aPendingSwitchId, _retval)
    }


}



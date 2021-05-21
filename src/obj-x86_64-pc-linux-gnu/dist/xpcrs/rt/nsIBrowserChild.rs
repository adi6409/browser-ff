//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIBrowserChild.idl
//


/// `interface nsIBrowserChild : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserChild {
    vtable: *const nsIBrowserChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserChild.
unsafe impl XpCom for nsIBrowserChild {
    const IID: nsIID = nsID(0x1fb79c27, 0xe760, 0x4088,
        [0xb1, 0x9c, 0x1c, 0xe3, 0x67, 0x3e, 0xc2, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserChild {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserChild.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserChildCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserChild`.
    fn coerce_from(v: &nsIBrowserChild) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserChildCoerce for nsIBrowserChild {
    #[inline]
    fn coerce_from(v: &nsIBrowserChild) -> &Self {
        v
    }
}

impl nsIBrowserChild {
    /// Cast this `nsIBrowserChild` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserChild {
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
impl<T: nsISupportsCoerce> nsIBrowserChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserChild) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserChild
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserChildVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ContentFrameMessageManager messageManager; */
    pub GetMessageManager: unsafe extern "system" fn (this: *const nsIBrowserChild, aMessageManager: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute nsIWebBrowserChrome3 webBrowserChrome; */
    pub GetWebBrowserChrome: unsafe extern "system" fn (this: *const nsIBrowserChild, aWebBrowserChrome: *mut*const nsIWebBrowserChrome3) -> ::nserror::nsresult,

    /* attribute nsIWebBrowserChrome3 webBrowserChrome; */
    pub SetWebBrowserChrome: unsafe extern "system" fn (this: *const nsIBrowserChild, aWebBrowserChrome: *const nsIWebBrowserChrome3) -> ::nserror::nsresult,

    /* [notxpcom] void sendRequestFocus (in boolean canFocus, in CallerType aCallerType); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SendRequestFocus: *const ::libc::c_void,

    /* [noscript] void remoteSizeShellTo (in int32_t width, in int32_t height, in int32_t shellItemWidth, in int32_t shellItemHeight); */
    pub RemoteSizeShellTo: unsafe extern "system" fn (this: *const nsIBrowserChild, width: int32_t, height: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> ::nserror::nsresult,

    /* void remoteDropLinks (in Array<nsIDroppedLinkItem> links); */
    pub RemoteDropLinks: unsafe extern "system" fn (this: *const nsIBrowserChild, links: *const thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult,

    /* readonly attribute uint64_t tabId; */
    pub GetTabId: unsafe extern "system" fn (this: *const nsIBrowserChild, aTabId: *mut uint64_t) -> ::nserror::nsresult,

    /* attribute boolean hasSiblings; */
    pub GetHasSiblings: unsafe extern "system" fn (this: *const nsIBrowserChild, aHasSiblings: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean hasSiblings; */
    pub SetHasSiblings: unsafe extern "system" fn (this: *const nsIBrowserChild, aHasSiblings: bool) -> ::nserror::nsresult,

    /* void beginSendingWebProgressEventsToParent (); */
    pub BeginSendingWebProgressEventsToParent: unsafe extern "system" fn (this: *const nsIBrowserChild) -> ::nserror::nsresult,

    /* void notifyNavigationFinished (); */
    pub NotifyNavigationFinished: unsafe extern "system" fn (this: *const nsIBrowserChild) -> ::nserror::nsresult,

    /* readonly attribute uint64_t chromeOuterWindowID; */
    pub GetChromeOuterWindowID: unsafe extern "system" fn (this: *const nsIBrowserChild, aChromeOuterWindowID: *mut uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserChild {


    /// `readonly attribute ContentFrameMessageManager messageManager;`
    #[inline]
    pub unsafe fn GetMessageManager(&self, aMessageManager: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetMessageManager)(self, aMessageManager)
    }



    /// `attribute nsIWebBrowserChrome3 webBrowserChrome;`
    #[inline]
    pub unsafe fn GetWebBrowserChrome(&self, aWebBrowserChrome: *mut*const nsIWebBrowserChrome3) -> ::nserror::nsresult {
        ((*self.vtable).GetWebBrowserChrome)(self, aWebBrowserChrome)
    }



    /// `attribute nsIWebBrowserChrome3 webBrowserChrome;`
    #[inline]
    pub unsafe fn SetWebBrowserChrome(&self, aWebBrowserChrome: *const nsIWebBrowserChrome3) -> ::nserror::nsresult {
        ((*self.vtable).SetWebBrowserChrome)(self, aWebBrowserChrome)
    }



    /// `[notxpcom] void sendRequestFocus (in boolean canFocus, in CallerType aCallerType);`
    const _SendRequestFocus: () = ();


    /// `[noscript] void remoteSizeShellTo (in int32_t width, in int32_t height, in int32_t shellItemWidth, in int32_t shellItemHeight);`
    #[inline]
    pub unsafe fn RemoteSizeShellTo(&self, width: int32_t, height: int32_t, shellItemWidth: int32_t, shellItemHeight: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).RemoteSizeShellTo)(self, width, height, shellItemWidth, shellItemHeight)
    }



    /// `void remoteDropLinks (in Array<nsIDroppedLinkItem> links);`
    #[inline]
    pub unsafe fn RemoteDropLinks(&self, links: *const thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult {
        ((*self.vtable).RemoteDropLinks)(self, links)
    }



    /// `readonly attribute uint64_t tabId;`
    #[inline]
    pub unsafe fn GetTabId(&self, aTabId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTabId)(self, aTabId)
    }



    /// `attribute boolean hasSiblings;`
    #[inline]
    pub unsafe fn GetHasSiblings(&self, aHasSiblings: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasSiblings)(self, aHasSiblings)
    }



    /// `attribute boolean hasSiblings;`
    #[inline]
    pub unsafe fn SetHasSiblings(&self, aHasSiblings: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHasSiblings)(self, aHasSiblings)
    }


    /// ```text
    /// /**
    ///    * Tell the nsIBrowserChild that it should begin sending its nsIWebProgress
    ///    * events to its nsIBrowserParent.
    ///    *
    ///    * This should be called once the frame script for the nsIBrowserChild has
    ///    * loaded.
    ///    */
    /// ```
    ///

    /// `void beginSendingWebProgressEventsToParent ();`
    #[inline]
    pub unsafe fn BeginSendingWebProgressEventsToParent(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginSendingWebProgressEventsToParent)(self, )
    }


    /// ```text
    /// /**
    ///    * Send a message from the BrowserChild to the BrowserParent that a
    ///    * nsIWebNavigation navigation finished in the child.
    ///    */
    /// ```
    ///

    /// `void notifyNavigationFinished ();`
    #[inline]
    pub unsafe fn NotifyNavigationFinished(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyNavigationFinished)(self, )
    }


    /// ```text
    /// /**
    ///    * Id of the chrome window the tab is within.
    ///    */
    /// ```
    ///

    /// `readonly attribute uint64_t chromeOuterWindowID;`
    #[inline]
    pub unsafe fn GetChromeOuterWindowID(&self, aChromeOuterWindowID: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeOuterWindowID)(self, aChromeOuterWindowID)
    }


}



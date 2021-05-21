//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsILoadContext.idl
//


/// `interface nsILoadContext : nsISupports`
///

/// ```text
/// /**
///  * An nsILoadContext represents the context of a load.  This interface
///  * can be queried for various information about where the load is
///  * happening.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadContext {
    vtable: *const nsILoadContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadContext.
unsafe impl XpCom for nsILoadContext {
    const IID: nsIID = nsID(0x2813a7a3, 0xd084, 0x4d00,
        [0xac, 0xd0, 0xf7, 0x66, 0x20, 0x31, 0x5c, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadContext {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadContext.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadContextCoerce {
    /// Cheaply cast a value of this type from a `nsILoadContext`.
    fn coerce_from(v: &nsILoadContext) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadContextCoerce for nsILoadContext {
    #[inline]
    fn coerce_from(v: &nsILoadContext) -> &Self {
        v
    }
}

impl nsILoadContext {
    /// Cast this `nsILoadContext` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadContext {
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
impl<T: nsISupportsCoerce> nsILoadContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContext) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadContext
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadContextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute mozIDOMWindowProxy associatedWindow; */
    pub GetAssociatedWindow: unsafe extern "system" fn (this: *const nsILoadContext, aAssociatedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy topWindow; */
    pub GetTopWindow: unsafe extern "system" fn (this: *const nsILoadContext, aTopWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute Element topFrameElement; */
    pub GetTopFrameElement: unsafe extern "system" fn (this: *const nsILoadContext, aTopFrameElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute boolean isContent; */
    pub GetIsContent: unsafe extern "system" fn (this: *const nsILoadContext, aIsContent: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean usePrivateBrowsing; */
    pub GetUsePrivateBrowsing: unsafe extern "system" fn (this: *const nsILoadContext, aUsePrivateBrowsing: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean usePrivateBrowsing; */
    pub SetUsePrivateBrowsing: unsafe extern "system" fn (this: *const nsILoadContext, aUsePrivateBrowsing: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean useRemoteTabs; */
    pub GetUseRemoteTabs: unsafe extern "system" fn (this: *const nsILoadContext, aUseRemoteTabs: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean useRemoteSubframes; */
    pub GetUseRemoteSubframes: unsafe extern "system" fn (this: *const nsILoadContext, aUseRemoteSubframes: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useTrackingProtection; */
    pub GetUseTrackingProtection: unsafe extern "system" fn (this: *const nsILoadContext, aUseTrackingProtection: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useTrackingProtection; */
    pub SetUseTrackingProtection: unsafe extern "system" fn (this: *const nsILoadContext, aUseTrackingProtection: bool) -> ::nserror::nsresult,

    /* [noscript] void SetPrivateBrowsing (in boolean aInPrivateBrowsing); */
    pub SetPrivateBrowsing: unsafe extern "system" fn (this: *const nsILoadContext, aInPrivateBrowsing: bool) -> ::nserror::nsresult,

    /* [noscript] void SetRemoteTabs (in boolean aUseRemoteTabs); */
    pub SetRemoteTabs: unsafe extern "system" fn (this: *const nsILoadContext, aUseRemoteTabs: bool) -> ::nserror::nsresult,

    /* [noscript] void SetRemoteSubframes (in boolean aUseRemoteSubframes); */
    pub SetRemoteSubframes: unsafe extern "system" fn (this: *const nsILoadContext, aUseRemoteSubframes: bool) -> ::nserror::nsresult,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetScriptableOriginAttributes: *const ::libc::c_void,

    /* [noscript,notxpcom] void GetOriginAttributes (out OriginAttributes aAttrs); */
    /// Unable to generate binding because `native type mozilla::OriginAttributes unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadContext {

    /// ```text
    /// /**
    ///    * associatedWindow is the window with which the load is associated, if any.
    ///    * Note that the load may be triggered by a document which is different from
    ///    * the document in associatedWindow, and in fact the source of the load need
    ///    * not be same-origin with the document in associatedWindow.  This attribute
    ///    * may be null if there is no associated window.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy associatedWindow;`
    #[inline]
    pub unsafe fn GetAssociatedWindow(&self, aAssociatedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetAssociatedWindow)(self, aAssociatedWindow)
    }


    /// ```text
    /// /**
    ///    * topWindow is the top window which is of same type as associatedWindow.
    ///    * This is equivalent to associatedWindow.top, but is provided here as a
    ///    * convenience.  All the same caveats as associatedWindow of apply, of
    ///    * course.  This attribute may be null if there is no associated window.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy topWindow;`
    #[inline]
    pub unsafe fn GetTopWindow(&self, aTopWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetTopWindow)(self, aTopWindow)
    }


    /// ```text
    /// /**
    ///    * topFrameElement is the <iframe>, <frame>, or <browser> element which
    ///    * contains the topWindow with which the load is associated.
    ///    *
    ///    * Note that we may have a topFrameElement even when we don't have an
    ///    * associatedWindow, if the topFrameElement's content lives out of process.
    ///    * topFrameElement is available in single-process and multiprocess contexts.
    ///    * Note that topFrameElement may be in chrome even when the nsILoadContext is
    ///    * associated with content.
    ///    */
    /// ```
    ///

    /// `readonly attribute Element topFrameElement;`
    #[inline]
    pub unsafe fn GetTopFrameElement(&self, aTopFrameElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetTopFrameElement)(self, aTopFrameElement)
    }


    /// ```text
    /// /**
    ///    * True if the load context is content (as opposed to chrome).  This is
    ///    * determined based on the type of window the load is performed in, NOT based
    ///    * on any URIs that might be around.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isContent;`
    #[inline]
    pub unsafe fn GetIsContent(&self, aIsContent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsContent)(self, aIsContent)
    }



    /// `attribute boolean usePrivateBrowsing;`
    #[inline]
    pub unsafe fn GetUsePrivateBrowsing(&self, aUsePrivateBrowsing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUsePrivateBrowsing)(self, aUsePrivateBrowsing)
    }



    /// `attribute boolean usePrivateBrowsing;`
    #[inline]
    pub unsafe fn SetUsePrivateBrowsing(&self, aUsePrivateBrowsing: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUsePrivateBrowsing)(self, aUsePrivateBrowsing)
    }


    /// ```text
    /// /**
    ///    * Attribute that determines if remote (out-of-process) tabs should be used.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean useRemoteTabs;`
    #[inline]
    pub unsafe fn GetUseRemoteTabs(&self, aUseRemoteTabs: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseRemoteTabs)(self, aUseRemoteTabs)
    }


    /// ```text
    /// /**
    ///    * Determines if out-of-process iframes should be used.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean useRemoteSubframes;`
    #[inline]
    pub unsafe fn GetUseRemoteSubframes(&self, aUseRemoteSubframes: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseRemoteSubframes)(self, aUseRemoteSubframes)
    }



    /// `attribute boolean useTrackingProtection;`
    #[inline]
    pub unsafe fn GetUseTrackingProtection(&self, aUseTrackingProtection: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseTrackingProtection)(self, aUseTrackingProtection)
    }



    /// `attribute boolean useTrackingProtection;`
    #[inline]
    pub unsafe fn SetUseTrackingProtection(&self, aUseTrackingProtection: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUseTrackingProtection)(self, aUseTrackingProtection)
    }


    /// ```text
    /// /**
    ///    * Set the private browsing state of the load context, meant to be used internally.
    ///    */
    /// ```
    ///

    /// `[noscript] void SetPrivateBrowsing (in boolean aInPrivateBrowsing);`
    #[inline]
    pub unsafe fn SetPrivateBrowsing(&self, aInPrivateBrowsing: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrivateBrowsing)(self, aInPrivateBrowsing)
    }


    /// ```text
    /// /**
    ///    * Set the remote tabs state of the load context, meant to be used internally.
    ///    */
    /// ```
    ///

    /// `[noscript] void SetRemoteTabs (in boolean aUseRemoteTabs);`
    #[inline]
    pub unsafe fn SetRemoteTabs(&self, aUseRemoteTabs: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetRemoteTabs)(self, aUseRemoteTabs)
    }


    /// ```text
    /// /**
    ///    * Set the remote subframes bit of this load context. Exclusively meant to be used internally.
    ///    */
    /// ```
    ///

    /// `[noscript] void SetRemoteSubframes (in boolean aUseRemoteSubframes);`
    #[inline]
    pub unsafe fn SetRemoteSubframes(&self, aUseRemoteSubframes: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetRemoteSubframes)(self, aUseRemoteSubframes)
    }


    /// ```text
    /// /**
    ///    * A dictionary of the non-default origin attributes associated with this
    ///    * nsILoadContext.
    ///    */
    /// ```
    ///

    /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes;`
    const _GetScriptableOriginAttributes: () = ();

    /// ```text
    /// /**
    ///    * The C++ getter for origin attributes.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void GetOriginAttributes (out OriginAttributes aAttrs);`
    const _GetOriginAttributes: () = ();

}



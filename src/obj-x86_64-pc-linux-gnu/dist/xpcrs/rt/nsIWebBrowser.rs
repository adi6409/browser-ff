//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowser.idl
//


/// `interface nsIWebBrowser : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebBrowser interface is implemented by web browser objects.
///  * Embedders use this interface during initialisation to associate
///  * the new web browser instance with the embedders chrome and
///  * to register any listeners. The interface may also be used at runtime
///  * to obtain the content DOM window and from that the rest of the DOM.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowser {
    vtable: *const nsIWebBrowserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowser.
unsafe impl XpCom for nsIWebBrowser {
    const IID: nsIID = nsID(0x4052b6da, 0x4faa, 0x4646,
        [0xb3, 0xa1, 0x7e, 0x16, 0xa0, 0x1c, 0x2d, 0xc2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowser`.
    fn coerce_from(v: &nsIWebBrowser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserCoerce for nsIWebBrowser {
    #[inline]
    fn coerce_from(v: &nsIWebBrowser) -> &Self {
        v
    }
}

impl nsIWebBrowser {
    /// Cast this `nsIWebBrowser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowser {
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
impl<T: nsISupportsCoerce> nsIWebBrowserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute nsIWebBrowserChrome containerWindow; */
    pub GetContainerWindow: unsafe extern "system" fn (this: *const nsIWebBrowser, aContainerWindow: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult,

    /* attribute nsIWebBrowserChrome containerWindow; */
    pub SetContainerWindow: unsafe extern "system" fn (this: *const nsIWebBrowser, aContainerWindow: *const nsIWebBrowserChrome) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy contentDOMWindow; */
    pub GetContentDOMWindow: unsafe extern "system" fn (this: *const nsIWebBrowser, aContentDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* [binaryname(SetOriginAttributes),noscript,nostdcall,notxpcom] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub SetOriginAttributes: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowser {

    /// ```text
    /// /**
    ///      * The chrome object associated with the browser instance. The embedder
    ///      * must create one chrome object for <I>each</I> browser object
    ///      * that is instantiated. The embedder must associate the two by setting
    ///      * this property to point to the chrome object before creating the browser
    ///      * window via the browser's <CODE>nsIBaseWindow</CODE> interface.
    ///      *
    ///      * The chrome object must also implement <CODE>nsIEmbeddingSiteWindow</CODE>.
    ///      *
    ///      * The chrome may optionally implement <CODE>nsIInterfaceRequestor</CODE>,
    ///      * <CODE>nsIWebBrowserChromeFocus</CODE>,
    ///      * <CODE>nsIContextMenuListener</CODE> and
    ///      * <CODE>nsITooltipListener</CODE> to receive additional notifications
    ///      * from the browser object.
    ///      *
    ///      * The chrome object may optionally implement
    ///      * <CODE>nsIWebProgressListener</CODE> to register a progress listener
    ///      * object. If the implementation does this, it must also implement
    ///      * <CODE>nsIWeakReference</CODE>.
    ///      *
    ///      * @note The implementation should not refcount the supplied chrome
    ///      *       object; it should assume that a non <CODE>nullptr</CODE> value is
    ///      *       always valid. The embedder must explicitly set this value back
    ///      *       to nullptr if the chrome object is destroyed before the browser
    ///      *       object.
    ///      *
    ///      * @see nsIBaseWindow
    ///      * @see nsIWebBrowserChrome
    ///      * @see nsIEmbeddingSiteWindow
    ///      * @see nsIInterfaceRequestor
    ///      * @see nsIWebBrowserChromeFocus
    ///      * @see nsIContextMenuListener
    ///      * @see nsITooltipListener
    ///      * @see nsIWeakReference
    ///      * @see nsIWebProgressListener
    ///      */
    /// ```
    ///

    /// `attribute nsIWebBrowserChrome containerWindow;`
    #[inline]
    pub unsafe fn GetContainerWindow(&self, aContainerWindow: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult {
        ((*self.vtable).GetContainerWindow)(self, aContainerWindow)
    }


    /// ```text
    /// /**
    ///      * The chrome object associated with the browser instance. The embedder
    ///      * must create one chrome object for <I>each</I> browser object
    ///      * that is instantiated. The embedder must associate the two by setting
    ///      * this property to point to the chrome object before creating the browser
    ///      * window via the browser's <CODE>nsIBaseWindow</CODE> interface.
    ///      *
    ///      * The chrome object must also implement <CODE>nsIEmbeddingSiteWindow</CODE>.
    ///      *
    ///      * The chrome may optionally implement <CODE>nsIInterfaceRequestor</CODE>,
    ///      * <CODE>nsIWebBrowserChromeFocus</CODE>,
    ///      * <CODE>nsIContextMenuListener</CODE> and
    ///      * <CODE>nsITooltipListener</CODE> to receive additional notifications
    ///      * from the browser object.
    ///      *
    ///      * The chrome object may optionally implement
    ///      * <CODE>nsIWebProgressListener</CODE> to register a progress listener
    ///      * object. If the implementation does this, it must also implement
    ///      * <CODE>nsIWeakReference</CODE>.
    ///      *
    ///      * @note The implementation should not refcount the supplied chrome
    ///      *       object; it should assume that a non <CODE>nullptr</CODE> value is
    ///      *       always valid. The embedder must explicitly set this value back
    ///      *       to nullptr if the chrome object is destroyed before the browser
    ///      *       object.
    ///      *
    ///      * @see nsIBaseWindow
    ///      * @see nsIWebBrowserChrome
    ///      * @see nsIEmbeddingSiteWindow
    ///      * @see nsIInterfaceRequestor
    ///      * @see nsIWebBrowserChromeFocus
    ///      * @see nsIContextMenuListener
    ///      * @see nsITooltipListener
    ///      * @see nsIWeakReference
    ///      * @see nsIWebProgressListener
    ///      */
    /// ```
    ///

    /// `attribute nsIWebBrowserChrome containerWindow;`
    #[inline]
    pub unsafe fn SetContainerWindow(&self, aContainerWindow: *const nsIWebBrowserChrome) -> ::nserror::nsresult {
        ((*self.vtable).SetContainerWindow)(self, aContainerWindow)
    }


    /// ```text
    /// /**
    ///      * The top-level DOM window. The embedder may walk the entire
    ///      * DOM starting from this value.
    ///      */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy contentDOMWindow;`
    #[inline]
    pub unsafe fn GetContentDOMWindow(&self, aContentDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetContentDOMWindow)(self, aContentDOMWindow)
    }


    /// ```text
    /// /**
    ///      * Set Origin Attributes on the nsIWebBrowser.
    ///      * The Origin Attributes will be passed to the docshell once it has been
    ///      * created
    ///      */
    /// ```
    ///

    /// `[binaryname(SetOriginAttributes),noscript,nostdcall,notxpcom] void binarySetOriginAttributes (in const_OriginAttributesRef aOriginAttrs);`
    const _SetOriginAttributes: () = ();

}



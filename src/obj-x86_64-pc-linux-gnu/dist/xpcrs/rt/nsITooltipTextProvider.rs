//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipTextProvider.idl
//


/// `interface nsITooltipTextProvider : nsISupports`
///

/// ```text
/// /**
///  * An interface implemented by a tooltip text provider service. This
///  * service is called to discover what tooltip text is associated
///  * with the node that the pointer is positioned over.
///  *
///  * Embedders may implement and register their own tooltip text provider
///  * service if they wish to provide different tooltip text.
///  *
///  * The default service returns the text stored in the TITLE
///  * attribute of the node or a containing parent.
///  *
///  * @note
///  * The tooltip text provider service is registered with the contract
///  * defined in NS_TOOLTIPTEXTPROVIDER_CONTRACTID.
///  *
///  * @see nsITooltipListener
///  * @see nsIComponentManager
///  * @see Node
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITooltipTextProvider {
    vtable: *const nsITooltipTextProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITooltipTextProvider.
unsafe impl XpCom for nsITooltipTextProvider {
    const IID: nsIID = nsID(0xb128a1e6, 0x44f3, 0x4331,
        [0x8f, 0xbe, 0x5a, 0xf3, 0x60, 0xff, 0x21, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITooltipTextProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITooltipTextProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITooltipTextProviderCoerce {
    /// Cheaply cast a value of this type from a `nsITooltipTextProvider`.
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITooltipTextProviderCoerce for nsITooltipTextProvider {
    #[inline]
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self {
        v
    }
}

impl nsITooltipTextProvider {
    /// Cast this `nsITooltipTextProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITooltipTextProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITooltipTextProvider {
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
impl<T: nsISupportsCoerce> nsITooltipTextProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITooltipTextProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITooltipTextProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITooltipTextProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean getNodeText (in Node aNode, out wstring aText, out wstring aDirection); */
    pub GetNodeText: unsafe extern "system" fn (this: *const nsITooltipTextProvider, aNode: *const libc::c_void, aText: *mut *const i16, aDirection: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITooltipTextProvider {

    /// ```text
    /// /**
    ///      * Called to obtain the tooltip text for a node.
    ///      *
    ///      * @arg aNode      The node to obtain the text from.
    ///      * @arg aText      The tooltip text.
    ///      * @arg aDirection The text direction (ltr or rtl) to use
    ///      *
    ///      * @return <CODE>PR_TRUE</CODE> if tooltip text is associated
    ///      *         with the node and was returned in the aText argument;
    ///      *         <CODE>PR_FALSE</CODE> otherwise.
    ///      */
    /// ```
    ///

    /// `boolean getNodeText (in Node aNode, out wstring aText, out wstring aDirection);`
    #[inline]
    pub unsafe fn GetNodeText(&self, aNode: *const libc::c_void, aText: *mut *const i16, aDirection: *mut *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNodeText)(self, aNode, aText, aDirection, _retval)
    }


}



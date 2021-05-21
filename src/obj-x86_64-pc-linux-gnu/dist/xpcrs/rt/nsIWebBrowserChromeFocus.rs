//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIWebBrowserChromeFocus.idl
//


/// `interface nsIWebBrowserChromeFocus : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebBrowserChromeFocus is implemented by the same object as the
///  * nsIEmbeddingSiteWindow. It represents the focus up-calls from mozilla
///  * to the embedding chrome. See mozilla bug #70224 for gratuitous info.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebBrowserChromeFocus {
    vtable: *const nsIWebBrowserChromeFocusVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebBrowserChromeFocus.
unsafe impl XpCom for nsIWebBrowserChromeFocus {
    const IID: nsIID = nsID(0x947b2ee6, 0x51ed, 0x4c2b,
        [0x9f, 0x45, 0x42, 0x6c, 0x27, 0xca, 0x84, 0xc6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebBrowserChromeFocus {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebBrowserChromeFocus.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebBrowserChromeFocusCoerce {
    /// Cheaply cast a value of this type from a `nsIWebBrowserChromeFocus`.
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebBrowserChromeFocusCoerce for nsIWebBrowserChromeFocus {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self {
        v
    }
}

impl nsIWebBrowserChromeFocus {
    /// Cast this `nsIWebBrowserChromeFocus` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebBrowserChromeFocusCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebBrowserChromeFocus {
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
impl<T: nsISupportsCoerce> nsIWebBrowserChromeFocusCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebBrowserChromeFocus) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebBrowserChromeFocus
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebBrowserChromeFocusVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void focusNextElement (in bool aForDocumentNavigation); */
    pub FocusNextElement: unsafe extern "system" fn (this: *const nsIWebBrowserChromeFocus, aForDocumentNavigation: bool) -> ::nserror::nsresult,

    /* void focusPrevElement (in bool aForDocumentNavigation); */
    pub FocusPrevElement: unsafe extern "system" fn (this: *const nsIWebBrowserChromeFocus, aForDocumentNavigation: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebBrowserChromeFocus {

    /// ```text
    /// /**
    ///      * Set the focus at the next focusable element in the chrome. If
    ///      * aForDocumentNavigation is true, this was a document navigation, so
    ///      * focus the parent window.
    ///      */
    /// ```
    ///

    /// `void focusNextElement (in bool aForDocumentNavigation);`
    #[inline]
    pub unsafe fn FocusNextElement(&self, aForDocumentNavigation: bool) -> ::nserror::nsresult {
        ((*self.vtable).FocusNextElement)(self, aForDocumentNavigation)
    }


    /// ```text
    /// /**
    ///      * Set the focus at the previous focusable element in the chrome.
    ///      */
    /// ```
    ///

    /// `void focusPrevElement (in bool aForDocumentNavigation);`
    #[inline]
    pub unsafe fn FocusPrevElement(&self, aForDocumentNavigation: bool) -> ::nserror::nsresult {
        ((*self.vtable).FocusPrevElement)(self, aForDocumentNavigation)
    }


}



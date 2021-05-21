//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIWebPageDescriptor.idl
//


/// `interface nsIWebPageDescriptor : nsISupports`
///

/// ```text
/// /**
///  * The nsIWebPageDescriptor interface allows content being displayed in one
///  * window to be loaded into another window without refetching it from the
///  * network.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebPageDescriptor {
    vtable: *const nsIWebPageDescriptorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebPageDescriptor.
unsafe impl XpCom for nsIWebPageDescriptor {
    const IID: nsIID = nsID(0x6f30b676, 0x3710, 0x4c2c,
        [0x80, 0xb1, 0x03, 0x95, 0xfb, 0x26, 0x51, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebPageDescriptor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebPageDescriptor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebPageDescriptorCoerce {
    /// Cheaply cast a value of this type from a `nsIWebPageDescriptor`.
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebPageDescriptorCoerce for nsIWebPageDescriptor {
    #[inline]
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self {
        v
    }
}

impl nsIWebPageDescriptor {
    /// Cast this `nsIWebPageDescriptor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebPageDescriptorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebPageDescriptor {
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
impl<T: nsISupportsCoerce> nsIWebPageDescriptorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebPageDescriptor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebPageDescriptor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebPageDescriptorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void loadPageAsViewSource (in nsIDocShell otherDocShell, in AString aURL); */
    pub LoadPageAsViewSource: unsafe extern "system" fn (this: *const nsIWebPageDescriptor, otherDocShell: *const nsIDocShell, aURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsISupports currentDescriptor; */
    pub GetCurrentDescriptor: unsafe extern "system" fn (this: *const nsIWebPageDescriptor, aCurrentDescriptor: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebPageDescriptor {

    /// ```text
    /// /**
    ///   * Tells the object to load the page that otherDocShell is currently loading,
    ///   * or has loaded already, as view source, with the url being `aURL`.
    ///   *
    ///   * @throws NS_ERROR_FAILURE - NS_ERROR_INVALID_POINTER
    ///   */
    /// ```
    ///

    /// `void loadPageAsViewSource (in nsIDocShell otherDocShell, in AString aURL);`
    #[inline]
    pub unsafe fn LoadPageAsViewSource(&self, otherDocShell: *const nsIDocShell, aURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).LoadPageAsViewSource)(self, otherDocShell, aURL)
    }


    /// ```text
    /// /**
    ///   * Retrieves the page descriptor for the curent document.
    ///   * @note, currentDescriptor is currently always an nsISHEntry object or null.
    ///   */
    /// ```
    ///

    /// `readonly attribute nsISupports currentDescriptor;`
    #[inline]
    pub unsafe fn GetCurrentDescriptor(&self, aCurrentDescriptor: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentDescriptor)(self, aCurrentDescriptor)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocumentLoaderFactory.idl
//


/// `interface nsIDocumentLoaderFactory : nsISupports`
///

/// ```text
/// /**
///  * To get a component that implements nsIDocumentLoaderFactory
///  * for a given mimetype, use nsICategoryManager to find an entry
///  * with the mimetype as its name in the category "Gecko-Content-Viewers".
///  * The value of the entry is the contractid of the component.
///  * The component is a service, so use GetService, not CreateInstance to get it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocumentLoaderFactory {
    vtable: *const nsIDocumentLoaderFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocumentLoaderFactory.
unsafe impl XpCom for nsIDocumentLoaderFactory {
    const IID: nsIID = nsID(0xe795239e, 0x9d3c, 0x47c4,
        [0xb0, 0x63, 0x9e, 0x60, 0x0f, 0xb3, 0xb2, 0x87]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocumentLoaderFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocumentLoaderFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocumentLoaderFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIDocumentLoaderFactory`.
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocumentLoaderFactoryCoerce for nsIDocumentLoaderFactory {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self {
        v
    }
}

impl nsIDocumentLoaderFactory {
    /// Cast this `nsIDocumentLoaderFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocumentLoaderFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocumentLoaderFactory {
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
impl<T: nsISupportsCoerce> nsIDocumentLoaderFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentLoaderFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocumentLoaderFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocumentLoaderFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult); */
    pub CreateInstance: unsafe extern "system" fn (this: *const nsIDocumentLoaderFactory, aCommand: *const libc::c_char, aChannel: *const nsIChannel, aLoadGroup: *const nsILoadGroup, aContentType: *const ::nsstring::nsACString, aContainer: *const nsIDocShell, aExtraInfo: *const nsISupports, aDocListenerResult: *mut*const nsIStreamListener, _retval: *mut*const nsIContentViewer) -> ::nserror::nsresult,

    /* nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in Document aDocument, in string aCommand); */
    pub CreateInstanceForDocument: unsafe extern "system" fn (this: *const nsIDocumentLoaderFactory, aContainer: *const nsISupports, aDocument: *const libc::c_void, aCommand: *const libc::c_char, _retval: *mut*const nsIContentViewer) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocumentLoaderFactory {


    /// `nsIContentViewer createInstance (in string aCommand, in nsIChannel aChannel, in nsILoadGroup aLoadGroup, in ACString aContentType, in nsIDocShell aContainer, in nsISupports aExtraInfo, out nsIStreamListener aDocListenerResult);`
    #[inline]
    pub unsafe fn CreateInstance(&self, aCommand: *const libc::c_char, aChannel: *const nsIChannel, aLoadGroup: *const nsILoadGroup, aContentType: *const ::nsstring::nsACString, aContainer: *const nsIDocShell, aExtraInfo: *const nsISupports, aDocListenerResult: *mut*const nsIStreamListener, _retval: *mut*const nsIContentViewer) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstance)(self, aCommand, aChannel, aLoadGroup, aContentType, aContainer, aExtraInfo, aDocListenerResult, _retval)
    }



    /// `nsIContentViewer createInstanceForDocument (in nsISupports aContainer, in Document aDocument, in string aCommand);`
    #[inline]
    pub unsafe fn CreateInstanceForDocument(&self, aContainer: *const nsISupports, aDocument: *const libc::c_void, aCommand: *const libc::c_char, _retval: *mut*const nsIContentViewer) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstanceForDocument)(self, aContainer, aDocument, aCommand, _retval)
    }


}



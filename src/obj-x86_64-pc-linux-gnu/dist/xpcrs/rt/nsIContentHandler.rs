//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIContentHandler.idl
//


/// `interface nsIContentHandler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentHandler {
    vtable: *const nsIContentHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentHandler.
unsafe impl XpCom for nsIContentHandler {
    const IID: nsIID = nsID(0x49439df2, 0xb3d2, 0x441c,
        [0xbf, 0x62, 0x86, 0x6b, 0xda, 0xf5, 0x6f, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIContentHandler`.
    fn coerce_from(v: &nsIContentHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentHandlerCoerce for nsIContentHandler {
    #[inline]
    fn coerce_from(v: &nsIContentHandler) -> &Self {
        v
    }
}

impl nsIContentHandler {
    /// Cast this `nsIContentHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentHandler {
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
impl<T: nsISupportsCoerce> nsIContentHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest); */
    pub HandleContent: unsafe extern "system" fn (this: *const nsIContentHandler, aContentType: *const libc::c_char, aWindowContext: *const nsIInterfaceRequestor, aRequest: *const nsIRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentHandler {

    /// ```text
    /// /**
    ///    * Tells the content handler to take over handling the content. If this
    ///    * function succeeds, the URI Loader will leave this request alone, ignoring
    ///    * progress notifications. Failure of this method will cause the request to be
    ///    * cancelled, unless the error code is NS_ERROR_WONT_HANDLE_CONTENT (see
        ///    * below).
    ///    *
    ///    * @param aWindowContext
    ///    *        Window context, used to get things like the current nsIDOMWindow
    ///    *        for this request. May be null.
    ///    * @param aContentType
    ///    *        The content type of aRequest
    ///    * @param aRequest
    ///    *        A request whose content type is already known.
    ///    *
    ///    * @throw NS_ERROR_WONT_HANDLE_CONTENT Indicates that this handler does not
    ///    *        want to handle this content. A different way for handling this
    ///    *        content should be tried.
    ///    */
    /// ```
    ///

    /// `void handleContent (in string aContentType, in nsIInterfaceRequestor aWindowContext, in nsIRequest aRequest);`
    #[inline]
    pub unsafe fn HandleContent(&self, aContentType: *const libc::c_char, aWindowContext: *const nsIInterfaceRequestor, aRequest: *const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).HandleContent)(self, aContentType, aWindowContext, aRequest)
    }


}



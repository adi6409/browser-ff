//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIMozBrowserFrame.idl
//


/// `interface nsIMozBrowserFrame : nsIDOMMozBrowserFrame`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMozBrowserFrame {
    vtable: *const nsIMozBrowserFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMozBrowserFrame.
unsafe impl XpCom for nsIMozBrowserFrame {
    const IID: nsIID = nsID(0x0c0a862c, 0x1a47, 0x43c0,
        [0xae, 0x9e, 0xd5, 0x18, 0x35, 0xe3, 0xe1, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMozBrowserFrame {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMozBrowserFrame.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMozBrowserFrameCoerce {
    /// Cheaply cast a value of this type from a `nsIMozBrowserFrame`.
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMozBrowserFrameCoerce for nsIMozBrowserFrame {
    #[inline]
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self {
        v
    }
}

impl nsIMozBrowserFrame {
    /// Cast this `nsIMozBrowserFrame` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMozBrowserFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMozBrowserFrame {
    type Target = nsIDOMMozBrowserFrame;
    #[inline]
    fn deref(&self) -> &nsIDOMMozBrowserFrame {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDOMMozBrowserFrameCoerce> nsIMozBrowserFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMozBrowserFrame) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMozBrowserFrame
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMozBrowserFrameVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMMozBrowserFrameVTable,

    /* [infallible] readonly attribute boolean reallyIsBrowser; */
    pub GetReallyIsBrowser: unsafe extern "system" fn (this: *const nsIMozBrowserFrame, aReallyIsBrowser: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void initializeBrowserAPI (); */
    pub InitializeBrowserAPI: unsafe extern "system" fn (this: *const nsIMozBrowserFrame) -> ::nserror::nsresult,

    /* [noscript] void destroyBrowserFrameScripts (); */
    pub DestroyBrowserFrameScripts: unsafe extern "system" fn (this: *const nsIMozBrowserFrame) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMozBrowserFrame {

    /// ```text
    /// /**
    ///    * Gets whether this frame really is a browser frame.
    ///    *
    ///    * In order to really be a browser frame, this frame's
    ///    * nsIDOMMozBrowserFrame::mozbrowser attribute must be true, and the frame
    ///    * may have to pass various security checks.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean reallyIsBrowser;`
    #[inline]
    pub unsafe fn GetReallyIsBrowser(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetReallyIsBrowser)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Initialize the API, and add frame message listener that supports API
    ///    * invocations.
    ///    */
    /// ```
    ///

    /// `[noscript] void initializeBrowserAPI ();`
    #[inline]
    pub unsafe fn InitializeBrowserAPI(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InitializeBrowserAPI)(self, )
    }


    /// ```text
    /// /**
    ///    * Notify frame scripts that support the API to destroy.
    ///    */
    /// ```
    ///

    /// `[noscript] void destroyBrowserFrameScripts ();`
    #[inline]
    pub unsafe fn DestroyBrowserFrameScripts(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DestroyBrowserFrameScripts)(self, )
    }


}



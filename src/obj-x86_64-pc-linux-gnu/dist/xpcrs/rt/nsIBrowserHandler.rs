//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/nsIBrowserHandler.idl
//


/// `interface nsIBrowserHandler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowserHandler {
    vtable: *const nsIBrowserHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowserHandler.
unsafe impl XpCom for nsIBrowserHandler {
    const IID: nsIID = nsID(0x8d3f5a9d, 0x118d, 0x4548,
        [0xa1, 0x37, 0xcf, 0x77, 0x18, 0x67, 0x90, 0x69]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowserHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowserHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowserHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowserHandler`.
    fn coerce_from(v: &nsIBrowserHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowserHandlerCoerce for nsIBrowserHandler {
    #[inline]
    fn coerce_from(v: &nsIBrowserHandler) -> &Self {
        v
    }
}

impl nsIBrowserHandler {
    /// Cast this `nsIBrowserHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowserHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowserHandler {
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
impl<T: nsISupportsCoerce> nsIBrowserHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowserHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowserHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowserHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AUTF8String startPage; */
    pub GetStartPage: unsafe extern "system" fn (this: *const nsIBrowserHandler, aStartPage: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String startPage; */
    pub SetStartPage: unsafe extern "system" fn (this: *const nsIBrowserHandler, aStartPage: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String defaultArgs; */
    pub GetDefaultArgs: unsafe extern "system" fn (this: *const nsIBrowserHandler, aDefaultArgs: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String defaultArgs; */
    pub SetDefaultArgs: unsafe extern "system" fn (this: *const nsIBrowserHandler, aDefaultArgs: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute boolean kiosk; */
    pub GetKiosk: unsafe extern "system" fn (this: *const nsIBrowserHandler, aKiosk: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean kiosk; */
    pub SetKiosk: unsafe extern "system" fn (this: *const nsIBrowserHandler, aKiosk: bool) -> ::nserror::nsresult,

    /* AUTF8String getFeatures (in nsICommandLine aCmdLine); */
    pub GetFeatures: unsafe extern "system" fn (this: *const nsIBrowserHandler, aCmdLine: *const nsICommandLine, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowserHandler {


    /// `attribute AUTF8String startPage;`
    #[inline]
    pub unsafe fn GetStartPage(&self, aStartPage: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetStartPage)(self, aStartPage)
    }



    /// `attribute AUTF8String startPage;`
    #[inline]
    pub unsafe fn SetStartPage(&self, aStartPage: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetStartPage)(self, aStartPage)
    }



    /// `attribute AUTF8String defaultArgs;`
    #[inline]
    pub unsafe fn GetDefaultArgs(&self, aDefaultArgs: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultArgs)(self, aDefaultArgs)
    }



    /// `attribute AUTF8String defaultArgs;`
    #[inline]
    pub unsafe fn SetDefaultArgs(&self, aDefaultArgs: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultArgs)(self, aDefaultArgs)
    }



    /// `attribute boolean kiosk;`
    #[inline]
    pub unsafe fn GetKiosk(&self, aKiosk: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetKiosk)(self, aKiosk)
    }



    /// `attribute boolean kiosk;`
    #[inline]
    pub unsafe fn SetKiosk(&self, aKiosk: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetKiosk)(self, aKiosk)
    }


    /// ```text
    /// /**
    ///    * Extract the width and height specified on the command line, if present.
    ///    * @return A feature string with a prepended comma, e.g. ",width=500,height=400"
    ///    */
    /// ```
    ///

    /// `AUTF8String getFeatures (in nsICommandLine aCmdLine);`
    #[inline]
    pub unsafe fn GetFeatures(&self, aCmdLine: *const nsICommandLine, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatures)(self, aCmdLine, _retval)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgressParams.idl
//


/// `interface nsIPrintProgressParams : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintProgressParams {
    vtable: *const nsIPrintProgressParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintProgressParams.
unsafe impl XpCom for nsIPrintProgressParams {
    const IID: nsIID = nsID(0xca89b55b, 0x6faf, 0x4051,
        [0x96, 0x45, 0x1c, 0x03, 0xef, 0x51, 0x08, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintProgressParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintProgressParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintProgressParamsCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintProgressParams`.
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintProgressParamsCoerce for nsIPrintProgressParams {
    #[inline]
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self {
        v
    }
}

impl nsIPrintProgressParams {
    /// Cast this `nsIPrintProgressParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintProgressParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintProgressParams {
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
impl<T: nsISupportsCoerce> nsIPrintProgressParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintProgressParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintProgressParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintProgressParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString docTitle; */
    pub GetDocTitle: unsafe extern "system" fn (this: *const nsIPrintProgressParams, aDocTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString docTitle; */
    pub SetDocTitle: unsafe extern "system" fn (this: *const nsIPrintProgressParams, aDocTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString docURL; */
    pub GetDocURL: unsafe extern "system" fn (this: *const nsIPrintProgressParams, aDocURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString docURL; */
    pub SetDocURL: unsafe extern "system" fn (this: *const nsIPrintProgressParams, aDocURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintProgressParams {


    /// `attribute AString docTitle;`
    #[inline]
    pub unsafe fn GetDocTitle(&self, aDocTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocTitle)(self, aDocTitle)
    }



    /// `attribute AString docTitle;`
    #[inline]
    pub unsafe fn SetDocTitle(&self, aDocTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDocTitle)(self, aDocTitle)
    }



    /// `attribute AString docURL;`
    #[inline]
    pub unsafe fn GetDocURL(&self, aDocURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocURL)(self, aDocURL)
    }



    /// `attribute AString docURL;`
    #[inline]
    pub unsafe fn SetDocURL(&self, aDocURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDocURL)(self, aDocURL)
    }


}



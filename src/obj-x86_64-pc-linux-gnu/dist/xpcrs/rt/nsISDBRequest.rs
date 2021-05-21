//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBRequest.idl
//


/// `interface nsISDBRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISDBRequest {
    vtable: *const nsISDBRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISDBRequest.
unsafe impl XpCom for nsISDBRequest {
    const IID: nsIID = nsID(0x13f05bcf, 0x715c, 0x427e,
        [0xaa, 0xc8, 0xdf, 0x9b, 0x2c, 0x1e, 0xc1, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISDBRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISDBRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISDBRequestCoerce {
    /// Cheaply cast a value of this type from a `nsISDBRequest`.
    fn coerce_from(v: &nsISDBRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISDBRequestCoerce for nsISDBRequest {
    #[inline]
    fn coerce_from(v: &nsISDBRequest) -> &Self {
        v
    }
}

impl nsISDBRequest {
    /// Cast this `nsISDBRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISDBRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISDBRequest {
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
impl<T: nsISupportsCoerce> nsISDBRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISDBRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISDBRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISDBRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute nsIVariant result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsISDBRequest, aResult: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsresult resultCode; */
    pub GetResultCode: unsafe extern "system" fn (this: *const nsISDBRequest, aResultCode: *mut ::nserror::nsresult) -> ::nserror::nsresult,

    /* attribute nsISDBCallback callback; */
    pub GetCallback: unsafe extern "system" fn (this: *const nsISDBRequest, aCallback: *mut*const nsISDBCallback) -> ::nserror::nsresult,

    /* attribute nsISDBCallback callback; */
    pub SetCallback: unsafe extern "system" fn (this: *const nsISDBRequest, aCallback: *const nsISDBCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISDBRequest {


    /// `[must_use] readonly attribute nsIVariant result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }



    /// `[must_use] readonly attribute nsresult resultCode;`
    #[inline]
    pub unsafe fn GetResultCode(&self, aResultCode: *mut ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).GetResultCode)(self, aResultCode)
    }



    /// `attribute nsISDBCallback callback;`
    #[inline]
    pub unsafe fn GetCallback(&self, aCallback: *mut*const nsISDBCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetCallback)(self, aCallback)
    }



    /// `attribute nsISDBCallback callback;`
    #[inline]
    pub unsafe fn SetCallback(&self, aCallback: *const nsISDBCallback) -> ::nserror::nsresult {
        ((*self.vtable).SetCallback)(self, aCallback)
    }


}



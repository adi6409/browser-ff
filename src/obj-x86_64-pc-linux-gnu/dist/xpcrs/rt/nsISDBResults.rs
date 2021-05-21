//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBResults.idl
//


/// `interface nsISDBResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISDBResult {
    vtable: *const nsISDBResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISDBResult.
unsafe impl XpCom for nsISDBResult {
    const IID: nsIID = nsID(0xbca19e01, 0xb34e, 0x4a48,
        [0x88, 0x75, 0x2f, 0x4c, 0xb8, 0x71, 0xfe, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISDBResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISDBResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISDBResultCoerce {
    /// Cheaply cast a value of this type from a `nsISDBResult`.
    fn coerce_from(v: &nsISDBResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISDBResultCoerce for nsISDBResult {
    #[inline]
    fn coerce_from(v: &nsISDBResult) -> &Self {
        v
    }
}

impl nsISDBResult {
    /// Cast this `nsISDBResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISDBResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISDBResult {
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
impl<T: nsISupportsCoerce> nsISDBResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISDBResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISDBResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISDBResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] Array<uint8_t> getAsArray (); */
    pub GetAsArray: unsafe extern "system" fn (this: *const nsISDBResult, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] jsval getAsArrayBuffer (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetAsArrayBuffer: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISDBResult {


    /// `[must_use] Array<uint8_t> getAsArray ();`
    #[inline]
    pub unsafe fn GetAsArray(&self, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetAsArray)(self, _retval)
    }



    /// `[implicit_jscontext,must_use] jsval getAsArrayBuffer ();`
    const _GetAsArrayBuffer: () = ();

}



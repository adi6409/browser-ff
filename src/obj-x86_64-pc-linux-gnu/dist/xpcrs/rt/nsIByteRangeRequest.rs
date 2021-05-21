//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIByteRangeRequest.idl
//


/// `interface nsIByteRangeRequest : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIByteRangeRequest {
    vtable: *const nsIByteRangeRequestVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIByteRangeRequest.
unsafe impl XpCom for nsIByteRangeRequest {
    const IID: nsIID = nsID(0xc1b1f426, 0x7e83, 0x4759,
        [0x9f, 0x88, 0x0e, 0x1b, 0x17, 0xf4, 0x93, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIByteRangeRequest {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIByteRangeRequest.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIByteRangeRequestCoerce {
    /// Cheaply cast a value of this type from a `nsIByteRangeRequest`.
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIByteRangeRequestCoerce for nsIByteRangeRequest {
    #[inline]
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self {
        v
    }
}

impl nsIByteRangeRequest {
    /// Cast this `nsIByteRangeRequest` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIByteRangeRequestCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIByteRangeRequest {
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
impl<T: nsISupportsCoerce> nsIByteRangeRequestCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIByteRangeRequest) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIByteRangeRequest
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIByteRangeRequestVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isByteRangeRequest; */
    pub GetIsByteRangeRequest: unsafe extern "system" fn (this: *const nsIByteRangeRequest, aIsByteRangeRequest: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long long startRange; */
    pub GetStartRange: unsafe extern "system" fn (this: *const nsIByteRangeRequest, aStartRange: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long endRange; */
    pub GetEndRange: unsafe extern "system" fn (this: *const nsIByteRangeRequest, aEndRange: *mut i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIByteRangeRequest {

    /// ```text
    /// /**
    ///      * Returns true IFF this request is a byte range request, otherwise it
    ///      * returns false (This is effectively the same as checking to see if
        ///      * |startRequest| is zero and |endRange| is the content length.)
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isByteRangeRequest;`
    #[inline]
    pub unsafe fn GetIsByteRangeRequest(&self, aIsByteRangeRequest: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsByteRangeRequest)(self, aIsByteRangeRequest)
    }


    /// ```text
    /// /**
    ///      * Absolute start position in remote file for this request.
    ///      */
    /// ```
    ///

    /// `readonly attribute long long startRange;`
    #[inline]
    pub unsafe fn GetStartRange(&self, aStartRange: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetStartRange)(self, aStartRange)
    }


    /// ```text
    /// /**
    ///      * Absolute end postion in remote file for this request
    ///      */
    /// ```
    ///

    /// `readonly attribute long long endRange;`
    #[inline]
    pub unsafe fn GetEndRange(&self, aEndRange: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetEndRange)(self, aEndRange)
    }


}



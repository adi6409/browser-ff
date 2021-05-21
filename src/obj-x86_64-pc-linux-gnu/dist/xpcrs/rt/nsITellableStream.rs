//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsITellableStream.idl
//


/// `interface nsITellableStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITellableStream {
    vtable: *const nsITellableStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITellableStream.
unsafe impl XpCom for nsITellableStream {
    const IID: nsIID = nsID(0xee942946, 0x4538, 0x45d2,
        [0xbf, 0x05, 0xff, 0xdb, 0xf5, 0x93, 0x26, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITellableStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITellableStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITellableStreamCoerce {
    /// Cheaply cast a value of this type from a `nsITellableStream`.
    fn coerce_from(v: &nsITellableStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITellableStreamCoerce for nsITellableStream {
    #[inline]
    fn coerce_from(v: &nsITellableStream) -> &Self {
        v
    }
}

impl nsITellableStream {
    /// Cast this `nsITellableStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITellableStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITellableStream {
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
impl<T: nsISupportsCoerce> nsITellableStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITellableStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITellableStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITellableStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* long long tell (); */
    pub Tell: unsafe extern "system" fn (this: *const nsITellableStream, _retval: *mut i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITellableStream {

    /// ```text
    /// /**
    ///      *  tell
    ///      *
    ///      *  This method reports the current offset, in bytes, from the start of the
    ///      *  stream.
    ///      *
    ///      *   @throws NS_BASE_STREAM_CLOSED if called on a closed stream.
    ///      */
    /// ```
    ///

    /// `long long tell ();`
    #[inline]
    pub unsafe fn Tell(&self, _retval: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).Tell)(self, _retval)
    }


}



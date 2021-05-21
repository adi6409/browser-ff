//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamTee.idl
//


/// `interface nsIInputStreamTee : nsIInputStream`
///

/// ```text
/// /**
///  * A nsIInputStreamTee is a wrapper for an input stream, that when read
///  * reads the specified amount of data from its |source| and copies that
///  * data to its |sink|.  |sink| must be a blocking output stream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamTee {
    vtable: *const nsIInputStreamTeeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamTee.
unsafe impl XpCom for nsIInputStreamTee {
    const IID: nsIID = nsID(0x90a9d790, 0x3bca, 0x421e,
        [0xa7, 0x3b, 0x98, 0xf6, 0x8e, 0x13, 0xc9, 0x17]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamTee {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamTee.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamTeeCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamTee`.
    fn coerce_from(v: &nsIInputStreamTee) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamTeeCoerce for nsIInputStreamTee {
    #[inline]
    fn coerce_from(v: &nsIInputStreamTee) -> &Self {
        v
    }
}

impl nsIInputStreamTee {
    /// Cast this `nsIInputStreamTee` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamTeeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamTee {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIInputStreamCoerce> nsIInputStreamTeeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamTee) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamTee
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamTeeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* attribute nsIInputStream source; */
    pub GetSource: unsafe extern "system" fn (this: *const nsIInputStreamTee, aSource: *mut *const nsIInputStream) -> ::nserror::nsresult,

    /* attribute nsIInputStream source; */
    pub SetSource: unsafe extern "system" fn (this: *const nsIInputStreamTee, aSource: *const nsIInputStream) -> ::nserror::nsresult,

    /* attribute nsIOutputStream sink; */
    pub GetSink: unsafe extern "system" fn (this: *const nsIInputStreamTee, aSink: *mut*const nsIOutputStream) -> ::nserror::nsresult,

    /* attribute nsIOutputStream sink; */
    pub SetSink: unsafe extern "system" fn (this: *const nsIInputStreamTee, aSink: *const nsIOutputStream) -> ::nserror::nsresult,

    /* attribute nsIEventTarget eventTarget; */
    pub GetEventTarget: unsafe extern "system" fn (this: *const nsIInputStreamTee, aEventTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult,

    /* attribute nsIEventTarget eventTarget; */
    pub SetEventTarget: unsafe extern "system" fn (this: *const nsIInputStreamTee, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamTee {


    /// `attribute nsIInputStream source;`
    #[inline]
    pub unsafe fn GetSource(&self, aSource: *mut *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetSource)(self, aSource)
    }



    /// `attribute nsIInputStream source;`
    #[inline]
    pub unsafe fn SetSource(&self, aSource: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetSource)(self, aSource)
    }



    /// `attribute nsIOutputStream sink;`
    #[inline]
    pub unsafe fn GetSink(&self, aSink: *mut*const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetSink)(self, aSink)
    }



    /// `attribute nsIOutputStream sink;`
    #[inline]
    pub unsafe fn SetSink(&self, aSink: *const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetSink)(self, aSink)
    }


    /// ```text
    /// /**
    ///      * If |eventTarget| is set, copying to sink is done asynchronously using
    ///      * the event-target (e.g. a thread). If |eventTarget| is not set, copying
    ///      * to sink happens synchronously while reading from the source.
    ///      */
    /// ```
    ///

    /// `attribute nsIEventTarget eventTarget;`
    #[inline]
    pub unsafe fn GetEventTarget(&self, aEventTarget: *mut*const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).GetEventTarget)(self, aEventTarget)
    }


    /// ```text
    /// /**
    ///      * If |eventTarget| is set, copying to sink is done asynchronously using
    ///      * the event-target (e.g. a thread). If |eventTarget| is not set, copying
    ///      * to sink happens synchronously while reading from the source.
    ///      */
    /// ```
    ///

    /// `attribute nsIEventTarget eventTarget;`
    #[inline]
    pub unsafe fn SetEventTarget(&self, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).SetEventTarget)(self, aEventTarget)
    }


}



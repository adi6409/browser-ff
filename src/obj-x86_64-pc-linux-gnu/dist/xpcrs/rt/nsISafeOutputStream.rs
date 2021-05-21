//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsISafeOutputStream.idl
//


/// `interface nsISafeOutputStream : nsISupports`
///

/// ```text
/// /**
///  * This interface provides a mechanism to control an output stream
///  * that takes care not to overwrite an existing target until it is known
///  * that all writes to the destination succeeded.
///  *
///  * An object that supports this interface is intended to also support
///  * nsIOutputStream.
///  *
///  * For example, a file output stream that supports this interface writes to
///  * a temporary file, and moves it over the original file when |finish| is
///  * called only if the stream can be successfully closed and all writes
///  * succeeded.  If |finish| is called but something went wrong during
///  * writing, it will delete the temporary file and not touch the original.
///  * If the stream is closed by calling |close| directly, or the stream
///  * goes away, the original file will not be overwritten, and the temporary
///  * file will be deleted.
///  *
///  * Currently, this interface is implemented only for file output streams.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISafeOutputStream {
    vtable: *const nsISafeOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISafeOutputStream.
unsafe impl XpCom for nsISafeOutputStream {
    const IID: nsIID = nsID(0x5f914307, 0x5c34, 0x4e1f,
        [0x8e, 0x32, 0xec, 0x74, 0x9d, 0x25, 0xb2, 0x7a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISafeOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISafeOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISafeOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsISafeOutputStream`.
    fn coerce_from(v: &nsISafeOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISafeOutputStreamCoerce for nsISafeOutputStream {
    #[inline]
    fn coerce_from(v: &nsISafeOutputStream) -> &Self {
        v
    }
}

impl nsISafeOutputStream {
    /// Cast this `nsISafeOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISafeOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISafeOutputStream {
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
impl<T: nsISupportsCoerce> nsISafeOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISafeOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISafeOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISafeOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void finish (); */
    pub Finish: unsafe extern "system" fn (this: *const nsISafeOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISafeOutputStream {

    /// ```text
    /// /**
    ///      * Call this method to close the stream and cause the original target
    ///      * to be overwritten. Note: if any call to |write| failed to write out
    ///      * all of the data given to it, then calling this method will |close| the
    ///      * stream and return failure. Further, if closing the stream fails, this
    ///      * method will return failure. The original target will be overwritten only
    ///      * if all calls to |write| succeeded and the stream was successfully closed.
    ///      */
    /// ```
    ///

    /// `void finish ();`
    #[inline]
    pub unsafe fn Finish(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, )
    }


}



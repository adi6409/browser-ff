//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginInputStream.idl
//


/// `interface nsIPluginInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * The nsIPluginInputStream interface ...
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPluginInputStream {
    vtable: *const nsIPluginInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPluginInputStream.
unsafe impl XpCom for nsIPluginInputStream {
    const IID: nsIID = nsID(0xaf160530, 0x542a, 0x11d2,
        [0x81, 0x64, 0x00, 0x60, 0x08, 0x11, 0x9d, 0x7a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPluginInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPluginInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPluginInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIPluginInputStream`.
    fn coerce_from(v: &nsIPluginInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPluginInputStreamCoerce for nsIPluginInputStream {
    #[inline]
    fn coerce_from(v: &nsIPluginInputStream) -> &Self {
        v
    }
}

impl nsIPluginInputStream {
    /// Cast this `nsIPluginInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPluginInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPluginInputStream {
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
impl<T: nsIInputStreamCoerce> nsIPluginInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPluginInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPluginInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void getLastModified (out unsigned long aResult); */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsIPluginInputStream, aResult: *mut u32) -> ::nserror::nsresult,

    /* void requestRead (out NPByteRange aRangeList); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub RequestRead: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPluginInputStream {

    /// ```text
    /// /**
    ///    *  Corresponds to NPStream's lastmodified field.)
///    */
/// ```
///

/// `void getLastModified (out unsigned long aResult);`
#[inline]
pub unsafe fn GetLastModified(&self, aResult: *mut u32) -> ::nserror::nsresult {
    ((*self.vtable).GetLastModified)(self, aResult)
}



/// `void requestRead (out NPByteRange aRangeList);`
const _RequestRead: () = ();

}



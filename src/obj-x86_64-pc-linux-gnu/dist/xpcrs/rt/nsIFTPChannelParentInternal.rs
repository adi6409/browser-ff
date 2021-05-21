//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannelParentInternal.idl
//


/// `interface nsIFTPChannelParentInternal : nsISupports`
///

/// ```text
/// /**
///  * This is an internal interface for FTP parent channel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFTPChannelParentInternal {
    vtable: *const nsIFTPChannelParentInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFTPChannelParentInternal.
unsafe impl XpCom for nsIFTPChannelParentInternal {
    const IID: nsIID = nsID(0x87b58410, 0x83cb, 0x42a7,
        [0xb5, 0x7b, 0x27, 0xc0, 0x7e, 0xf8, 0x28, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFTPChannelParentInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFTPChannelParentInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFTPChannelParentInternalCoerce {
    /// Cheaply cast a value of this type from a `nsIFTPChannelParentInternal`.
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFTPChannelParentInternalCoerce for nsIFTPChannelParentInternal {
    #[inline]
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self {
        v
    }
}

impl nsIFTPChannelParentInternal {
    /// Cast this `nsIFTPChannelParentInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFTPChannelParentInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFTPChannelParentInternal {
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
impl<T: nsISupportsCoerce> nsIFTPChannelParentInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPChannelParentInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFTPChannelParentInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFTPChannelParentInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setErrorMsg (in string msg, in boolean useUTF8); */
    pub SetErrorMsg: unsafe extern "system" fn (this: *const nsIFTPChannelParentInternal, msg: *const libc::c_char, useUTF8: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFTPChannelParentInternal {


    /// `void setErrorMsg (in string msg, in boolean useUTF8);`
    #[inline]
    pub unsafe fn SetErrorMsg(&self, msg: *const libc::c_char, useUTF8: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetErrorMsg)(self, msg, useUTF8)
    }


}



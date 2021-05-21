//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIObjectInputStream.idl
//


/// `interface nsIObjectInputStream : nsIBinaryInputStream`
///

/// ```text
/// /**
///  * @see nsIObjectOutputStream
///  * @see nsIBinaryInputStream
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIObjectInputStream {
    vtable: *const nsIObjectInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIObjectInputStream.
unsafe impl XpCom for nsIObjectInputStream {
    const IID: nsIID = nsID(0x6c248606, 0x4eae, 0x46fa,
        [0x9d, 0xf0, 0xba, 0x58, 0x50, 0x23, 0x68, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIObjectInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIObjectInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIObjectInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIObjectInputStream`.
    fn coerce_from(v: &nsIObjectInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIObjectInputStreamCoerce for nsIObjectInputStream {
    #[inline]
    fn coerce_from(v: &nsIObjectInputStream) -> &Self {
        v
    }
}

impl nsIObjectInputStream {
    /// Cast this `nsIObjectInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIObjectInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIObjectInputStream {
    type Target = nsIBinaryInputStream;
    #[inline]
    fn deref(&self) -> &nsIBinaryInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIBinaryInputStreamCoerce> nsIObjectInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIObjectInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIObjectInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIBinaryInputStreamVTable,

    /* nsISupports readObject (in boolean aIsStrongRef); */
    pub ReadObject: unsafe extern "system" fn (this: *const nsIObjectInputStream, aIsStrongRef: bool, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* [notxpcom] nsresult readID (out nsID aID); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub ReadID: *const ::libc::c_void,

    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    /// Unable to generate binding because `native type char unsupported`
    pub GetBuffer: *const ::libc::c_void,

    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    /// Unable to generate binding because `native type char unsupported`
    pub PutBuffer: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIObjectInputStream {

    /// ```text
    /// /**
    ///      * Read an object from this stream to satisfy a strong or weak reference
    ///      * to one of its interfaces.  If the interface was not along the primary
    ///      * inheritance chain ending in the "root" or XPCOM-identity nsISupports,
    ///      * readObject will QueryInterface from the deserialized object root to the
    ///      * correct interface, which was specified when the object was serialized.
    ///      *
    ///      * @see nsIObjectOutputStream
    ///      */
    /// ```
    ///

    /// `nsISupports readObject (in boolean aIsStrongRef);`
    #[inline]
    pub unsafe fn ReadObject(&self, aIsStrongRef: bool, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).ReadObject)(self, aIsStrongRef, _retval)
    }



    /// `[notxpcom] nsresult readID (out nsID aID);`
    const _ReadID: () = ();

    /// ```text
    /// /**
    ///      * Optimized deserialization support -- see nsIStreamBufferAccess.idl.
    ///      */
    /// ```
    ///

    /// `[notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask);`
    const _GetBuffer: () = ();


    /// `[notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength);`
    const _PutBuffer: () = ();

}



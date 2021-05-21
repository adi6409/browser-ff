//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIObjectOutputStream.idl
//


/// `interface nsIObjectOutputStream : nsIBinaryOutputStream`
///

/// ```text
/// /**
///  * @See nsIObjectInputStream
///  * @See nsIBinaryOutputStream
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIObjectOutputStream {
    vtable: *const nsIObjectOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIObjectOutputStream.
unsafe impl XpCom for nsIObjectOutputStream {
    const IID: nsIID = nsID(0x92c898ac, 0x5fde, 0x4b99,
        [0x87, 0xb3, 0x5d, 0x48, 0x64, 0x22, 0x09, 0x4b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIObjectOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIObjectOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIObjectOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIObjectOutputStream`.
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIObjectOutputStreamCoerce for nsIObjectOutputStream {
    #[inline]
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self {
        v
    }
}

impl nsIObjectOutputStream {
    /// Cast this `nsIObjectOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIObjectOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIObjectOutputStream {
    type Target = nsIBinaryOutputStream;
    #[inline]
    fn deref(&self) -> &nsIBinaryOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIBinaryOutputStreamCoerce> nsIObjectOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIObjectOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIObjectOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIBinaryOutputStreamVTable,

    /* void writeObject (in nsISupports aObject, in boolean aIsStrongRef); */
    pub WriteObject: unsafe extern "system" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports, aIsStrongRef: bool) -> ::nserror::nsresult,

    /* void writeSingleRefObject (in nsISupports aObject); */
    pub WriteSingleRefObject: unsafe extern "system" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports) -> ::nserror::nsresult,

    /* void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef); */
    pub WriteCompoundObject: unsafe extern "system" fn (this: *const nsIObjectOutputStream, aObject: *const nsISupports, aIID: *const nsIID, aIsStrongRef: bool) -> ::nserror::nsresult,

    /* void writeID (in nsIDRef aID); */
    pub WriteID: unsafe extern "system" fn (this: *const nsIObjectOutputStream, aID: *const nsID) -> ::nserror::nsresult,

    /* [notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask); */
    /// Unable to generate binding because `native type char unsupported`
    pub GetBuffer: *const ::libc::c_void,

    /* [notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength); */
    /// Unable to generate binding because `native type char unsupported`
    pub PutBuffer: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIObjectOutputStream {

    /// ```text
    /// /**
    ///      * Write the object whose "root" or XPCOM-identity nsISupports is aObject.
    ///      * The cause for writing this object is a strong or weak reference, so the
    ///      * aIsStrongRef argument must tell which kind of pointer is being followed
    ///      * here during serialization.
    ///      *
    ///      * If the object has only one strong reference in the serialization and no
    ///      * weak refs, use writeSingleRefObject.  This is a valuable optimization:
    ///      * it saves space in the stream, and cycles on both ends of the process.
    ///      *
    ///      * If the reference being serialized is a pointer to an interface not on
    ///      * the primary inheritance chain ending in the root nsISupports, you must
    ///      * call writeCompoundObject instead of this method.
    ///      */
    /// ```
    ///

    /// `void writeObject (in nsISupports aObject, in boolean aIsStrongRef);`
    #[inline]
    pub unsafe fn WriteObject(&self, aObject: *const nsISupports, aIsStrongRef: bool) -> ::nserror::nsresult {
        ((*self.vtable).WriteObject)(self, aObject, aIsStrongRef)
    }


    /// ```text
    /// /**
    ///      * Write an object referenced singly and strongly via its root nsISupports
    ///      * or a subclass of its root nsISupports.  There must not be other refs to
    ///      * aObject in memory, or in the serialization.
    ///      */
    /// ```
    ///

    /// `void writeSingleRefObject (in nsISupports aObject);`
    #[inline]
    pub unsafe fn WriteSingleRefObject(&self, aObject: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).WriteSingleRefObject)(self, aObject)
    }


    /// ```text
    /// /**
    ///      * Write the object referenced by an interface pointer at aObject that
    ///      * inherits from a non-primary nsISupports, i.e., a reference to one of
    ///      * the multiply inherited interfaces derived from an nsISupports other
    ///      * than the root or XPCOM-identity nsISupports; or a reference to an
    ///      * inner object in the case of true XPCOM aggregation.  aIID identifies
    ///      * this interface.
    ///      */
    /// ```
    ///

    /// `void writeCompoundObject (in nsISupports aObject, in nsIIDRef aIID, in boolean aIsStrongRef);`
    #[inline]
    pub unsafe fn WriteCompoundObject(&self, aObject: *const nsISupports, aIID: *const nsIID, aIsStrongRef: bool) -> ::nserror::nsresult {
        ((*self.vtable).WriteCompoundObject)(self, aObject, aIID, aIsStrongRef)
    }



    /// `void writeID (in nsIDRef aID);`
    #[inline]
    pub unsafe fn WriteID(&self, aID: *const nsID) -> ::nserror::nsresult {
        ((*self.vtable).WriteID)(self, aID)
    }


    /// ```text
    /// /**
    ///      * Optimized serialization support -- see nsIStreamBufferAccess.idl.
    ///      */
    /// ```
    ///

    /// `[notxpcom] charPtr getBuffer (in uint32_t aLength, in uint32_t aAlignMask);`
    const _GetBuffer: () = ();


    /// `[notxpcom] void putBuffer (in charPtr aBuffer, in uint32_t aLength);`
    const _PutBuffer: () = ();

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISerializable.idl
//


/// `interface nsISerializable : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISerializable {
    vtable: *const nsISerializableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISerializable.
unsafe impl XpCom for nsISerializable {
    const IID: nsIID = nsID(0x91cca981, 0xc26d, 0x44a8,
        [0xbe, 0xbe, 0xd9, 0xed, 0x48, 0x91, 0x50, 0x3a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISerializable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISerializable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISerializableCoerce {
    /// Cheaply cast a value of this type from a `nsISerializable`.
    fn coerce_from(v: &nsISerializable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISerializableCoerce for nsISerializable {
    #[inline]
    fn coerce_from(v: &nsISerializable) -> &Self {
        v
    }
}

impl nsISerializable {
    /// Cast this `nsISerializable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISerializableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISerializable {
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
impl<T: nsISupportsCoerce> nsISerializableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISerializable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISerializable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISerializableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void read (in nsIObjectInputStream aInputStream); */
    pub Read: unsafe extern "system" fn (this: *const nsISerializable, aInputStream: *const nsIObjectInputStream) -> ::nserror::nsresult,

    /* void write (in nsIObjectOutputStream aOutputStream); */
    pub Write: unsafe extern "system" fn (this: *const nsISerializable, aOutputStream: *const nsIObjectOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISerializable {

    /// ```text
    /// /**
    ///      * Initialize the object implementing nsISerializable, which must have
    ///      * been freshly constructed via CreateInstance.  All data members that
    ///      * can't be set to default values must have been serialized by write,
    ///      * and should be read from aInputStream in the same order by this method.
    ///      */
    /// ```
    ///

    /// `[must_use] void read (in nsIObjectInputStream aInputStream);`
    #[inline]
    pub unsafe fn Read(&self, aInputStream: *const nsIObjectInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Read)(self, aInputStream)
    }


    /// ```text
    /// /**
    ///      * Serialize the object implementing nsISerializable to aOutputStream, by
    ///      * writing each data member that must be recovered later to reconstitute
    ///      * a working replica of this object, in a canonical member and byte order,
    ///      * to aOutputStream.
    ///      *
    ///      * NB: a class that implements nsISerializable *must* also implement
    ///      * nsIClassInfo, in particular nsIClassInfo::GetClassID.
    ///      */
    /// ```
    ///

    /// `void write (in nsIObjectOutputStream aOutputStream);`
    #[inline]
    pub unsafe fn Write(&self, aOutputStream: *const nsIObjectOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).Write)(self, aOutputStream)
    }


}



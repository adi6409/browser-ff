//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/converters/nsICompressConvStats.idl
//


/// `interface nsICompressConvStats : nsISupports`
///

/// ```text
/// /**
///  * nsICompressConvStats
///  *
///  * This interface allows for the observation of decoded resource sizes
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICompressConvStats {
    vtable: *const nsICompressConvStatsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICompressConvStats.
unsafe impl XpCom for nsICompressConvStats {
    const IID: nsIID = nsID(0x58172ad0, 0x46a9, 0x4893,
        [0x8f, 0xde, 0xcd, 0x90, 0x9c, 0x10, 0x79, 0x2a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICompressConvStats {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICompressConvStats.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICompressConvStatsCoerce {
    /// Cheaply cast a value of this type from a `nsICompressConvStats`.
    fn coerce_from(v: &nsICompressConvStats) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICompressConvStatsCoerce for nsICompressConvStats {
    #[inline]
    fn coerce_from(v: &nsICompressConvStats) -> &Self {
        v
    }
}

impl nsICompressConvStats {
    /// Cast this `nsICompressConvStats` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICompressConvStatsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICompressConvStats {
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
impl<T: nsISupportsCoerce> nsICompressConvStatsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICompressConvStats) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICompressConvStats
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICompressConvStatsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint64_t decodedDataLength; */
    pub GetDecodedDataLength: unsafe extern "system" fn (this: *const nsICompressConvStats, aDecodedDataLength: *mut uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICompressConvStats {


    /// `readonly attribute uint64_t decodedDataLength;`
    #[inline]
    pub unsafe fn GetDecodedDataLength(&self, aDecodedDataLength: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetDecodedDataLength)(self, aDecodedDataLength)
    }


}



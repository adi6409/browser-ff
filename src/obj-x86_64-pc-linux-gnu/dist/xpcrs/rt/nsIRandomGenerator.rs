//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRandomGenerator.idl
//


/// `interface nsIRandomGenerator : nsISupports`
///

/// ```text
/// /**
///  * Interface used to generate random data.
///  *
///  * @threadsafe
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRandomGenerator {
    vtable: *const nsIRandomGeneratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRandomGenerator.
unsafe impl XpCom for nsIRandomGenerator {
    const IID: nsIID = nsID(0x2362d97a, 0x747a, 0x4576,
        [0x88, 0x63, 0x69, 0x76, 0x67, 0x30, 0x92, 0x09]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRandomGenerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRandomGenerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRandomGeneratorCoerce {
    /// Cheaply cast a value of this type from a `nsIRandomGenerator`.
    fn coerce_from(v: &nsIRandomGenerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRandomGeneratorCoerce for nsIRandomGenerator {
    #[inline]
    fn coerce_from(v: &nsIRandomGenerator) -> &Self {
        v
    }
}

impl nsIRandomGenerator {
    /// Cast this `nsIRandomGenerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRandomGeneratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRandomGenerator {
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
impl<T: nsISupportsCoerce> nsIRandomGeneratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRandomGenerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRandomGenerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRandomGeneratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void generateRandomBytes (in unsigned long aLength, [array, size_is (aLength), retval] out octet aBuffer); */
    pub GenerateRandomBytes: unsafe extern "system" fn (this: *const nsIRandomGenerator, aLength: u32, aBuffer: *mut *mut u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRandomGenerator {

    /// ```text
    /// /**
    ///    * Generates the specified amount of random bytes.
    ///    *
    ///    * @param aLength
    ///    *        The length of the data to generate.
    ///    * @param aBuffer
    ///    *        A buffer that contains random bytes of size aLength.
    ///    */
    /// ```
    ///

    /// `void generateRandomBytes (in unsigned long aLength, [array, size_is (aLength), retval] out octet aBuffer);`
    #[inline]
    pub unsafe fn GenerateRandomBytes(&self, aLength: u32, aBuffer: *mut *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GenerateRandomBytes)(self, aLength, aBuffer)
    }


}



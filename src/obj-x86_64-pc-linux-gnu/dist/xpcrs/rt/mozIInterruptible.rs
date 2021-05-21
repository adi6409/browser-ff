//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/services/interfaces/mozIInterruptible.idl
//


/// `interface mozIInterruptible : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIInterruptible {
    vtable: *const mozIInterruptibleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIInterruptible.
unsafe impl XpCom for mozIInterruptible {
    const IID: nsIID = nsID(0x1c06bfd3, 0x76b1, 0x46fa,
        [0xa6, 0x4a, 0xdb, 0x68, 0x2d, 0x47, 0x83, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIInterruptible {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIInterruptible.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIInterruptibleCoerce {
    /// Cheaply cast a value of this type from a `mozIInterruptible`.
    fn coerce_from(v: &mozIInterruptible) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIInterruptibleCoerce for mozIInterruptible {
    #[inline]
    fn coerce_from(v: &mozIInterruptible) -> &Self {
        v
    }
}

impl mozIInterruptible {
    /// Cast this `mozIInterruptible` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIInterruptibleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIInterruptible {
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
impl<T: nsISupportsCoerce> mozIInterruptibleCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIInterruptible) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIInterruptible
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIInterruptibleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void interrupt (); */
    pub Interrupt: unsafe extern "system" fn (this: *const mozIInterruptible) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIInterruptible {


    /// `void interrupt ();`
    #[inline]
    pub unsafe fn Interrupt(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Interrupt)(self, )
    }


}



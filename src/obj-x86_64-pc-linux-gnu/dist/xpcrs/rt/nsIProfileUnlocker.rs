//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/profile/nsIProfileUnlocker.idl
//


/// `interface nsIProfileUnlocker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfileUnlocker {
    vtable: *const nsIProfileUnlockerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfileUnlocker.
unsafe impl XpCom for nsIProfileUnlocker {
    const IID: nsIID = nsID(0x08923af1, 0xe7a3, 0x4fae,
        [0xba, 0x02, 0x12, 0x85, 0x02, 0x19, 0x39, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfileUnlocker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfileUnlocker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfileUnlockerCoerce {
    /// Cheaply cast a value of this type from a `nsIProfileUnlocker`.
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfileUnlockerCoerce for nsIProfileUnlocker {
    #[inline]
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self {
        v
    }
}

impl nsIProfileUnlocker {
    /// Cast this `nsIProfileUnlocker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfileUnlockerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfileUnlocker {
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
impl<T: nsISupportsCoerce> nsIProfileUnlockerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfileUnlocker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfileUnlocker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfileUnlockerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void unlock (in unsigned long aSeverity); */
    pub Unlock: unsafe extern "system" fn (this: *const nsIProfileUnlocker, aSeverity: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfileUnlocker {

    pub const ATTEMPT_QUIT: i64 = 0;


    pub const FORCE_QUIT: i64 = 1;

    /// ```text
    /// /**
    ///    * Try to unlock the specified profile by attempting or forcing the
    ///    * process that currently holds the lock to quit.
    ///    *
    ///    * @param aSeverity either ATTEMPT_QUIT or FORCE_QUIT
    ///    * @throws NS_ERROR_FAILURE if unlocking failed.
    ///    */
    /// ```
    ///

    /// `void unlock (in unsigned long aSeverity);`
    #[inline]
    pub unsafe fn Unlock(&self, aSeverity: u32) -> ::nserror::nsresult {
        ((*self.vtable).Unlock)(self, aSeverity)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsICancelable.idl
//


/// `interface nsICancelable : nsISupports`
///

/// ```text
/// /**
///  * This interface provides a means to cancel an operation that is in progress.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICancelable {
    vtable: *const nsICancelableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICancelable.
unsafe impl XpCom for nsICancelable {
    const IID: nsIID = nsID(0xd94ac0a0, 0xbb18, 0x46b8,
        [0x84, 0x4e, 0x84, 0x15, 0x90, 0x64, 0xb0, 0xbd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICancelable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICancelable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICancelableCoerce {
    /// Cheaply cast a value of this type from a `nsICancelable`.
    fn coerce_from(v: &nsICancelable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICancelableCoerce for nsICancelable {
    #[inline]
    fn coerce_from(v: &nsICancelable) -> &Self {
        v
    }
}

impl nsICancelable {
    /// Cast this `nsICancelable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICancelableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICancelable {
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
impl<T: nsISupportsCoerce> nsICancelableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICancelable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICancelable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICancelableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void cancel (in nsresult aReason); */
    pub Cancel: unsafe extern "system" fn (this: *const nsICancelable, aReason: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICancelable {

    /// ```text
    /// /**
    ///    * Call this method to request that this object abort whatever operation it
    ///    * may be performing.
    ///    *
    ///    * @param aReason
    ///    *        Pass a failure code to indicate the reason why this operation is
    ///    *        being canceled.  It is an error to pass a success code.
    ///    */
    /// ```
    ///

    /// `void cancel (in nsresult aReason);`
    #[inline]
    pub unsafe fn Cancel(&self, aReason: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Cancel)(self, aReason)
    }


}



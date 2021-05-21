//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDeprecationWarner.idl
//


/// `interface nsIDeprecationWarner : nsISupports`
///

/// ```text
/// /**
///  * Interface for warning about deprecated operations.  Consumers should
///  * attach this interface to the channel's notification callbacks/loadgroup.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDeprecationWarner {
    vtable: *const nsIDeprecationWarnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDeprecationWarner.
unsafe impl XpCom for nsIDeprecationWarner {
    const IID: nsIID = nsID(0x665c5124, 0x2c52, 0x41ba,
        [0xae, 0x72, 0x23, 0x93, 0xf8, 0xe7, 0x6c, 0x25]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDeprecationWarner {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDeprecationWarner.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDeprecationWarnerCoerce {
    /// Cheaply cast a value of this type from a `nsIDeprecationWarner`.
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDeprecationWarnerCoerce for nsIDeprecationWarner {
    #[inline]
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self {
        v
    }
}

impl nsIDeprecationWarner {
    /// Cast this `nsIDeprecationWarner` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDeprecationWarnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDeprecationWarner {
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
impl<T: nsISupportsCoerce> nsIDeprecationWarnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDeprecationWarner) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDeprecationWarner
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDeprecationWarnerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void issueWarning (in uint32_t aWarning, [optional] in bool aAsError); */
    pub IssueWarning: unsafe extern "system" fn (this: *const nsIDeprecationWarner, aWarning: uint32_t, aAsError: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDeprecationWarner {

    /// ```text
    /// /**
    ///      * Issue a deprecation warning.
    ///      *
    ///      * @param aWarning a warning code as declared in nsDeprecatedOperationList.h.
    ///      * @param aAsError optional boolean flag indicating whether the warning
    ///      *                 should be treated as an error.
    ///      */
    /// ```
    ///

    /// `void issueWarning (in uint32_t aWarning, [optional] in bool aAsError);`
    #[inline]
    pub unsafe fn IssueWarning(&self, aWarning: uint32_t, aAsError: bool) -> ::nserror::nsresult {
        ((*self.vtable).IssueWarning)(self, aWarning, aAsError)
    }


}



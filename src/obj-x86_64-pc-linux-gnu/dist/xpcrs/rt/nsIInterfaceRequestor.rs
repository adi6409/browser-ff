//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIInterfaceRequestor.idl
//


/// `interface nsIInterfaceRequestor : nsISupports`
///

/// ```text
/// /**
///  * The nsIInterfaceRequestor interface defines a generic interface for
///  * requesting interfaces that a given object might provide access to.
///  * This is very similar to QueryInterface found in nsISupports.
///  * The main difference is that interfaces returned from GetInterface()
///  * are not required to provide a way back to the object implementing this
///  * interface.  The semantics of QI() dictate that given an interface A that
///  * you QI() on to get to interface B, you must be able to QI on B to get back
///  * to A.  This interface however allows you to obtain an interface C from A
///  * that may or most likely will not have the ability to get back to A.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInterfaceRequestor {
    vtable: *const nsIInterfaceRequestorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInterfaceRequestor.
unsafe impl XpCom for nsIInterfaceRequestor {
    const IID: nsIID = nsID(0x033a1470, 0x8b2a, 0x11d3,
        [0xaf, 0x88, 0x00, 0xa0, 0x24, 0xff, 0xc0, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInterfaceRequestor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInterfaceRequestor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInterfaceRequestorCoerce {
    /// Cheaply cast a value of this type from a `nsIInterfaceRequestor`.
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInterfaceRequestorCoerce for nsIInterfaceRequestor {
    #[inline]
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self {
        v
    }
}

impl nsIInterfaceRequestor {
    /// Cast this `nsIInterfaceRequestor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInterfaceRequestorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInterfaceRequestor {
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
impl<T: nsISupportsCoerce> nsIInterfaceRequestorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInterfaceRequestor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInterfaceRequestor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInterfaceRequestorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub GetInterface: unsafe extern "system" fn (this: *const nsIInterfaceRequestor, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInterfaceRequestor {

    /// ```text
    /// /**
    ///     * Retrieves the specified interface pointer.
    ///     *
    ///     * @param uuid The IID of the interface being requested.
    ///     * @param result [out] The interface pointer to be filled in if
    ///     *               the interface is accessible.
    ///     * @throws NS_NOINTERFACE - interface not accessible.
    ///     * @throws NS_ERROR* - method failure.
    ///     */
    /// ```
    ///

    /// `void getInterface (in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetInterface(&self, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetInterface)(self, uuid, result)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIFactory.idl
//


/// `interface nsIFactory : nsISupports`
///

/// ```text
/// /**
///  * A class factory allows the creation of nsISupports derived
///  * components without specifying a concrete base class.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFactory {
    vtable: *const nsIFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFactory.
unsafe impl XpCom for nsIFactory {
    const IID: nsIID = nsID(0x00000001, 0x0000, 0x0000,
        [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIFactory`.
    fn coerce_from(v: &nsIFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFactoryCoerce for nsIFactory {
    #[inline]
    fn coerce_from(v: &nsIFactory) -> &Self {
        v
    }
}

impl nsIFactory {
    /// Cast this `nsIFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFactory {
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
impl<T: nsISupportsCoerce> nsIFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result); */
    pub CreateInstance: unsafe extern "system" fn (this: *const nsIFactory, aOuter: *const nsISupports, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void lockFactory (in boolean lock); */
    pub LockFactory: unsafe extern "system" fn (this: *const nsIFactory, lock: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFactory {

    /// ```text
    /// /**
    ///     * Creates an instance of a component.
    ///     *
    ///     * @param aOuter Pointer to a component that wishes to be aggregated
    ///     *               in the resulting instance. This will be nullptr if no
    ///     *               aggregation is requested.
    ///     * @param iid    The IID of the interface being requested in
    ///     *               the component which is being currently created.
    ///     * @param result [out] Pointer to the newly created instance, if successful.
    ///     * @throws NS_NOINTERFACE - Interface not accessible.
    ///     * @throws NS_ERROR_NO_AGGREGATION - if an 'outer' object is supplied, but the
    ///     *                                   component is not aggregatable.
    ///     *         NS_ERROR* - Method failure.
    ///     */
    /// ```
    ///

    /// `void createInstance (in nsISupports aOuter, in nsIIDRef iid, [iid_is (iid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn CreateInstance(&self, aOuter: *const nsISupports, iid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstance)(self, aOuter, iid, result)
    }


    /// ```text
    /// /**
    ///     * LockFactory provides the client a way to keep the component
    ///     * in memory until it is finished with it. The client can call
    ///     * LockFactory(PR_TRUE) to lock the factory and LockFactory(PR_FALSE)
    ///     * to release the factory.
    ///     *
    ///     * @param lock - Must be PR_TRUE or PR_FALSE
    ///     * @throws NS_ERROR* - Method failure.
    ///     */
    /// ```
    ///

    /// `void lockFactory (in boolean lock);`
    #[inline]
    pub unsafe fn LockFactory(&self, lock: bool) -> ::nserror::nsresult {
        ((*self.vtable).LockFactory)(self, lock)
    }


}



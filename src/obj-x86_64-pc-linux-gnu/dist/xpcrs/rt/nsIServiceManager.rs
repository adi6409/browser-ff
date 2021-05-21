//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIServiceManager.idl
//


/// `interface nsIServiceManager : nsISupports`
///

/// ```text
/// /**
///  * The nsIServiceManager manager interface provides a means to obtain
///  * global services in an application. The service manager depends on the
///  * repository to find and instantiate factories to obtain services.
///  *
///  * Users of the service manager must first obtain a pointer to the global
///  * service manager by calling NS_GetServiceManager. After that,
///  * they can request specific services by calling GetService. When they are
///  * finished they can NS_RELEASE() the service as usual.
///  *
///  * A user of a service may keep references to particular services indefinitely
///  * and only must call Release when it shuts down.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIServiceManager {
    vtable: *const nsIServiceManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIServiceManager.
unsafe impl XpCom for nsIServiceManager {
    const IID: nsIID = nsID(0x8bb35ed9, 0xe332, 0x462d,
        [0x91, 0x55, 0x4a, 0x00, 0x2a, 0xb5, 0xc9, 0x58]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIServiceManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIServiceManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIServiceManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIServiceManager`.
    fn coerce_from(v: &nsIServiceManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIServiceManagerCoerce for nsIServiceManager {
    #[inline]
    fn coerce_from(v: &nsIServiceManager) -> &Self {
        v
    }
}

impl nsIServiceManager {
    /// Cast this `nsIServiceManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIServiceManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIServiceManager {
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
impl<T: nsISupportsCoerce> nsIServiceManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIServiceManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIServiceManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIServiceManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub GetService: unsafe extern "system" fn (this: *const nsIServiceManager, aClass: *const nsCID, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub GetServiceByContractID: unsafe extern "system" fn (this: *const nsIServiceManager, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID); */
    pub IsServiceInstantiated: unsafe extern "system" fn (this: *const nsIServiceManager, aClass: *const nsCID, aIID: *const nsIID, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID); */
    pub IsServiceInstantiatedByContractID: unsafe extern "system" fn (this: *const nsIServiceManager, aContractID: *const libc::c_char, aIID: *const nsIID, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIServiceManager {

    /// ```text
    /// /**
    ///      * getServiceByContractID
    ///      *
    ///      * Returns the instance that implements aClass or aContractID and the
    ///      * interface aIID.  This may result in the instance being created.
    ///      *
    ///      * @param aClass or aContractID : aClass or aContractID of object
    ///      *                                instance requested
    ///      * @param aIID : IID of interface requested
    ///      * @param result : resulting service
    ///      */
    /// ```
    ///

    /// `void getService (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetService(&self, aClass: *const nsCID, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetService)(self, aClass, aIID, result)
    }



    /// `void getServiceByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetServiceByContractID(&self, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceByContractID)(self, aContractID, aIID, result)
    }


    /// ```text
    /// /**
    ///      * isServiceInstantiated
    ///      *
    ///      * isServiceInstantiated will return a true if the service has already
    ///      * been created, or false otherwise. Throws if the service does not
    ///      * implement the given IID.
    ///      *
    ///      * @param aClass or aContractID : aClass or aContractID of object
    ///      *                                instance requested
    ///      * @param aIID : IID of interface requested
    ///      * @throws NS_NOINTERFACE if the IID given isn't supported by the object
    ///      */
    /// ```
    ///

    /// `boolean isServiceInstantiated (in nsCIDRef aClass, in nsIIDRef aIID);`
    #[inline]
    pub unsafe fn IsServiceInstantiated(&self, aClass: *const nsCID, aIID: *const nsIID, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsServiceInstantiated)(self, aClass, aIID, _retval)
    }



    /// `boolean isServiceInstantiatedByContractID (in string aContractID, in nsIIDRef aIID);`
    #[inline]
    pub unsafe fn IsServiceInstantiatedByContractID(&self, aContractID: *const libc::c_char, aIID: *const nsIID, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsServiceInstantiatedByContractID)(self, aContractID, aIID, _retval)
    }


}



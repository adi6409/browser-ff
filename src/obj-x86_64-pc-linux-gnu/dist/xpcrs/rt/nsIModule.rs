//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIModule.idl
//


/// `interface nsIModule : nsISupports`
///

/// ```text
/// /**
///  * The nsIModule interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIModule {
    vtable: *const nsIModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIModule.
unsafe impl XpCom for nsIModule {
    const IID: nsIID = nsID(0x7392d032, 0x5371, 0x11d3,
        [0x99, 0x4e, 0x00, 0x80, 0x5f, 0xd2, 0x6f, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIModule {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIModule.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIModuleCoerce {
    /// Cheaply cast a value of this type from a `nsIModule`.
    fn coerce_from(v: &nsIModule) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIModuleCoerce for nsIModule {
    #[inline]
    fn coerce_from(v: &nsIModule) -> &Self {
        v
    }
}

impl nsIModule {
    /// Cast this `nsIModule` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIModule {
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
impl<T: nsISupportsCoerce> nsIModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIModule) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIModule
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIModuleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult); */
    pub GetClassObject: unsafe extern "system" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aClass: *const nsCID, aIID: *const nsIID, aResult: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType); */
    pub RegisterSelf: unsafe extern "system" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> ::nserror::nsresult,

    /* void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr); */
    pub UnregisterSelf: unsafe extern "system" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char) -> ::nserror::nsresult,

    /* boolean canUnload (in nsIComponentManager aCompMgr); */
    pub CanUnload: unsafe extern "system" fn (this: *const nsIModule, aCompMgr: *const nsIComponentManager, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIModule {

    /// ```text
    /// /**
    ///      * Object Instance Creation
    ///      *
    ///      * Obtains a Class Object from a nsIModule for a given CID and IID pair.
    ///      * This class object can either be query to a nsIFactory or a may be
    ///      * query to a nsIClassInfo.
    ///      *
    ///      * @param aCompMgr  : The global component manager
    ///      * @param aClass    : ClassID of object instance requested
    ///      * @param aIID      : IID of interface requested
    ///      *
    ///      */
    /// ```
    ///

    /// `void getClassObject (in nsIComponentManager aCompMgr, in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult aResult);`
    #[inline]
    pub unsafe fn GetClassObject(&self, aCompMgr: *const nsIComponentManager, aClass: *const nsCID, aIID: *const nsIID, aResult: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetClassObject)(self, aCompMgr, aClass, aIID, aResult)
    }


    /// ```text
    /// /**
    ///      * One time registration callback
    ///      *
    ///      * When the nsIModule is discovered, this method will be
    ///      * called so that any setup registration can be preformed.
    ///      *
    ///      * @param aCompMgr  : The global component manager
    ///      * @param aLocation : The location of the nsIModule on disk
    ///      * @param aLoaderStr: Opaque loader specific string
    ///      * @param aType     : Loader Type being used to load this module
    ///      */
    /// ```
    ///

    /// `void registerSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr, in string aType);`
    #[inline]
    pub unsafe fn RegisterSelf(&self, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RegisterSelf)(self, aCompMgr, aLocation, aLoaderStr, aType)
    }


    /// ```text
    /// /**
    ///      * One time unregistration callback
    ///      *
    ///      * When the nsIModule is being unregistered, this method will be
    ///      * called so that any unregistration can be preformed
    ///      *
    ///      * @param aCompMgr   : The global component manager
    ///      * @param aLocation  : The location of the nsIModule on disk
    ///      * @param aLoaderStr : Opaque loader specific string
    ///      *
    ///      */
    /// ```
    ///

    /// `void unregisterSelf (in nsIComponentManager aCompMgr, in nsIFile aLocation, in string aLoaderStr);`
    #[inline]
    pub unsafe fn UnregisterSelf(&self, aCompMgr: *const nsIComponentManager, aLocation: *const nsIFile, aLoaderStr: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterSelf)(self, aCompMgr, aLocation, aLoaderStr)
    }


    /// ```text
    /// /**
    ///     * Module load management
    ///     *
    ///     * @param aCompMgr  : The global component manager
    ///     *
    ///     * @return indicates to the caller if the module can be unloaded.
    ///     * 		Returning PR_TRUE isn't a guarantee that the module will be
    ///     *		unloaded. It constitues only willingness of the module to be
    ///     *		unloaded.  It is very important to ensure that no outstanding
    ///     *       references to the module's code/data exist before returning
    ///     *       PR_TRUE.
    ///     *		Returning PR_FALSE guaratees that the module won't be unloaded.
    ///     */
    /// ```
    ///

    /// `boolean canUnload (in nsIComponentManager aCompMgr);`
    #[inline]
    pub unsafe fn CanUnload(&self, aCompMgr: *const nsIComponentManager, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanUnload)(self, aCompMgr, _retval)
    }


}



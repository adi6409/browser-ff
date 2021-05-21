//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIComponentRegistrar.idl
//


/// `interface nsIComponentRegistrar : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIComponentRegistrar {
    vtable: *const nsIComponentRegistrarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIComponentRegistrar.
unsafe impl XpCom for nsIComponentRegistrar {
    const IID: nsIID = nsID(0x2417cbfe, 0x65ad, 0x48a6,
        [0xb4, 0xb6, 0xeb, 0x84, 0xdb, 0x17, 0x43, 0x92]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIComponentRegistrar {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIComponentRegistrar.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIComponentRegistrarCoerce {
    /// Cheaply cast a value of this type from a `nsIComponentRegistrar`.
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIComponentRegistrarCoerce for nsIComponentRegistrar {
    #[inline]
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self {
        v
    }
}

impl nsIComponentRegistrar {
    /// Cast this `nsIComponentRegistrar` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIComponentRegistrarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIComponentRegistrar {
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
impl<T: nsISupportsCoerce> nsIComponentRegistrarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIComponentRegistrar) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIComponentRegistrar
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIComponentRegistrarVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void autoRegister (in nsIFile aSpec); */
    pub AutoRegister: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aSpec: *const nsIFile) -> ::nserror::nsresult,

    /* void autoUnregister (in nsIFile aSpec); */
    pub AutoUnregister: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aSpec: *const nsIFile) -> ::nserror::nsresult,

    /* void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory); */
    pub RegisterFactory: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFactory: *const nsIFactory) -> ::nserror::nsresult,

    /* void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory); */
    pub UnregisterFactory: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aFactory: *const nsIFactory) -> ::nserror::nsresult,

    /* void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType); */
    pub RegisterFactoryLocation: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFile: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> ::nserror::nsresult,

    /* void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile); */
    pub UnregisterFactoryLocation: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* boolean isCIDRegistered (in nsCIDRef aClass); */
    pub IsCIDRegistered: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isContractIDRegistered (in string aContractID); */
    pub IsContractIDRegistered: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aContractID: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* Array<ACString> getContractIDs (); */
    pub GetContractIDs: unsafe extern "system" fn (this: *const nsIComponentRegistrar, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* string CIDToContractID (in nsCIDRef aClass); */
    pub CIDToContractID: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aClass: *const nsCID, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* nsCIDPtr contractIDToCID (in string aContractID); */
    pub ContractIDToCID: unsafe extern "system" fn (this: *const nsIComponentRegistrar, aContractID: *const libc::c_char, _retval: *mut *mut nsCID) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIComponentRegistrar {

    /// ```text
    /// /**
    ///      * autoRegister
    ///      *
    ///      * Register a .manifest file, or an entire directory containing
    ///      * these files. Registration lasts for this run only, and is not cached.
    ///      *
    ///      * @note Formerly this method would register component files directly. This
    ///      *       is no longer supported.
    ///      */
    /// ```
    ///

    /// `void autoRegister (in nsIFile aSpec);`
    #[inline]
    pub unsafe fn AutoRegister(&self, aSpec: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).AutoRegister)(self, aSpec)
    }


    /// ```text
    /// /**
    ///      * autoUnregister
    ///      * @status OBSOLETE: This method is no longer implemented, but preserved
    ///      *                   in this interface for binary compatibility with
    ///      *                   Mozilla 1.9.2.
    ///      */
    /// ```
    ///

    /// `void autoUnregister (in nsIFile aSpec);`
    #[inline]
    pub unsafe fn AutoUnregister(&self, aSpec: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).AutoUnregister)(self, aSpec)
    }


    /// ```text
    /// /**
    ///      * registerFactory
    ///      *
    ///      * Register a factory with a given ContractID, CID and Class Name.
    ///      *
    ///      * @param aClass      : CID of object
    ///      * @param aClassName  : Class Name of CID (unused)
    ///      * @param aContractID : ContractID associated with CID aClass. May be null
    ///      *                      if no contract ID is needed.
    ///      * @param aFactory    : Factory that will be registered for CID aClass.
    ///      *                      If aFactory is null, the contract will be associated
    ///      *                      with a previously registered CID.
    ///      */
    /// ```
    ///

    /// `void registerFactory (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFactory aFactory);`
    #[inline]
    pub unsafe fn RegisterFactory(&self, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFactory: *const nsIFactory) -> ::nserror::nsresult {
        ((*self.vtable).RegisterFactory)(self, aClass, aClassName, aContractID, aFactory)
    }


    /// ```text
    /// /**
    ///      * unregisterFactory
    ///      *
    ///      * Unregister a factory associated with CID aClass.
    ///      *
    ///      * @param aClass   : CID being unregistered
    ///      * @param aFactory : Factory previously registered to create instances of
    ///      *                   CID aClass.
    ///      *
    ///      * @throws NS_ERROR* Method failure.
    ///      */
    /// ```
    ///

    /// `void unregisterFactory (in nsCIDRef aClass, in nsIFactory aFactory);`
    #[inline]
    pub unsafe fn UnregisterFactory(&self, aClass: *const nsCID, aFactory: *const nsIFactory) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterFactory)(self, aClass, aFactory)
    }


    /// ```text
    /// /**
    ///      * registerFactoryLocation
    ///      * @status OBSOLETE: This method is no longer implemented, but preserved
    ///      *                   in this interface for binary compatibility with
    ///      *                   Mozilla 1.9.2.
    ///      */
    /// ```
    ///

    /// `void registerFactoryLocation (in nsCIDRef aClass, in string aClassName, in string aContractID, in nsIFile aFile, in string aLoaderStr, in string aType);`
    #[inline]
    pub unsafe fn RegisterFactoryLocation(&self, aClass: *const nsCID, aClassName: *const libc::c_char, aContractID: *const libc::c_char, aFile: *const nsIFile, aLoaderStr: *const libc::c_char, aType: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RegisterFactoryLocation)(self, aClass, aClassName, aContractID, aFile, aLoaderStr, aType)
    }


    /// ```text
    /// /**
    ///      * unregisterFactoryLocation
    ///      * @status OBSOLETE: This method is no longer implemented, but preserved
    ///      *                   in this interface for binary compatibility with
    ///      *                   Mozilla 1.9.2.
    ///      */
    /// ```
    ///

    /// `void unregisterFactoryLocation (in nsCIDRef aClass, in nsIFile aFile);`
    #[inline]
    pub unsafe fn UnregisterFactoryLocation(&self, aClass: *const nsCID, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterFactoryLocation)(self, aClass, aFile)
    }


    /// ```text
    /// /**
    ///      * isCIDRegistered
    ///      *
    ///      * Returns true if a factory is registered for the CID.
    ///      *
    ///      * @param aClass : CID queried for registeration
    ///      * @return       : true if a factory is registered for CID
    ///      *                 false otherwise.
    ///      */
    /// ```
    ///

    /// `boolean isCIDRegistered (in nsCIDRef aClass);`
    #[inline]
    pub unsafe fn IsCIDRegistered(&self, aClass: *const nsCID, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCIDRegistered)(self, aClass, _retval)
    }


    /// ```text
    /// /**
    ///      * isContractIDRegistered
    ///      *
    ///      * Returns true if a factory is registered for the contract id.
    ///      *
    ///      * @param aClass : contract id queried for registeration
    ///      * @return       : true if a factory is registered for contract id
    ///      *                 false otherwise.
    ///      */
    /// ```
    ///

    /// `boolean isContractIDRegistered (in string aContractID);`
    #[inline]
    pub unsafe fn IsContractIDRegistered(&self, aContractID: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsContractIDRegistered)(self, aContractID, _retval)
    }


    /// ```text
    /// /**
    ///      * getContractIDs
    ///      *
    ///      * Return the list of all registered ContractIDs.
    ///      *
    ///      * @return : Array of ContractIDs. Elements of the array are the string
    ///      *           encoding of Contract IDs.
    ///      */
    /// ```
    ///

    /// `Array<ACString> getContractIDs ();`
    #[inline]
    pub unsafe fn GetContractIDs(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetContractIDs)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * CIDToContractID
    ///      * @status OBSOLETE: This method is no longer implemented, but preserved
    ///      *                   in this interface for binary compatibility with
    ///      *                   Mozilla 1.9.2.
    ///      */
    /// ```
    ///

    /// `string CIDToContractID (in nsCIDRef aClass);`
    #[inline]
    pub unsafe fn CIDToContractID(&self, aClass: *const nsCID, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).CIDToContractID)(self, aClass, _retval)
    }


    /// ```text
    /// /**
    ///      * contractIDToCID
    ///      *
    ///      * Returns the CID for a given Contract ID, if one exists and is registered.
    ///      *
    ///      * @return : Contract ID.
    ///      */
    /// ```
    ///

    /// `nsCIDPtr contractIDToCID (in string aContractID);`
    #[inline]
    pub unsafe fn ContractIDToCID(&self, aContractID: *const libc::c_char, _retval: *mut *mut nsCID) -> ::nserror::nsresult {
        ((*self.vtable).ContractIDToCID)(self, aContractID, _retval)
    }


}



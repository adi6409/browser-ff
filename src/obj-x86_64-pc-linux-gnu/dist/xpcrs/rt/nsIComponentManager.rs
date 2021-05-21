//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIComponentManager.idl
//


/// `interface nsIComponentManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIComponentManager {
    vtable: *const nsIComponentManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIComponentManager.
unsafe impl XpCom for nsIComponentManager {
    const IID: nsIID = nsID(0xd604ffc3, 0x1ba3, 0x4f6c,
        [0xb6, 0x5f, 0x1e, 0xd4, 0x19, 0x93, 0x64, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIComponentManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIComponentManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIComponentManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIComponentManager`.
    fn coerce_from(v: &nsIComponentManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIComponentManagerCoerce for nsIComponentManager {
    #[inline]
    fn coerce_from(v: &nsIComponentManager) -> &Self {
        v
    }
}

impl nsIComponentManager {
    /// Cast this `nsIComponentManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIComponentManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIComponentManager {
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
impl<T: nsISupportsCoerce> nsIComponentManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIComponentManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIComponentManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIComponentManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub GetClassObject: unsafe extern "system" fn (this: *const nsIComponentManager, aClass: *const nsCID, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub GetClassObjectByContractID: unsafe extern "system" fn (this: *const nsIComponentManager, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub CreateInstance: unsafe extern "system" fn (this: *const nsIComponentManager, aClass: *const nsCID, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result); */
    pub CreateInstanceByContractID: unsafe extern "system" fn (this: *const nsIComponentManager, aContractID: *const libc::c_char, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void addBootstrappedManifestLocation (in nsIFile aLocation); */
    pub AddBootstrappedManifestLocation: unsafe extern "system" fn (this: *const nsIComponentManager, aLocation: *const nsIFile) -> ::nserror::nsresult,

    /* void removeBootstrappedManifestLocation (in nsIFile aLocation); */
    pub RemoveBootstrappedManifestLocation: unsafe extern "system" fn (this: *const nsIComponentManager, aLocation: *const nsIFile) -> ::nserror::nsresult,

    /* nsIArray getManifestLocations (); */
    pub GetManifestLocations: unsafe extern "system" fn (this: *const nsIComponentManager, _retval: *mut*const nsIArray) -> ::nserror::nsresult,

    /* nsIUTF8StringEnumerator getComponentJSMs (); */
    pub GetComponentJSMs: unsafe extern "system" fn (this: *const nsIComponentManager, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIComponentManager {

    /// ```text
    /// /**
    ///      * getClassObject
    ///      *
    ///      * Returns the factory object that can be used to create instances of
    ///      * CID aClass
    ///      *
    ///      * @param aClass The classid of the factory that is being requested
    ///      */
    /// ```
    ///

    /// `void getClassObject (in nsCIDRef aClass, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetClassObject(&self, aClass: *const nsCID, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetClassObject)(self, aClass, aIID, result)
    }


    /// ```text
    /// /**
    ///      * getClassObjectByContractID
    ///      *
    ///      * Returns the factory object that can be used to create instances of
    ///      * CID aClass
    ///      *
    ///      * @param aClass The classid of the factory that is being requested
    ///      */
    /// ```
    ///

    /// `void getClassObjectByContractID (in string aContractID, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn GetClassObjectByContractID(&self, aContractID: *const libc::c_char, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetClassObjectByContractID)(self, aContractID, aIID, result)
    }


    /// ```text
    /// /**
    ///      * createInstance
    ///      *
    ///      * Create an instance of the CID aClass and return the interface aIID.
    ///      *
    ///      * @param aClass : ClassID of object instance requested
    ///      * @param aDelegate : Used for aggregation
    ///      * @param aIID : IID of interface requested
    ///      */
    /// ```
    ///

    /// `void createInstance (in nsCIDRef aClass, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn CreateInstance(&self, aClass: *const nsCID, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstance)(self, aClass, aDelegate, aIID, result)
    }


    /// ```text
    /// /**
    ///      * createInstanceByContractID
    ///      *
    ///      * Create an instance of the CID that implements aContractID and return the
    ///      * interface aIID.
    ///      *
    ///      * @param aContractID : aContractID of object instance requested
    ///      * @param aDelegate : Used for aggregation
    ///      * @param aIID : IID of interface requested
    ///      */
    /// ```
    ///

    /// `void createInstanceByContractID (in string aContractID, in nsISupports aDelegate, in nsIIDRef aIID, [iid_is (aIID), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn CreateInstanceByContractID(&self, aContractID: *const libc::c_char, aDelegate: *const nsISupports, aIID: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateInstanceByContractID)(self, aContractID, aDelegate, aIID, result)
    }


    /// ```text
    /// /**
    ///      * addBootstrappedManifestLocation
    ///      *
    ///      * Adds a bootstrapped manifest location on runtime.
    ///      *
    ///      * @param aLocation : A directory where chrome.manifest resides,
    ///      *                    or an XPI with it on the root.
    ///      */
    /// ```
    ///

    /// `void addBootstrappedManifestLocation (in nsIFile aLocation);`
    #[inline]
    pub unsafe fn AddBootstrappedManifestLocation(&self, aLocation: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).AddBootstrappedManifestLocation)(self, aLocation)
    }


    /// ```text
    /// /**
    ///      * removeBootstrappedManifestLocation
    ///      *
    ///      * Removes a bootstrapped manifest location on runtime.
    ///      *
    ///      * @param aLocation : A directory where chrome.manifest resides,
    ///      *                    or an XPI with it on the root.
    ///      */
    /// ```
    ///

    /// `void removeBootstrappedManifestLocation (in nsIFile aLocation);`
    #[inline]
    pub unsafe fn RemoveBootstrappedManifestLocation(&self, aLocation: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).RemoveBootstrappedManifestLocation)(self, aLocation)
    }


    /// ```text
    /// /**
    ///      * getManifestLocations
    ///      *
    ///      * Get an array of nsIURIs of all registered and builtin manifest locations.
    ///      */
    /// ```
    ///

    /// `nsIArray getManifestLocations ();`
    #[inline]
    pub unsafe fn GetManifestLocations(&self, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetManifestLocations)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns a list of JSM URLs which are used to create components. This
    ///      * should only be used in automation.
    ///      */
    /// ```
    ///

    /// `nsIUTF8StringEnumerator getComponentJSMs ();`
    #[inline]
    pub unsafe fn GetComponentJSMs(&self, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetComponentJSMs)(self, _retval)
    }


}



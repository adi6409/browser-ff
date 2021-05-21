//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIPKCS11ModuleDB.idl
//


/// `interface nsIPKCS11ModuleDB : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPKCS11ModuleDB {
    vtable: *const nsIPKCS11ModuleDBVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPKCS11ModuleDB.
unsafe impl XpCom for nsIPKCS11ModuleDB {
    const IID: nsIID = nsID(0xff9fbcd7, 0x9517, 0x4334,
        [0xb9, 0x7a, 0xce, 0xed, 0x78, 0x90, 0x99, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPKCS11ModuleDB {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPKCS11ModuleDB.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPKCS11ModuleDBCoerce {
    /// Cheaply cast a value of this type from a `nsIPKCS11ModuleDB`.
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPKCS11ModuleDBCoerce for nsIPKCS11ModuleDB {
    #[inline]
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self {
        v
    }
}

impl nsIPKCS11ModuleDB {
    /// Cast this `nsIPKCS11ModuleDB` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPKCS11ModuleDBCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPKCS11ModuleDB {
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
impl<T: nsISupportsCoerce> nsIPKCS11ModuleDBCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPKCS11ModuleDB) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPKCS11ModuleDB
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPKCS11ModuleDBVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void deleteModule (in AString moduleName); */
    pub DeleteModule: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB, moduleName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use] void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags); */
    pub AddModule: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB, moduleName: *const ::nsstring::nsAString, libraryFullPath: *const ::nsstring::nsAString, cryptoMechanismFlags: i32, cipherFlags: i32) -> ::nserror::nsresult,

    /* [must_use] nsISimpleEnumerator listModules (); */
    pub ListModules: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean canToggleFIPS; */
    pub GetCanToggleFIPS: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB, aCanToggleFIPS: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void toggleFIPSMode (); */
    pub ToggleFIPSMode: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB) -> ::nserror::nsresult,

    /* [must_use] readonly attribute boolean isFIPSEnabled; */
    pub GetIsFIPSEnabled: unsafe extern "system" fn (this: *const nsIPKCS11ModuleDB, aIsFIPSEnabled: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPKCS11ModuleDB {


    /// `[must_use] void deleteModule (in AString moduleName);`
    #[inline]
    pub unsafe fn DeleteModule(&self, moduleName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).DeleteModule)(self, moduleName)
    }



    /// `[must_use] void addModule (in AString moduleName, in AString libraryFullPath, in long cryptoMechanismFlags, in long cipherFlags);`
    #[inline]
    pub unsafe fn AddModule(&self, moduleName: *const ::nsstring::nsAString, libraryFullPath: *const ::nsstring::nsAString, cryptoMechanismFlags: i32, cipherFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).AddModule)(self, moduleName, libraryFullPath, cryptoMechanismFlags, cipherFlags)
    }



    /// `[must_use] nsISimpleEnumerator listModules ();`
    #[inline]
    pub unsafe fn ListModules(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).ListModules)(self, _retval)
    }



    /// `[must_use] readonly attribute boolean canToggleFIPS;`
    #[inline]
    pub unsafe fn GetCanToggleFIPS(&self, aCanToggleFIPS: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanToggleFIPS)(self, aCanToggleFIPS)
    }



    /// `[must_use] void toggleFIPSMode ();`
    #[inline]
    pub unsafe fn ToggleFIPSMode(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ToggleFIPSMode)(self, )
    }



    /// `[must_use] readonly attribute boolean isFIPSEnabled;`
    #[inline]
    pub unsafe fn GetIsFIPSEnabled(&self, aIsFIPSEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFIPSEnabled)(self, aIsFIPSEnabled)
    }


}



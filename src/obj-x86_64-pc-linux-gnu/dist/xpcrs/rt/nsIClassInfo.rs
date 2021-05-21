//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/components/nsIClassInfo.idl
//


/// `interface nsIClassInfo : nsISupports`
///

/// ```text
/// /**
///  * Provides information about a specific implementation class.  If you want
///  * your class to implement nsIClassInfo, see nsIClassInfoImpl.h for
///  * instructions--you most likely do not want to inherit from nsIClassInfo.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClassInfo {
    vtable: *const nsIClassInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClassInfo.
unsafe impl XpCom for nsIClassInfo {
    const IID: nsIID = nsID(0xa60569d7, 0xd401, 0x4677,
        [0xba, 0x63, 0x2a, 0xa5, 0x97, 0x1a, 0xf2, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClassInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClassInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClassInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIClassInfo`.
    fn coerce_from(v: &nsIClassInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClassInfoCoerce for nsIClassInfo {
    #[inline]
    fn coerce_from(v: &nsIClassInfo) -> &Self {
        v
    }
}

impl nsIClassInfo {
    /// Cast this `nsIClassInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClassInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClassInfo {
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
impl<T: nsISupportsCoerce> nsIClassInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClassInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClassInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClassInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<nsIIDRef> interfaces; */
    pub GetInterfaces: unsafe extern "system" fn (this: *const nsIClassInfo, aInterfaces: *mut thin_vec::ThinVec<nsIID>) -> ::nserror::nsresult,

    /* nsIXPCScriptable getScriptableHelper (); */
    pub GetScriptableHelper: unsafe extern "system" fn (this: *const nsIClassInfo, _retval: *mut*const nsIXPCScriptable) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String contractID; */
    pub GetContractID: unsafe extern "system" fn (this: *const nsIClassInfo, aContractID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String classDescription; */
    pub GetClassDescription: unsafe extern "system" fn (this: *const nsIClassInfo, aClassDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute nsCIDPtr classID; */
    pub GetClassID: unsafe extern "system" fn (this: *const nsIClassInfo, aClassID: *mut *mut nsCID) -> ::nserror::nsresult,

    /* readonly attribute uint32_t flags; */
    pub GetFlags: unsafe extern "system" fn (this: *const nsIClassInfo, aFlags: *mut uint32_t) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsCID classIDNoAlloc; */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetClassIDNoAlloc: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClassInfo {
    /// ```text
    /// /**
    ///      * Bitflags for 'flags' attribute.
    ///      */
    /// ```
    ///

    pub const SINGLETON: i64 = 1;


    pub const THREADSAFE: i64 = 2;


    pub const MAIN_THREAD_ONLY: i64 = 4;


    pub const DOM_OBJECT: i64 = 8;


    pub const PLUGIN_OBJECT: i64 = 16;


    pub const SINGLETON_CLASSINFO: i64 = 32;


    pub const RESERVED: i64 = 2147483648;

    /// ```text
    /// /**
    ///      * Returns a list of the interfaces which instances of this class promise
    ///      * to implement. Note that nsISupports is an implicit member of any such
    ///      * list, and need not be included.
    ///      */
    /// ```
    ///

    /// `readonly attribute Array<nsIIDRef> interfaces;`
    #[inline]
    pub unsafe fn GetInterfaces(&self, aInterfaces: *mut thin_vec::ThinVec<nsIID>) -> ::nserror::nsresult {
        ((*self.vtable).GetInterfaces)(self, aInterfaces)
    }


    /// ```text
    /// /**
    ///      * Return an object to assist XPConnect in supplying JavaScript-specific
    ///      * behavior to callers of the instance object, or null if not needed.
    ///      */
    /// ```
    ///

    /// `nsIXPCScriptable getScriptableHelper ();`
    #[inline]
    pub unsafe fn GetScriptableHelper(&self, _retval: *mut*const nsIXPCScriptable) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptableHelper)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * A contract ID through which an instance of this class can be created
    ///      * (or accessed as a service, if |flags & SINGLETON|), or null/void.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String contractID;`
    #[inline]
    pub unsafe fn GetContractID(&self, aContractID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetContractID)(self, aContractID)
    }


    /// ```text
    /// /**
    ///      * A human readable string naming the class, or null/void.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String classDescription;`
    #[inline]
    pub unsafe fn GetClassDescription(&self, aClassDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetClassDescription)(self, aClassDescription)
    }


    /// ```text
    /// /**
    ///      * A class ID through which an instance of this class can be created
    ///      * (or accessed as a service, if |flags & SINGLETON|), or null.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsCIDPtr classID;`
    #[inline]
    pub unsafe fn GetClassID(&self, aClassID: *mut *mut nsCID) -> ::nserror::nsresult {
        ((*self.vtable).GetClassID)(self, aClassID)
    }



    /// `readonly attribute uint32_t flags;`
    #[inline]
    pub unsafe fn GetFlags(&self, aFlags: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFlags)(self, aFlags)
    }


    /// ```text
    /// /**
    ///      * Also a class ID through which an instance of this class can be created
    ///      * (or accessed as a service, if |flags & SINGLETON|).  If the class does
    ///      * not have a CID, it should return NS_ERROR_NOT_AVAILABLE.  This attribute
    ///      * exists so C++ callers can avoid allocating and freeing a CID, as would
    ///      * happen if they used classID.
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute nsCID classIDNoAlloc;`
    const _GetClassIDNoAlloc: () = ();

}



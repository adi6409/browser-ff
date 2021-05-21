//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginMetaInfo.idl
//


/// `interface nsILoginMetaInfo : nsISupports`
///

/// ```text
/// /**
///  * An object containing metainfo for a login stored by the login manager.
///  *
///  * Code using login manager can generally ignore this interface. When adding
///  * logins, default value will be created. When modifying logins, these
///  * properties will be unchanged unless a change is explicitly requested [by
    ///  * using modifyLogin() with a nsIPropertyBag]. When deleting a login or
///  * comparing logins, these properties are ignored.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginMetaInfo {
    vtable: *const nsILoginMetaInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginMetaInfo.
unsafe impl XpCom for nsILoginMetaInfo {
    const IID: nsIID = nsID(0x20d8eb40, 0xc494, 0x497f,
        [0xb2, 0xa6, 0xaa, 0xa3, 0x2f, 0x80, 0x7e, 0xbd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginMetaInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginMetaInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginMetaInfoCoerce {
    /// Cheaply cast a value of this type from a `nsILoginMetaInfo`.
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginMetaInfoCoerce for nsILoginMetaInfo {
    #[inline]
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self {
        v
    }
}

impl nsILoginMetaInfo {
    /// Cast this `nsILoginMetaInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginMetaInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginMetaInfo {
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
impl<T: nsISupportsCoerce> nsILoginMetaInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginMetaInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginMetaInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginMetaInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString guid; */
    pub GetGuid: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aGuid: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString guid; */
    pub SetGuid: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aGuid: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute unsigned long long timeCreated; */
    pub GetTimeCreated: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimeCreated: *mut u64) -> ::nserror::nsresult,

    /* attribute unsigned long long timeCreated; */
    pub SetTimeCreated: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimeCreated: u64) -> ::nserror::nsresult,

    /* attribute unsigned long long timeLastUsed; */
    pub GetTimeLastUsed: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimeLastUsed: *mut u64) -> ::nserror::nsresult,

    /* attribute unsigned long long timeLastUsed; */
    pub SetTimeLastUsed: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimeLastUsed: u64) -> ::nserror::nsresult,

    /* attribute unsigned long long timePasswordChanged; */
    pub GetTimePasswordChanged: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimePasswordChanged: *mut u64) -> ::nserror::nsresult,

    /* attribute unsigned long long timePasswordChanged; */
    pub SetTimePasswordChanged: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimePasswordChanged: u64) -> ::nserror::nsresult,

    /* attribute unsigned long timesUsed; */
    pub GetTimesUsed: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimesUsed: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long timesUsed; */
    pub SetTimesUsed: unsafe extern "system" fn (this: *const nsILoginMetaInfo, aTimesUsed: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginMetaInfo {

    /// ```text
    /// /**
    ///    * The GUID to uniquely identify the login. This can be any arbitrary
    ///    * string, but a format as created by nsIUUIDGenerator is recommended.
    ///    * For example, "{d4e1a1f6-5ea0-40ee-bff5-da57982f21cf}"
    ///    *
    ///    * addLogin will generate a random value unless a value is provided.
    ///    *
    ///    * addLogin and modifyLogin will throw if the GUID already exists.
    ///    */
    /// ```
    ///

    /// `attribute AString guid;`
    #[inline]
    pub unsafe fn GetGuid(&self, aGuid: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetGuid)(self, aGuid)
    }


    /// ```text
    /// /**
    ///    * The GUID to uniquely identify the login. This can be any arbitrary
    ///    * string, but a format as created by nsIUUIDGenerator is recommended.
    ///    * For example, "{d4e1a1f6-5ea0-40ee-bff5-da57982f21cf}"
    ///    *
    ///    * addLogin will generate a random value unless a value is provided.
    ///    *
    ///    * addLogin and modifyLogin will throw if the GUID already exists.
    ///    */
    /// ```
    ///

    /// `attribute AString guid;`
    #[inline]
    pub unsafe fn SetGuid(&self, aGuid: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetGuid)(self, aGuid)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was first created.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timeCreated;`
    #[inline]
    pub unsafe fn GetTimeCreated(&self, aTimeCreated: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeCreated)(self, aTimeCreated)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was first created.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timeCreated;`
    #[inline]
    pub unsafe fn SetTimeCreated(&self, aTimeCreated: u64) -> ::nserror::nsresult {
        ((*self.vtable).SetTimeCreated)(self, aTimeCreated)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was last submitted
    ///    * in a form or used to begin an HTTP auth session.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timeLastUsed;`
    #[inline]
    pub unsafe fn GetTimeLastUsed(&self, aTimeLastUsed: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeLastUsed)(self, aTimeLastUsed)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was last submitted
    ///    * in a form or used to begin an HTTP auth session.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timeLastUsed;`
    #[inline]
    pub unsafe fn SetTimeLastUsed(&self, aTimeLastUsed: u64) -> ::nserror::nsresult {
        ((*self.vtable).SetTimeLastUsed)(self, aTimeLastUsed)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was last modified.
    ///    *
    ///    * Contrary to what the name may suggest, this attribute takes into account
    ///    * not only the password but also the username attribute.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timePasswordChanged;`
    #[inline]
    pub unsafe fn GetTimePasswordChanged(&self, aTimePasswordChanged: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetTimePasswordChanged)(self, aTimePasswordChanged)
    }


    /// ```text
    /// /**
    ///    * The time, in Unix Epoch milliseconds, when the login was last modified.
    ///    *
    ///    * Contrary to what the name may suggest, this attribute takes into account
    ///    * not only the password but also the username attribute.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long long timePasswordChanged;`
    #[inline]
    pub unsafe fn SetTimePasswordChanged(&self, aTimePasswordChanged: u64) -> ::nserror::nsresult {
        ((*self.vtable).SetTimePasswordChanged)(self, aTimePasswordChanged)
    }


    /// ```text
    /// /**
    ///    * The number of times the login was submitted in a form or used to begin
    ///    * an HTTP auth session.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long timesUsed;`
    #[inline]
    pub unsafe fn GetTimesUsed(&self, aTimesUsed: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTimesUsed)(self, aTimesUsed)
    }


    /// ```text
    /// /**
    ///    * The number of times the login was submitted in a form or used to begin
    ///    * an HTTP auth session.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long timesUsed;`
    #[inline]
    pub unsafe fn SetTimesUsed(&self, aTimesUsed: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetTimesUsed)(self, aTimesUsed)
    }


}



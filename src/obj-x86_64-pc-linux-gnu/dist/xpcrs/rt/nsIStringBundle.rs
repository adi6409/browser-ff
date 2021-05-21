//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/strres/nsIStringBundle.idl
//


/// `interface nsIStringBundle : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStringBundle {
    vtable: *const nsIStringBundleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStringBundle.
unsafe impl XpCom for nsIStringBundle {
    const IID: nsIID = nsID(0xd85a17c2, 0xaa7c, 0x11d2,
        [0x9b, 0x8c, 0x00, 0x80, 0x5f, 0x8a, 0x16, 0xd9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStringBundle {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStringBundle.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStringBundleCoerce {
    /// Cheaply cast a value of this type from a `nsIStringBundle`.
    fn coerce_from(v: &nsIStringBundle) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStringBundleCoerce for nsIStringBundle {
    #[inline]
    fn coerce_from(v: &nsIStringBundle) -> &Self {
        v
    }
}

impl nsIStringBundle {
    /// Cast this `nsIStringBundle` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStringBundleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStringBundle {
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
impl<T: nsISupportsCoerce> nsIStringBundleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringBundle) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStringBundle
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStringBundleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString GetStringFromID (in long aID); */
    pub GetStringFromID: unsafe extern "system" fn (this: *const nsIStringBundle, aID: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [binaryname(GetStringFromAUTF8Name)] AString GetStringFromName (in AUTF8String aName); */
    pub GetStringFromAUTF8Name: unsafe extern "system" fn (this: *const nsIStringBundle, aName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [binaryname(GetStringFromName),noscript] AString GetStringFromNameCpp (in string aName); */
    pub GetStringFromName: unsafe extern "system" fn (this: *const nsIStringBundle, aName: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString formatStringFromID (in long aID, in Array<AString> params); */
    pub FormatStringFromID: unsafe extern "system" fn (this: *const nsIStringBundle, aID: i32, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [binaryname(FormatStringFromAUTF8Name)] AString formatStringFromName (in AUTF8String aName, in Array<AString> params); */
    pub FormatStringFromAUTF8Name: unsafe extern "system" fn (this: *const nsIStringBundle, aName: *const ::nsstring::nsACString, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [binaryname(FormatStringFromName),noscript] AString formatStringFromNameCpp (in string aName, in Array<AString> params); */
    pub FormatStringFromName: unsafe extern "system" fn (this: *const nsIStringBundle, aName: *const libc::c_char, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsISimpleEnumerator getSimpleEnumeration (); */
    pub GetSimpleEnumeration: unsafe extern "system" fn (this: *const nsIStringBundle, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* void asyncPreload (); */
    pub AsyncPreload: unsafe extern "system" fn (this: *const nsIStringBundle) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStringBundle {


    /// `AString GetStringFromID (in long aID);`
    #[inline]
    pub unsafe fn GetStringFromID(&self, aID: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringFromID)(self, aID, _retval)
    }



    /// `[binaryname(GetStringFromAUTF8Name)] AString GetStringFromName (in AUTF8String aName);`
    #[inline]
    pub unsafe fn GetStringFromAUTF8Name(&self, aName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringFromAUTF8Name)(self, aName, _retval)
    }



    /// `[binaryname(GetStringFromName),noscript] AString GetStringFromNameCpp (in string aName);`
    #[inline]
    pub unsafe fn GetStringFromName(&self, aName: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetStringFromName)(self, aName, _retval)
    }



    /// `AString formatStringFromID (in long aID, in Array<AString> params);`
    #[inline]
    pub unsafe fn FormatStringFromID(&self, aID: i32, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatStringFromID)(self, aID, params, _retval)
    }



    /// `[binaryname(FormatStringFromAUTF8Name)] AString formatStringFromName (in AUTF8String aName, in Array<AString> params);`
    #[inline]
    pub unsafe fn FormatStringFromAUTF8Name(&self, aName: *const ::nsstring::nsACString, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatStringFromAUTF8Name)(self, aName, params, _retval)
    }



    /// `[binaryname(FormatStringFromName),noscript] AString formatStringFromNameCpp (in string aName, in Array<AString> params);`
    #[inline]
    pub unsafe fn FormatStringFromName(&self, aName: *const libc::c_char, params: *const thin_vec::ThinVec<::nsstring::nsString>, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatStringFromName)(self, aName, params, _retval)
    }



    /// `nsISimpleEnumerator getSimpleEnumeration ();`
    #[inline]
    pub unsafe fn GetSimpleEnumeration(&self, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetSimpleEnumeration)(self, _retval)
    }



    /// `void asyncPreload ();`
    #[inline]
    pub unsafe fn AsyncPreload(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AsyncPreload)(self, )
    }


}


/// `interface nsIStringBundleService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStringBundleService {
    vtable: *const nsIStringBundleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStringBundleService.
unsafe impl XpCom for nsIStringBundleService {
    const IID: nsIID = nsID(0xd85a17c0, 0xaa7c, 0x11d2,
        [0x9b, 0x8c, 0x00, 0x80, 0x5f, 0x8a, 0x16, 0xd9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStringBundleService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStringBundleService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStringBundleServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIStringBundleService`.
    fn coerce_from(v: &nsIStringBundleService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStringBundleServiceCoerce for nsIStringBundleService {
    #[inline]
    fn coerce_from(v: &nsIStringBundleService) -> &Self {
        v
    }
}

impl nsIStringBundleService {
    /// Cast this `nsIStringBundleService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStringBundleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStringBundleService {
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
impl<T: nsISupportsCoerce> nsIStringBundleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringBundleService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStringBundleService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStringBundleServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIStringBundle createBundle (in string aURLSpec); */
    pub CreateBundle: unsafe extern "system" fn (this: *const nsIStringBundleService, aURLSpec: *const libc::c_char, _retval: *mut *const nsIStringBundle) -> ::nserror::nsresult,

    /* AString formatStatusMessage (in nsresult aStatus, in wstring aStatusArg); */
    pub FormatStatusMessage: unsafe extern "system" fn (this: *const nsIStringBundleService, aStatus: ::nserror::nsresult, aStatusArg: *const i16, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void flushBundles (); */
    pub FlushBundles: unsafe extern "system" fn (this: *const nsIStringBundleService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStringBundleService {


    /// `nsIStringBundle createBundle (in string aURLSpec);`
    #[inline]
    pub unsafe fn CreateBundle(&self, aURLSpec: *const libc::c_char, _retval: *mut *const nsIStringBundle) -> ::nserror::nsresult {
        ((*self.vtable).CreateBundle)(self, aURLSpec, _retval)
    }


    /// ```text
    /// /**
    ///    * Formats a message string from a status code and status arguments.
    ///    * @param aStatus - The status code. This is mapped into a string ID and
    ///    *            and used in the string lookup process (see nsIErrorService).
    ///    * @param aStatusArg - The status message argument(s). Multiple arguments
    ///    *            can be separated by newline ('\n') characters.
    ///    * @return the formatted message
    ///    */
    /// ```
    ///

    /// `AString formatStatusMessage (in nsresult aStatus, in wstring aStatusArg);`
    #[inline]
    pub unsafe fn FormatStatusMessage(&self, aStatus: ::nserror::nsresult, aStatusArg: *const i16, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).FormatStatusMessage)(self, aStatus, aStatusArg, _retval)
    }


    /// ```text
    /// /**
    ///    * flushes the string bundle cache - useful when the locale changes or
    ///    * when we need to get some extra memory back
    ///    *
    ///    * at some point, we might want to make this flush all the bundles,
    ///    * because any bundles that are floating around when the locale changes
    ///    * will suddenly contain bad data
    ///    *
    ///    */
    /// ```
    ///

    /// `void flushBundles ();`
    #[inline]
    pub unsafe fn FlushBundles(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).FlushBundles)(self, )
    }


}



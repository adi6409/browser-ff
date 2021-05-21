//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIGSettingsService.idl
//


/// `interface nsIGSettingsCollection : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGSettingsCollection {
    vtable: *const nsIGSettingsCollectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGSettingsCollection.
unsafe impl XpCom for nsIGSettingsCollection {
    const IID: nsIID = nsID(0x16d5b0ed, 0xe756, 0x4f1b,
        [0xa8, 0xce, 0x91, 0x32, 0xe8, 0x69, 0xac, 0xd8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGSettingsCollection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGSettingsCollection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGSettingsCollectionCoerce {
    /// Cheaply cast a value of this type from a `nsIGSettingsCollection`.
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGSettingsCollectionCoerce for nsIGSettingsCollection {
    #[inline]
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self {
        v
    }
}

impl nsIGSettingsCollection {
    /// Cast this `nsIGSettingsCollection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGSettingsCollectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGSettingsCollection {
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
impl<T: nsISupportsCoerce> nsIGSettingsCollectionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGSettingsCollection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGSettingsCollection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGSettingsCollectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setString (in AUTF8String key, in AUTF8String value); */
    pub SetString: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setBoolean (in AUTF8String key, in boolean value); */
    pub SetBoolean: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, value: bool) -> ::nserror::nsresult,

    /* void setInt (in AUTF8String key, in long value); */
    pub SetInt: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, value: i32) -> ::nserror::nsresult,

    /* AUTF8String getString (in AUTF8String key); */
    pub GetString: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean getBoolean (in AUTF8String key); */
    pub GetBoolean: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* long getInt (in AUTF8String key); */
    pub GetInt: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, _retval: *mut i32) -> ::nserror::nsresult,

    /* nsIArray getStringList (in AUTF8String key); */
    pub GetStringList: unsafe extern "system" fn (this: *const nsIGSettingsCollection, key: *const ::nsstring::nsACString, _retval: *mut*const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGSettingsCollection {


    /// `void setString (in AUTF8String key, in AUTF8String value);`
    #[inline]
    pub unsafe fn SetString(&self, key: *const ::nsstring::nsACString, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetString)(self, key, value)
    }



    /// `void setBoolean (in AUTF8String key, in boolean value);`
    #[inline]
    pub unsafe fn SetBoolean(&self, key: *const ::nsstring::nsACString, value: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetBoolean)(self, key, value)
    }



    /// `void setInt (in AUTF8String key, in long value);`
    #[inline]
    pub unsafe fn SetInt(&self, key: *const ::nsstring::nsACString, value: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetInt)(self, key, value)
    }



    /// `AUTF8String getString (in AUTF8String key);`
    #[inline]
    pub unsafe fn GetString(&self, key: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetString)(self, key, _retval)
    }



    /// `boolean getBoolean (in AUTF8String key);`
    #[inline]
    pub unsafe fn GetBoolean(&self, key: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBoolean)(self, key, _retval)
    }



    /// `long getInt (in AUTF8String key);`
    #[inline]
    pub unsafe fn GetInt(&self, key: *const ::nsstring::nsACString, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetInt)(self, key, _retval)
    }



    /// `nsIArray getStringList (in AUTF8String key);`
    #[inline]
    pub unsafe fn GetStringList(&self, key: *const ::nsstring::nsACString, _retval: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetStringList)(self, key, _retval)
    }


}


/// `interface nsIGSettingsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGSettingsService {
    vtable: *const nsIGSettingsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGSettingsService.
unsafe impl XpCom for nsIGSettingsService {
    const IID: nsIID = nsID(0x849c088b, 0x57d1, 0x4f24,
        [0xb7, 0xb2, 0x3d, 0xc4, 0xac, 0xb0, 0x4c, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGSettingsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGSettingsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGSettingsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIGSettingsService`.
    fn coerce_from(v: &nsIGSettingsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGSettingsServiceCoerce for nsIGSettingsService {
    #[inline]
    fn coerce_from(v: &nsIGSettingsService) -> &Self {
        v
    }
}

impl nsIGSettingsService {
    /// Cast this `nsIGSettingsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGSettingsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGSettingsService {
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
impl<T: nsISupportsCoerce> nsIGSettingsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGSettingsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGSettingsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGSettingsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema); */
    pub GetCollectionForSchema: unsafe extern "system" fn (this: *const nsIGSettingsService, schema: *const ::nsstring::nsACString, _retval: *mut *const nsIGSettingsCollection) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGSettingsService {


    /// `nsIGSettingsCollection getCollectionForSchema (in AUTF8String schema);`
    #[inline]
    pub unsafe fn GetCollectionForSchema(&self, schema: *const ::nsstring::nsACString, _retval: *mut *const nsIGSettingsCollection) -> ::nserror::nsresult {
        ((*self.vtable).GetCollectionForSchema)(self, schema, _retval)
    }


}



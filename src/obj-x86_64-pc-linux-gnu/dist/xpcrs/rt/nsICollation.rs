//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/nsICollation.idl
//


/// `interface nsICollationFactory : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICollationFactory {
    vtable: *const nsICollationFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICollationFactory.
unsafe impl XpCom for nsICollationFactory {
    const IID: nsIID = nsID(0x04971e14, 0xd6b3, 0x4ada,
        [0x8c, 0xbb, 0xc3, 0xa1, 0x38, 0x42, 0xb3, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICollationFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICollationFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICollationFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsICollationFactory`.
    fn coerce_from(v: &nsICollationFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICollationFactoryCoerce for nsICollationFactory {
    #[inline]
    fn coerce_from(v: &nsICollationFactory) -> &Self {
        v
    }
}

impl nsICollationFactory {
    /// Cast this `nsICollationFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICollationFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICollationFactory {
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
impl<T: nsISupportsCoerce> nsICollationFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICollationFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICollationFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICollationFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsICollation CreateCollation (); */
    pub CreateCollation: unsafe extern "system" fn (this: *const nsICollationFactory, _retval: *mut*const nsICollation) -> ::nserror::nsresult,

    /* nsICollation CreateCollationForLocale (in ACString locale); */
    pub CreateCollationForLocale: unsafe extern "system" fn (this: *const nsICollationFactory, locale: *const ::nsstring::nsACString, _retval: *mut*const nsICollation) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICollationFactory {

    /// ```text
    /// /**
    ///      * Create a new collation for the current application locale.
    ///      *
    ///      * @return A new collation.
    ///      */
    /// ```
    ///

    /// `nsICollation CreateCollation ();`
    #[inline]
    pub unsafe fn CreateCollation(&self, _retval: *mut*const nsICollation) -> ::nserror::nsresult {
        ((*self.vtable).CreateCollation)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Create a new collation for a given locale.
    ///      *
    ///      * @return A new collation.
    ///      */
    /// ```
    ///

    /// `nsICollation CreateCollationForLocale (in ACString locale);`
    #[inline]
    pub unsafe fn CreateCollationForLocale(&self, locale: *const ::nsstring::nsACString, _retval: *mut*const nsICollation) -> ::nserror::nsresult {
        ((*self.vtable).CreateCollationForLocale)(self, locale, _retval)
    }


}


/// `interface nsICollation : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICollation {
    vtable: *const nsICollationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICollation.
unsafe impl XpCom for nsICollation {
    const IID: nsIID = nsID(0xb0132cc0, 0x3786, 0x4557,
        [0x98, 0x74, 0x91, 0x0d, 0x7d, 0xef, 0x5f, 0x93]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICollation {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICollation.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICollationCoerce {
    /// Cheaply cast a value of this type from a `nsICollation`.
    fn coerce_from(v: &nsICollation) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICollationCoerce for nsICollation {
    #[inline]
    fn coerce_from(v: &nsICollation) -> &Self {
        v
    }
}

impl nsICollation {
    /// Cast this `nsICollation` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICollationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICollation {
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
impl<T: nsISupportsCoerce> nsICollationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICollation) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICollation
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICollationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void initialize (in ACString locale); */
    pub Initialize: unsafe extern "system" fn (this: *const nsICollation, locale: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* long compareString (in long strength, in AString string1, in AString string2); */
    pub CompareString: unsafe extern "system" fn (this: *const nsICollation, strength: i32, string1: *const ::nsstring::nsAString, string2: *const ::nsstring::nsAString, _retval: *mut i32) -> ::nserror::nsresult,

    /* [noscript] Array<octet> allocateRawSortKey (in long strength, in AString stringIn); */
    pub AllocateRawSortKey: unsafe extern "system" fn (this: *const nsICollation, strength: i32, stringIn: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<u8>) -> ::nserror::nsresult,

    /* [noscript] long compareRawSortKey (in Array<octet> key1, in Array<octet> key2); */
    pub CompareRawSortKey: unsafe extern "system" fn (this: *const nsICollation, key1: *const thin_vec::ThinVec<u8>, key2: *const thin_vec::ThinVec<u8>, _retval: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICollation {

    pub const kCollationStrengthDefault: i64 = 0;


    pub const kCollationCaseInsensitiveAscii: i64 = 1;


    pub const kCollationAccentInsenstive: i64 = 2;


    pub const kCollationCaseSensitive: i64 = 0;


    pub const kCollationCaseInSensitive: i64 = 3;


    /// `void initialize (in ACString locale);`
    #[inline]
    pub unsafe fn Initialize(&self, locale: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Initialize)(self, locale)
    }



    /// `long compareString (in long strength, in AString string1, in AString string2);`
    #[inline]
    pub unsafe fn CompareString(&self, strength: i32, string1: *const ::nsstring::nsAString, string2: *const ::nsstring::nsAString, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).CompareString)(self, strength, string1, string2, _retval)
    }



    /// `[noscript] Array<octet> allocateRawSortKey (in long strength, in AString stringIn);`
    #[inline]
    pub unsafe fn AllocateRawSortKey(&self, strength: i32, stringIn: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<u8>) -> ::nserror::nsresult {
        ((*self.vtable).AllocateRawSortKey)(self, strength, stringIn, _retval)
    }



    /// `[noscript] long compareRawSortKey (in Array<octet> key1, in Array<octet> key2);`
    #[inline]
    pub unsafe fn CompareRawSortKey(&self, key1: *const thin_vec::ThinVec<u8>, key2: *const thin_vec::ThinVec<u8>, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).CompareRawSortKey)(self, key1, key2, _retval)
    }


}



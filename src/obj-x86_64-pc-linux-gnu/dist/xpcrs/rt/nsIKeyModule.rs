//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIKeyModule.idl
//


/// `interface nsIKeyObject : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyObject {
    vtable: *const nsIKeyObjectVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyObject.
unsafe impl XpCom for nsIKeyObject {
    const IID: nsIID = nsID(0xee2dc516, 0xba7b, 0x4e77,
        [0x89, 0xfe, 0xc4, 0x3b, 0x88, 0x6e, 0xf7, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyObject {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyObject.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyObjectCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyObject`.
    fn coerce_from(v: &nsIKeyObject) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyObjectCoerce for nsIKeyObject {
    #[inline]
    fn coerce_from(v: &nsIKeyObject) -> &Self {
        v
    }
}

impl nsIKeyObject {
    /// Cast this `nsIKeyObject` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyObjectCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyObject {
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
impl<T: nsISupportsCoerce> nsIKeyObjectCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyObject) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyObject
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyObjectVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use,noscript] void initKey (in short aAlgorithm, in PK11SymKeyPtr aKey); */
    /// Unable to generate binding because `native type PK11SymKey unsupported`
    pub InitKey: *const ::libc::c_void,

    /* [must_use,noscript] PK11SymKeyPtr getKeyObj (); */
    /// Unable to generate binding because `native type PK11SymKey unsupported`
    pub GetKeyObj: *const ::libc::c_void,

    /* [must_use] short getType (); */
    pub GetType: unsafe extern "system" fn (this: *const nsIKeyObject, _retval: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyObject {

    pub const SYM_KEY: i64 = 1;


    pub const HMAC: i64 = 257;


    /// `[must_use,noscript] void initKey (in short aAlgorithm, in PK11SymKeyPtr aKey);`
    const _InitKey: () = ();


    /// `[must_use,noscript] PK11SymKeyPtr getKeyObj ();`
    const _GetKeyObj: () = ();


    /// `[must_use] short getType ();`
    #[inline]
    pub unsafe fn GetType(&self, _retval: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, _retval)
    }


}


/// `interface nsIKeyObjectFactory : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIKeyObjectFactory {
    vtable: *const nsIKeyObjectFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIKeyObjectFactory.
unsafe impl XpCom for nsIKeyObjectFactory {
    const IID: nsIID = nsID(0x838bdbf1, 0x8244, 0x448f,
        [0x8b, 0xcd, 0xce, 0xde, 0x70, 0x22, 0x7d, 0x75]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIKeyObjectFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIKeyObjectFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIKeyObjectFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsIKeyObjectFactory`.
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIKeyObjectFactoryCoerce for nsIKeyObjectFactory {
    #[inline]
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self {
        v
    }
}

impl nsIKeyObjectFactory {
    /// Cast this `nsIKeyObjectFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIKeyObjectFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIKeyObjectFactory {
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
impl<T: nsISupportsCoerce> nsIKeyObjectFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIKeyObjectFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIKeyObjectFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIKeyObjectFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
    pub KeyFromString: unsafe extern "system" fn (this: *const nsIKeyObjectFactory, aAlgorithm: i16, aKey: *const ::nsstring::nsACString, _retval: *mut *const nsIKeyObject) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIKeyObjectFactory {


    /// `[must_use] nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey);`
    #[inline]
    pub unsafe fn KeyFromString(&self, aAlgorithm: i16, aKey: *const ::nsstring::nsACString, _retval: *mut *const nsIKeyObject) -> ::nserror::nsresult {
        ((*self.vtable).KeyFromString)(self, aAlgorithm, aKey, _retval)
    }


}



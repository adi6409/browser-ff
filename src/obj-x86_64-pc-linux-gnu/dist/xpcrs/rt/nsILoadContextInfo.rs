//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsILoadContextInfo.idl
//


/// `interface nsILoadContextInfo : nsISupports`
///

/// ```text
/// /**
///  * Helper interface to carry informatin about the load context
///  * encapsulating origin attributes and IsAnonymous, IsPrivite properties.
///  * It shall be used where nsILoadContext cannot be used or is not
///  * available.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadContextInfo {
    vtable: *const nsILoadContextInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadContextInfo.
unsafe impl XpCom for nsILoadContextInfo {
    const IID: nsIID = nsID(0x555e2f8a, 0xa1f6, 0x41dd,
        [0x88, 0xca, 0xed, 0x4e, 0xd6, 0xb9, 0x8a, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadContextInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadContextInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadContextInfoCoerce {
    /// Cheaply cast a value of this type from a `nsILoadContextInfo`.
    fn coerce_from(v: &nsILoadContextInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadContextInfoCoerce for nsILoadContextInfo {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfo) -> &Self {
        v
    }
}

impl nsILoadContextInfo {
    /// Cast this `nsILoadContextInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadContextInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadContextInfo {
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
impl<T: nsISupportsCoerce> nsILoadContextInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadContextInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadContextInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isPrivate; */
    pub GetIsPrivate: unsafe extern "system" fn (this: *const nsILoadContextInfo, aIsPrivate: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isAnonymous; */
    pub GetIsAnonymous: unsafe extern "system" fn (this: *const nsILoadContextInfo, aIsAnonymous: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [binaryname(OriginAttributesPtr),noscript,nostdcall,notxpcom] OriginAttributesNativePtr binaryOriginAttributesPtr (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub OriginAttributesPtr: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadContextInfo {

    /// ```text
    /// /**
    ///    * Whether the context is in a Private Browsing mode
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isPrivate;`
    #[inline]
    pub unsafe fn GetIsPrivate(&self, aIsPrivate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsPrivate)(self, aIsPrivate)
    }


    /// ```text
    /// /**
    ///    * Whether the load is initiated as anonymous
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isAnonymous;`
    #[inline]
    pub unsafe fn GetIsAnonymous(&self, aIsAnonymous: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAnonymous)(self, aIsAnonymous)
    }


    /// ```text
    /// /**
    ///    * OriginAttributes hiding all the security context attributes
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval originAttributes;`
    const _GetOriginAttributes: () = ();


    /// `[binaryname(OriginAttributesPtr),noscript,nostdcall,notxpcom] OriginAttributesNativePtr binaryOriginAttributesPtr ();`
    const _OriginAttributesPtr: () = ();

}


/// `interface nsILoadContextInfoFactory : nsISupports`
///

/// ```text
/// /**
///  * Since OriginAttributes struct limits the implementation of
///  * nsILoadContextInfo (that needs to be thread safe) to C++,
///  * we need a scriptable factory to create instances of that
///  * interface from JS.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoadContextInfoFactory {
    vtable: *const nsILoadContextInfoFactoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoadContextInfoFactory.
unsafe impl XpCom for nsILoadContextInfoFactory {
    const IID: nsIID = nsID(0xc1c7023d, 0x4318, 0x4f99,
        [0x83, 0x07, 0xb5, 0xcc, 0xf0, 0x55, 0x87, 0x93]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoadContextInfoFactory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoadContextInfoFactory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoadContextInfoFactoryCoerce {
    /// Cheaply cast a value of this type from a `nsILoadContextInfoFactory`.
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoadContextInfoFactoryCoerce for nsILoadContextInfoFactory {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self {
        v
    }
}

impl nsILoadContextInfoFactory {
    /// Cast this `nsILoadContextInfoFactory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoadContextInfoFactoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoadContextInfoFactory {
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
impl<T: nsISupportsCoerce> nsILoadContextInfoFactoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoadContextInfoFactory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoadContextInfoFactory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoadContextInfoFactoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsILoadContextInfo default; */
    pub GetDefault: unsafe extern "system" fn (this: *const nsILoadContextInfoFactory, aDefault: *mut *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* readonly attribute nsILoadContextInfo private; */
    pub GetPrivate: unsafe extern "system" fn (this: *const nsILoadContextInfoFactory, aPrivate: *mut *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* readonly attribute nsILoadContextInfo anonymous; */
    pub GetAnonymous: unsafe extern "system" fn (this: *const nsILoadContextInfoFactory, aAnonymous: *mut *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* [implicit_jscontext] nsILoadContextInfo custom (in boolean aAnonymous, in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Custom: *const ::libc::c_void,

    /* nsILoadContextInfo fromLoadContext (in nsILoadContext aLoadContext, in boolean aAnonymous); */
    pub FromLoadContext: unsafe extern "system" fn (this: *const nsILoadContextInfoFactory, aLoadContext: *const nsILoadContext, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> ::nserror::nsresult,

    /* nsILoadContextInfo fromWindow (in nsIDOMWindow aWindow, in boolean aAnonymous); */
    pub FromWindow: unsafe extern "system" fn (this: *const nsILoadContextInfoFactory, aWindow: *const nsIDOMWindow, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoadContextInfoFactory {


    /// `readonly attribute nsILoadContextInfo default;`
    #[inline]
    pub unsafe fn GetDefault(&self, aDefault: *mut *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetDefault)(self, aDefault)
    }



    /// `readonly attribute nsILoadContextInfo private;`
    #[inline]
    pub unsafe fn GetPrivate(&self, aPrivate: *mut *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetPrivate)(self, aPrivate)
    }



    /// `readonly attribute nsILoadContextInfo anonymous;`
    #[inline]
    pub unsafe fn GetAnonymous(&self, aAnonymous: *mut *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetAnonymous)(self, aAnonymous)
    }



    /// `[implicit_jscontext] nsILoadContextInfo custom (in boolean aAnonymous, in jsval aOriginAttributes);`
    const _Custom: () = ();


    /// `nsILoadContextInfo fromLoadContext (in nsILoadContext aLoadContext, in boolean aAnonymous);`
    #[inline]
    pub unsafe fn FromLoadContext(&self, aLoadContext: *const nsILoadContext, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).FromLoadContext)(self, aLoadContext, aAnonymous, _retval)
    }



    /// `nsILoadContextInfo fromWindow (in nsIDOMWindow aWindow, in boolean aAnonymous);`
    #[inline]
    pub unsafe fn FromWindow(&self, aWindow: *const nsIDOMWindow, aAnonymous: bool, _retval: *mut *const nsILoadContextInfo) -> ::nserror::nsresult {
        ((*self.vtable).FromWindow)(self, aWindow, aAnonymous, _retval)
    }


}



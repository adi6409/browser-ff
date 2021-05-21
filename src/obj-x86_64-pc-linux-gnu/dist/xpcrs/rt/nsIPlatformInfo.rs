//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIPlatformInfo.idl
//


/// `interface nsIPlatformInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPlatformInfo {
    vtable: *const nsIPlatformInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPlatformInfo.
unsafe impl XpCom for nsIPlatformInfo {
    const IID: nsIID = nsID(0xab6650cf, 0x0806, 0x4aea,
        [0xb8, 0xf2, 0x40, 0xfd, 0xae, 0x74, 0xf1, 0xcc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPlatformInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPlatformInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPlatformInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIPlatformInfo`.
    fn coerce_from(v: &nsIPlatformInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPlatformInfoCoerce for nsIPlatformInfo {
    #[inline]
    fn coerce_from(v: &nsIPlatformInfo) -> &Self {
        v
    }
}

impl nsIPlatformInfo {
    /// Cast this `nsIPlatformInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPlatformInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPlatformInfo {
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
impl<T: nsISupportsCoerce> nsIPlatformInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPlatformInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPlatformInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPlatformInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString platformVersion; */
    pub GetPlatformVersion: unsafe extern "system" fn (this: *const nsIPlatformInfo, aPlatformVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString platformBuildID; */
    pub GetPlatformBuildID: unsafe extern "system" fn (this: *const nsIPlatformInfo, aPlatformBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPlatformInfo {

    /// ```text
    /// /**
    ///    * The version of the XULRunner platform.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString platformVersion;`
    #[inline]
    pub unsafe fn GetPlatformVersion(&self, aPlatformVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatformVersion)(self, aPlatformVersion)
    }


    /// ```text
    /// /**
    ///    * The build ID/date of gecko and the XULRunner platform.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString platformBuildID;`
    #[inline]
    pub unsafe fn GetPlatformBuildID(&self, aPlatformBuildID: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatformBuildID)(self, aPlatformBuildID)
    }


}



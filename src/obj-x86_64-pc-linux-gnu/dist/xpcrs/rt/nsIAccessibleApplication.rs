//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleApplication.idl
//


/// `interface nsIAccessibleApplication : nsISupports`
///

/// ```text
/// /**
///  * This interface is implemented by top level accessible object in hierarchy and
///  * provides information about application.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleApplication {
    vtable: *const nsIAccessibleApplicationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleApplication.
unsafe impl XpCom for nsIAccessibleApplication {
    const IID: nsIID = nsID(0x79251626, 0x387c, 0x4531,
        [0x89, 0xf3, 0x68, 0x0d, 0x31, 0xd6, 0xcf, 0x05]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleApplication {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleApplication.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleApplicationCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleApplication`.
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleApplicationCoerce for nsIAccessibleApplication {
    #[inline]
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self {
        v
    }
}

impl nsIAccessibleApplication {
    /// Cast this `nsIAccessibleApplication` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleApplicationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleApplication {
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
impl<T: nsISupportsCoerce> nsIAccessibleApplicationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleApplication) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleApplication
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleApplicationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString appName; */
    pub GetAppName: unsafe extern "system" fn (this: *const nsIAccessibleApplication, aAppName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString appVersion; */
    pub GetAppVersion: unsafe extern "system" fn (this: *const nsIAccessibleApplication, aAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString platformName; */
    pub GetPlatformName: unsafe extern "system" fn (this: *const nsIAccessibleApplication, aPlatformName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString platformVersion; */
    pub GetPlatformVersion: unsafe extern "system" fn (this: *const nsIAccessibleApplication, aPlatformVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleApplication {

    /// ```text
    /// /**
    ///    * Returns the application name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString appName;`
    #[inline]
    pub unsafe fn GetAppName(&self, aAppName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppName)(self, aAppName)
    }


    /// ```text
    /// /**
    ///    * Returns the application version.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString appVersion;`
    #[inline]
    pub unsafe fn GetAppVersion(&self, aAppVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppVersion)(self, aAppVersion)
    }


    /// ```text
    /// /**
    ///    * Returns the platform name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString platformName;`
    #[inline]
    pub unsafe fn GetPlatformName(&self, aPlatformName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatformName)(self, aPlatformName)
    }


    /// ```text
    /// /**
    ///    * Returns the platform version.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString platformVersion;`
    #[inline]
    pub unsafe fn GetPlatformVersion(&self, aPlatformVersion: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPlatformVersion)(self, aPlatformVersion)
    }


}



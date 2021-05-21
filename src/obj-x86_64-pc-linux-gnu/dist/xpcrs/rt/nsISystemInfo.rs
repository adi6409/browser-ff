//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsISystemInfo.idl
//


/// `interface nsISystemInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISystemInfo {
    vtable: *const nsISystemInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISystemInfo.
unsafe impl XpCom for nsISystemInfo {
    const IID: nsIID = nsID(0x09a0502b, 0xcedc, 0x4cae,
        [0xbf, 0x7c, 0x35, 0x66, 0x2d, 0xbd, 0x12, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISystemInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISystemInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISystemInfoCoerce {
    /// Cheaply cast a value of this type from a `nsISystemInfo`.
    fn coerce_from(v: &nsISystemInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISystemInfoCoerce for nsISystemInfo {
    #[inline]
    fn coerce_from(v: &nsISystemInfo) -> &Self {
        v
    }
}

impl nsISystemInfo {
    /// Cast this `nsISystemInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISystemInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISystemInfo {
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
impl<T: nsISupportsCoerce> nsISystemInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISystemInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISystemInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISystemInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] readonly attribute Promise diskInfo; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetDiskInfo: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise countryCode; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetCountryCode: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise osInfo; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetOsInfo: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute Promise processInfo; */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetProcessInfo: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISystemInfo {

    /// ```text
    /// /**
    ///    * Asynchronously get info about what types of disks we're using for the
    ///    * profile and binary.
    ///    * Note: only implemented on Windows, will return null elsewhere.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise diskInfo;`
    const _GetDiskInfo: () = ();

    /// ```text
    /// /**
    ///    * Asynchronously get CountryCode info.
    ///    * Note: only implemented on macOS and Windows, will return null elsewhere.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise countryCode;`
    const _GetCountryCode: () = ();

    /// ```text
    /// /**
    ///    * Asynchronously gets OS info on the system's install year.
    ///    * Note: only implemented on Windows, will return null elsewhere.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise osInfo;`
    const _GetOsInfo: () = ();

    /// ```text
    /// /**
    ///    * Asynchronously gets process info that indicates if the process is running
    ///    * under Wow64 and WowARM64.
    ///    * Note: only implemented on Windows, will return null elsewhere.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute Promise processInfo;`
    const _GetProcessInfo: () = ();

}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/crashes/nsICrashService.idl
//


/// `interface nsICrashService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICrashService {
    vtable: *const nsICrashServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICrashService.
unsafe impl XpCom for nsICrashService {
    const IID: nsIID = nsID(0x70bd93ff, 0x88fa, 0x4600,
        [0x8a, 0xf8, 0x57, 0xc8, 0xd0, 0x02, 0xdb, 0xac]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICrashService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICrashService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICrashServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICrashService`.
    fn coerce_from(v: &nsICrashService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICrashServiceCoerce for nsICrashService {
    #[inline]
    fn coerce_from(v: &nsICrashService) -> &Self {
        v
    }
}

impl nsICrashService {
    /// Cast this `nsICrashService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICrashServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICrashService {
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
impl<T: nsISupportsCoerce> nsICrashServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICrashService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICrashService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICrashServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise addCrash (in long processType, in long crashType, in AString id); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AddCrash: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICrashService {

    pub const PROCESS_TYPE_MAIN: i64 = 0;


    pub const PROCESS_TYPE_PLUGIN: i64 = 1;


    pub const PROCESS_TYPE_CONTENT: i64 = 2;


    pub const PROCESS_TYPE_IPDLUNITTEST: i64 = 3;


    pub const PROCESS_TYPE_GMPLUGIN: i64 = 4;


    pub const PROCESS_TYPE_GPU: i64 = 5;


    pub const PROCESS_TYPE_VR: i64 = 6;


    pub const PROCESS_TYPE_RDD: i64 = 7;


    pub const PROCESS_TYPE_SOCKET: i64 = 8;


    pub const PROCESS_TYPE_SANDBOX_BROKER: i64 = 9;


    pub const PROCESS_TYPE_FORKSERVER: i64 = 10;


    pub const CRASH_TYPE_CRASH: i64 = 0;


    pub const CRASH_TYPE_HANG: i64 = 1;

    /// ```text
    /// /**
    ///    * Records the occurrence of a crash.
    ///    *
    ///    * @param processType
    ///    *        One of the PROCESS_TYPE constants defined below.
    ///    * @param crashType
    ///    *        One of the CRASH_TYPE constants defined below.
    ///    * @param id
    ///    *        Crash ID. Likely a UUID.
    ///    *
    ///    * @return A promise that resolves after the crash has been stored
    ///    */
    /// ```
    ///

    /// `Promise addCrash (in long processType, in long crashType, in AString id);`
    const _AddCrash: () = ();

}



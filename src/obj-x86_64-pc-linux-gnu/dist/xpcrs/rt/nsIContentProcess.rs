//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIContentProcess.idl
//


/// `interface nsIContentProcessInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentProcessInfo {
    vtable: *const nsIContentProcessInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentProcessInfo.
unsafe impl XpCom for nsIContentProcessInfo {
    const IID: nsIID = nsID(0x456f58be, 0x29dd, 0x4973,
        [0x88, 0x5b, 0x95, 0xae, 0xce, 0x1c, 0x9a, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentProcessInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentProcessInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentProcessInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIContentProcessInfo`.
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentProcessInfoCoerce for nsIContentProcessInfo {
    #[inline]
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self {
        v
    }
}

impl nsIContentProcessInfo {
    /// Cast this `nsIContentProcessInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentProcessInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentProcessInfo {
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
impl<T: nsISupportsCoerce> nsIContentProcessInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentProcessInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentProcessInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentProcessInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isAlive; */
    pub GetIsAlive: unsafe extern "system" fn (this: *const nsIContentProcessInfo, aIsAlive: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute int32_t processId; */
    pub GetProcessId: unsafe extern "system" fn (this: *const nsIContentProcessInfo, aProcessId: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t tabCount; */
    pub GetTabCount: unsafe extern "system" fn (this: *const nsIContentProcessInfo, aTabCount: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute nsISupports messageManager; */
    pub GetMessageManager: unsafe extern "system" fn (this: *const nsIContentProcessInfo, aMessageManager: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentProcessInfo {

    /// ```text
    /// /**
    ///    * Is this content process alive?
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isAlive;`
    #[inline]
    pub unsafe fn GetIsAlive(&self, aIsAlive: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAlive)(self, aIsAlive)
    }


    /// ```text
    /// /**
    ///    * The content process's PID.
    ///    * Throws if the process is not alive.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t processId;`
    #[inline]
    pub unsafe fn GetProcessId(&self, aProcessId: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessId)(self, aProcessId)
    }


    /// ```text
    /// /**
    ///    * Number of opened tabs living in this content process.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t tabCount;`
    #[inline]
    pub unsafe fn GetTabCount(&self, aTabCount: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTabCount)(self, aTabCount)
    }


    /// ```text
    /// /**
    ///    * The process manager for this ContentParent (so a process message manager
        ///    * as opposed to a frame message manager.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsISupports messageManager;`
        #[inline]
        pub unsafe fn GetMessageManager(&self, aMessageManager: *mut *const nsISupports) -> ::nserror::nsresult {
            ((*self.vtable).GetMessageManager)(self, aMessageManager)
        }


    }


    /// `interface nsIContentProcessProvider : nsISupports`
    ///


    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIContentProcessProvider {
        vtable: *const nsIContentProcessProviderVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIContentProcessProvider.
    unsafe impl XpCom for nsIContentProcessProvider {
        const IID: nsIID = nsID(0x83ffb063, 0x5f65, 0x4c45,
            [0xae, 0x07, 0x3f, 0x55, 0x3e, 0x08, 0x09, 0xbb]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIContentProcessProvider {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIContentProcessProvider.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIContentProcessProviderCoerce {
        /// Cheaply cast a value of this type from a `nsIContentProcessProvider`.
        fn coerce_from(v: &nsIContentProcessProvider) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIContentProcessProviderCoerce for nsIContentProcessProvider {
        #[inline]
        fn coerce_from(v: &nsIContentProcessProvider) -> &Self {
            v
        }
    }

    impl nsIContentProcessProvider {
        /// Cast this `nsIContentProcessProvider` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIContentProcessProviderCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIContentProcessProvider {
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
    impl<T: nsISupportsCoerce> nsIContentProcessProviderCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIContentProcessProvider) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIContentProcessProvider
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIContentProcessProviderVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* int32_t provideProcess (in AUTF8String aType, in Array<nsIContentProcessInfo> aAliveProcesses, in uint32_t aMaxCount); */
        pub ProvideProcess: unsafe extern "system" fn (this: *const nsIContentProcessProvider, aType: *const ::nsstring::nsACString, aAliveProcesses: *const thin_vec::ThinVec<RefPtr<nsIContentProcessInfo>>, aMaxCount: uint32_t, _retval: *mut int32_t) -> ::nserror::nsresult,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIContentProcessProvider {
        /// ```text
        /// /**
        ///    * Return this from provideProcess to create a new process.
        ///    */
        /// ```
        ///

        pub const NEW_PROCESS: i64 = -1;

        /// ```text
        /// /**
        ///    * Given aAliveProcesses, choose which process of aType to use. Return
        ///    * nsIContentProcessProvider.NEW_PROCESS to ask the caller to create a new
        ///    * content process.
        ///    */
        /// ```
        ///

        /// `int32_t provideProcess (in AUTF8String aType, in Array<nsIContentProcessInfo> aAliveProcesses, in uint32_t aMaxCount);`
        #[inline]
        pub unsafe fn ProvideProcess(&self, aType: *const ::nsstring::nsACString, aAliveProcesses: *const thin_vec::ThinVec<RefPtr<nsIContentProcessInfo>>, aMaxCount: uint32_t, _retval: *mut int32_t) -> ::nserror::nsresult {
            ((*self.vtable).ProvideProcess)(self, aType, aAliveProcesses, aMaxCount, _retval)
        }


    }



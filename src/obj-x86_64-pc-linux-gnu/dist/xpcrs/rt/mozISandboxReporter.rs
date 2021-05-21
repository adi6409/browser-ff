//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/sandbox/linux/interfaces/mozISandboxReporter.idl
//


/// `interface mozISandboxReport : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISandboxReport {
    vtable: *const mozISandboxReportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISandboxReport.
unsafe impl XpCom for mozISandboxReport {
    const IID: nsIID = nsID(0xed1e84d3, 0x3346, 0x42e1,
        [0xb2, 0x8c, 0xe7, 0x6a, 0x77, 0xf5, 0x49, 0xf0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISandboxReport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISandboxReport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISandboxReportCoerce {
    /// Cheaply cast a value of this type from a `mozISandboxReport`.
    fn coerce_from(v: &mozISandboxReport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISandboxReportCoerce for mozISandboxReport {
    #[inline]
    fn coerce_from(v: &mozISandboxReport) -> &Self {
        v
    }
}

impl mozISandboxReport {
    /// Cast this `mozISandboxReport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISandboxReportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISandboxReport {
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
impl<T: nsISupportsCoerce> mozISandboxReportCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISandboxReport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISandboxReport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISandboxReportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint64_t msecAgo; */
    pub GetMsecAgo: unsafe extern "system" fn (this: *const mozISandboxReport, aMsecAgo: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t pid; */
    pub GetPid: unsafe extern "system" fn (this: *const mozISandboxReport, aPid: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t tid; */
    pub GetTid: unsafe extern "system" fn (this: *const mozISandboxReport, aTid: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute ACString procType; */
    pub GetProcType: unsafe extern "system" fn (this: *const mozISandboxReport, aProcType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t syscall; */
    pub GetSyscall: unsafe extern "system" fn (this: *const mozISandboxReport, aSyscall: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t numArgs; */
    pub GetNumArgs: unsafe extern "system" fn (this: *const mozISandboxReport, aNumArgs: *mut uint32_t) -> ::nserror::nsresult,

    /* ACString getArg (in uint32_t aIndex); */
    pub GetArg: unsafe extern "system" fn (this: *const mozISandboxReport, aIndex: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISandboxReport {


    /// `readonly attribute uint64_t msecAgo;`
    #[inline]
    pub unsafe fn GetMsecAgo(&self, aMsecAgo: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetMsecAgo)(self, aMsecAgo)
    }



    /// `readonly attribute int32_t pid;`
    #[inline]
    pub unsafe fn GetPid(&self, aPid: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPid)(self, aPid)
    }



    /// `readonly attribute int32_t tid;`
    #[inline]
    pub unsafe fn GetTid(&self, aTid: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTid)(self, aTid)
    }



    /// `readonly attribute ACString procType;`
    #[inline]
    pub unsafe fn GetProcType(&self, aProcType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProcType)(self, aProcType)
    }



    /// `readonly attribute uint32_t syscall;`
    #[inline]
    pub unsafe fn GetSyscall(&self, aSyscall: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSyscall)(self, aSyscall)
    }



    /// `readonly attribute uint32_t numArgs;`
    #[inline]
    pub unsafe fn GetNumArgs(&self, aNumArgs: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetNumArgs)(self, aNumArgs)
    }



    /// `ACString getArg (in uint32_t aIndex);`
    #[inline]
    pub unsafe fn GetArg(&self, aIndex: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetArg)(self, aIndex, _retval)
    }


}


/// `interface mozISandboxReportArray : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISandboxReportArray {
    vtable: *const mozISandboxReportArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISandboxReportArray.
unsafe impl XpCom for mozISandboxReportArray {
    const IID: nsIID = nsID(0x6e8ff6e5, 0x05c9, 0x42d3,
        [0x85, 0x3d, 0x40, 0x52, 0x3f, 0xd8, 0x6a, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISandboxReportArray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISandboxReportArray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISandboxReportArrayCoerce {
    /// Cheaply cast a value of this type from a `mozISandboxReportArray`.
    fn coerce_from(v: &mozISandboxReportArray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISandboxReportArrayCoerce for mozISandboxReportArray {
    #[inline]
    fn coerce_from(v: &mozISandboxReportArray) -> &Self {
        v
    }
}

impl mozISandboxReportArray {
    /// Cast this `mozISandboxReportArray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISandboxReportArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISandboxReportArray {
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
impl<T: nsISupportsCoerce> mozISandboxReportArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISandboxReportArray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISandboxReportArray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISandboxReportArrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint64_t begin; */
    pub GetBegin: unsafe extern "system" fn (this: *const mozISandboxReportArray, aBegin: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t end; */
    pub GetEnd: unsafe extern "system" fn (this: *const mozISandboxReportArray, aEnd: *mut uint64_t) -> ::nserror::nsresult,

    /* mozISandboxReport getElement (in uint64_t aIndex); */
    pub GetElement: unsafe extern "system" fn (this: *const mozISandboxReportArray, aIndex: uint64_t, _retval: *mut *const mozISandboxReport) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISandboxReportArray {


    /// `readonly attribute uint64_t begin;`
    #[inline]
    pub unsafe fn GetBegin(&self, aBegin: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBegin)(self, aBegin)
    }



    /// `readonly attribute uint64_t end;`
    #[inline]
    pub unsafe fn GetEnd(&self, aEnd: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetEnd)(self, aEnd)
    }



    /// `mozISandboxReport getElement (in uint64_t aIndex);`
    #[inline]
    pub unsafe fn GetElement(&self, aIndex: uint64_t, _retval: *mut *const mozISandboxReport) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, aIndex, _retval)
    }


}


/// `interface mozISandboxReporter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISandboxReporter {
    vtable: *const mozISandboxReporterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISandboxReporter.
unsafe impl XpCom for mozISandboxReporter {
    const IID: nsIID = nsID(0x8535bdf7, 0x6d9e, 0x4853,
        [0xac, 0xf9, 0xa1, 0x46, 0x44, 0x9c, 0x4a, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISandboxReporter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISandboxReporter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISandboxReporterCoerce {
    /// Cheaply cast a value of this type from a `mozISandboxReporter`.
    fn coerce_from(v: &mozISandboxReporter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISandboxReporterCoerce for mozISandboxReporter {
    #[inline]
    fn coerce_from(v: &mozISandboxReporter) -> &Self {
        v
    }
}

impl mozISandboxReporter {
    /// Cast this `mozISandboxReporter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISandboxReporterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISandboxReporter {
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
impl<T: nsISupportsCoerce> mozISandboxReporterCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISandboxReporter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISandboxReporter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISandboxReporterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* mozISandboxReportArray snapshot (); */
    pub Snapshot: unsafe extern "system" fn (this: *const mozISandboxReporter, _retval: *mut *const mozISandboxReportArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISandboxReporter {


    /// `mozISandboxReportArray snapshot ();`
    #[inline]
    pub unsafe fn Snapshot(&self, _retval: *mut *const mozISandboxReportArray) -> ::nserror::nsresult {
        ((*self.vtable).Snapshot)(self, _retval)
    }


}



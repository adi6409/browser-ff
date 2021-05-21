//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/processtools/nsIProcessToolsService.idl
//


/// `interface nsIProcessToolsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProcessToolsService {
    vtable: *const nsIProcessToolsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProcessToolsService.
unsafe impl XpCom for nsIProcessToolsService {
    const IID: nsIID = nsID(0x1341f571, 0xebed, 0x4305,
        [0xb2, 0x64, 0x4d, 0x8f, 0xc3, 0xb6, 0xb1, 0x1c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProcessToolsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProcessToolsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProcessToolsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIProcessToolsService`.
    fn coerce_from(v: &nsIProcessToolsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProcessToolsServiceCoerce for nsIProcessToolsService {
    #[inline]
    fn coerce_from(v: &nsIProcessToolsService) -> &Self {
        v
    }
}

impl nsIProcessToolsService {
    /// Cast this `nsIProcessToolsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProcessToolsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProcessToolsService {
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
impl<T: nsISupportsCoerce> nsIProcessToolsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProcessToolsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProcessToolsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProcessToolsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void kill (in unsigned long long pid); */
    pub Kill: unsafe extern "system" fn (this: *const nsIProcessToolsService, pid: u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long pid; */
    pub GetPid: unsafe extern "system" fn (this: *const nsIProcessToolsService, aPid: *mut u64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProcessToolsService {

    /// ```text
    /// /**
    ///      * Kill a process running on this system.
    ///      *
    ///      * Does not cause a crash report to be generated and sent.
    ///      *
    ///      * # Note
    ///      *
    ///      * `pid` is the unique-to-the-system process identifier, as
    ///      * obtained with attribute `pid` of this service.
    ///      *
    ///      * Under Un*ix, that's what you obtain with `getpid()`, etc.
    ///      * Under Windows, that's what you obtain with `GetCurrentProcessId()`,
    ///      * NOT the same thing as the process `HANDLE`.
    ///      *
    ///      * # Failure
    ///      *
    ///      * Under Windows, if two processes race to `kill()` a third process,
    ///      * or two threads race to `kill()` a process there is a (small) window
    ///      * during which this can cause a crash in the losing process.
    ///      *
    ///      * # Caveats
    ///      *
    ///      * Under Windows, process killing is asynchronous. Therefore, this
    ///      * function can return before process `pid` is actually dead.
    ///      */
    /// ```
    ///

    /// `void kill (in unsigned long long pid);`
    #[inline]
    pub unsafe fn Kill(&self, pid: u64) -> ::nserror::nsresult {
        ((*self.vtable).Kill)(self, pid)
    }


    /// ```text
    /// /**
    ///      * The pid for the current process.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long long pid;`
    #[inline]
    pub unsafe fn GetPid(&self, aPid: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetPid)(self, aPid)
    }


}



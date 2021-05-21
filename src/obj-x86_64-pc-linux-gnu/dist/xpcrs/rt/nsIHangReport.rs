//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIHangReport.idl
//


/// `interface nsIHangReport : nsISupports`
///

/// ```text
/// /**
///  * When a content process hangs, Gecko notifies "process-hang-report" observers
///  * and passes an nsIHangReport for the subject parameter. There is at most one
///  * nsIHangReport associated with a given content process. As long as the content
///  * process stays stuck, the "process-hang-report" observer will continue to be
///  * notified at regular intervals (approximately once per second). The content
///  * process will continue to run uninhibitedly during this time.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHangReport {
    vtable: *const nsIHangReportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHangReport.
unsafe impl XpCom for nsIHangReport {
    const IID: nsIID = nsID(0x5fcffbb9, 0xbe62, 0x49b1,
        [0xb8, 0xa1, 0x36, 0xe8, 0x20, 0x78, 0x7a, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHangReport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHangReport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHangReportCoerce {
    /// Cheaply cast a value of this type from a `nsIHangReport`.
    fn coerce_from(v: &nsIHangReport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHangReportCoerce for nsIHangReport {
    #[inline]
    fn coerce_from(v: &nsIHangReport) -> &Self {
        v
    }
}

impl nsIHangReport {
    /// Cast this `nsIHangReport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHangReportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHangReport {
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
impl<T: nsISupportsCoerce> nsIHangReportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHangReport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHangReport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHangReportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long hangType; */
    pub GetHangType: unsafe extern "system" fn (this: *const nsIHangReport, aHangType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute Element scriptBrowser; */
    pub GetScriptBrowser: unsafe extern "system" fn (this: *const nsIHangReport, aScriptBrowser: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute ACString scriptFileName; */
    pub GetScriptFileName: unsafe extern "system" fn (this: *const nsIHangReport, aScriptFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute double hangDuration; */
    pub GetHangDuration: unsafe extern "system" fn (this: *const nsIHangReport, aHangDuration: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute AString addonId; */
    pub GetAddonId: unsafe extern "system" fn (this: *const nsIHangReport, aAddonId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute ACString pluginName; */
    pub GetPluginName: unsafe extern "system" fn (this: *const nsIHangReport, aPluginName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long childID; */
    pub GetChildID: unsafe extern "system" fn (this: *const nsIHangReport, aChildID: *mut u64) -> ::nserror::nsresult,

    /* void userCanceled (); */
    pub UserCanceled: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* void terminateScript (); */
    pub TerminateScript: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* void terminateGlobal (); */
    pub TerminateGlobal: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* void terminatePlugin (); */
    pub TerminatePlugin: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* void beginStartingDebugger (); */
    pub BeginStartingDebugger: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* void endStartingDebugger (); */
    pub EndStartingDebugger: unsafe extern "system" fn (this: *const nsIHangReport) -> ::nserror::nsresult,

    /* bool isReportForBrowser (in FrameLoader aFrameLoader); */
    pub IsReportForBrowser: unsafe extern "system" fn (this: *const nsIHangReport, aFrameLoader: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHangReport {

    pub const SLOW_SCRIPT: i64 = 1;


    pub const PLUGIN_HANG: i64 = 2;


    /// `readonly attribute unsigned long hangType;`
    #[inline]
    pub unsafe fn GetHangType(&self, aHangType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetHangType)(self, aHangType)
    }



    /// `readonly attribute Element scriptBrowser;`
    #[inline]
    pub unsafe fn GetScriptBrowser(&self, aScriptBrowser: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptBrowser)(self, aScriptBrowser)
    }



    /// `readonly attribute ACString scriptFileName;`
    #[inline]
    pub unsafe fn GetScriptFileName(&self, aScriptFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetScriptFileName)(self, aScriptFileName)
    }



    /// `readonly attribute double hangDuration;`
    #[inline]
    pub unsafe fn GetHangDuration(&self, aHangDuration: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetHangDuration)(self, aHangDuration)
    }



    /// `readonly attribute AString addonId;`
    #[inline]
    pub unsafe fn GetAddonId(&self, aAddonId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAddonId)(self, aAddonId)
    }



    /// `readonly attribute ACString pluginName;`
    #[inline]
    pub unsafe fn GetPluginName(&self, aPluginName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPluginName)(self, aPluginName)
    }



    /// `readonly attribute unsigned long long childID;`
    #[inline]
    pub unsafe fn GetChildID(&self, aChildID: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetChildID)(self, aChildID)
    }



    /// `void userCanceled ();`
    #[inline]
    pub unsafe fn UserCanceled(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UserCanceled)(self, )
    }



    /// `void terminateScript ();`
    #[inline]
    pub unsafe fn TerminateScript(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TerminateScript)(self, )
    }



    /// `void terminateGlobal ();`
    #[inline]
    pub unsafe fn TerminateGlobal(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TerminateGlobal)(self, )
    }



    /// `void terminatePlugin ();`
    #[inline]
    pub unsafe fn TerminatePlugin(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).TerminatePlugin)(self, )
    }



    /// `void beginStartingDebugger ();`
    #[inline]
    pub unsafe fn BeginStartingDebugger(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginStartingDebugger)(self, )
    }



    /// `void endStartingDebugger ();`
    #[inline]
    pub unsafe fn EndStartingDebugger(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndStartingDebugger)(self, )
    }



    /// `bool isReportForBrowser (in FrameLoader aFrameLoader);`
    #[inline]
    pub unsafe fn IsReportForBrowser(&self, aFrameLoader: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsReportForBrowser)(self, aFrameLoader, _retval)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/tools/profiler/gecko/nsIProfiler.idl
//


/// `interface nsIProfilerStartParams : nsISupports`
///

/// ```text
/// /**
///  * Start-up parameters for subprocesses are passed through nsIObserverService,
///  * which, unfortunately, means we need to implement nsISupports in order to
///  * go through it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfilerStartParams {
    vtable: *const nsIProfilerStartParamsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfilerStartParams.
unsafe impl XpCom for nsIProfilerStartParams {
    const IID: nsIID = nsID(0x0a175ba7, 0x8fcf, 0x4ce9,
        [0x9c, 0x4b, 0xcc, 0xc6, 0x27, 0x2f, 0x44, 0x25]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfilerStartParams {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfilerStartParams.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfilerStartParamsCoerce {
    /// Cheaply cast a value of this type from a `nsIProfilerStartParams`.
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfilerStartParamsCoerce for nsIProfilerStartParams {
    #[inline]
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self {
        v
    }
}

impl nsIProfilerStartParams {
    /// Cast this `nsIProfilerStartParams` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfilerStartParamsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfilerStartParams {
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
impl<T: nsISupportsCoerce> nsIProfilerStartParamsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfilerStartParams) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfilerStartParams
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfilerStartParamsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t entries; */
    pub GetEntries: unsafe extern "system" fn (this: *const nsIProfilerStartParams, aEntries: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute double duration; */
    pub GetDuration: unsafe extern "system" fn (this: *const nsIProfilerStartParams, aDuration: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double interval; */
    pub GetInterval: unsafe extern "system" fn (this: *const nsIProfilerStartParams, aInterval: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute uint32_t features; */
    pub GetFeatures: unsafe extern "system" fn (this: *const nsIProfilerStartParams, aFeatures: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t activeBrowsingContextID; */
    pub GetActiveBrowsingContextID: unsafe extern "system" fn (this: *const nsIProfilerStartParams, aActiveBrowsingContextID: *mut uint64_t) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] StringArrayRef getFilters (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetFilters: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfilerStartParams {


    /// `readonly attribute uint32_t entries;`
    #[inline]
    pub unsafe fn GetEntries(&self, aEntries: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetEntries)(self, aEntries)
    }



    /// `readonly attribute double duration;`
    #[inline]
    pub unsafe fn GetDuration(&self, aDuration: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDuration)(self, aDuration)
    }



    /// `readonly attribute double interval;`
    #[inline]
    pub unsafe fn GetInterval(&self, aInterval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetInterval)(self, aInterval)
    }



    /// `readonly attribute uint32_t features;`
    #[inline]
    pub unsafe fn GetFeatures(&self, aFeatures: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatures)(self, aFeatures)
    }



    /// `readonly attribute uint64_t activeBrowsingContextID;`
    #[inline]
    pub unsafe fn GetActiveBrowsingContextID(&self, aActiveBrowsingContextID: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveBrowsingContextID)(self, aActiveBrowsingContextID)
    }



    /// `[noscript,nostdcall,notxpcom] StringArrayRef getFilters ();`
    const _GetFilters: () = ();

}


/// `interface nsIProfiler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProfiler {
    vtable: *const nsIProfilerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProfiler.
unsafe impl XpCom for nsIProfiler {
    const IID: nsIID = nsID(0xead3f75c, 0x0e0e, 0x4fbb,
        [0x90, 0x1c, 0x1e, 0x53, 0x92, 0xef, 0x5b, 0x2a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProfiler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProfiler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProfilerCoerce {
    /// Cheaply cast a value of this type from a `nsIProfiler`.
    fn coerce_from(v: &nsIProfiler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProfilerCoerce for nsIProfiler {
    #[inline]
    fn coerce_from(v: &nsIProfiler) -> &Self {
        v
    }
}

impl nsIProfiler {
    /// Cast this `nsIProfiler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProfilerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProfiler {
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
impl<T: nsISupportsCoerce> nsIProfilerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProfiler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProfiler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProfilerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean CanProfile (); */
    pub CanProfile: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut bool) -> ::nserror::nsresult,

    /* void StartProfiler (in uint32_t aEntries, in double aInterval, in Array<AUTF8String> aFeatures, [optional] in Array<AUTF8String> aFilters, [optional] in uint64_t aActiveBrowsingContextID, [optional] in double aDuration); */
    pub StartProfiler: unsafe extern "system" fn (this: *const nsIProfiler, aEntries: uint32_t, aInterval: libc::c_double, aFeatures: *const thin_vec::ThinVec<::nsstring::nsCString>, aFilters: *const thin_vec::ThinVec<::nsstring::nsCString>, aActiveBrowsingContextID: uint64_t, aDuration: libc::c_double) -> ::nserror::nsresult,

    /* void StopProfiler (); */
    pub StopProfiler: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* boolean IsPaused (); */
    pub IsPaused: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut bool) -> ::nserror::nsresult,

    /* void Pause (); */
    pub Pause: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* void Resume (); */
    pub Resume: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* boolean IsSamplingPaused (); */
    pub IsSamplingPaused: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut bool) -> ::nserror::nsresult,

    /* void PauseSampling (); */
    pub PauseSampling: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* void ResumeSampling (); */
    pub ResumeSampling: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise waitOnePeriodicSampling (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub WaitOnePeriodicSampling: *const ::libc::c_void,

    /* string GetProfile ([optional] in double aSinceTime); */
    pub GetProfile: unsafe extern "system" fn (this: *const nsIProfiler, aSinceTime: libc::c_double, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval getProfileData ([optional] in double aSinceTime); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetProfileData: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getProfileDataAsync ([optional] in double aSinceTime); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetProfileDataAsync: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getProfileDataAsArrayBuffer ([optional] in double aSinceTime); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetProfileDataAsArrayBuffer: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getProfileDataAsGzippedArrayBuffer ([optional] in double aSinceTime); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetProfileDataAsGzippedArrayBuffer: *const ::libc::c_void,

    /* [implicit_jscontext] Promise dumpProfileToFileAsync (in ACString aFilename, [optional] in double aSinceTime); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub DumpProfileToFileAsync: *const ::libc::c_void,

    /* boolean IsActive (); */
    pub IsActive: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut bool) -> ::nserror::nsresult,

    /* void ClearAllPages (); */
    pub ClearAllPages: unsafe extern "system" fn (this: *const nsIProfiler) -> ::nserror::nsresult,

    /* Array<AUTF8String> GetFeatures (); */
    pub GetFeatures: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval activeConfiguration; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetActiveConfiguration: *const ::libc::c_void,

    /* Array<AUTF8String> GetAllFeatures (); */
    pub GetAllFeatures: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void GetBufferInfo (out uint32_t aCurrentPosition, out uint32_t aTotalSize, out uint32_t aGeneration); */
    pub GetBufferInfo: unsafe extern "system" fn (this: *const nsIProfiler, aCurrentPosition: *mut uint32_t, aTotalSize: *mut uint32_t, aGeneration: *mut uint32_t) -> ::nserror::nsresult,

    /* double getElapsedTime (); */
    pub GetElapsedTime: unsafe extern "system" fn (this: *const nsIProfiler, _retval: *mut libc::c_double) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval sharedLibraries; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetSharedLibraries: *const ::libc::c_void,

    /* [implicit_jscontext] Promise getSymbolTable (in ACString aDebugPath, in ACString aBreakpadID); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSymbolTable: *const ::libc::c_void,

    /* void dumpProfileToFile (in string aFilename); */
    pub DumpProfileToFile: unsafe extern "system" fn (this: *const nsIProfiler, aFilename: *const libc::c_char) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void receiveShutdownProfile (in nsCString aProfile); */
    /// Unable to generate binding because `native type const nsCString unsupported`
    pub ReceiveShutdownProfile: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProfiler {


    /// `boolean CanProfile ();`
    #[inline]
    pub unsafe fn CanProfile(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanProfile)(self, _retval)
    }



    /// `void StartProfiler (in uint32_t aEntries, in double aInterval, in Array<AUTF8String> aFeatures, [optional] in Array<AUTF8String> aFilters, [optional] in uint64_t aActiveBrowsingContextID, [optional] in double aDuration);`
    #[inline]
    pub unsafe fn StartProfiler(&self, aEntries: uint32_t, aInterval: libc::c_double, aFeatures: *const thin_vec::ThinVec<::nsstring::nsCString>, aFilters: *const thin_vec::ThinVec<::nsstring::nsCString>, aActiveBrowsingContextID: uint64_t, aDuration: libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).StartProfiler)(self, aEntries, aInterval, aFeatures, aFilters, aActiveBrowsingContextID, aDuration)
    }



    /// `void StopProfiler ();`
    #[inline]
    pub unsafe fn StopProfiler(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopProfiler)(self, )
    }



    /// `boolean IsPaused ();`
    #[inline]
    pub unsafe fn IsPaused(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPaused)(self, _retval)
    }



    /// `void Pause ();`
    #[inline]
    pub unsafe fn Pause(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Pause)(self, )
    }



    /// `void Resume ();`
    #[inline]
    pub unsafe fn Resume(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Resume)(self, )
    }



    /// `boolean IsSamplingPaused ();`
    #[inline]
    pub unsafe fn IsSamplingPaused(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSamplingPaused)(self, _retval)
    }



    /// `void PauseSampling ();`
    #[inline]
    pub unsafe fn PauseSampling(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PauseSampling)(self, )
    }



    /// `void ResumeSampling ();`
    #[inline]
    pub unsafe fn ResumeSampling(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumeSampling)(self, )
    }



    /// `[implicit_jscontext] Promise waitOnePeriodicSampling ();`
    const _WaitOnePeriodicSampling: () = ();


    /// `string GetProfile ([optional] in double aSinceTime);`
    #[inline]
    pub unsafe fn GetProfile(&self, aSinceTime: libc::c_double, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetProfile)(self, aSinceTime, _retval)
    }



    /// `[implicit_jscontext] jsval getProfileData ([optional] in double aSinceTime);`
    const _GetProfileData: () = ();


    /// `[implicit_jscontext] Promise getProfileDataAsync ([optional] in double aSinceTime);`
    const _GetProfileDataAsync: () = ();


    /// `[implicit_jscontext] Promise getProfileDataAsArrayBuffer ([optional] in double aSinceTime);`
    const _GetProfileDataAsArrayBuffer: () = ();


    /// `[implicit_jscontext] Promise getProfileDataAsGzippedArrayBuffer ([optional] in double aSinceTime);`
    const _GetProfileDataAsGzippedArrayBuffer: () = ();

    /// ```text
    /// /**
    ///    * Returns a promise that resolves once the file has been written.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise dumpProfileToFileAsync (in ACString aFilename, [optional] in double aSinceTime);`
    const _DumpProfileToFileAsync: () = ();


    /// `boolean IsActive ();`
    #[inline]
    pub unsafe fn IsActive(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsActive)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Clear all registered and unregistered page information in prifiler.
    ///    */
    /// ```
    ///

    /// `void ClearAllPages ();`
    #[inline]
    pub unsafe fn ClearAllPages(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearAllPages)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns an array of the features that are supported in this build.
    ///    * Features may vary depending on platform and build flags.
    ///    */
    /// ```
    ///

    /// `Array<AUTF8String> GetFeatures ();`
    #[inline]
    pub unsafe fn GetFeatures(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetFeatures)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns a JavaScript object that contains a description of the currently configured
    ///    * state of the profiler when the profiler is active. This can be useful to assert
    ///    * the UI of the profiler's recording panel in tests. It returns null when the profiler
    ///    * is not active.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval activeConfiguration;`
    const _GetActiveConfiguration: () = ();

    /// ```text
    /// /**
    ///    * Returns an array of all features that are supported by the profiler.
    ///    * The array may contain features that are not supported in this build.
    ///    */
    /// ```
    ///

    /// `Array<AUTF8String> GetAllFeatures ();`
    #[inline]
    pub unsafe fn GetAllFeatures(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAllFeatures)(self, _retval)
    }



    /// `void GetBufferInfo (out uint32_t aCurrentPosition, out uint32_t aTotalSize, out uint32_t aGeneration);`
    #[inline]
    pub unsafe fn GetBufferInfo(&self, aCurrentPosition: *mut uint32_t, aTotalSize: *mut uint32_t, aGeneration: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBufferInfo)(self, aCurrentPosition, aTotalSize, aGeneration)
    }


    /// ```text
    /// /**
    ///    * Returns the elapsed time, in milliseconds, since the profiler's epoch.
    ///    * The epoch is guaranteed to be constant for the duration of the
    ///    * process, but is otherwise arbitrary.
    ///    */
    /// ```
    ///

    /// `double getElapsedTime ();`
    #[inline]
    pub unsafe fn GetElapsedTime(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetElapsedTime)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Contains an array of shared library objects.
    ///    * Every object has the properties:
    ///    *  - start:      The start address of the memory region occupied by this library.
    ///    *  - end:        The end address of the memory region occupied by this library.
    ///    *  - offset:     Usually zero, except on Linux / Android if the first mapped
    ///    *                section of the library has been mapped to an address that's
    ///    *                different from the library's base address.
    ///    *                Then offset = start - baseAddress.
    ///    *  - name:       The name (file basename) of the binary.
    ///    *  - path:       The full absolute path to the binary.
    ///    *  - debugName:  On Windows, the name of the pdb file for the binary. On other
    ///    *                platforms, the same as |name|.
    ///    *  - debugPath:  On Windows, the full absolute path of the pdb file for the
    ///    *                binary. On other platforms, the same as |path|.
    ///    *  - arch:       On Mac, the name of the architecture that identifies the right
    ///    *                binary image of a fat binary. Example values are "i386", "x86_64",
    ///    *                and "x86_64h". (x86_64h is used for binaries that contain
        ///    *                instructions that are specific to the Intel Haswell microarchitecture.)
    ///    *                On non-Mac platforms, arch is "".
    ///    *  - breakpadId: A unique identifier string for this library, as used by breakpad.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval sharedLibraries;`
    const _GetSharedLibraries: () = ();

    /// ```text
    /// /**
    ///    * Returns a promise that resolves to a SymbolTableAsTuple for the binary at
    ///    * the given path.
    ///    *
    ///    * SymbolTable as tuple: [addrs, index, buffer]
    ///    * Contains a symbol table, which can be used to map addresses to strings.
    ///    *
    ///    * The first element of this tuple, commonly named "addrs", is a sorted array of
    ///    * symbol addresses, as library-relative offsets in bytes, in ascending order.
    ///    * The third element of this tuple, commonly named "buffer", is a buffer of
    ///    * bytes that contains all strings from this symbol table, in the order of the
    ///    * addresses they correspond to, in utf-8 encoded form, all concatenated
    ///    * together.
    ///    * The second element of this tuple, commonly named "index", contains positions
    ///    * into "buffer". For every address, that position is where the string for that
    ///    * address starts in the buffer.
    ///    * index.length == addrs.length + 1.
    ///    * index[addrs.length] is the end position of the last string in the buffer.
    ///    *
    ///    * The string for the address addrs[i] is
    ///    * (new TextDecoder()).decode(buffer.subarray(index[i], index[i + 1]))
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise getSymbolTable (in ACString aDebugPath, in ACString aBreakpadID);`
    const _GetSymbolTable: () = ();

    /// ```text
    /// /**
    ///    * Dump the collected profile to a file.
    ///    */
    /// ```
    ///

    /// `void dumpProfileToFile (in string aFilename);`
    #[inline]
    pub unsafe fn DumpProfileToFile(&self, aFilename: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).DumpProfileToFile)(self, aFilename)
    }



    /// `[noscript,nostdcall,notxpcom] void receiveShutdownProfile (in nsCString aProfile);`
    const _ReceiveShutdownProfile: () = ();

}



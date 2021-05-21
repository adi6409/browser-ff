//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIException.idl
//


/// `interface nsIStackFrame : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStackFrame {
    vtable: *const nsIStackFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStackFrame.
unsafe impl XpCom for nsIStackFrame {
    const IID: nsIID = nsID(0x28bfb2a2, 0x5ea6, 0x4738,
        [0x91, 0x8b, 0x04, 0x9d, 0xc4, 0xd5, 0x1f, 0x0b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStackFrame {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStackFrame.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStackFrameCoerce {
    /// Cheaply cast a value of this type from a `nsIStackFrame`.
    fn coerce_from(v: &nsIStackFrame) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStackFrameCoerce for nsIStackFrame {
    #[inline]
    fn coerce_from(v: &nsIStackFrame) -> &Self {
        v
    }
}

impl nsIStackFrame {
    /// Cast this `nsIStackFrame` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStackFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStackFrame {
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
impl<T: nsISupportsCoerce> nsIStackFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStackFrame) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStackFrame
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStackFrameVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [binaryname(FilenameXPCOM),implicit_jscontext] readonly attribute AString filename; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetFilenameXPCOM: *const ::libc::c_void,

    /* [binaryname(NameXPCOM),implicit_jscontext] readonly attribute AString name; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetNameXPCOM: *const ::libc::c_void,

    /* [binaryname(SourceIdXPCOM),implicit_jscontext] readonly attribute int32_t sourceId; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetSourceIdXPCOM: *const ::libc::c_void,

    /* [binaryname(LineNumberXPCOM),implicit_jscontext] readonly attribute int32_t lineNumber; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetLineNumberXPCOM: *const ::libc::c_void,

    /* [binaryname(ColumnNumberXPCOM),implicit_jscontext] readonly attribute int32_t columnNumber; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetColumnNumberXPCOM: *const ::libc::c_void,

    /* readonly attribute AUTF8String sourceLine; */
    pub GetSourceLine: unsafe extern "system" fn (this: *const nsIStackFrame, aSourceLine: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [binaryname(AsyncCauseXPCOM),implicit_jscontext] readonly attribute AString asyncCause; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetAsyncCauseXPCOM: *const ::libc::c_void,

    /* [binaryname(AsyncCallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame asyncCaller; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetAsyncCallerXPCOM: *const ::libc::c_void,

    /* [binaryname(CallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame caller; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetCallerXPCOM: *const ::libc::c_void,

    /* [binaryname(FormattedStackXPCOM),implicit_jscontext] readonly attribute AString formattedStack; */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetFormattedStackXPCOM: *const ::libc::c_void,

    /* readonly attribute jsval nativeSavedFrame; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetNativeSavedFrame: *const ::libc::c_void,

    /* [binaryname(ToStringXPCOM),implicit_jscontext] AUTF8String toString (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub ToStringXPCOM: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void getFilename (in JSContext aCx, out AString aFilename); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetFilename: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void getName (in JSContext aCx, out AString aName); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetName: *const ::libc::c_void,

    /* [nostdcall,notxpcom] int32_t getSourceId (in JSContext aCx); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetSourceId: *const ::libc::c_void,

    /* [nostdcall,notxpcom] int32_t getLineNumber (in JSContext aCx); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetLineNumber: *const ::libc::c_void,

    /* [nostdcall,notxpcom] int32_t getColumnNumber (in JSContext aCx); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetColumnNumber: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void getAsyncCause (in JSContext aCx, out AString aAsyncCause); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetAsyncCause: *const ::libc::c_void,

    /* [nostdcall,notxpcom] StackFrameRef getAsyncCaller (in JSContext aCx); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetAsyncCaller: *const ::libc::c_void,

    /* [nostdcall,notxpcom] StackFrameRef getCaller (in JSContext aCx); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetCaller: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void getFormattedStack (in JSContext aCx, out AString aFormattedStack); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub GetFormattedStack: *const ::libc::c_void,

    /* [binaryname(ToString),nostdcall,notxpcom] void toStringInfallible (in JSContext aCx, out AUTF8String aString); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub ToString: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStackFrame {


    /// `[binaryname(FilenameXPCOM),implicit_jscontext] readonly attribute AString filename;`
    const _GetFilenameXPCOM: () = ();


    /// `[binaryname(NameXPCOM),implicit_jscontext] readonly attribute AString name;`
    const _GetNameXPCOM: () = ();


    /// `[binaryname(SourceIdXPCOM),implicit_jscontext] readonly attribute int32_t sourceId;`
    const _GetSourceIdXPCOM: () = ();


    /// `[binaryname(LineNumberXPCOM),implicit_jscontext] readonly attribute int32_t lineNumber;`
    const _GetLineNumberXPCOM: () = ();


    /// `[binaryname(ColumnNumberXPCOM),implicit_jscontext] readonly attribute int32_t columnNumber;`
    const _GetColumnNumberXPCOM: () = ();


    /// `readonly attribute AUTF8String sourceLine;`
    #[inline]
    pub unsafe fn GetSourceLine(&self, aSourceLine: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceLine)(self, aSourceLine)
    }



    /// `[binaryname(AsyncCauseXPCOM),implicit_jscontext] readonly attribute AString asyncCause;`
    const _GetAsyncCauseXPCOM: () = ();


    /// `[binaryname(AsyncCallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame asyncCaller;`
    const _GetAsyncCallerXPCOM: () = ();


    /// `[binaryname(CallerXPCOM),implicit_jscontext] readonly attribute nsIStackFrame caller;`
    const _GetCallerXPCOM: () = ();


    /// `[binaryname(FormattedStackXPCOM),implicit_jscontext] readonly attribute AString formattedStack;`
    const _GetFormattedStackXPCOM: () = ();


    /// `readonly attribute jsval nativeSavedFrame;`
    const _GetNativeSavedFrame: () = ();


    /// `[binaryname(ToStringXPCOM),implicit_jscontext] AUTF8String toString ();`
    const _ToStringXPCOM: () = ();


    /// `[nostdcall,notxpcom] void getFilename (in JSContext aCx, out AString aFilename);`
    const _GetFilename: () = ();


    /// `[nostdcall,notxpcom] void getName (in JSContext aCx, out AString aName);`
    const _GetName: () = ();


    /// `[nostdcall,notxpcom] int32_t getSourceId (in JSContext aCx);`
    const _GetSourceId: () = ();


    /// `[nostdcall,notxpcom] int32_t getLineNumber (in JSContext aCx);`
    const _GetLineNumber: () = ();


    /// `[nostdcall,notxpcom] int32_t getColumnNumber (in JSContext aCx);`
    const _GetColumnNumber: () = ();


    /// `[nostdcall,notxpcom] void getAsyncCause (in JSContext aCx, out AString aAsyncCause);`
    const _GetAsyncCause: () = ();


    /// `[nostdcall,notxpcom] StackFrameRef getAsyncCaller (in JSContext aCx);`
    const _GetAsyncCaller: () = ();


    /// `[nostdcall,notxpcom] StackFrameRef getCaller (in JSContext aCx);`
    const _GetCaller: () = ();


    /// `[nostdcall,notxpcom] void getFormattedStack (in JSContext aCx, out AString aFormattedStack);`
    const _GetFormattedStack: () = ();


    /// `[binaryname(ToString),nostdcall,notxpcom] void toStringInfallible (in JSContext aCx, out AUTF8String aString);`
    const _ToString: () = ();

}


/// `interface nsIException : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIException {
    vtable: *const nsIExceptionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIException.
unsafe impl XpCom for nsIException {
    const IID: nsIID = nsID(0x4371b5bf, 0x6845, 0x487f,
        [0x8d, 0x9d, 0x3f, 0x1e, 0x4a, 0x9b, 0xad, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIException {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIException.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExceptionCoerce {
    /// Cheaply cast a value of this type from a `nsIException`.
    fn coerce_from(v: &nsIException) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExceptionCoerce for nsIException {
    #[inline]
    fn coerce_from(v: &nsIException) -> &Self {
        v
    }
}

impl nsIException {
    /// Cast this `nsIException` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExceptionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIException {
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
impl<T: nsISupportsCoerce> nsIExceptionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIException) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIException
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExceptionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIException {


}



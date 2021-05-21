//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/bindings/nsIScriptError.idl
//


/// `interface nsIScriptErrorNote : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptErrorNote {
    vtable: *const nsIScriptErrorNoteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptErrorNote.
unsafe impl XpCom for nsIScriptErrorNote {
    const IID: nsIID = nsID(0xe8933fc9, 0xc302, 0x4e12,
        [0xa5, 0x5b, 0x4f, 0x88, 0x61, 0x1d, 0x9c, 0x6c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptErrorNote {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptErrorNote.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptErrorNoteCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptErrorNote`.
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptErrorNoteCoerce for nsIScriptErrorNote {
    #[inline]
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self {
        v
    }
}

impl nsIScriptErrorNote {
    /// Cast this `nsIScriptErrorNote` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptErrorNoteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptErrorNote {
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
impl<T: nsISupportsCoerce> nsIScriptErrorNoteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptErrorNote) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptErrorNote
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptErrorNoteVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString errorMessage; */
    pub GetErrorMessage: unsafe extern "system" fn (this: *const nsIScriptErrorNote, aErrorMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString sourceName; */
    pub GetSourceName: unsafe extern "system" fn (this: *const nsIScriptErrorNote, aSourceName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t sourceId; */
    pub GetSourceId: unsafe extern "system" fn (this: *const nsIScriptErrorNote, aSourceId: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lineNumber; */
    pub GetLineNumber: unsafe extern "system" fn (this: *const nsIScriptErrorNote, aLineNumber: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t columnNumber; */
    pub GetColumnNumber: unsafe extern "system" fn (this: *const nsIScriptErrorNote, aColumnNumber: *mut uint32_t) -> ::nserror::nsresult,

    /* AUTF8String toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsIScriptErrorNote, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptErrorNote {


    /// `readonly attribute AString errorMessage;`
    #[inline]
    pub unsafe fn GetErrorMessage(&self, aErrorMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorMessage)(self, aErrorMessage)
    }



    /// `readonly attribute AString sourceName;`
    #[inline]
    pub unsafe fn GetSourceName(&self, aSourceName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceName)(self, aSourceName)
    }


    /// ```text
    /// /**
    ///      * Unique identifier within the process for the script source this note is
    ///      * associated with, or zero.
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t sourceId;`
    #[inline]
    pub unsafe fn GetSourceId(&self, aSourceId: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceId)(self, aSourceId)
    }



    /// `readonly attribute uint32_t lineNumber;`
    #[inline]
    pub unsafe fn GetLineNumber(&self, aLineNumber: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLineNumber)(self, aLineNumber)
    }



    /// `readonly attribute uint32_t columnNumber;`
    #[inline]
    pub unsafe fn GetColumnNumber(&self, aColumnNumber: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnNumber)(self, aColumnNumber)
    }



    /// `AUTF8String toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


/// `interface nsIScriptError : nsIConsoleMessage`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptError {
    vtable: *const nsIScriptErrorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptError.
unsafe impl XpCom for nsIScriptError {
    const IID: nsIID = nsID(0x63eb4d3e, 0x7d99, 0x4150,
        [0xb4, 0xf3, 0x11, 0x31, 0x4f, 0x9d, 0x82, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptError {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptError.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptErrorCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptError`.
    fn coerce_from(v: &nsIScriptError) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptErrorCoerce for nsIScriptError {
    #[inline]
    fn coerce_from(v: &nsIScriptError) -> &Self {
        v
    }
}

impl nsIScriptError {
    /// Cast this `nsIScriptError` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptErrorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptError {
    type Target = nsIConsoleMessage;
    #[inline]
    fn deref(&self) -> &nsIConsoleMessage {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIConsoleMessageCoerce> nsIScriptErrorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptError) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptError
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptErrorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIConsoleMessageVTable,

    /* readonly attribute AString errorMessage; */
    pub GetErrorMessage: unsafe extern "system" fn (this: *const nsIScriptError, aErrorMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString sourceName; */
    pub GetSourceName: unsafe extern "system" fn (this: *const nsIScriptError, aSourceName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString sourceLine; */
    pub GetSourceLine: unsafe extern "system" fn (this: *const nsIScriptError, aSourceLine: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute uint32_t sourceId; */
    pub GetSourceId: unsafe extern "system" fn (this: *const nsIScriptError, aSourceId: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t lineNumber; */
    pub GetLineNumber: unsafe extern "system" fn (this: *const nsIScriptError, aLineNumber: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t columnNumber; */
    pub GetColumnNumber: unsafe extern "system" fn (this: *const nsIScriptError, aColumnNumber: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute uint32_t flags; */
    pub GetFlags: unsafe extern "system" fn (this: *const nsIScriptError, aFlags: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute string category; */
    pub GetCategory: unsafe extern "system" fn (this: *const nsIScriptError, aCategory: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long outerWindowID; */
    pub GetOuterWindowID: unsafe extern "system" fn (this: *const nsIScriptError, aOuterWindowID: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long long innerWindowID; */
    pub GetInnerWindowID: unsafe extern "system" fn (this: *const nsIScriptError, aInnerWindowID: *mut u64) -> ::nserror::nsresult,

    /* readonly attribute boolean isFromPrivateWindow; */
    pub GetIsFromPrivateWindow: unsafe extern "system" fn (this: *const nsIScriptError, aIsFromPrivateWindow: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isFromChromeContext; */
    pub GetIsFromChromeContext: unsafe extern "system" fn (this: *const nsIScriptError, aIsFromChromeContext: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isForwardedFromContentProcess; */
    pub GetIsForwardedFromContentProcess: unsafe extern "system" fn (this: *const nsIScriptError, aIsForwardedFromContentProcess: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isForwardedFromContentProcess; */
    pub SetIsForwardedFromContentProcess: unsafe extern "system" fn (this: *const nsIScriptError, aIsForwardedFromContentProcess: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isPromiseRejection; */
    pub GetIsPromiseRejection: unsafe extern "system" fn (this: *const nsIScriptError, aIsPromiseRejection: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void initIsPromiseRejection (in bool isPromiseRejection); */
    pub InitIsPromiseRejection: unsafe extern "system" fn (this: *const nsIScriptError, isPromiseRejection: bool) -> ::nserror::nsresult,

    /* attribute jsval exception; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetException: *const ::libc::c_void,

    /* attribute jsval exception; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetException: *const ::libc::c_void,

    /* readonly attribute boolean hasException; */
    pub GetHasException: unsafe extern "system" fn (this: *const nsIScriptError, aHasException: *mut bool) -> ::nserror::nsresult,

    /* attribute jsval stack; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetStack: *const ::libc::c_void,

    /* attribute jsval stack; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub SetStack: *const ::libc::c_void,

    /* [noscript] readonly attribute jsval stackGlobal; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetStackGlobal: *const ::libc::c_void,

    /* attribute AString errorMessageName; */
    pub GetErrorMessageName: unsafe extern "system" fn (this: *const nsIScriptError, aErrorMessageName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString errorMessageName; */
    pub SetErrorMessageName: unsafe extern "system" fn (this: *const nsIScriptError, aErrorMessageName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIArray notes; */
    pub GetNotes: unsafe extern "system" fn (this: *const nsIScriptError, aNotes: *mut *const nsIArray) -> ::nserror::nsresult,

    /* attribute AString cssSelectors; */
    pub GetCssSelectors: unsafe extern "system" fn (this: *const nsIScriptError, aCssSelectors: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString cssSelectors; */
    pub SetCssSelectors: unsafe extern "system" fn (this: *const nsIScriptError, aCssSelectors: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void init (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in string category, [optional] in bool fromPrivateWindow, [optional] in bool fromChromeContext); */
    pub Init: unsafe extern "system" fn (this: *const nsIScriptError, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const libc::c_char, fromPrivateWindow: bool, fromChromeContext: bool) -> ::nserror::nsresult,

    /* void initWithWindowID (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
    pub InitWithWindowID: unsafe extern "system" fn (this: *const nsIScriptError, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult,

    /* void initWithSanitizedSource (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
    pub InitWithSanitizedSource: unsafe extern "system" fn (this: *const nsIScriptError, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult,

    /* void initWithSourceURI (in AString message, in nsIURI sourceURI, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext); */
    pub InitWithSourceURI: unsafe extern "system" fn (this: *const nsIScriptError, message: *const ::nsstring::nsAString, sourceURI: *const nsIURI, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult,

    /* void initSourceId (in uint32_t sourceId); */
    pub InitSourceId: unsafe extern "system" fn (this: *const nsIScriptError, sourceId: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptError {
    /// ```text
    /// /** pseudo-flag for default case */
    /// ```
    ///

    pub const errorFlag: i64 = 0;

    /// ```text
    /// /** message is warning */
    /// ```
    ///

    pub const warningFlag: i64 = 1;

    /// ```text
    /// /** just a log message */
    /// ```
    ///

    pub const infoFlag: i64 = 8;

    /// ```text
    /// /**
    ///      * The error message without any context/line number information.
    ///      *
    ///      * @note nsIConsoleMessage.message will return the error formatted
    ///      *       with file/line information.
    ///      */
    /// ```
    ///

    /// `readonly attribute AString errorMessage;`
    #[inline]
    pub unsafe fn GetErrorMessage(&self, aErrorMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorMessage)(self, aErrorMessage)
    }



    /// `readonly attribute AString sourceName;`
    #[inline]
    pub unsafe fn GetSourceName(&self, aSourceName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceName)(self, aSourceName)
    }



    /// `readonly attribute AString sourceLine;`
    #[inline]
    pub unsafe fn GetSourceLine(&self, aSourceLine: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceLine)(self, aSourceLine)
    }


    /// ```text
    /// /**
    ///      * Unique identifier within the process for the script source this error is
    ///      * associated with, or zero.
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t sourceId;`
    #[inline]
    pub unsafe fn GetSourceId(&self, aSourceId: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceId)(self, aSourceId)
    }



    /// `readonly attribute uint32_t lineNumber;`
    #[inline]
    pub unsafe fn GetLineNumber(&self, aLineNumber: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLineNumber)(self, aLineNumber)
    }



    /// `readonly attribute uint32_t columnNumber;`
    #[inline]
    pub unsafe fn GetColumnNumber(&self, aColumnNumber: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetColumnNumber)(self, aColumnNumber)
    }



    /// `readonly attribute uint32_t flags;`
    #[inline]
    pub unsafe fn GetFlags(&self, aFlags: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFlags)(self, aFlags)
    }


    /// ```text
    /// /**
    ///      * Categories I know about -
    ///      * XUL javascript
    ///      * content javascript (both of these from nsDocShell, currently)
    ///      * system javascript (errors in JS components and other system JS)
    ///      */
    /// ```
    ///

    /// `readonly attribute string category;`
    #[inline]
    pub unsafe fn GetCategory(&self, aCategory: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetCategory)(self, aCategory)
    }



    /// `readonly attribute unsigned long long outerWindowID;`
    #[inline]
    pub unsafe fn GetOuterWindowID(&self, aOuterWindowID: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetOuterWindowID)(self, aOuterWindowID)
    }



    /// `readonly attribute unsigned long long innerWindowID;`
    #[inline]
    pub unsafe fn GetInnerWindowID(&self, aInnerWindowID: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).GetInnerWindowID)(self, aInnerWindowID)
    }



    /// `readonly attribute boolean isFromPrivateWindow;`
    #[inline]
    pub unsafe fn GetIsFromPrivateWindow(&self, aIsFromPrivateWindow: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFromPrivateWindow)(self, aIsFromPrivateWindow)
    }



    /// `readonly attribute boolean isFromChromeContext;`
    #[inline]
    pub unsafe fn GetIsFromChromeContext(&self, aIsFromChromeContext: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFromChromeContext)(self, aIsFromChromeContext)
    }



    /// `attribute boolean isForwardedFromContentProcess;`
    #[inline]
    pub unsafe fn GetIsForwardedFromContentProcess(&self, aIsForwardedFromContentProcess: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsForwardedFromContentProcess)(self, aIsForwardedFromContentProcess)
    }



    /// `attribute boolean isForwardedFromContentProcess;`
    #[inline]
    pub unsafe fn SetIsForwardedFromContentProcess(&self, aIsForwardedFromContentProcess: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsForwardedFromContentProcess)(self, aIsForwardedFromContentProcess)
    }



    /// `readonly attribute boolean isPromiseRejection;`
    #[inline]
    pub unsafe fn GetIsPromiseRejection(&self, aIsPromiseRejection: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsPromiseRejection)(self, aIsPromiseRejection)
    }



    /// `[noscript] void initIsPromiseRejection (in bool isPromiseRejection);`
    #[inline]
    pub unsafe fn InitIsPromiseRejection(&self, isPromiseRejection: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitIsPromiseRejection)(self, isPromiseRejection)
    }



    /// `attribute jsval exception;`
    const _GetException: () = ();


    /// `attribute jsval exception;`
    const _SetException: () = ();


    /// `readonly attribute boolean hasException;`
    #[inline]
    pub unsafe fn GetHasException(&self, aHasException: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasException)(self, aHasException)
    }



    /// `attribute jsval stack;`
    const _GetStack: () = ();


    /// `attribute jsval stack;`
    const _SetStack: () = ();

    /// ```text
    /// /**
    ///      * If |stack| is an object, then stackGlobal must be a global object that's
    ///      * same-compartment with |stack|. This can be used to enter the correct
    ///      * realm when working with the stack object. We can't use the object itself
    ///      * because it might be a cross-compartment wrapper and CCWs are not
    ///      * associated with a single realm/global.
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute jsval stackGlobal;`
    const _GetStackGlobal: () = ();

    /// ```text
    /// /**
    ///      * The name of a template string associated with the error message.  See
    ///      * js/public/friend/ErrorNumbers.msg.
    ///      */
    /// ```
    ///

    /// `attribute AString errorMessageName;`
    #[inline]
    pub unsafe fn GetErrorMessageName(&self, aErrorMessageName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetErrorMessageName)(self, aErrorMessageName)
    }


    /// ```text
    /// /**
    ///      * The name of a template string associated with the error message.  See
    ///      * js/public/friend/ErrorNumbers.msg.
    ///      */
    /// ```
    ///

    /// `attribute AString errorMessageName;`
    #[inline]
    pub unsafe fn SetErrorMessageName(&self, aErrorMessageName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetErrorMessageName)(self, aErrorMessageName)
    }



    /// `readonly attribute nsIArray notes;`
    #[inline]
    pub unsafe fn GetNotes(&self, aNotes: *mut *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetNotes)(self, aNotes)
    }


    /// ```text
    /// /**
    ///      * If the ScriptError is a CSS parser error, this value will contain the
    ///      * CSS selectors of the CSS ruleset where the error occured.
    ///      */
    /// ```
    ///

    /// `attribute AString cssSelectors;`
    #[inline]
    pub unsafe fn GetCssSelectors(&self, aCssSelectors: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCssSelectors)(self, aCssSelectors)
    }


    /// ```text
    /// /**
    ///      * If the ScriptError is a CSS parser error, this value will contain the
    ///      * CSS selectors of the CSS ruleset where the error occured.
    ///      */
    /// ```
    ///

    /// `attribute AString cssSelectors;`
    #[inline]
    pub unsafe fn SetCssSelectors(&self, aCssSelectors: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetCssSelectors)(self, aCssSelectors)
    }



    /// `void init (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in string category, [optional] in bool fromPrivateWindow, [optional] in bool fromChromeContext);`
    #[inline]
    pub unsafe fn Init(&self, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const libc::c_char, fromPrivateWindow: bool, fromChromeContext: bool) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, fromPrivateWindow, fromChromeContext)
    }



    /// `void initWithWindowID (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext);`
    #[inline]
    pub unsafe fn InitWithWindowID(&self, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitWithWindowID)(self, message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext)
    }



    /// `void initWithSanitizedSource (in AString message, in AString sourceName, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext);`
    #[inline]
    pub unsafe fn InitWithSanitizedSource(&self, message: *const ::nsstring::nsAString, sourceName: *const ::nsstring::nsAString, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitWithSanitizedSource)(self, message, sourceName, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext)
    }



    /// `void initWithSourceURI (in AString message, in nsIURI sourceURI, in AString sourceLine, in uint32_t lineNumber, in uint32_t columnNumber, in uint32_t flags, in ACString category, in unsigned long long innerWindowID, [optional] in bool fromChromeContext);`
    #[inline]
    pub unsafe fn InitWithSourceURI(&self, message: *const ::nsstring::nsAString, sourceURI: *const nsIURI, sourceLine: *const ::nsstring::nsAString, lineNumber: uint32_t, columnNumber: uint32_t, flags: uint32_t, category: *const ::nsstring::nsACString, innerWindowID: u64, fromChromeContext: bool) -> ::nserror::nsresult {
        ((*self.vtable).InitWithSourceURI)(self, message, sourceURI, sourceLine, lineNumber, columnNumber, flags, category, innerWindowID, fromChromeContext)
    }



    /// `void initSourceId (in uint32_t sourceId);`
    #[inline]
    pub unsafe fn InitSourceId(&self, sourceId: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).InitSourceId)(self, sourceId)
    }


}



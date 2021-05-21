//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLine.idl
//


/// `interface nsICommandLine : nsISupports`
///

/// ```text
/// /**
///  * Represents the command line used to invoke a XUL application. This may be the
///  * original command-line of this instance, or a command line remoted from another
///  * instance of the application.
///  *
///  * DEFINITIONS:
///  * "arguments" are any values found on the command line.
///  * "flags" are switches. In normalized form they are preceded by a single dash.
///  * Some flags may take "parameters", e.g. "--url <param>".
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandLine {
    vtable: *const nsICommandLineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandLine.
unsafe impl XpCom for nsICommandLine {
    const IID: nsIID = nsID(0xbc3173bd, 0xaa46, 0x46a0,
        [0x9d, 0x25, 0xd9, 0x86, 0x7a, 0x96, 0x59, 0xb6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandLine {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandLine.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandLineCoerce {
    /// Cheaply cast a value of this type from a `nsICommandLine`.
    fn coerce_from(v: &nsICommandLine) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandLineCoerce for nsICommandLine {
    #[inline]
    fn coerce_from(v: &nsICommandLine) -> &Self {
        v
    }
}

impl nsICommandLine {
    /// Cast this `nsICommandLine` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandLineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandLine {
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
impl<T: nsISupportsCoerce> nsICommandLineCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLine) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandLine
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandLineVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsICommandLine, aLength: *mut i32) -> ::nserror::nsresult,

    /* AString getArgument (in long aIndex); */
    pub GetArgument: unsafe extern "system" fn (this: *const nsICommandLine, aIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* long findFlag (in AString aFlag, in boolean aCaseSensitive); */
    pub FindFlag: unsafe extern "system" fn (this: *const nsICommandLine, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut i32) -> ::nserror::nsresult,

    /* void removeArguments (in long aStart, in long aEnd); */
    pub RemoveArguments: unsafe extern "system" fn (this: *const nsICommandLine, aStart: i32, aEnd: i32) -> ::nserror::nsresult,

    /* boolean handleFlag (in AString aFlag, in boolean aCaseSensitive); */
    pub HandleFlag: unsafe extern "system" fn (this: *const nsICommandLine, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive); */
    pub HandleFlagWithParam: unsafe extern "system" fn (this: *const nsICommandLine, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long state; */
    pub GetState: unsafe extern "system" fn (this: *const nsICommandLine, aState: *mut u32) -> ::nserror::nsresult,

    /* attribute boolean preventDefault; */
    pub GetPreventDefault: unsafe extern "system" fn (this: *const nsICommandLine, aPreventDefault: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean preventDefault; */
    pub SetPreventDefault: unsafe extern "system" fn (this: *const nsICommandLine, aPreventDefault: bool) -> ::nserror::nsresult,

    /* readonly attribute nsIFile workingDirectory; */
    pub GetWorkingDirectory: unsafe extern "system" fn (this: *const nsICommandLine, aWorkingDirectory: *mut*const nsIFile) -> ::nserror::nsresult,

    /* nsIFile resolveFile (in AString aArgument); */
    pub ResolveFile: unsafe extern "system" fn (this: *const nsICommandLine, aArgument: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult,

    /* nsIURI resolveURI (in AString aArgument); */
    pub ResolveURI: unsafe extern "system" fn (this: *const nsICommandLine, aArgument: *const ::nsstring::nsAString, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandLine {

    pub const STATE_INITIAL_LAUNCH: i64 = 0;


    pub const STATE_REMOTE_AUTO: i64 = 1;


    pub const STATE_REMOTE_EXPLICIT: i64 = 2;

    /// ```text
    /// /**
    ///    * Number of arguments in the command line. The application name is not
    ///    * part of the command line.
    ///    */
    /// ```
    ///

    /// `readonly attribute long length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///    * Get an argument from the array of command-line arguments.
    ///    *
    ///    * On windows, flags of the form /flag are normalized to -flag. /flag:param
    ///    * are normalized to -flag param.
    ///    *
    ///    * On *nix and mac flags of the form --flag are normalized to -flag. --flag=param
    ///    * are normalized to the form -flag param.
    ///    *
    ///    * @param aIndex The argument to retrieve. This index is 0-based, and does
    ///    *               not include the application name.
    ///    * @return       The indexth argument.
    ///    * @throws       NS_ERROR_INVALID_ARG if aIndex is out of bounds.
    ///    */
    /// ```
    ///

    /// `AString getArgument (in long aIndex);`
    #[inline]
    pub unsafe fn GetArgument(&self, aIndex: i32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetArgument)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Find a command-line flag.
    ///    *
    ///    * @param aFlag          The flag name to locate. Do not include the initial
    ///    *                       hyphen.
    ///    * @param aCaseSensitive Whether to do case-sensitive comparisons.
    ///    * @return               The position of the flag in the command line.
    ///    */
    /// ```
    ///

    /// `long findFlag (in AString aFlag, in boolean aCaseSensitive);`
    #[inline]
    pub unsafe fn FindFlag(&self, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).FindFlag)(self, aFlag, aCaseSensitive, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove arguments from the command line. This normally occurs after
    ///    * a handler has processed the arguments.
    ///    *
    ///    * @param aStart  Index to begin removing.
    ///    * @param aEnd    Index to end removing, inclusive.
    ///    */
    /// ```
    ///

    /// `void removeArguments (in long aStart, in long aEnd);`
    #[inline]
    pub unsafe fn RemoveArguments(&self, aStart: i32, aEnd: i32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveArguments)(self, aStart, aEnd)
    }


    /// ```text
    /// /**
    ///    * A helper method which will find a flag and remove it in one step.
    ///    *
    ///    * @param aFlag  The flag name to find and remove.
    ///    * @param aCaseSensitive Whether to do case-sensitive comparisons.
    ///    * @return       Whether the flag was found.
    ///    */
    /// ```
    ///

    /// `boolean handleFlag (in AString aFlag, in boolean aCaseSensitive);`
    #[inline]
    pub unsafe fn HandleFlag(&self, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HandleFlag)(self, aFlag, aCaseSensitive, _retval)
    }


    /// ```text
    /// /**
    ///    * Find a flag with a parameter and remove both. This is a helper
    ///    * method that combines "findFlag" and "removeArguments" in one step.
    ///    *
    ///    * @return   null (a void astring) if the flag is not found. The parameter value
    ///    *           if found. Note that null and the empty string are not the same.
    ///    * @throws   NS_ERROR_INVALID_ARG if the flag exists without a parameter
    ///    *
    ///    * @param aFlag The flag name to find and remove.
    ///    * @param aCaseSensitive Whether to do case-sensitive flag search.
    ///    */
    /// ```
    ///

    /// `AString handleFlagWithParam (in AString aFlag, in boolean aCaseSensitive);`
    #[inline]
    pub unsafe fn HandleFlagWithParam(&self, aFlag: *const ::nsstring::nsAString, aCaseSensitive: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).HandleFlagWithParam)(self, aFlag, aCaseSensitive, _retval)
    }


    /// ```text
    /// /**
    ///    * The type of command line being processed.
    ///    *
    ///    * STATE_INITIAL_LAUNCH  is the first launch of the application instance.
    ///    * STATE_REMOTE_AUTO     is a remote command line automatically redirected to
    ///    *                       this instance.
    ///    * STATE_REMOTE_EXPLICIT is a remote command line explicitly redirected to
    ///    *                       this instance using xremote/windde/appleevents.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * There may be a command-line handler which performs a default action if
    ///    * there was no explicit action on the command line (open a default browser
        ///    * window, for example). This flag allows the default action to be prevented.
    ///    */
    /// ```
    ///

    /// `attribute boolean preventDefault;`
    #[inline]
    pub unsafe fn GetPreventDefault(&self, aPreventDefault: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPreventDefault)(self, aPreventDefault)
    }


    /// ```text
    /// /**
    ///    * There may be a command-line handler which performs a default action if
    ///    * there was no explicit action on the command line (open a default browser
        ///    * window, for example). This flag allows the default action to be prevented.
    ///    */
    /// ```
    ///

    /// `attribute boolean preventDefault;`
    #[inline]
    pub unsafe fn SetPreventDefault(&self, aPreventDefault: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPreventDefault)(self, aPreventDefault)
    }


    /// ```text
    /// /**
    ///    * The working directory for this command line. Use this property instead
    ///    * of the working directory for the current process, since a redirected
    ///    * command line may have had a different working directory.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if the working directory was not specified.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile workingDirectory;`
    #[inline]
    pub unsafe fn GetWorkingDirectory(&self, aWorkingDirectory: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetWorkingDirectory)(self, aWorkingDirectory)
    }


    /// ```text
    /// /**
    ///    * Resolve a file-path argument into an nsIFile. This method gracefully
    ///    * handles relative or absolute file paths, according to the working
    ///    * directory of this command line.
    ///    * If the path is relative and there is no working directory available,
    ///    * this may return null.
    ///    *
    ///    * @param aArgument  The path to resolve.
    ///    *
    ///    */
    /// ```
    ///

    /// `nsIFile resolveFile (in AString aArgument);`
    #[inline]
    pub unsafe fn ResolveFile(&self, aArgument: *const ::nsstring::nsAString, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).ResolveFile)(self, aArgument, _retval)
    }


    /// ```text
    /// /**
    ///    * Resolves a URI argument into a URI. This method has platform-specific
    ///    * logic for converting an absolute URI or a relative file-path into the
    ///    * appropriate URI object; it gracefully handles win32 C:\ paths which would
    ///    * confuse the ioservice if passed directly.
    ///    *
    ///    * @param aArgument  The command-line argument to resolve.
    ///    */
    /// ```
    ///

    /// `nsIURI resolveURI (in AString aArgument);`
    #[inline]
    pub unsafe fn ResolveURI(&self, aArgument: *const ::nsstring::nsAString, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ResolveURI)(self, aArgument, _retval)
    }


}



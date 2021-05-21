//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIDebug2.idl
//


/// `interface nsIDebug2 : nsISupports`
///

/// ```text
/// /**
///  *   For use by consumers in scripted languages (JavaScript, Java, Python,
    ///  *   Perl, ...).
///  *
///  *   @note C/C++ consumers who are planning to use the nsIDebug2 interface with
///  *   the "@mozilla.org/xpcom;1" contract should use NS_DebugBreak from xpcom
///  *   glue instead.
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDebug2 {
    vtable: *const nsIDebug2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDebug2.
unsafe impl XpCom for nsIDebug2 {
    const IID: nsIID = nsID(0x9641dc15, 0x10fb, 0x42e3,
        [0xa2, 0x85, 0x18, 0xbe, 0x90, 0xa5, 0xc1, 0x0b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDebug2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDebug2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDebug2Coerce {
    /// Cheaply cast a value of this type from a `nsIDebug2`.
    fn coerce_from(v: &nsIDebug2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDebug2Coerce for nsIDebug2 {
    #[inline]
    fn coerce_from(v: &nsIDebug2) -> &Self {
        v
    }
}

impl nsIDebug2 {
    /// Cast this `nsIDebug2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDebug2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDebug2 {
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
impl<T: nsISupportsCoerce> nsIDebug2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIDebug2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDebug2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDebug2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean isDebugBuild; */
    pub GetIsDebugBuild: unsafe extern "system" fn (this: *const nsIDebug2, aIsDebugBuild: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long assertionCount; */
    pub GetAssertionCount: unsafe extern "system" fn (this: *const nsIDebug2, aAssertionCount: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute bool isDebuggerAttached; */
    pub GetIsDebuggerAttached: unsafe extern "system" fn (this: *const nsIDebug2, aIsDebuggerAttached: *mut bool) -> ::nserror::nsresult,

    /* void assertion (in string aStr, in string aExpr, in string aFile, in long aLine); */
    pub Assertion: unsafe extern "system" fn (this: *const nsIDebug2, aStr: *const libc::c_char, aExpr: *const libc::c_char, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult,

    /* void warning (in string aStr, in string aFile, in long aLine); */
    pub Warning: unsafe extern "system" fn (this: *const nsIDebug2, aStr: *const libc::c_char, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult,

    /* void break (in string aFile, in long aLine); */
    pub Break: unsafe extern "system" fn (this: *const nsIDebug2, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult,

    /* void abort (in string aFile, in long aLine); */
    pub Abort: unsafe extern "system" fn (this: *const nsIDebug2, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult,

    /* void rustPanic (in string aMessage); */
    pub RustPanic: unsafe extern "system" fn (this: *const nsIDebug2, aMessage: *const libc::c_char) -> ::nserror::nsresult,

    /* void rustLog (in string aTarget, in string aMessage); */
    pub RustLog: unsafe extern "system" fn (this: *const nsIDebug2, aTarget: *const libc::c_char, aMessage: *const libc::c_char) -> ::nserror::nsresult,

    /* void crashWithOOM (); */
    pub CrashWithOOM: unsafe extern "system" fn (this: *const nsIDebug2) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDebug2 {

    /// ```text
    /// /**
    ///      * Whether XPCOM was compiled with DEBUG defined.  This often
    ///      * correlates to whether other code (e.g., Firefox, XULRunner) was
    ///      * compiled with DEBUG defined.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isDebugBuild;`
    #[inline]
    pub unsafe fn GetIsDebugBuild(&self, aIsDebugBuild: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDebugBuild)(self, aIsDebugBuild)
    }


    /// ```text
    /// /**
    ///      * The number of assertions since process start.
    ///      */
    /// ```
    ///

    /// `readonly attribute long assertionCount;`
    #[inline]
    pub unsafe fn GetAssertionCount(&self, aAssertionCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetAssertionCount)(self, aAssertionCount)
    }


    /// ```text
    /// /**
    ///      * Whether a debugger is currently attached.
    ///      * Supports Windows + Mac
    ///      */
    /// ```
    ///

    /// `readonly attribute bool isDebuggerAttached;`
    #[inline]
    pub unsafe fn GetIsDebuggerAttached(&self, aIsDebuggerAttached: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDebuggerAttached)(self, aIsDebuggerAttached)
    }


    /// ```text
    /// /**
    ///      * Show an assertion and trigger nsIDebug2.break().
    ///      *
    ///      * @param aStr assertion message
    ///      * @param aExpr expression that failed
    ///      * @param aFile file containing assertion
    ///      * @param aLine line number of assertion
    ///      */
    /// ```
    ///

    /// `void assertion (in string aStr, in string aExpr, in string aFile, in long aLine);`
    #[inline]
    pub unsafe fn Assertion(&self, aStr: *const libc::c_char, aExpr: *const libc::c_char, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult {
        ((*self.vtable).Assertion)(self, aStr, aExpr, aFile, aLine)
    }


    /// ```text
    /// /**
    ///      * Show a warning.
    ///      *
    ///      * @param aStr warning message
    ///      * @param aFile file containing assertion
    ///      * @param aLine line number of assertion
    ///      */
    /// ```
    ///

    /// `void warning (in string aStr, in string aFile, in long aLine);`
    #[inline]
    pub unsafe fn Warning(&self, aStr: *const libc::c_char, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult {
        ((*self.vtable).Warning)(self, aStr, aFile, aLine)
    }


    /// ```text
    /// /**
    ///      * Request to break into a debugger.
    ///      *
    ///      * @param aFile file containing break request
    ///      * @param aLine line number of break request
    ///      */
    /// ```
    ///

    /// `void break (in string aFile, in long aLine);`
    #[inline]
    pub unsafe fn Break(&self, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult {
        ((*self.vtable).Break)(self, aFile, aLine)
    }


    /// ```text
    /// /**
    ///      * Request the process to trigger a fatal abort.
    ///      *
    ///      * @param aFile file containing abort request
    ///      * @param aLine line number of abort request
    ///      */
    /// ```
    ///

    /// `void abort (in string aFile, in long aLine);`
    #[inline]
    pub unsafe fn Abort(&self, aFile: *const libc::c_char, aLine: i32) -> ::nserror::nsresult {
        ((*self.vtable).Abort)(self, aFile, aLine)
    }


    /// ```text
    /// /**
    ///      * Request the process to trigger a fatal panic!() from Rust code.
    ///      *
    ///      * @param aMessage the string to pass to panic!().
    ///      */
    /// ```
    ///

    /// `void rustPanic (in string aMessage);`
    #[inline]
    pub unsafe fn RustPanic(&self, aMessage: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RustPanic)(self, aMessage)
    }


    /// ```text
    /// /**
    ///      * Request the process to log a message for a target and level from Rust code.
    ///      *
    ///      * @param aTarget the string representing the log target.
    ///      * @param aMessage the string representing the log message.
    ///      */
    /// ```
    ///

    /// `void rustLog (in string aTarget, in string aMessage);`
    #[inline]
    pub unsafe fn RustLog(&self, aTarget: *const libc::c_char, aMessage: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).RustLog)(self, aTarget, aMessage)
    }


    /// ```text
    /// /**
    ///      * Cause an Out of Memory Crash.
    ///      */
    /// ```
    ///

    /// `void crashWithOOM ();`
    #[inline]
    pub unsafe fn CrashWithOOM(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CrashWithOOM)(self, )
    }


}



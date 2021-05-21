//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineHandler.idl
//


/// `interface nsICommandLineHandler : nsISupports`
///

/// ```text
/// /**
///  * Handles arguments on the command line of an XUL application.
///  *
///  * Each handler is registered in the category "command-line-handler".
///  * The entries in this category are read in alphabetical order, and each
///  * category value is treated as a service contractid implementing this
///  * interface.
///  *
///  * By convention, handler with ordinary priority should begin with "m".
///  *
///  * Example:
///  * Category             Entry          Value
///  * command-line-handler c-extensions   @mozilla.org/extension-manager/clh;1
///  * command-line-handler m-edit         @mozilla.org/composer/clh;1
///  * command-line-handler m-irc          @mozilla.org/chatzilla/clh;1
///  * command-line-handler y-final        @mozilla.org/browser/clh-final;1
///  *
///  * @note What do we do about localizing helpInfo? Do we make each handler do it,
///  *       or provide a generic solution of some sort? Don't freeze this interface
///  *       without thinking about this!
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandLineHandler {
    vtable: *const nsICommandLineHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandLineHandler.
unsafe impl XpCom for nsICommandLineHandler {
    const IID: nsIID = nsID(0xd4b123df, 0x51ee, 0x48b1,
        [0xa6, 0x63, 0x00, 0x21, 0x80, 0xe6, 0x0d, 0x3b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandLineHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandLineHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandLineHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsICommandLineHandler`.
    fn coerce_from(v: &nsICommandLineHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandLineHandlerCoerce for nsICommandLineHandler {
    #[inline]
    fn coerce_from(v: &nsICommandLineHandler) -> &Self {
        v
    }
}

impl nsICommandLineHandler {
    /// Cast this `nsICommandLineHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandLineHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandLineHandler {
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
impl<T: nsISupportsCoerce> nsICommandLineHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandLineHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandLineHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handle (in nsICommandLine aCommandLine); */
    pub Handle: unsafe extern "system" fn (this: *const nsICommandLineHandler, aCommandLine: *const nsICommandLine) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String helpInfo; */
    pub GetHelpInfo: unsafe extern "system" fn (this: *const nsICommandLineHandler, aHelpInfo: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandLineHandler {

    /// ```text
    /// /**
    ///    * Process a command line. If this handler finds arguments that it
    ///    * understands, it should perform the appropriate actions (such as opening
        ///    * a window), and remove the arguments from the command-line array.
    ///    *
    ///    * @throw NS_ERROR_ABORT to immediately cease command-line handling
    ///    *        (if this is STATE_INITIAL_LAUNCH, quits the app).
    ///    *        All other exceptions are silently ignored.
    ///    */
    /// ```
    ///

    /// `void handle (in nsICommandLine aCommandLine);`
    #[inline]
    pub unsafe fn Handle(&self, aCommandLine: *const nsICommandLine) -> ::nserror::nsresult {
        ((*self.vtable).Handle)(self, aCommandLine)
    }


    /// ```text
    /// /**
    ///    * When the app is launched with the --help argument, this attribute
    ///    * is retrieved and displayed to the user (on stdout). The text should
    ///    * have embedded newlines which wrap at 76 columns, and should include
    ///    * a newline at the end. By convention, the right column which contains flag
    ///    * descriptions begins at the 24th character.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String helpInfo;`
    #[inline]
    pub unsafe fn GetHelpInfo(&self, aHelpInfo: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHelpInfo)(self, aHelpInfo)
    }


}



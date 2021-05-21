//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/commandlines/nsICommandLineRunner.idl
//


/// `interface nsICommandLineRunner : nsICommandLine`
///

/// ```text
/// /**
///  * Extension of nsICommandLine that allows for initialization of new command lines
///  * and running the command line actions by processing the command line handlers.
///  *
///  * @status INTERNAL - This interface is not meant for use by embedders, and is
///  *                    not intended to be frozen. If you are an embedder and need
///  *                    functionality provided by this interface, talk to Benjamin
///  *                    Smedberg <benjamin@smedbergs.us>.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICommandLineRunner {
    vtable: *const nsICommandLineRunnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICommandLineRunner.
unsafe impl XpCom for nsICommandLineRunner {
    const IID: nsIID = nsID(0xc9f2996c, 0xb25a, 0x4d3d,
        [0x82, 0x1f, 0x4c, 0xd0, 0xc4, 0xbc, 0x8a, 0xfb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICommandLineRunner {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICommandLineRunner.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICommandLineRunnerCoerce {
    /// Cheaply cast a value of this type from a `nsICommandLineRunner`.
    fn coerce_from(v: &nsICommandLineRunner) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICommandLineRunnerCoerce for nsICommandLineRunner {
    #[inline]
    fn coerce_from(v: &nsICommandLineRunner) -> &Self {
        v
    }
}

impl nsICommandLineRunner {
    /// Cast this `nsICommandLineRunner` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICommandLineRunnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICommandLineRunner {
    type Target = nsICommandLine;
    #[inline]
    fn deref(&self) -> &nsICommandLine {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICommandLineCoerce> nsICommandLineRunnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICommandLineRunner) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICommandLineRunner
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICommandLineRunnerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICommandLineVTable,

    /* void init (in long argc, in nsArgvArray argv, in nsIFile workingDir, in unsigned long state); */
    /// Unable to generate binding because `native type const char* const unsupported`
    pub Init: *const ::libc::c_void,

    /* void run (); */
    pub Run: unsafe extern "system" fn (this: *const nsICommandLineRunner) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String helpText; */
    pub GetHelpText: unsafe extern "system" fn (this: *const nsICommandLineRunner, aHelpText: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICommandLineRunner {

    /// ```text
    /// /**
    ///    * This method assumes a native character set, and is meant to be called
    ///    * with the argc/argv passed to main(). Talk to bsmedberg if you need to
    ///    * create a command line using other data. argv will not be altered in any
    ///    * way.
    ///    *
    ///    * On Windows, the "native" character set is UTF-8, not the native codepage.
    ///    *
    ///    * @param workingDir The working directory for resolving file and URI paths.
    ///    *                   Can be null, in which case resolving files will not
    ///    *                   work, and only absolute URIs will be resolvable.
    ///    * @param state      The nsICommandLine.state flag.
    ///    */
    /// ```
    ///

    /// `void init (in long argc, in nsArgvArray argv, in nsIFile workingDir, in unsigned long state);`
    const _Init: () = ();

    /// ```text
    /// /**
    ///    * Process the command-line handlers in the proper order, calling "handle()" on
    ///    * each.
    ///    *
    ///    * @throws NS_ERROR_ABORT if any handler throws NS_ERROR_ABORT. All other errors
    ///    *         thrown by handlers will be silently ignored.
    ///    */
    /// ```
    ///

    /// `void run ();`
    #[inline]
    pub unsafe fn Run(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Run)(self, )
    }


    /// ```text
    /// /**
    ///    * Process and combine the help text provided by each command-line handler.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String helpText;`
    #[inline]
    pub unsafe fn GetHelpText(&self, aHelpText: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHelpText)(self, aHelpText)
    }


}



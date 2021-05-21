//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/mozIJSSubScriptLoader.idl
//


/// `interface mozIJSSubScriptLoader : nsISupports`
///

/// ```text
/// /**
///  * Interface for synchronous script loads from local file: or jar: sources.
///  * For asynchronous script loads, ChromeUtils.compileScript() should be used
///  * instead.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIJSSubScriptLoader {
    vtable: *const mozIJSSubScriptLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIJSSubScriptLoader.
unsafe impl XpCom for mozIJSSubScriptLoader {
    const IID: nsIID = nsID(0x19533e7b, 0xf321, 0x4ef1,
        [0xbc, 0x59, 0x6e, 0x81, 0x2d, 0xc2, 0xa7, 0x33]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIJSSubScriptLoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIJSSubScriptLoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIJSSubScriptLoaderCoerce {
    /// Cheaply cast a value of this type from a `mozIJSSubScriptLoader`.
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIJSSubScriptLoaderCoerce for mozIJSSubScriptLoader {
    #[inline]
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self {
        v
    }
}

impl mozIJSSubScriptLoader {
    /// Cast this `mozIJSSubScriptLoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIJSSubScriptLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIJSSubScriptLoader {
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
impl<T: nsISupportsCoerce> mozIJSSubScriptLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIJSSubScriptLoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIJSSubScriptLoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIJSSubScriptLoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval loadSubScript (in AString url, [optional] in jsval obj); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub LoadSubScript: *const ::libc::c_void,

    /* [implicit_jscontext] jsval loadSubScriptWithOptions (in AString url, in jsval options); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub LoadSubScriptWithOptions: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIJSSubScriptLoader {

    /// ```text
    /// /**
    ///      * This method should only be called from JS!
    ///      * In JS, the signature looks like:
    ///      * rv loadSubScript (url [, obj] [, charset]);
    ///      * @param url the url of the UTF-8-encoded sub-script, it MUST be either a
    ///      *            file:, resource:, blob:, or chrome: url, and MUST be local.
    ///      * @param obj an optional object to evaluate the script onto, it
    ///      *            defaults to the global object of the caller.
    ///      * @retval rv the value returned by the sub-script
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval loadSubScript (in AString url, [optional] in jsval obj);`
    const _LoadSubScript: () = ();

    /// ```text
    /// /**
    ///      * This method should only be called from JS!
    ///      * In JS, the signature looks like:
    ///      * rv = loadSubScript (url, optionsObject)
    ///      * @param url the url of the UTF-8-encoded sub-script, which MUST be either
    ///      *            a file:, resource:, blob:, or chrome: url, and MUST be local.
    ///      * @param optionsObject an object with parameters. Valid parameters are:
    ///      *                      - target:  an object to evaluate onto (default: global object of the caller)
    ///      *                      - ignoreCache: if set to true, will bypass the cache for reading the file.
    ///      *                      - async: if set to true, the script will be loaded
    ///      *                        asynchronously, and a Promise is returned which
    ///      *                        resolves to its result when execution is complete.
    ///      *                      - wantReturnValue: If true, the script will return
    ///      *                        the value of the last statement that it evaluated.
    ///      *                        This option disables most optimizations in the
    ///      *                        top-level scope, and should be avoided if at all
    ///      *                        possible. Defaults to false.
    ///      * @retval rv the value returned by the sub-script
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] jsval loadSubScriptWithOptions (in AString url, in jsval options);`
    const _LoadSubScriptWithOptions: () = ();

}



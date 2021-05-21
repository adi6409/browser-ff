//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/js/xpconnect/idl/nsIXPCScriptable.idl
//


/// `interface nsIXPCScriptable : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIXPCScriptable {
    vtable: *const nsIXPCScriptableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIXPCScriptable.
unsafe impl XpCom for nsIXPCScriptable {
    const IID: nsIID = nsID(0x19b70b26, 0x7c3f, 0x437f,
        [0xa0, 0x4a, 0x2a, 0x8f, 0x9e, 0x28, 0xb6, 0x17]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIXPCScriptable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIXPCScriptable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIXPCScriptableCoerce {
    /// Cheaply cast a value of this type from a `nsIXPCScriptable`.
    fn coerce_from(v: &nsIXPCScriptable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIXPCScriptableCoerce for nsIXPCScriptable {
    #[inline]
    fn coerce_from(v: &nsIXPCScriptable) -> &Self {
        v
    }
}

impl nsIXPCScriptable {
    /// Cast this `nsIXPCScriptable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIXPCScriptableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIXPCScriptable {
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
impl<T: nsISupportsCoerce> nsIXPCScriptableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIXPCScriptable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIXPCScriptable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIXPCScriptableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String className; */
    pub GetClassName: unsafe extern "system" fn (this: *const nsIXPCScriptable, aClassName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] uint32_t getScriptableFlags (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetScriptableFlags: *const ::libc::c_void,

    /* [nostdcall,notxpcom] JSClassPtr getJSClass (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetJSClass: *const ::libc::c_void,

    /* void preCreate (in nsISupports nativeObj, in JSContextPtr cx, in JSObjectPtr globalObj, out JSObjectPtr parentObj); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub PreCreate: *const ::libc::c_void,

    /* boolean newEnumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSMutableHandleIdVector properties, in boolean enumerableOnly); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub NewEnumerate: *const ::libc::c_void,

    /* boolean resolve (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, out boolean resolvedp); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub Resolve: *const ::libc::c_void,

    /* void finalize (in nsIXPConnectWrappedNative wrapper, in JSFreeOpPtr fop, in JSObjectPtr obj); */
    /// Unable to generate binding because `native type JSFreeOp unsupported`
    pub Finalize: *const ::libc::c_void,

    /* boolean call (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub Call: *const ::libc::c_void,

    /* boolean construct (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub Construct: *const ::libc::c_void,

    /* boolean hasInstance (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsval val, out boolean bp); */
    /// Unable to generate binding because `native type JSContext unsupported`
    pub HasInstance: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIXPCScriptable {

    /// ```text
    /// /**
    ///  * Note: This is not really an XPCOM interface.  For example, callers must
    ///  * guarantee that they set the *_retval of the various methods that return a
    ///  * boolean to PR_TRUE before making the call.  Implementations may skip writing
    ///  * to *_retval unless they want to return PR_FALSE.
    ///  */
    /// ```
    ///

    /// `readonly attribute AUTF8String className;`
    #[inline]
    pub unsafe fn GetClassName(&self, aClassName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetClassName)(self, aClassName)
    }



    /// `[nostdcall,notxpcom] uint32_t getScriptableFlags ();`
    const _GetScriptableFlags: () = ();


    /// `[nostdcall,notxpcom] JSClassPtr getJSClass ();`
    const _GetJSClass: () = ();


    /// `void preCreate (in nsISupports nativeObj, in JSContextPtr cx, in JSObjectPtr globalObj, out JSObjectPtr parentObj);`
    const _PreCreate: () = ();


    /// `boolean newEnumerate (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSMutableHandleIdVector properties, in boolean enumerableOnly);`
    const _NewEnumerate: () = ();


    /// `boolean resolve (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsid id, out boolean resolvedp);`
    const _Resolve: () = ();


    /// `void finalize (in nsIXPConnectWrappedNative wrapper, in JSFreeOpPtr fop, in JSObjectPtr obj);`
    const _Finalize: () = ();


    /// `boolean call (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args);`
    const _Call: () = ();


    /// `boolean construct (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in JSCallArgsRef args);`
    const _Construct: () = ();


    /// `boolean hasInstance (in nsIXPConnectWrappedNative wrapper, in JSContextPtr cx, in JSObjectPtr obj, in jsval val, out boolean bp);`
    const _HasInstance: () = ();

}



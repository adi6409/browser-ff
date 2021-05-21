//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIScriptChannel.idl
//


/// `interface nsIScriptChannel : nsISupports`
///

/// ```text
/// /**
///  * An interface representing a channel which will have to execute some sort of
///  * program provided via its URI to compute the data it should return.
///  *
///  * If a channel implements this interface, the execution of the program in
///  * question will be restricted in the following ways:
///  *
///  * - If the channel does not have an owner principal, the program will not be
///  *   executed at all, no matter what.  This is necessary because in this
///  *   circumstance we have no way to tell whether script execution is allowed at
///  *   all for the originating security context of this channel.
///  * - If the channel has an owner principal, how it is executed is controlled by
///  *   this interface.  However if the owner principal does not subsume the
///  *   principal of the environment in which the program is to be executed the
///  *   execution will be forced to happen in a sandbox.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptChannel {
    vtable: *const nsIScriptChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptChannel.
unsafe impl XpCom for nsIScriptChannel {
    const IID: nsIID = nsID(0x33234b99, 0x9588, 0x4c7d,
        [0x9d, 0xa6, 0x86, 0xb8, 0xb7, 0xcb, 0xa5, 0x65]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptChannel`.
    fn coerce_from(v: &nsIScriptChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptChannelCoerce for nsIScriptChannel {
    #[inline]
    fn coerce_from(v: &nsIScriptChannel) -> &Self {
        v
    }
}

impl nsIScriptChannel {
    /// Cast this `nsIScriptChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptChannel {
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
impl<T: nsISupportsCoerce> nsIScriptChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned long executionPolicy; */
    pub GetExecutionPolicy: unsafe extern "system" fn (this: *const nsIScriptChannel, aExecutionPolicy: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long executionPolicy; */
    pub SetExecutionPolicy: unsafe extern "system" fn (this: *const nsIScriptChannel, aExecutionPolicy: u32) -> ::nserror::nsresult,

    /* attribute boolean executeAsync; */
    pub GetExecuteAsync: unsafe extern "system" fn (this: *const nsIScriptChannel, aExecuteAsync: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean executeAsync; */
    pub SetExecuteAsync: unsafe extern "system" fn (this: *const nsIScriptChannel, aExecuteAsync: bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute boolean isDocumentLoad; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIsDocumentLoad: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptChannel {
    /// ```text
    /// /**
    ///    * Possible ways of executing the program.
    ///    */
    /// /**
    ///    * Don't execute at all.
    ///    */
    /// ```
    ///

    pub const NO_EXECUTION: i64 = 0;

    /// ```text
    /// /**
    ///    * There used to be an EXECUTE_IN_SANDBOX = 1 value.  It has been removed, but
    ///    * we're not changing the value of EXECUTE_NORMAL to avoid breaking compat.
    ///    */
    /// /**
    ///    * Execute against the target environment if the principals allow it.
    ///    */
    /// ```
    ///

    pub const EXECUTE_NORMAL: i64 = 2;

    /// ```text
    /// /**
    ///    * Whether and how the program represented by this channel is to be executed.
    ///    * The default value if this property has never been set on this channel MUST
    ///    * be either EXECUTE_IN_SANDBOX or NO_EXECUTION.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG when set to an unrecognized value.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long executionPolicy;`
    #[inline]
    pub unsafe fn GetExecutionPolicy(&self, aExecutionPolicy: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetExecutionPolicy)(self, aExecutionPolicy)
    }


    /// ```text
    /// /**
    ///    * Whether and how the program represented by this channel is to be executed.
    ///    * The default value if this property has never been set on this channel MUST
    ///    * be either EXECUTE_IN_SANDBOX or NO_EXECUTION.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG when set to an unrecognized value.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long executionPolicy;`
    #[inline]
    pub unsafe fn SetExecutionPolicy(&self, aExecutionPolicy: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetExecutionPolicy)(self, aExecutionPolicy)
    }


    /// ```text
    /// /**
    ///    * Control whether the program should be executed synchronosly when
    ///    * the channel's AsyncOpen method is called or whether it should be
    ///    * executed asynchronously.  In both cases, any data that the
    ///    * channel returns will be returned asynchronously; the only thing
    ///    * this property affects is when the program executes.
    ///    *
    ///    * The default value of this property is TRUE.
    ///    *
    ///    * Setting this property after asyncOpen has been called on the
    ///    * channel has no effect.
    ///    */
    /// ```
    ///

    /// `attribute boolean executeAsync;`
    #[inline]
    pub unsafe fn GetExecuteAsync(&self, aExecuteAsync: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetExecuteAsync)(self, aExecuteAsync)
    }


    /// ```text
    /// /**
    ///    * Control whether the program should be executed synchronosly when
    ///    * the channel's AsyncOpen method is called or whether it should be
    ///    * executed asynchronously.  In both cases, any data that the
    ///    * channel returns will be returned asynchronously; the only thing
    ///    * this property affects is when the program executes.
    ///    *
    ///    * The default value of this property is TRUE.
    ///    *
    ///    * Setting this property after asyncOpen has been called on the
    ///    * channel has no effect.
    ///    */
    /// ```
    ///

    /// `attribute boolean executeAsync;`
    #[inline]
    pub unsafe fn SetExecuteAsync(&self, aExecuteAsync: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetExecuteAsync)(self, aExecuteAsync)
    }


    /// ```text
    /// /**
    ///    * Check whether this script channel is a document load.  This is
    ///    * needed because script channels can lie about their
    ///    * LOAD_DOCUMENT_URI flag until they have run the script.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute boolean isDocumentLoad;`
    const _GetIsDocumentLoad: () = ();

}



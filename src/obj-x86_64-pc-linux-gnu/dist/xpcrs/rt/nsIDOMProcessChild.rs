//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIDOMProcessChild.idl
//


/// `interface nsIDOMProcessChild : nsISupports`
///

/// ```text
/// /**
///  * Child actor interface for a process which can host DOM content.
///  *
///  * Implemented by either `InProcessChild` for the parent process, or
///  * `ContentChild` for a content process.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMProcessChild {
    vtable: *const nsIDOMProcessChildVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMProcessChild.
unsafe impl XpCom for nsIDOMProcessChild {
    const IID: nsIID = nsID(0xb0c6e5f3, 0x02f1, 0x4f11,
        [0xa0, 0xaf, 0x33, 0x6f, 0xc2, 0x31, 0xf3, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMProcessChild {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMProcessChild.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMProcessChildCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMProcessChild`.
    fn coerce_from(v: &nsIDOMProcessChild) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMProcessChildCoerce for nsIDOMProcessChild {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessChild) -> &Self {
        v
    }
}

impl nsIDOMProcessChild {
    /// Cast this `nsIDOMProcessChild` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMProcessChildCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMProcessChild {
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
impl<T: nsISupportsCoerce> nsIDOMProcessChildCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessChild) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMProcessChild
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMProcessChildVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long long childID; */
    pub GetChildID: unsafe extern "system" fn (this: *const nsIDOMProcessChild, aChildID: *mut u64) -> ::nserror::nsresult,

    /* [implicit_jscontext] JSProcessActorChild getActor (in ACString name); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetActor: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean canSend; */
    pub GetCanSend: unsafe extern "system" fn (this: *const nsIDOMProcessChild, aCanSend: *mut bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] ContentChildPtr AsContentChild (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AsContentChild: *const ::libc::c_void,

    /* [nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AsJSActorManager: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMProcessChild {

    /// ```text
    /// /**
    ///    * Internal child process ID. `0` is reserved for the parent process.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute unsigned long long childID;`
    #[inline]
    pub unsafe fn GetChildID(&self) -> u64 {
        let mut result = <u64 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetChildID)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Lookup a JSProcessActorChild managed by this interface by name.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] JSProcessActorChild getActor (in ACString name);`
    const _GetActor: () = ();

    /// ```text
    /// /** Can the actor still send messages? */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean canSend;`
    #[inline]
    pub unsafe fn GetCanSend(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCanSend)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[nostdcall,notxpcom] ContentChildPtr AsContentChild ();`
    const _AsContentChild: () = ();

    /// ```text
    /// /** Cast this nsIDOMProcessChild to a JSActorManager */
    /// ```
    ///

    /// `[nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager ();`
    const _AsJSActorManager: () = ();

}



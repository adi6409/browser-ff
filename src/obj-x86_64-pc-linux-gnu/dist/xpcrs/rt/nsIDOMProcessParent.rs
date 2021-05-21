//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/ipc/nsIDOMProcessParent.idl
//


/// `interface nsIDOMProcessParent : nsISupports`
///

/// ```text
/// /**
///  * Parent actor interface for a process which can host DOM content.
///  *
///  * Implemented by either `InProcessParent` for the parent process, or
///  * `ContentParent` for a content process.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMProcessParent {
    vtable: *const nsIDOMProcessParentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMProcessParent.
unsafe impl XpCom for nsIDOMProcessParent {
    const IID: nsIID = nsID(0x81fc08b9, 0xc901, 0x471f,
        [0xab, 0x0d, 0x87, 0x63, 0x62, 0xeb, 0xa7, 0x70]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMProcessParent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMProcessParent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMProcessParentCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMProcessParent`.
    fn coerce_from(v: &nsIDOMProcessParent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMProcessParentCoerce for nsIDOMProcessParent {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessParent) -> &Self {
        v
    }
}

impl nsIDOMProcessParent {
    /// Cast this `nsIDOMProcessParent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMProcessParentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMProcessParent {
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
impl<T: nsISupportsCoerce> nsIDOMProcessParentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMProcessParent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMProcessParent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMProcessParentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute unsigned long long childID; */
    pub GetChildID: unsafe extern "system" fn (this: *const nsIDOMProcessParent, aChildID: *mut u64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute long osPid; */
    pub GetOsPid: unsafe extern "system" fn (this: *const nsIDOMProcessParent, aOsPid: *mut i32) -> ::nserror::nsresult,

    /* [implicit_jscontext] JSProcessActorParent getActor (in ACString name); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub GetActor: *const ::libc::c_void,

    /* [infallible] readonly attribute boolean canSend; */
    pub GetCanSend: unsafe extern "system" fn (this: *const nsIDOMProcessParent, aCanSend: *mut bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] ContentParentPtr AsContentParent (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AsContentParent: *const ::libc::c_void,

    /* [nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub AsJSActorManager: *const ::libc::c_void,

    /* readonly attribute ACString remoteType; */
    pub GetRemoteType: unsafe extern "system" fn (this: *const nsIDOMProcessParent, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMProcessParent {

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
    ///    * OS ID of the process.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute long osPid;`
    #[inline]
    pub unsafe fn GetOsPid(&self) -> i32 {
        let mut result = <i32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetOsPid)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Lookup a JSProcessActorParent managed by this interface by name.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] JSProcessActorParent getActor (in ACString name);`
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



    /// `[nostdcall,notxpcom] ContentParentPtr AsContentParent ();`
    const _AsContentParent: () = ();

    /// ```text
    /// /** Cast this nsIDOMProcessParent to a JSActorManager */
    /// ```
    ///

    /// `[nostdcall,notxpcom] JSActorManagerPtr AsJSActorManager ();`
    const _AsJSActorManager: () = ();

    /// ```text
    /// /**
    ///    * Remote type of the process.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString remoteType;`
    #[inline]
    pub unsafe fn GetRemoteType(&self, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteType)(self, aRemoteType)
    }


}



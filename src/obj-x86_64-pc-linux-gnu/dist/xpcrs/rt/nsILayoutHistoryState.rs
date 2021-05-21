//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/base/nsILayoutHistoryState.idl
//


/// `interface nsILayoutHistoryState : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILayoutHistoryState {
    vtable: *const nsILayoutHistoryStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILayoutHistoryState.
unsafe impl XpCom for nsILayoutHistoryState {
    const IID: nsIID = nsID(0xaef27cb3, 0x4df9, 0x4eeb,
        [0xb0, 0xb0, 0xac, 0x56, 0xcf, 0x86, 0x1d, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILayoutHistoryState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILayoutHistoryState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILayoutHistoryStateCoerce {
    /// Cheaply cast a value of this type from a `nsILayoutHistoryState`.
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILayoutHistoryStateCoerce for nsILayoutHistoryState {
    #[inline]
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self {
        v
    }
}

impl nsILayoutHistoryState {
    /// Cast this `nsILayoutHistoryState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILayoutHistoryStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILayoutHistoryState {
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
impl<T: nsISupportsCoerce> nsILayoutHistoryStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILayoutHistoryState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILayoutHistoryState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILayoutHistoryStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean hasStates; */
    pub GetHasStates: unsafe extern "system" fn (this: *const nsILayoutHistoryState, aHasStates: *mut bool) -> ::nserror::nsresult,

    /* Array<ACString> getKeys (); */
    pub GetKeys: unsafe extern "system" fn (this: *const nsILayoutHistoryState, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* void getPresState (in ACString aKey, out float aScrollX, out float aScrollY, out boolean aAllowScrollOriginDowngrade, out float aRes); */
    pub GetPresState: unsafe extern "system" fn (this: *const nsILayoutHistoryState, aKey: *const ::nsstring::nsACString, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float, aAllowScrollOriginDowngrade: *mut bool, aRes: *mut libc::c_float) -> ::nserror::nsresult,

    /* void addNewPresState (in ACString aKey, in float aScrollX, in float aScrollY, in boolean aAllowScrollOriginDowngrade, in float aRes); */
    pub AddNewPresState: unsafe extern "system" fn (this: *const nsILayoutHistoryState, aKey: *const ::nsstring::nsACString, aScrollX: libc::c_float, aScrollY: libc::c_float, aAllowScrollOriginDowngrade: bool, aRes: libc::c_float) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void AddState (in nsCString aKey, in PresStateUnique aState); */
    /// Unable to generate binding because `native type const nsCString unsupported`
    pub AddState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] PresStatePtr GetState (in nsCString aKey); */
    /// Unable to generate binding because `native type const nsCString unsupported`
    pub GetState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void RemoveState (in nsCString aKey); */
    /// Unable to generate binding because `native type const nsCString unsupported`
    pub RemoveState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] boolean HasStates (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub HasStates: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void SetScrollPositionOnly (in constBool aFlag); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub SetScrollPositionOnly: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void ResetScrollState (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ResetScrollState: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void GetContents (out boolean aScrollPositionOnly, out Array<ACString> aKeys, out Array<PresState> aStates); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetContents: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void Reset (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub Reset: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILayoutHistoryState {

    /// ```text
    /// /**
    ///   * Whether this LayoutHistoryState contains any PresStates.
    ///   */
    /// ```
    ///

    /// `readonly attribute boolean hasStates;`
    #[inline]
    pub unsafe fn GetHasStates(&self, aHasStates: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasStates)(self, aHasStates)
    }


    /// ```text
    /// /**
    ///   * Get the keys of all PresStates held by this LayoutHistoryState.
    ///   * Note: Check hasStates first.
    ///   */
    /// ```
    ///

    /// `Array<ACString> getKeys ();`
    #[inline]
    pub unsafe fn GetKeys(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetKeys)(self, _retval)
    }



    /// `void getPresState (in ACString aKey, out float aScrollX, out float aScrollY, out boolean aAllowScrollOriginDowngrade, out float aRes);`
    #[inline]
    pub unsafe fn GetPresState(&self, aKey: *const ::nsstring::nsACString, aScrollX: *mut libc::c_float, aScrollY: *mut libc::c_float, aAllowScrollOriginDowngrade: *mut bool, aRes: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetPresState)(self, aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes)
    }


    /// ```text
    /// /**
    ///    * Constructs a new PresState object based on the supplied data
    ///    * and adds it to the LayoutHistoryState.
    ///    */
    /// ```
    ///

    /// `void addNewPresState (in ACString aKey, in float aScrollX, in float aScrollY, in boolean aAllowScrollOriginDowngrade, in float aRes);`
    #[inline]
    pub unsafe fn AddNewPresState(&self, aKey: *const ::nsstring::nsACString, aScrollX: libc::c_float, aScrollY: libc::c_float, aAllowScrollOriginDowngrade: bool, aRes: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).AddNewPresState)(self, aKey, aScrollX, aScrollY, aAllowScrollOriginDowngrade, aRes)
    }


    /// ```text
    /// /**
    ///    * Set |aState| as the state object for |aKey|.
    ///    * This _transfers_ownership_ of |aState| to the LayoutHistoryState.
    ///    * It will be freed when RemoveState() is called or when the
    ///    * LayoutHistoryState is destroyed.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void AddState (in nsCString aKey, in PresStateUnique aState);`
    const _AddState: () = ();

    /// ```text
    /// /**
    ///    * Look up the state object for |aKey|.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] PresStatePtr GetState (in nsCString aKey);`
    const _GetState: () = ();

    /// ```text
    /// /**
    ///    * Remove the state object for |aKey|.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void RemoveState (in nsCString aKey);`
    const _RemoveState: () = ();

    /// ```text
    /// /**
    ///    * Check whether this history has any states in it
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] boolean HasStates ();`
    const _HasStates: () = ();

    /// ```text
    /// /**
    ///    * Sets whether this history can contain only scroll position history
    ///    * or all possible history
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void SetScrollPositionOnly (in constBool aFlag);`
    const _SetScrollPositionOnly: () = ();

    /// ```text
    /// /**
    ///    * Resets PresState::GetScrollState of all PresState objects to 0,0.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void ResetScrollState ();`
    const _ResetScrollState: () = ();

    /// ```text
    /// /**
    ///    * Get the contents of the layout history.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void GetContents (out boolean aScrollPositionOnly, out Array<ACString> aKeys, out Array<PresState> aStates);`
    const _GetContents: () = ();

    /// ```text
    /// /**
    ///    * Remove all the states and clear the scroll position only flag.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void Reset ();`
    const _Reset: () = ();

}



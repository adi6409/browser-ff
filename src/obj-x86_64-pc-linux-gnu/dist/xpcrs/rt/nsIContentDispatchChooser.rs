//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIContentDispatchChooser.idl
//


/// `interface nsIContentDispatchChooser : nsISupports`
///

/// ```text
/// /**
///  * This is used to ask a user what they would like to do with a given piece of
///  * content.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentDispatchChooser {
    vtable: *const nsIContentDispatchChooserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentDispatchChooser.
unsafe impl XpCom for nsIContentDispatchChooser {
    const IID: nsIID = nsID(0x456ca3b2, 0x02be, 0x4f97,
        [0x89, 0xa2, 0x08, 0xc0, 0x8d, 0x3a, 0xd8, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentDispatchChooser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentDispatchChooser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentDispatchChooserCoerce {
    /// Cheaply cast a value of this type from a `nsIContentDispatchChooser`.
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentDispatchChooserCoerce for nsIContentDispatchChooser {
    #[inline]
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self {
        v
    }
}

impl nsIContentDispatchChooser {
    /// Cast this `nsIContentDispatchChooser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentDispatchChooserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentDispatchChooser {
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
impl<T: nsISupportsCoerce> nsIContentDispatchChooserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentDispatchChooser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentDispatchChooser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentDispatchChooserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleURI (in nsIHandlerInfo aHandler, in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in BrowsingContext aBrowsingContext); */
    pub HandleURI: unsafe extern "system" fn (this: *const nsIContentDispatchChooser, aHandler: *const nsIHandlerInfo, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentDispatchChooser {

    /// ```text
    /// /**
    ///   * Opens the handler associated with the given resource.
    ///   * If the caller does not have permission or no handler is set, we ask the
    ///   * user to grant permission and pick a handler.
    ///   *
    ///   * @param aHander
    ///   *        The interface describing the details of how this content should or
    ///   *        can be handled.
    ///   * @param aURI
    ///   *        The URI of the resource that we are asking about.
    ///   * @param aTriggeringPrincipal
    ///   *        The principal making the request.
    ///   * @param aBrowsingContext
    ///   *        The browsing context where the load should happen.
    ///   */
    /// ```
    ///

    /// `void handleURI (in nsIHandlerInfo aHandler, in nsIURI aURI, in nsIPrincipal aTriggeringPrincipal, in BrowsingContext aBrowsingContext);`
    #[inline]
    pub unsafe fn HandleURI(&self, aHandler: *const nsIHandlerInfo, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).HandleURI)(self, aHandler, aURI, aTriggeringPrincipal, aBrowsingContext)
    }


}



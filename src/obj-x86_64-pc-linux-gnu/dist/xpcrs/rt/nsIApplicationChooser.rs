//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIApplicationChooser.idl
//


/// `interface nsIApplicationChooserFinishedCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationChooserFinishedCallback {
    vtable: *const nsIApplicationChooserFinishedCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationChooserFinishedCallback.
unsafe impl XpCom for nsIApplicationChooserFinishedCallback {
    const IID: nsIID = nsID(0x8144404d, 0xe6c7, 0x4861,
        [0xbc, 0xca, 0x47, 0xde, 0x91, 0x2e, 0xe8, 0x11]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationChooserFinishedCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationChooserFinishedCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationChooserFinishedCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationChooserFinishedCallback`.
    fn coerce_from(v: &nsIApplicationChooserFinishedCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationChooserFinishedCallbackCoerce for nsIApplicationChooserFinishedCallback {
    #[inline]
    fn coerce_from(v: &nsIApplicationChooserFinishedCallback) -> &Self {
        v
    }
}

impl nsIApplicationChooserFinishedCallback {
    /// Cast this `nsIApplicationChooserFinishedCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationChooserFinishedCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationChooserFinishedCallback {
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
impl<T: nsISupportsCoerce> nsIApplicationChooserFinishedCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationChooserFinishedCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationChooserFinishedCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationChooserFinishedCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void done (in nsIHandlerApp handlerApp); */
    pub Done: unsafe extern "system" fn (this: *const nsIApplicationChooserFinishedCallback, handlerApp: *const nsIHandlerApp) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationChooserFinishedCallback {


    /// `void done (in nsIHandlerApp handlerApp);`
    #[inline]
    pub unsafe fn Done(&self, handlerApp: *const nsIHandlerApp) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, handlerApp)
    }


}


/// `interface nsIApplicationChooser : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIApplicationChooser {
    vtable: *const nsIApplicationChooserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIApplicationChooser.
unsafe impl XpCom for nsIApplicationChooser {
    const IID: nsIID = nsID(0xf7a149da, 0x612a, 0x46ba,
        [0x8a, 0x2f, 0x54, 0x78, 0x6f, 0xc2, 0x87, 0x91]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIApplicationChooser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIApplicationChooser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIApplicationChooserCoerce {
    /// Cheaply cast a value of this type from a `nsIApplicationChooser`.
    fn coerce_from(v: &nsIApplicationChooser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIApplicationChooserCoerce for nsIApplicationChooser {
    #[inline]
    fn coerce_from(v: &nsIApplicationChooser) -> &Self {
        v
    }
}

impl nsIApplicationChooser {
    /// Cast this `nsIApplicationChooser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIApplicationChooserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIApplicationChooser {
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
impl<T: nsISupportsCoerce> nsIApplicationChooserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIApplicationChooser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIApplicationChooser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIApplicationChooserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy parent, in ACString title); */
    pub Init: unsafe extern "system" fn (this: *const nsIApplicationChooser, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void open (in ACString contentType, in nsIApplicationChooserFinishedCallback applicationChooserFinishedCallback); */
    pub Open: unsafe extern "system" fn (this: *const nsIApplicationChooser, contentType: *const ::nsstring::nsACString, applicationChooserFinishedCallback: *const nsIApplicationChooserFinishedCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIApplicationChooser {

    /// ```text
    /// /**
    ///   * Initialize the application chooser picker widget.  The application chooser
    ///   * is not valid until this method is called.
    ///   *
    ///   * @param      parent   nsIDOMWindow parent.  This dialog will be dependent
    ///   *                      on this parent. parent must be non-null.
    ///   * @param      title    The title for the file widget
    ///   *
    ///   */
    /// ```
    ///

    /// `void init (in mozIDOMWindowProxy parent, in ACString title);`
    #[inline]
    pub unsafe fn Init(&self, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, parent, title)
    }


    /// ```text
    /// /**
    ///    * Open application chooser dialog.
    ///    *
    ///    * @param    contentType   content type of file to open
    ///    * @param    applicationChooserFinishedCallback  callback fuction to run when dialog is closed
    ///    */
    /// ```
    ///

    /// `void open (in ACString contentType, in nsIApplicationChooserFinishedCallback applicationChooserFinishedCallback);`
    #[inline]
    pub unsafe fn Open(&self, contentType: *const ::nsstring::nsACString, applicationChooserFinishedCallback: *const nsIApplicationChooserFinishedCallback) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, contentType, applicationChooserFinishedCallback)
    }


}



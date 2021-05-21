//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIColorPicker.idl
//


/// `interface nsIColorPickerShownCallback : nsISupports`
///

/// ```text
/// /**
///  * nsIColorPicker is representing colors as strings because the internal
///  * representation will depend on the underlying backend.
///  * The format of the colors taken in input and returned will always follow the
///  * format of the <input type='color'> value as described in the HTML
///  * specifications.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIColorPickerShownCallback {
    vtable: *const nsIColorPickerShownCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIColorPickerShownCallback.
unsafe impl XpCom for nsIColorPickerShownCallback {
    const IID: nsIID = nsID(0xd2ce78d1, 0x40b5, 0x49d1,
        [0xb6, 0x6d, 0x58, 0x01, 0xfc, 0xb9, 0xa3, 0x85]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIColorPickerShownCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIColorPickerShownCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIColorPickerShownCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIColorPickerShownCallback`.
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIColorPickerShownCallbackCoerce for nsIColorPickerShownCallback {
    #[inline]
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self {
        v
    }
}

impl nsIColorPickerShownCallback {
    /// Cast this `nsIColorPickerShownCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIColorPickerShownCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIColorPickerShownCallback {
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
impl<T: nsISupportsCoerce> nsIColorPickerShownCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIColorPickerShownCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIColorPickerShownCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIColorPickerShownCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void update (in AString color); */
    pub Update: unsafe extern "system" fn (this: *const nsIColorPickerShownCallback, color: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void done (in AString color); */
    pub Done: unsafe extern "system" fn (this: *const nsIColorPickerShownCallback, color: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIColorPickerShownCallback {

    /// ```text
    /// /**
    ///   * Callback called when the color picker requests a color update.
    ///   * This callback can not be called after done() was called.
    ///   * When this callback is used, the consumer can assume that the color value has
    ///   * changed.
    ///   *
    ///   * @param  color  The new selected color value following the format specifed on
    ///   *                top of this file.
    ///   */
    /// ```
    ///

    /// `void update (in AString color);`
    #[inline]
    pub unsafe fn Update(&self, color: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Update)(self, color)
    }


    /// ```text
    /// /**
    ///   * Callback called when the color picker is dismissed.
    ///   * When this callback is used, the color might have changed or could stay the
    ///   * same.
    ///   * If the color has not changed, the color parameter will be the empty string.
    ///   *
    ///   * @param  color  The new selected color value following the format specifed on
    ///   *                top of this file or the empty string.
    ///   */
    /// ```
    ///

    /// `void done (in AString color);`
    #[inline]
    pub unsafe fn Done(&self, color: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, color)
    }


}


/// `interface nsIColorPicker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIColorPicker {
    vtable: *const nsIColorPickerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIColorPicker.
unsafe impl XpCom for nsIColorPicker {
    const IID: nsIID = nsID(0xde229d37, 0xa8a6, 0x46f1,
        [0x96, 0x9a, 0x0c, 0x1d, 0xe3, 0x3d, 0x0a, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIColorPicker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIColorPicker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIColorPickerCoerce {
    /// Cheaply cast a value of this type from a `nsIColorPicker`.
    fn coerce_from(v: &nsIColorPicker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIColorPickerCoerce for nsIColorPicker {
    #[inline]
    fn coerce_from(v: &nsIColorPicker) -> &Self {
        v
    }
}

impl nsIColorPicker {
    /// Cast this `nsIColorPicker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIColorPickerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIColorPicker {
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
impl<T: nsISupportsCoerce> nsIColorPickerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIColorPicker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIColorPicker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIColorPickerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor); */
    pub Init: unsafe extern "system" fn (this: *const nsIColorPicker, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsAString, initialColor: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void open (in nsIColorPickerShownCallback aColorPickerShownCallback); */
    pub Open: unsafe extern "system" fn (this: *const nsIColorPicker, aColorPickerShownCallback: *const nsIColorPickerShownCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIColorPicker {

    /// ```text
    /// /**
    ///   * Initialize the color picker widget. The color picker will not be shown until
    ///   * open() is called.
    ///   * If the backend doesn't support setting a title to the native color picker
    ///   * widget, the title parameter might be ignored.
    ///   * If the initialColor parameter does not follow the format specified on top of
    ///   * this file, the behavior will be unspecified. The initialColor could be the
    ///   * one used by the underlying backend or an arbitrary one. The backend could
    ///   * also assert.
    ///   *
    ///   * @param      parent       nsIDOMWindow parent. This dialog will be dependent
    ///   *                          on this parent. parent must be non-null.
    ///   * @param      title        The title for the color picker widget.
    ///   * @param      initialColor The color to show when the widget is opened. The
    ///   *                          parameter has to follow the format specified on top
    ///   *                          of this file.
    ///   */
    /// ```
    ///

    /// `void init (in mozIDOMWindowProxy parent, in AString title, in AString initialColor);`
    #[inline]
    pub unsafe fn Init(&self, parent: *const mozIDOMWindowProxy, title: *const ::nsstring::nsAString, initialColor: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, parent, title, initialColor)
    }


    /// ```text
    /// /**
    ///   * Opens the color dialog asynchrounously.
    ///   * The results are provided via the callback object.
    ///   */
    /// ```
    ///

    /// `void open (in nsIColorPickerShownCallback aColorPickerShownCallback);`
    #[inline]
    pub unsafe fn Open(&self, aColorPickerShownCallback: *const nsIColorPickerShownCallback) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, aColorPickerShownCallback)
    }


}



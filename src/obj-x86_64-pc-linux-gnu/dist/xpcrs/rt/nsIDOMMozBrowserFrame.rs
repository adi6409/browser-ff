//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/html/nsIDOMMozBrowserFrame.idl
//


/// `interface nsIDOMMozBrowserFrame : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMMozBrowserFrame {
    vtable: *const nsIDOMMozBrowserFrameVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMMozBrowserFrame.
unsafe impl XpCom for nsIDOMMozBrowserFrame {
    const IID: nsIID = nsID(0x4cafe116, 0x581b, 0x4194,
        [0xb0, 0xde, 0x7f, 0x02, 0x37, 0x8f, 0xc5, 0x1d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMMozBrowserFrame {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMMozBrowserFrame.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMMozBrowserFrameCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMMozBrowserFrame`.
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMMozBrowserFrameCoerce for nsIDOMMozBrowserFrame {
    #[inline]
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self {
        v
    }
}

impl nsIDOMMozBrowserFrame {
    /// Cast this `nsIDOMMozBrowserFrame` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMMozBrowserFrameCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMMozBrowserFrame {
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
impl<T: nsISupportsCoerce> nsIDOMMozBrowserFrameCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMozBrowserFrame) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMMozBrowserFrame
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMMozBrowserFrameVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] attribute boolean mozbrowser; */
    pub GetMozbrowser: unsafe extern "system" fn (this: *const nsIDOMMozBrowserFrame, aMozbrowser: *mut bool) -> ::nserror::nsresult,

    /* [infallible] attribute boolean mozbrowser; */
    pub SetMozbrowser: unsafe extern "system" fn (this: *const nsIDOMMozBrowserFrame, aMozbrowser: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMMozBrowserFrame {

    /// ```text
    /// /**
    ///    * <iframe> element may have the mozbrowser attribute.
    ///    *
    ///    * The mozbrowser attribute has no effect unless the <iframe> element is
    ///    * contained in a document privileged to create browser frames.
    ///    *
    ///    * An <iframe> element in a privileged document with the mozbrowser attribute
    ///    * emits a variety of events when various things happen inside the frame.
    ///    *
    ///    * This will be documented eventually, but for more information at the moment,
    ///    * see dom/browser-element/BrowserElement{Child,Parent}.js.
    ///    *
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean mozbrowser;`
    #[inline]
    pub unsafe fn GetMozbrowser(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetMozbrowser)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * <iframe> element may have the mozbrowser attribute.
    ///    *
    ///    * The mozbrowser attribute has no effect unless the <iframe> element is
    ///    * contained in a document privileged to create browser frames.
    ///    *
    ///    * An <iframe> element in a privileged document with the mozbrowser attribute
    ///    * emits a variety of events when various things happen inside the frame.
    ///    *
    ///    * This will be documented eventually, but for more information at the moment,
    ///    * see dom/browser-element/BrowserElement{Child,Parent}.js.
    ///    *
    ///    */
    /// ```
    ///

    /// `[infallible] attribute boolean mozbrowser;`
    #[inline]
    pub unsafe fn SetMozbrowser(&self, aMozbrowser: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetMozbrowser)(self, aMozbrowser)
    }


}



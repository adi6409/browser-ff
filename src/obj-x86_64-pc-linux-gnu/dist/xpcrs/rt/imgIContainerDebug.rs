//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIContainerDebug.idl
//


/// `interface imgIContainerDebug : nsISupports`
///

/// ```text
/// /**
///  * This interface is used in debug builds (and only there) in
///  * order to let automatic tests running JavaScript access
///  * internal state of imgContainers. This lets us test
///  * things like animation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIContainerDebug {
    vtable: *const imgIContainerDebugVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIContainerDebug.
unsafe impl XpCom for imgIContainerDebug {
    const IID: nsIID = nsID(0x52cbb839, 0x6e63, 0x4a70,
        [0xb2, 0x1a, 0x1d, 0xb4, 0xca, 0x70, 0x6c, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIContainerDebug {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIContainerDebug.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIContainerDebugCoerce {
    /// Cheaply cast a value of this type from a `imgIContainerDebug`.
    fn coerce_from(v: &imgIContainerDebug) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIContainerDebugCoerce for imgIContainerDebug {
    #[inline]
    fn coerce_from(v: &imgIContainerDebug) -> &Self {
        v
    }
}

impl imgIContainerDebug {
    /// Cast this `imgIContainerDebug` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIContainerDebugCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIContainerDebug {
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
impl<T: nsISupportsCoerce> imgIContainerDebugCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIContainerDebug) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIContainerDebug
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIContainerDebugVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t framesNotified; */
    pub GetFramesNotified: unsafe extern "system" fn (this: *const imgIContainerDebug, aFramesNotified: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIContainerDebug {

    /// ```text
    /// /**
    ///    * The # of frames this imgContainer has been notified about.
    ///    * That is equal to the # of times the animation timer has
    ///    * fired, and is usually equal to the # of frames actually
    ///    * drawn (but actual drawing might be disabled).
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t framesNotified;`
    #[inline]
    pub unsafe fn GetFramesNotified(&self, aFramesNotified: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFramesNotified)(self, aFramesNotified)
    }


}



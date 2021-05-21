//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleObjectAttributeChangedEvent.idl
//


/// `interface nsIAccessibleObjectAttributeChangedEvent : nsIAccessibleEvent`
///

/// ```text
/// /**
///  * Fired when an attribute of an accessible changes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleObjectAttributeChangedEvent {
    vtable: *const nsIAccessibleObjectAttributeChangedEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleObjectAttributeChangedEvent.
unsafe impl XpCom for nsIAccessibleObjectAttributeChangedEvent {
    const IID: nsIID = nsID(0xce41add2, 0x096e, 0x4606,
        [0xb1, 0xca, 0x74, 0x08, 0xc6, 0xd5, 0xb4, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleObjectAttributeChangedEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleObjectAttributeChangedEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleObjectAttributeChangedEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleObjectAttributeChangedEvent`.
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleObjectAttributeChangedEventCoerce for nsIAccessibleObjectAttributeChangedEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self {
        v
    }
}

impl nsIAccessibleObjectAttributeChangedEvent {
    /// Cast this `nsIAccessibleObjectAttributeChangedEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleObjectAttributeChangedEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleObjectAttributeChangedEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIAccessibleEventCoerce> nsIAccessibleObjectAttributeChangedEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleObjectAttributeChangedEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleObjectAttributeChangedEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleObjectAttributeChangedEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute AString changedAttribute; */
    pub GetChangedAttribute: unsafe extern "system" fn (this: *const nsIAccessibleObjectAttributeChangedEvent, aChangedAttribute: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleObjectAttributeChangedEvent {

    /// ```text
    /// /**
    ///    * Return the accessible attribute that changed.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString changedAttribute;`
    #[inline]
    pub unsafe fn GetChangedAttribute(&self, aChangedAttribute: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetChangedAttribute)(self, aChangedAttribute)
    }


}



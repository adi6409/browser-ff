//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/events/nsIDOMEventListener.idl
//


/// `interface nsIDOMEventListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMEventListener {
    vtable: *const nsIDOMEventListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMEventListener.
unsafe impl XpCom for nsIDOMEventListener {
    const IID: nsIID = nsID(0xdf31c120, 0xded6, 0x11d1,
        [0xbd, 0x85, 0x00, 0x80, 0x5f, 0x8a, 0xe3, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMEventListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMEventListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMEventListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMEventListener`.
    fn coerce_from(v: &nsIDOMEventListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMEventListenerCoerce for nsIDOMEventListener {
    #[inline]
    fn coerce_from(v: &nsIDOMEventListener) -> &Self {
        v
    }
}

impl nsIDOMEventListener {
    /// Cast this `nsIDOMEventListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMEventListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMEventListener {
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
impl<T: nsISupportsCoerce> nsIDOMEventListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMEventListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMEventListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMEventListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void handleEvent (in Event event); */
    pub HandleEvent: unsafe extern "system" fn (this: *const nsIDOMEventListener, event: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMEventListener {

    /// ```text
    /// /**
    ///  * The nsIDOMEventListener interface is a callback interface for
    ///  * listening to events in the Document Object Model.
    ///  *
    ///  * For more information on this interface please see
    ///  * http://www.w3.org/TR/DOM-Level-2-Events/
    ///  */
    /// /**
    ///    * This method is called whenever an event occurs of the type for which
    ///    * the EventListener interface was registered.
    ///    *
    ///    * @param   evt The Event contains contextual information about the
    ///    *              event. It also contains the stopPropagation and
    ///    *              preventDefault methods which are used in determining the
    ///    *              event's flow and default action.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void handleEvent (in Event event);`
    #[inline]
    pub unsafe fn HandleEvent(&self, event: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).HandleEvent)(self, event)
    }


}



//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIObserver.idl
//


/// `interface nsIObserver : nsISupports`
///

/// ```text
/// /**
///  * This interface is implemented by an object that wants
///  * to observe an event corresponding to a topic.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIObserver {
    vtable: *const nsIObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIObserver.
unsafe impl XpCom for nsIObserver {
    const IID: nsIID = nsID(0xdb242e01, 0xe4d9, 0x11d2,
        [0x9d, 0xde, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIObserver`.
    fn coerce_from(v: &nsIObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIObserverCoerce for nsIObserver {
    #[inline]
    fn coerce_from(v: &nsIObserver) -> &Self {
        v
    }
}

impl nsIObserver {
    /// Cast this `nsIObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIObserver {
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
impl<T: nsISupportsCoerce> nsIObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void observe (in nsISupports aSubject, in string aTopic, in wstring aData); */
    pub Observe: unsafe extern "system" fn (this: *const nsIObserver, aSubject: *const nsISupports, aTopic: *const libc::c_char, aData: *const i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIObserver {

    /// ```text
    /// /**
    ///     * Observe will be called when there is a notification for the
    ///     * topic |aTopic|.  This assumes that the object implementing
    ///     * this interface has been registered with an observer service
    ///     * such as the nsIObserverService.
    ///     *
    ///     * If you expect multiple topics/subjects, the impl is
    ///     * responsible for filtering.
    ///     *
    ///     * You should not modify, add, remove, or enumerate
    ///     * notifications in the implemention of observe.
    ///     *
    ///     * @param aSubject : Notification specific interface pointer.
    ///     * @param aTopic   : The notification topic or subject.
    ///     * @param aData    : Notification specific wide string.
    ///     *                    subject event.
    ///     */
    /// ```
    ///

    /// `void observe (in nsISupports aSubject, in string aTopic, in wstring aData);`
    #[inline]
    pub unsafe fn Observe(&self, aSubject: *const nsISupports, aTopic: *const libc::c_char, aData: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).Observe)(self, aSubject, aTopic, aData)
    }


}



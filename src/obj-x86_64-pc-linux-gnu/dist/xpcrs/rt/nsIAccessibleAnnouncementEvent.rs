//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleAnnouncementEvent.idl
//


/// `interface nsIAccessibleAnnouncementEvent : nsIAccessibleEvent`
///

/// ```text
/// /**
///  * Fired when announce() is called on the target accessible.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleAnnouncementEvent {
    vtable: *const nsIAccessibleAnnouncementEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleAnnouncementEvent.
unsafe impl XpCom for nsIAccessibleAnnouncementEvent {
    const IID: nsIID = nsID(0x8818e49c, 0x1286, 0x4fe6,
        [0xae, 0x82, 0x4d, 0x1b, 0x79, 0x5e, 0xc8, 0x8d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleAnnouncementEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleAnnouncementEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleAnnouncementEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleAnnouncementEvent`.
    fn coerce_from(v: &nsIAccessibleAnnouncementEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleAnnouncementEventCoerce for nsIAccessibleAnnouncementEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleAnnouncementEvent) -> &Self {
        v
    }
}

impl nsIAccessibleAnnouncementEvent {
    /// Cast this `nsIAccessibleAnnouncementEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleAnnouncementEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleAnnouncementEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleAnnouncementEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleAnnouncementEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleAnnouncementEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleAnnouncementEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute AString announcement; */
    pub GetAnnouncement: unsafe extern "system" fn (this: *const nsIAccessibleAnnouncementEvent, aAnnouncement: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short priority; */
    pub GetPriority: unsafe extern "system" fn (this: *const nsIAccessibleAnnouncementEvent, aPriority: *mut u16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleAnnouncementEvent {

    pub const POLITE: i64 = 0;


    pub const ASSERTIVE: i64 = 1;


    /// `readonly attribute AString announcement;`
    #[inline]
    pub unsafe fn GetAnnouncement(&self, aAnnouncement: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetAnnouncement)(self, aAnnouncement)
    }



    /// `readonly attribute unsigned short priority;`
    #[inline]
    pub unsafe fn GetPriority(&self, aPriority: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetPriority)(self, aPriority)
    }


}



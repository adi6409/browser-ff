//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgIScriptedNotificationObserver.idl
//


/// `interface imgIScriptedNotificationObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgIScriptedNotificationObserver {
    vtable: *const imgIScriptedNotificationObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgIScriptedNotificationObserver.
unsafe impl XpCom for imgIScriptedNotificationObserver {
    const IID: nsIID = nsID(0x10be55b3, 0x2029, 0x41a7,
        [0xa9, 0x75, 0x53, 0x8e, 0xfe, 0xd2, 0x50, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgIScriptedNotificationObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgIScriptedNotificationObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgIScriptedNotificationObserverCoerce {
    /// Cheaply cast a value of this type from a `imgIScriptedNotificationObserver`.
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgIScriptedNotificationObserverCoerce for imgIScriptedNotificationObserver {
    #[inline]
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self {
        v
    }
}

impl imgIScriptedNotificationObserver {
    /// Cast this `imgIScriptedNotificationObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgIScriptedNotificationObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgIScriptedNotificationObserver {
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
impl<T: nsISupportsCoerce> imgIScriptedNotificationObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &imgIScriptedNotificationObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgIScriptedNotificationObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgIScriptedNotificationObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void sizeAvailable (in imgIRequest aRequest); */
    pub SizeAvailable: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void frameUpdate (in imgIRequest aRequest); */
    pub FrameUpdate: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void frameComplete (in imgIRequest aRequest); */
    pub FrameComplete: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void loadComplete (in imgIRequest aRequest); */
    pub LoadComplete: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void decodeComplete (in imgIRequest aRequest); */
    pub DecodeComplete: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void discard (in imgIRequest aRequest); */
    pub Discard: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void isAnimated (in imgIRequest aRequest); */
    pub IsAnimated: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void hasTransparency (in imgIRequest aRequest); */
    pub HasTransparency: unsafe extern "system" fn (this: *const imgIScriptedNotificationObserver, aRequest: *const imgIRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgIScriptedNotificationObserver {


    /// `void sizeAvailable (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn SizeAvailable(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).SizeAvailable)(self, aRequest)
    }



    /// `void frameUpdate (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn FrameUpdate(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).FrameUpdate)(self, aRequest)
    }



    /// `void frameComplete (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn FrameComplete(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).FrameComplete)(self, aRequest)
    }



    /// `void loadComplete (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn LoadComplete(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).LoadComplete)(self, aRequest)
    }



    /// `void decodeComplete (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn DecodeComplete(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).DecodeComplete)(self, aRequest)
    }



    /// `void discard (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn Discard(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).Discard)(self, aRequest)
    }



    /// `void isAnimated (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn IsAnimated(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).IsAnimated)(self, aRequest)
    }



    /// `void hasTransparency (in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn HasTransparency(&self, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).HasTransparency)(self, aRequest)
    }


}



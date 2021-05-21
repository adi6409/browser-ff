//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/nsIDOMNavigatorUserMedia.idl
//


/// `interface nsIMediaDevice : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMediaDevice {
    vtable: *const nsIMediaDeviceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMediaDevice.
unsafe impl XpCom for nsIMediaDevice {
    const IID: nsIID = nsID(0xba3b2e08, 0x1c07, 0x4cd3,
        [0x88, 0x22, 0xf4, 0xd7, 0xe3, 0x5f, 0xf2, 0xae]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMediaDevice {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMediaDevice.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMediaDeviceCoerce {
    /// Cheaply cast a value of this type from a `nsIMediaDevice`.
    fn coerce_from(v: &nsIMediaDevice) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMediaDeviceCoerce for nsIMediaDevice {
    #[inline]
    fn coerce_from(v: &nsIMediaDevice) -> &Self {
        v
    }
}

impl nsIMediaDevice {
    /// Cast this `nsIMediaDevice` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMediaDeviceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMediaDevice {
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
impl<T: nsISupportsCoerce> nsIMediaDeviceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMediaDevice) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMediaDevice
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMediaDeviceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIMediaDevice, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIMediaDevice, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIMediaDevice, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString mediaSource; */
    pub GetMediaSource: unsafe extern "system" fn (this: *const nsIMediaDevice, aMediaSource: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString rawId; */
    pub GetRawId: unsafe extern "system" fn (this: *const nsIMediaDevice, aRawId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString groupId; */
    pub GetGroupId: unsafe extern "system" fn (this: *const nsIMediaDevice, aGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString rawGroupId; */
    pub GetRawGroupId: unsafe extern "system" fn (this: *const nsIMediaDevice, aRawGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean scary; */
    pub GetScary: unsafe extern "system" fn (this: *const nsIMediaDevice, aScary: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString rawName; */
    pub GetRawName: unsafe extern "system" fn (this: *const nsIMediaDevice, aRawName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMediaDevice {


    /// `readonly attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `readonly attribute AString mediaSource;`
    #[inline]
    pub unsafe fn GetMediaSource(&self, aMediaSource: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetMediaSource)(self, aMediaSource)
    }



    /// `readonly attribute AString rawId;`
    #[inline]
    pub unsafe fn GetRawId(&self, aRawId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRawId)(self, aRawId)
    }



    /// `readonly attribute AString groupId;`
    #[inline]
    pub unsafe fn GetGroupId(&self, aGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetGroupId)(self, aGroupId)
    }



    /// `readonly attribute AString rawGroupId;`
    #[inline]
    pub unsafe fn GetRawGroupId(&self, aRawGroupId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRawGroupId)(self, aRawGroupId)
    }



    /// `readonly attribute boolean scary;`
    #[inline]
    pub unsafe fn GetScary(&self, aScary: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetScary)(self, aScary)
    }



    /// `readonly attribute AString rawName;`
    #[inline]
    pub unsafe fn GetRawName(&self, aRawName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetRawName)(self, aRawName)
    }


}


